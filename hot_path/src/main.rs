use std::thread;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use tokio::runtime::Runtime;
use crossbeam_channel::Receiver;
use hecs::World;
use uuid::Uuid;
use colored::*;

mod common;
mod components;
mod ecs;
mod generated;
mod systems;
mod schemas;
mod config;
mod graphql;
mod persistence;

use common::GqlCommand;

// Helper function for UUID generation
fn uuid4() -> Uuid {
    Uuid::new_v4()
}

struct MemorySystem {
    world: World,
    world_ref: std::sync::Arc<std::sync::Mutex<World>>,
    command_receiver: Receiver<GqlCommand>,
    last_status_update: SystemTime,
    entity_count_history: Vec<usize>,
    start_time: SystemTime,
    law_specifications: systems::LawSpecifications,
}

impl MemorySystem {
    fn new(rx: Receiver<GqlCommand>, world_ref: std::sync::Arc<std::sync::Mutex<World>>) -> Self {
        let mut world = World::new();
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        // Spawn initial thread for demonstration
        let initial_decay = components::DecayComponent {
            strength: 1.0,
            half_life: 1800.0, // 30 minutes - very stable initial thread
            last_update: current_time,
        };
        
        let initial_thread = components::Thread {
            base: components::BaseEntity {
                id: Some(uuid4()),
                org_id: uuid4(),
                owner_id: uuid4(),
                created_at: Some(chrono::Utc::now()),
                updated_at: None,
                deleted_at: None,
                tags: vec!["initial".to_string()],
                component_ids: vec![],
                sub_type: Some("thread".to_string()),
                visibility: components::Visibility::Private,
                security_level: 0,
                access_scope: vec![components::AccessScope::View],
                version: 1,
                parent_version: None,
            },
        };
        
        world.spawn((
            initial_thread,
            components::DisplayText("Memory Lane".to_string()), 
            components::ThreadType("pathway".to_string()),
            components::EntityType("thread".to_string()), 
            initial_decay
        ));

        // Load law specifications from schema (not as entities, but as system configs)
        let law_specifications = systems::LawSpecifications::from_schema();

        // Note: World sync will be handled differently since World doesn't implement Clone
        // We'll implement entity sharing through queries instead

        println!("{}", "⚖️  Loaded 2 physics systems (decay, resonance)".bright_purple());
        println!("{}", "🧵 Memory System initialized with initial thread and physics systems".bright_green());

        Self {
            world,
            world_ref,
            command_receiver: rx,
            last_status_update: SystemTime::now(),
            entity_count_history: vec![1], // Just the initial thread
            start_time: SystemTime::now(),
            law_specifications,
        }
    }

    fn run(&mut self) {
        loop {
            // Process GraphQL commands
            self.process_commands();
            
            // Run ECS systems
            self.run_ecs_systems();
            
            // Sync world state for GraphQL queries (lightweight approach)
            self.sync_world_state();
            
            // Show status updates every 5 seconds
            if self.last_status_update.elapsed().unwrap() > Duration::from_secs(5) {
                self.show_status();
                self.last_status_update = SystemTime::now();
            }
            
            // Sleep briefly to avoid busy-waiting
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn sync_world_state(&self) {
        // Simple approach: serialize essential data for GraphQL queries
        if let Ok(mut shared_world) = self.world_ref.try_lock() {
            // Clear and rebuild shared world with current entities
            shared_world.clear();
            
            // Copy entities with essential components for debugging
            for (entity, (entity_type, display_text)) in self.world.query::<(&components::EntityType, &components::DisplayText)>().iter() {
                let decay = self.world.get::<&components::DecayComponent>(entity);
                
                // Create new entity in shared world with all components at once
                if let Ok(decay_comp) = decay {
                    shared_world.spawn((
                        components::EntityType(entity_type.0.clone()),
                        components::DisplayText(display_text.0.clone()),
                        components::DecayComponent {
                            strength: decay_comp.strength,
                            half_life: decay_comp.half_life,
                            last_update: decay_comp.last_update,
                        },
                    ));
                } else {
                    shared_world.spawn((
                        components::EntityType(entity_type.0.clone()),
                        components::DisplayText(display_text.0.clone()),
                    ));
                }
            }
        }
    }

    // Note: Removed sync_world_ref since World doesn't implement Clone
    // TODO: Implement proper entity data sharing mechanism for GraphQL queries

    fn process_commands(&mut self) {
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        for cmd in self.command_receiver.try_iter() {
            match cmd {
                GqlCommand::CreateMoment { text, thread_id } => {
                    let moment = components::Moment {
                        base: components::BaseEntity {
                            id: Some(uuid4()),
                            org_id: uuid4(),
                            owner_id: uuid4(),
                            created_at: Some(chrono::Utc::now()),
                            updated_at: None,
                            deleted_at: None,
                            tags: vec!["moment".to_string()],
                            component_ids: vec![],
                            sub_type: Some("moment".to_string()),
                            visibility: components::Visibility::Private,
                            security_level: 0,
                            access_scope: vec![components::AccessScope::View],
                            version: 1,
                            parent_version: None,
                        },
                        thread_id: thread_id.parse().unwrap_or_default(),
                        author_id: uuid4(),
                        binding_hint: None,
                        binding_id: None,
                        cardinality: None,
                    };
                    let decay = components::DecayComponent {
                        strength: 1.0,
                        half_life: 300.0, // 5 minutes
                        last_update: current_time,
                    };
                    self.world.spawn((
                        moment,
                        components::DisplayText(text.clone()), 
                        components::ThreadId(thread_id.clone()), 
                        components::EntityType("moment".to_string()), 
                        decay
                    ));
                    println!("{} {}", "✨ Created moment:".bright_yellow(), text.bright_white());
                }
                GqlCommand::CreateThread { name, thread_type } => {
                    let thread = components::Thread {
                        base: components::BaseEntity {
                            id: Some(uuid4()),
                            org_id: uuid4(),
                            owner_id: uuid4(),
                            created_at: Some(chrono::Utc::now()),
                            updated_at: None,
                            deleted_at: None,
                            tags: vec![thread_type.clone()],
                            component_ids: vec![],
                            sub_type: Some("thread".to_string()),
                            visibility: components::Visibility::Private,
                            security_level: 0,
                            access_scope: vec![components::AccessScope::View],
                            version: 1,
                            parent_version: None,
                        },
                    };
                    let decay = components::DecayComponent {
                        strength: 1.0,
                        half_life: 600.0, // 10 minutes
                        last_update: current_time,
                    };
                    self.world.spawn((
                        thread,
                        components::DisplayText(name.clone()), 
                        components::ThreadType(thread_type.clone()), 
                        components::ThreadId(name.clone()),
                        components::EntityType("thread".to_string()), 
                        decay
                    ));
                    println!("{} {} ({})", "🧵 Created thread:".bright_blue(), name.bright_white(), thread_type.bright_cyan());
                }
                GqlCommand::CreateFilament { content, thread_name } => {
                    let filament = components::Filament {
                        base: components::BaseEntity {
                            id: Some(uuid4()),
                            org_id: uuid4(),
                            owner_id: uuid4(),
                            created_at: Some(chrono::Utc::now()),
                            updated_at: None,
                            deleted_at: None,
                            tags: vec!["filament".to_string()],
                            component_ids: vec![],
                            sub_type: Some("filament".to_string()),
                            visibility: components::Visibility::Private,
                            security_level: 0,
                            access_scope: vec![components::AccessScope::View],
                            version: 1,
                            parent_version: None,
                        },
                    };
                    let decay = components::DecayComponent {
                        strength: 1.0,
                        half_life: 45.0,
                        last_update: current_time,
                    };
                    self.world.spawn((
                        filament,
                        components::DisplayText(content.clone()), 
                        components::ThreadName(thread_name.clone()), 
                        components::EntityType("filament".to_string()), 
                        decay
                    ));
                    println!("{} {} on {}", "🌱 Created filament:".bright_green(), content.bright_white(), thread_name.bright_cyan());
                }
                GqlCommand::CreateMotif { pattern, strength } => {
                    let motif = components::Motif {
                        base: components::BaseEntity {
                            id: Some(uuid4()),
                            org_id: uuid4(),
                            owner_id: uuid4(),
                            created_at: Some(chrono::Utc::now()),
                            updated_at: None,
                            deleted_at: None,
                            tags: vec!["motif".to_string()],
                            component_ids: vec![],
                            sub_type: Some("motif".to_string()),
                            visibility: components::Visibility::Private,
                            security_level: 0,
                            access_scope: vec![components::AccessScope::View],
                            version: 1,
                            parent_version: None,
                        },
                    };
                    let decay = components::DecayComponent {
                        strength: strength.max(0.1),
                        half_life: 90.0,
                        last_update: current_time,
                    };
                    self.world.spawn((
                        motif,
                        components::DisplayText(pattern.clone()), 
                        components::EntityType("motif".to_string()), 
                        decay
                    ));
                    println!("{} {} (strength: {})", "🎨 Created motif:".bright_magenta(), pattern.bright_white(), strength.to_string().bright_yellow());
                }
                GqlCommand::CreateBond { thread1, thread2, affinity } => {
                    let bond = components::Bond {
                        base: components::BaseEntity {
                            id: Some(uuid4()),
                            org_id: uuid4(),
                            owner_id: uuid4(),
                            created_at: Some(chrono::Utc::now()),
                            updated_at: None,
                            deleted_at: None,
                            tags: vec!["bond".to_string(), thread1.clone(), thread2.clone()],
                            component_ids: vec![],
                            sub_type: Some("bond".to_string()),
                            visibility: components::Visibility::Private,
                            security_level: 0,
                            access_scope: vec![components::AccessScope::View],
                            version: 1,
                            parent_version: None,
                        },
                        thread_ids: vec![],
                        affinity_score: affinity as f64,
                        bond_strength: (affinity * 0.8) as f64,
                        component_context: vec![],
                    };
                    let decay = components::DecayComponent {
                        strength: affinity,
                        half_life: 120.0,
                        last_update: current_time,
                    };
                    self.world.spawn((
                        bond, 
                        components::DisplayText(format!("{} ⟷ {}", thread1, thread2)), 
                        components::EntityType("bond".to_string()), 
                        decay
                    ));
                    println!("{} {} ⟷ {} (affinity: {})", "🔗 Created bond:".bright_red(), thread1.bright_white(), thread2.bright_white(), affinity.to_string().bright_yellow());
                }
                GqlCommand::CreateBinding { moment_id, thread_id } => {
                    let thread_uuid = thread_id.parse().unwrap_or_default();
                    let moment_uuid = moment_id.parse().unwrap_or_default();
                    
                    let binding_point = components::BindingPoint::new(
                        thread_uuid,
                        moment_uuid,
                        components::Cardinality::Actor
                    );
                    
                    let binding = components::Binding {
                        base: components::BaseEntity {
                            id: Some(uuid4()),
                            org_id: uuid4(),
                            owner_id: uuid4(),
                            created_at: Some(chrono::Utc::now()),
                            updated_at: None,
                            deleted_at: None,
                            tags: vec!["binding".to_string()],
                            component_ids: vec![],
                            sub_type: Some("binding".to_string()),
                            visibility: components::Visibility::Private,
                            security_level: 0,
                            access_scope: vec![components::AccessScope::View],
                            version: 1,
                            parent_version: None,
                        },
                        points: vec![binding_point],
                        thread_ids: vec![thread_uuid],
                    };
                    let decay = components::DecayComponent {
                        strength: 0.8,
                        half_life: 60.0,
                        last_update: current_time,
                    };
                    self.world.spawn((
                        binding,
                        components::DisplayText(format!("Binding {} → {}", moment_id, thread_id)),
                        components::EntityType("binding".to_string()),
                        decay
                    ));
                    println!("{} {} → {}", "🔗 Created binding:".bright_cyan(), moment_id[0..8].bright_white(), thread_id[0..8].bright_white());
                }
                GqlCommand::UpdateStrength { entity_id, new_strength } => {
                    // Parse entity ID (for now just log, real implementation would find entity by ID)
                    println!("{} {} to {}", "⚡ Update strength:".bright_yellow(), entity_id[0..8].bright_white(), new_strength.to_string().bright_green());
                    
                    // TODO: Implement entity lookup by ID and component mutation
                    // This requires adding entity ID tracking to components
                }
                GqlCommand::UpdateDisplayText { entity_id, new_text } => {
                    println!("{} {} to '{}'", "📝 Update text:".bright_yellow(), entity_id[0..8].bright_white(), new_text.bright_white());
                    
                    // TODO: Implement entity lookup by ID and component mutation
                }
                GqlCommand::AddEntityTag { entity_id, tag } => {
                    println!("{} {} with tag '{}'", "🏷️ Add tag:".bright_yellow(), entity_id[0..8].bright_white(), tag.bright_cyan());
                    
                    // TODO: Implement append-only tag addition to BaseEntity
                }
                GqlCommand::SoftDeleteEntity { entity_id } => {
                    println!("{} {}", "🗑️ Soft delete:".bright_red(), entity_id[0..8].bright_white());
                    
                    // TODO: Implement soft deletion by setting deleted_at timestamp
                }
            }
        }
    }

    fn run_ecs_systems(&mut self) {
        // Run all ECS systems (which delegate to physics systems)
        ecs::run_systems(&mut self.world, &self.law_specifications);
    }

    fn show_status(&mut self) {
        let entity_count = self.count_entities_by_type();
        let total_entities = entity_count.values().sum::<usize>();
        let system_stats = systems::get_system_stats(&self.world, &self.law_specifications);
        
        self.entity_count_history.push(total_entities);
        if self.entity_count_history.len() > 10 {
            self.entity_count_history.remove(0);
        }

        println!("\n{}", "━━━ Memory System Status ━━━".bright_purple().bold());
        
        // Show physics systems status
        println!("{} {} systems affecting {} entities", 
            "⚖️".bright_purple(), 
            system_stats.active_systems.to_string().bright_yellow(),
            system_stats.affected_entities.to_string().bright_cyan()
        );
        
        // Show entity counts (excluding law entities since laws are now systems)
        for (entity_type, count) in &entity_count {
            let icon = match entity_type.as_str() {
                "thread" => "🧵",
                "moment" => "✨", 
                "binding" => "🔗",
                "bond" => "🔗",
                "filament" => "🌱",
                "motif" => "🎨",
                _ => "📦",
            };
            println!("{} {}: {}", icon, entity_type.bright_white(), count.to_string().bright_yellow());
        }
        
        println!("{} {}", "Total entities:".bright_white(), total_entities.to_string().bright_green().bold());
        
        // Show trend
        if self.entity_count_history.len() >= 2 {
            let prev = self.entity_count_history[self.entity_count_history.len() - 2];
            let curr = total_entities;
            let trend = match curr.cmp(&prev) {
                std::cmp::Ordering::Greater => format!("↗ +{}", curr - prev).bright_green(),
                std::cmp::Ordering::Less => format!("↘ -{}", prev - curr).bright_red(),
                std::cmp::Ordering::Equal => "→ stable".bright_blue(),
            };
            println!("{} {}", "Trend:".bright_white(), trend);
        }
        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_purple());
    }

    fn count_entities_by_type(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        
        for (_entity, entity_type) in self.world.query::<&components::EntityType>().iter() {
            *counts.entry(entity_type.0.clone()).or_insert(0) += 1;
        }
        
        counts
    }

    // Note: Law loading is now handled by systems::LawSpecifications::from_schema()
    // Laws are no longer entities - they are system configurations
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    // Check for different modes
    if args.len() > 1 {
        match args[1].as_str() {
            "--debug" => {
                debug_mode();
                return;
            }
            "--schema-demo" => {
                schema_demo_mode();
                return;
            }
            "--help" => {
                print_help();
                return;
            }
            _ => {}
        }
    }
    
    println!("{}", "🧵 Familiar Memory System Starting...".bright_green().bold());
    println!("{}", "🚀 GraphiQL IDE will be available at http://127.0.0.1:8000".bright_blue());

    // Load configuration  
    let _settings = config::Settings::new().unwrap_or_else(|e| {
        eprintln!("Failed to load settings: {}", e);
        // Create a basic default settings
        config::Settings {
            api_key: "default_key".to_string(),
        }
    });

    // Set up GraphQL command channel
    let (tx, rx) = crossbeam_channel::unbounded();

    // Create shared world reference for GraphQL queries
    let world_ref = std::sync::Arc::new(std::sync::Mutex::new(World::new()));
    let world_for_gql = world_ref.clone();

    // Spawn the GraphQL server in a separate thread
    let gql_sender = tx.clone();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(graphql::run_graphql_server(gql_sender, world_for_gql));
    });

    // Initialize and run the memory system
    let mut memory_system = MemorySystem::new(rx, world_ref);
    memory_system.run();
}

/// 🐛 DEBUG MODE: Simple command-line world inspector
fn debug_mode() {
    println!("{}", "🐛 Debug Mode - ECS World Inspector".bright_yellow().bold());
    
    // Create a simple world with test data
    let mut world = World::new();
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    
    // Add some test entities
    world.spawn((
        components::EntityType("thread".to_string()),
        components::DisplayText("Debug Thread".to_string()),
        components::DecayComponent { strength: 1.0, half_life: 600.0, last_update: current_time },
    ));
    
    world.spawn((
        components::EntityType("moment".to_string()),
        components::DisplayText("Debug Memory".to_string()),
        components::DecayComponent { strength: 0.8, half_life: 300.0, last_update: current_time },
    ));
    
    world.spawn((
        components::EntityType("filament".to_string()),
        components::DisplayText("Rapid Decay Thought".to_string()),
        components::DecayComponent { strength: 0.9, half_life: 45.0, last_update: current_time },
    ));
    
    // Display world state
    println!("\n{}", "━━━ ECS World State ━━━".bright_purple());
    
    let mut entity_count = 0;
    for (entity, (etype, display_text, decay)) in world.query::<(
        &components::EntityType,
        &components::DisplayText,
        &components::DecayComponent
    )>().iter() {
        entity_count += 1;
        let icon = match etype.0.as_str() {
            "thread" => "🧵",
            "moment" => "✨", 
            "filament" => "🌱",
            _ => "📦",
        };
        
        println!("{} {} {} (strength: {:.2}, half-life: {}s)", 
            icon, 
            format!("Entity-{:?}", entity).bright_blue(),
            display_text.0.bright_white(),
            decay.strength.to_string().bright_yellow(),
            decay.half_life.to_string().bright_cyan()
        );
    }
    
    println!("\n{} {}", "Total entities:".bright_white(), entity_count.to_string().bright_green());
    
    // Show law specifications
    let law_specs = systems::LawSpecifications::from_schema();
    println!("\n{}", "━━━ Physics Laws ━━━".bright_purple());
    println!("🔄 Decay applies to: {:?}", law_specs.decay_law.applies_to);
    println!("⚡ Resonance threshold: {}", law_specs.resonance_law.threshold);
    
    println!("\n{}", "🚀 Run with GraphQL interface: cargo run".bright_green());
    println!("{}", "📊 GraphiQL available at: http://127.0.0.1:8000".bright_blue());
}

/// 🧪 SCHEMA DEMO MODE: Demonstrate schema-first component system
fn schema_demo_mode() {
    println!("{}", "🧪 ECS Familiar - Schema-First Demo".bright_yellow().bold());
    println!("{}", "This demonstrates the new YAML + Copier component generation system".bright_white());
    
    // Run the schema demos
    if let Err(e) = std::panic::catch_unwind(|| {
        familiar_hot_path::schema_demo::demo_schema_components();
        familiar_hot_path::schema_demo::compare_component_systems();
    }) {
        println!("{}", format!("❌ Demo error: {:?}", e).bright_red());
        println!("{}", "Make sure components_generated.rs exists by running: python3 generate_components.py".bright_yellow());
    }
}

/// Print help information
fn print_help() {
    println!("{}", "🧵 ECS Familiar - Schema-Driven Memory Simulation".bright_green().bold());
    println!();
    println!("{}", "USAGE:".bright_white().bold());
    println!("  cargo run                 # Start memory system with GraphQL interface");
    println!("  cargo run -- --debug      # Debug mode - inspect world state");  
    println!("  cargo run -- --schema-demo# Demo the new schema-first component system");
    println!("  cargo run -- --help       # Show this help");
    println!();
    println!("{}", "FEATURES:".bright_white().bold());
    println!("  🌐 GraphQL API at http://127.0.0.1:8000");
    println!("  📊 Real-time world debugging");
    println!("  ⚖️  Schema-driven physics laws");
    println!("  🧬 Type-safe component generation");
    println!("  🔄 Hot/cold path architecture");
    println!();
    println!("{}", "SCHEMA GENERATION:".bright_white().bold());
    println!("  python3 generate_components.py  # Generate Rust components from YAML");
    println!("  python3 cold_path/schema_integration.py  # Test Pydantic integration");
} 