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
mod law_executor;
mod schemas;
mod config;
mod graphql;

use common::GqlCommand;

// Helper function for UUID generation
fn uuid4() -> Uuid {
    Uuid::new_v4()
}

struct MemorySystem {
    world: World,
    command_receiver: Receiver<GqlCommand>,
    last_status_update: SystemTime,
    entity_count_history: Vec<usize>,
}

impl MemorySystem {
    fn new(rx: Receiver<GqlCommand>) -> Self {
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

        // Load predefined laws from schema
        Self::load_schema_laws(&mut world);

        println!("{}", "🧵 Memory System initialized with initial thread and schema laws".bright_green());

        Self {
            world,
            command_receiver: rx,
            last_status_update: SystemTime::now(),
            entity_count_history: vec![1],
        }
    }

    fn run(&mut self) {
        loop {
            // Process GraphQL commands
            self.process_commands();
            
            // Run ECS systems
            self.run_ecs_systems();
            
            // Show status updates every 5 seconds
            if self.last_status_update.elapsed().unwrap() > Duration::from_secs(5) {
                self.show_status();
                self.last_status_update = SystemTime::now();
            }
            
            // Sleep briefly to avoid busy-waiting
            thread::sleep(Duration::from_millis(100));
        }
    }

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

            }
        }
    }

    fn run_ecs_systems(&mut self) {
        // Run all schema-driven systems via law executor
        ecs::run_systems(&mut self.world);
    }

    fn show_status(&mut self) {
        let entity_count = self.count_entities_by_type();
        let total_entities = entity_count.values().sum::<usize>();
        
        self.entity_count_history.push(total_entities);
        if self.entity_count_history.len() > 10 {
            self.entity_count_history.remove(0);
        }

        println!("\n{}", "━━━ Memory System Status ━━━".bright_purple().bold());
        
        for (entity_type, count) in &entity_count {
            let icon = match entity_type.as_str() {
                "thread" => "🧵",
                "moment" => "✨", 
                "binding" => "🔗",
                "bond" => "🔗",
                "filament" => "🌱",
                "motif" => "🎨",
                "law" => "⚖️",
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

    /// Load predefined laws from the cold path schema
    fn load_schema_laws(world: &mut World) {
        // Decay Law - mathematical physics from cold path
        let decay_law = components::Law {
            name: "decay".to_string(),
            trigger: components::LawTrigger::OnTick,
            applies_to: vec!["filament".to_string(), "motif".to_string()],
            formula: "strength = strength * pow(0.5, time_elapsed / half_life)".to_string(),
            variables: vec!["strength".to_string(), "half_life".to_string(), "last_update".to_string()],
            constants: serde_json::Map::new(),
            constraints: Some({
                let mut constraints = serde_json::Map::new();
                let mut strength_constraint = serde_json::Map::new();
                strength_constraint.insert("min".to_string(), serde_json::json!(0.1));
                constraints.insert("strength".to_string(), serde_json::Value::Object(strength_constraint));
                constraints
            }),
        };
        
        world.spawn((
            decay_law,
            components::DisplayText("Physics: Exponential Decay".to_string()),
            components::EntityType("law".to_string()),
        ));

        // Resonance Law - mathematical physics from cold path
        let resonance_law = components::Law {
            name: "resonance".to_string(), 
            trigger: components::LawTrigger::OnAffinityMatch,
            applies_to: vec!["filament".to_string()],
            formula: "strength = min(strength * multiplier, max_strength) if strength > threshold else strength".to_string(),
            variables: vec!["strength".to_string()],
            constants: {
                let mut constants = serde_json::Map::new();
                constants.insert("threshold".to_string(), serde_json::json!(0.85));
                constants.insert("multiplier".to_string(), serde_json::json!(1.2));
                constants.insert("max_strength".to_string(), serde_json::json!(1.0));
                constants
            },
            constraints: None,
        };
        
        world.spawn((
            resonance_law,
            components::DisplayText("Physics: Resonance Amplification".to_string()),
            components::EntityType("law".to_string()),
        ));

        println!("{}", "⚖️  Loaded 2 physical laws (decay, resonance)".bright_purple());
    }
}

fn main() {
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

    // Spawn the GraphQL server in a separate thread
    let gql_sender = tx.clone();
    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(graphql::run_graphql_server(gql_sender));
    });

    // Initialize and run the memory system
    let mut memory_system = MemorySystem::new(rx);
    memory_system.run();
} 