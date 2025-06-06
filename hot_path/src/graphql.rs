use async_graphql::{
    EmptySubscription, Object, Schema, Context, SimpleObject,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
    Extension,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crossbeam_channel::Sender;
use crate::common::GqlCommand;
use hecs::World;
use std::sync::{Arc, Mutex};

#[derive(SimpleObject)]
pub struct EntityInfo {
    pub id: String,
    pub entity_type: String,
    pub display_text: String,
    pub created_at: String,
    pub strength: Option<f32>,
}

#[derive(SimpleObject)]
pub struct MemoryStats {
    pub total_entities: usize,
    pub entities_by_type: Vec<TypeCount>,
    pub active_laws: usize,
    pub memory_usage_mb: f64,
}

#[derive(SimpleObject)]
pub struct TypeCount {
    pub entity_type: String,
    pub count: usize,
}

#[derive(SimpleObject)]
pub struct ThreadWithMoments {
    pub thread_id: String,
    pub thread_name: String,
    pub thread_type: String,
    pub moments: Vec<EntityInfo>,
}

/// Root query object for the Familiar Memory API.
/// Provides read-only access to the memory simulation data.
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    /// Health check endpoint to verify the GraphQL service is running.
    /// 
    /// Returns: "ok" if the service is healthy
    /// 
    /// Example:
    /// ```graphql
    /// { health }
    /// ```
    async fn health(&self) -> &'static str {
        "ok"
    }

    /// Get comprehensive memory system statistics
    async fn memory_stats(&self, ctx: &Context<'_>) -> MemoryStats {
        let world = ctx.data::<Arc<Mutex<World>>>().unwrap();
        let world = world.lock().unwrap();
        
        let mut type_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut total = 0;
        let mut law_count = 0;

        for (_entity, entity_type) in world.query::<&crate::components::EntityType>().iter() {
            total += 1;
            *type_counts.entry(entity_type.0.clone()).or_insert(0) += 1;
            if entity_type.0 == "law" {
                law_count += 1;
            }
        }

        let entities_by_type = type_counts.into_iter()
            .map(|(entity_type, count)| TypeCount { entity_type, count })
            .collect();

        MemoryStats {
            total_entities: total,
            entities_by_type,
            active_laws: law_count,
            memory_usage_mb: 0.0, // TODO: Calculate actual memory usage
        }
    }

    /// Query entities by type with optional limit
    async fn entities_by_type(&self, ctx: &Context<'_>, entity_type: String, limit: Option<i32>) -> Vec<EntityInfo> {
        let world = ctx.data::<Arc<Mutex<World>>>().unwrap();
        let world = world.lock().unwrap();
        
        let mut entities = Vec::new();
        let limit = limit.unwrap_or(50) as usize;

        for (entity, (etype, display_text)) in world.query::<(&crate::components::EntityType, &crate::components::DisplayText)>().iter() {
            if etype.0 == entity_type && entities.len() < limit {
                let strength = world.get::<&crate::components::DecayComponent>(entity)
                    .map(|decay| decay.strength)
                    .ok();

                entities.push(EntityInfo {
                    id: format!("{:?}", entity),
                    entity_type: etype.0.clone(),
                    display_text: display_text.0.clone(),
                    created_at: "2024-01-01T00:00:00Z".to_string(), // TODO: Get real timestamp
                    strength,
                });
            }
        }

        entities
    }

    /// Get all threads with their associated moments
    async fn threads_with_moments(&self, ctx: &Context<'_>) -> Vec<ThreadWithMoments> {
        let world = ctx.data::<Arc<Mutex<World>>>().unwrap();
        let world = world.lock().unwrap();
        
        let mut threads = Vec::new();

        // Find all threads
        for (_entity, (etype, display_text, thread_type, thread_id)) in world.query::<(
            &crate::components::EntityType, 
            &crate::components::DisplayText,
            &crate::components::ThreadType,
            &crate::components::ThreadId
        )>().iter() {
            if etype.0 == "thread" {
                let mut moments = Vec::new();
                
                // Find moments for this thread
                for (moment_entity, (moment_type, moment_text, moment_thread_id)) in world.query::<(
                    &crate::components::EntityType,
                    &crate::components::DisplayText, 
                    &crate::components::ThreadId
                )>().iter() {
                    if moment_type.0 == "moment" && moment_thread_id.0 == thread_id.0 {
                        let strength = world.get::<&crate::components::DecayComponent>(moment_entity)
                            .map(|decay| decay.strength)
                            .ok();

                        moments.push(EntityInfo {
                            id: format!("{:?}", moment_entity),
                            entity_type: "moment".to_string(),
                            display_text: moment_text.0.clone(),
                            created_at: "2024-01-01T00:00:00Z".to_string(),
                            strength,
                        });
                    }
                }

                threads.push(ThreadWithMoments {
                    thread_id: thread_id.0.clone(),
                    thread_name: display_text.0.clone(),
                    thread_type: thread_type.0.clone(),
                    moments,
                });
            }
        }

        threads
    }

    /// Search entities by text content
    async fn search_entities(&self, ctx: &Context<'_>, query: String, limit: Option<i32>) -> Vec<EntityInfo> {
        let world = ctx.data::<Arc<Mutex<World>>>().unwrap();
        let world = world.lock().unwrap();
        
        let mut entities = Vec::new();
        let limit = limit.unwrap_or(20) as usize;
        let query_lower = query.to_lowercase();

        for (entity, (etype, display_text)) in world.query::<(&crate::components::EntityType, &crate::components::DisplayText)>().iter() {
            if display_text.0.to_lowercase().contains(&query_lower) && entities.len() < limit {
                let strength = world.get::<&crate::components::DecayComponent>(entity)
                    .map(|decay| decay.strength)
                    .ok();

                entities.push(EntityInfo {
                    id: format!("{:?}", entity),
                    entity_type: etype.0.clone(),
                    display_text: display_text.0.clone(),
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    strength,
                });
            }
        }

        entities
    }

    /// Get entities with strength above threshold (time-based filter)
    async fn strong_entities(&self, ctx: &Context<'_>, min_strength: f64, limit: Option<i32>) -> Vec<EntityInfo> {
        let world = ctx.data::<Arc<Mutex<World>>>().unwrap();
        let world = world.lock().unwrap();
        
        let mut entities = Vec::new();
        let limit = limit.unwrap_or(20) as usize;

        for (entity, (etype, display_text, decay)) in world.query::<(
            &crate::components::EntityType, 
            &crate::components::DisplayText,
            &crate::components::DecayComponent
        )>().iter() {
            if decay.strength >= min_strength as f32 && entities.len() < limit {
                entities.push(EntityInfo {
                    id: format!("{:?}", entity),
                    entity_type: etype.0.clone(),
                    display_text: display_text.0.clone(),
                    created_at: "2024-01-01T00:00:00Z".to_string(),
                    strength: Some(decay.strength),
                });
            }
        }

        entities.sort_by(|a, b| b.strength.partial_cmp(&a.strength).unwrap_or(std::cmp::Ordering::Equal));
        entities
    }
}

/// Root mutation object for the Familiar Memory API.
/// Provides write operations to create and modify memory entities.
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    /// Creates a new Moment entity scoped to a specific thread.
    /// 
    /// Moments represent individual memories or experiences that belong to exactly one thread.
    /// In the binding-centric model, moments are the primary carriers of content.
    /// 
    /// Arguments:
    /// - `text`: The textual content of the memory
    /// - `thread_id`: The ID of the thread this moment belongs to
    /// 
    /// Example:
    /// ```graphql
    /// mutation {
    ///   createMoment(text: "Alice laughs at Bob's joke", threadId: "Alice")
    /// }
    /// ```
    async fn create_moment(&self, ctx: &Context<'_>, text: String, thread_id: String) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::CreateMoment { text, thread_id }).is_ok()
    }

    /// Creates a new Thread entity (person, place, event, or concept).
    /// 
    /// Threads are memory categories that organize related moments. They can represent:
    /// - People (Alice, Bob, Grandma)
    /// - Places (Kitchen, Living Room) 
    /// - Events (Christmas Morning, Birthday Party)
    /// - Abstract concepts (Family Warmth, Childhood)
    /// 
    /// Arguments:
    /// - `name`: Unique identifier for the thread
    /// - `thread_type`: Category type (person, place, event, concept)
    /// 
    /// Example:
    /// ```graphql
    /// mutation {
    ///   createThread(name: "Alice", threadType: "person")
    /// }
    /// ```
    async fn create_thread(&self, ctx: &Context<'_>, name: String, thread_type: String) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::CreateThread { name, thread_type }).is_ok()
    }

    /// Creates a new Filament (interpretive data about a thread).
    /// 
    /// Filaments provide contextual interpretation and meaning layers for threads.
    /// They represent how a thread is understood or characterized.
    /// 
    /// Arguments:
    /// - `content`: The interpretive content or characterization
    /// - `thread_name`: The thread this filament describes
    /// 
    /// Example:
    /// ```graphql
    /// mutation {
    ///   createFilament(content: "always the family comedian", threadName: "Bob")
    /// }
    /// ```
    async fn create_filament(&self, ctx: &Context<'_>, content: String, thread_name: String) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::CreateFilament { content, thread_name }).is_ok()
    }

    /// Creates a new Motif (aggregated meaning pattern from moments).
    /// 
    /// Motifs represent emergent patterns that arise from multiple related moments.
    /// They capture recurring themes and meanings in the memory space.
    /// 
    /// Arguments:
    /// - `pattern`: The pattern name or description
    /// - `strength`: How strong this pattern is (0.0 to 1.0)
    /// 
    /// Example:
    /// ```graphql
    /// mutation {
    ///   createMotif(pattern: "family-warmth", strength: 0.9)
    /// }
    /// ```
    async fn create_motif(&self, ctx: &Context<'_>, pattern: String, strength: f32) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::CreateMotif { pattern, strength }).is_ok()
    }

    /// Creates a new Bond between two threads.
    /// 
    /// Bonds represent relationships and connections between different threads.
    /// They create convergence effects in the 3D visualization where threads are pulled together.
    /// 
    /// Arguments:
    /// - `thread1`: First thread in the relationship
    /// - `thread2`: Second thread in the relationship  
    /// - `affinity`: Strength of the relationship (0.0 to 1.0)
    /// 
    /// Example:
    /// ```graphql
    /// mutation {
    ///   createBond(thread1: "Alice", thread2: "Bob", affinity: 0.88)
    /// }
    /// ```
    async fn create_bond(&self, ctx: &Context<'_>, thread1: String, thread2: String, affinity: f32) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::CreateBond { thread1, thread2, affinity }).is_ok()
    }

    /// Creates a new Binding linking a moment to a secondary thread.
    /// 
    /// Bindings enable cross-thread connections in the binding-centric model.
    /// They allow moments to influence multiple threads and create convergence effects.
    /// 
    /// Arguments:
    /// - `moment_id`: The moment to bind
    /// - `thread_id`: The secondary thread to bind to
    /// 
    /// Example:
    /// ```graphql
    /// mutation {
    ///   createBinding(momentId: "123", threadId: "Kitchen")
    /// }
    /// ```
    async fn create_binding(&self, ctx: &Context<'_>, moment_id: String, thread_id: String) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::CreateBinding { moment_id, thread_id }).is_ok()
    }

    /// Update component strength for any entity (mutable component operation)
    async fn update_strength(&self, ctx: &Context<'_>, entity_id: String, new_strength: f32) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::UpdateStrength { entity_id, new_strength }).is_ok()
    }

    /// Update display text for any entity (mutable component operation)
    async fn update_display_text(&self, ctx: &Context<'_>, entity_id: String, new_text: String) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::UpdateDisplayText { entity_id, new_text }).is_ok()
    }

    /// Add a tag to an entity (append-only operation on immutable entity field)
    async fn add_entity_tag(&self, ctx: &Context<'_>, entity_id: String, tag: String) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::AddEntityTag { entity_id, tag }).is_ok()
    }

    /// Soft delete an entity (append-only operation - sets deleted_at timestamp)
    async fn soft_delete_entity(&self, ctx: &Context<'_>, entity_id: String) -> bool {
        let sender = ctx.data::<Sender<GqlCommand>>().unwrap();
        sender.send(GqlCommand::SoftDeleteEntity { entity_id }).is_ok()
    }
}

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

async fn graphql_handler(
    Extension(schema): Extension<AppSchema>, 
    req: GraphQLRequest
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Familiar Memory GraphiQL</title>
    <link rel="stylesheet" href="https://unpkg.com/graphiql@3.0.6/graphiql.min.css" />
    <style>
        body { margin: 0; overflow: hidden; height: 100vh; }
        #graphiql { height: 100vh; }
        .graphiql-container { height: 100vh !important; }
    </style>
</head>
<body>
    <div id="graphiql">Loading GraphiQL...</div>
    
    <script
        crossorigin
        src="https://unpkg.com/react@18/umd/react.development.js"
    ></script>
    <script
        crossorigin
        src="https://unpkg.com/react-dom@18/umd/react-dom.development.js"
    ></script>
    <script
        crossorigin
        src="https://unpkg.com/graphiql@3.0.6/graphiql.min.js"
    ></script>
    
    <script>
        const graphQLFetcher = graphiQLParams =>
            fetch('/', {
                method: 'post',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(graphiQLParams),
            })
            .then(response => response.json())
            .catch(() => response.text());

        const root = ReactDOM.createRoot(document.getElementById('graphiql'));
        root.render(
            React.createElement(GraphiQL, {
                fetcher: graphQLFetcher,
                defaultQuery: `# Welcome to Familiar Memory GraphiQL!
# 
# GraphiQL is an in-browser IDE for writing, validating, and
# testing GraphQL queries.
#
# Type queries into this side of the screen, and you will see 
# intelligent typeaheads aware of the current GraphQL type schema 
# and live syntax and validation errors highlighted within the text.
#
# GraphQL queries typically start with a "{" character. Lines that start
# with a # are ignored.
#
# Example query:
# Uncomment the line below to run a simple health check
#
# { health }
#
# Example mutations:
#
# mutation {
#   createThread(name: "Alice", threadType: "person")
#   createMoment(text: "A beautiful memory", threadId: "Alice")  
# }
#
# Keyboard shortcuts:
#  Prettify Query:  Shift-Ctrl-P (or press the prettify button above)
#
#  Merge Query (deprecated):  Shift-Ctrl-M (or press the merge button above)
#
#       Run Query:  Ctrl-Enter (or press the play button above)
#
#   Auto Complete:  Ctrl-Space (or just start typing)
#`,
                headerEditorEnabled: true,
                shouldPersistHeaders: true,
            })
        );
    </script>
</body>
</html>
    "#)
}

/// Runs the GraphQL server.
pub async fn run_graphql_server(sender: Sender<GqlCommand>, world: Arc<Mutex<World>>) {
    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(sender)
        .data(world)
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("🚀 GraphiQL IDE listening on http://{}", addr);

    let listener = TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
} 