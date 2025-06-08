// GENERATED CODE — DO NOT EDIT
// Generated via Copier from cold path YAML schemas
use chrono::Utc;
use colored::*;
use hecs::World;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use uuid::Uuid;

mod gen {
    pub mod components;
    pub mod systems;
}

use gen::components::*;
use gen::systems::*;

// Helper function for UUID generation
fn uuid4() -> Uuid {
    Uuid::new_v4()
}

struct EcsMemorySystem {
    world: World,
    law_specifications: LawSpecifications,
    last_status_update: SystemTime,
    entity_count_history: Vec<usize>,
}

impl EcsMemorySystem {
    fn new() -> Self {
        let mut world = World::new();

        // Create demo entities using schema-generated components
        println!(
            "{}",
            "🏗️ Creating demo entities with schema components...".bright_blue()
        );

        // Create EntityType entity
        let entitytype_entity = world.spawn((EntityType {
            entity_type: "entitytype_entity_type".to_string(),
        },));
        println!("  ✅ Created EntityType entity: {:?}", entitytype_entity);

        // Create DisplayText entity
        let displaytext_entity = world.spawn((DisplayText {
            content: "displaytext_content".to_string(),
        },));
        println!("  ✅ Created DisplayText entity: {:?}", displaytext_entity);

        // Create DecayComponent entity
        let decaycomponent_entity = world.spawn((DecayComponent {
            strength: 1.0,
            half_life: 300.0,
            last_update: 0.0,
        },));
        println!(
            "  ✅ Created DecayComponent entity: {:?}",
            decaycomponent_entity
        );

        // Create TemporalPosition entity
        let temporalposition_entity = world.spawn((TemporalPosition {
            timestamp: chrono::Utc::now(),
            precision: "day".to_string(),
            temporal_coordinates: Vec::new(),
            time_zone_offset: 0,
        },));
        println!(
            "  ✅ Created TemporalPosition entity: {:?}",
            temporalposition_entity
        );

        // Create MemoryLayer entity
        let memorylayer_entity = world.spawn((MemoryLayer {
            layer_type: "working".to_string(),
            access_frequency: 0.0,
            last_accessed: chrono::Utc::now(),
            consolidation_status: "active".to_string(),
            redis_key: "memorylayer_redis_key".to_string(),
        },));
        println!("  ✅ Created MemoryLayer entity: {:?}", memorylayer_entity);

        // Create Age entity
        let age_entity = world.spawn((Age {
            age_days: 0,
            created_at: chrono::Utc::now(),
        },));
        println!("  ✅ Created Age entity: {:?}", age_entity);

        // Create Mood entity
        let mood_entity = world.spawn((Mood {
            mood_state: "neutral".to_string(),
            intensity: 0.0,
        },));
        println!("  ✅ Created Mood entity: {:?}", mood_entity);

        // Create ThreadType entity
        let threadtype_entity = world.spawn((ThreadType {
            thread_type: "threadtype_thread_type".to_string(),
        },));
        println!("  ✅ Created ThreadType entity: {:?}", threadtype_entity);

        // Create ThreadName entity
        let threadname_entity = world.spawn((ThreadName {
            name: "threadname_name".to_string(),
        },));
        println!("  ✅ Created ThreadName entity: {:?}", threadname_entity);

        // Create ThreadId entity
        let threadid_entity = world.spawn((ThreadId {
            id: "threadid_id".to_string(),
        },));
        println!("  ✅ Created ThreadId entity: {:?}", threadid_entity);

        // Load physics law specifications from schema
        let law_specifications = LawSpecifications::from_schema();

        println!(
            "{}",
            "⚖️ Loaded physics systems from schema definitions".bright_purple()
        );
        println!("{}", "🧵 ECS Memory System initialized".bright_green());

        Self {
            world,
            law_specifications,
            last_status_update: SystemTime::now(),
            entity_count_history: vec![],
        }
    }

    fn run(&mut self) {
        loop {
            // Run ECS systems (schema-driven physics)
            self.run_physics_systems();

            // Show status updates every 5 seconds
            if self.last_status_update.elapsed().unwrap() > Duration::from_secs(5) {
                self.show_status();
                self.last_status_update = SystemTime::now();
            }

            // Sleep briefly to avoid busy-waiting
            thread::sleep(Duration::from_millis(100));
        }
    }

    fn run_physics_systems(&mut self) {
        // Apply all schema-defined physics laws as ECS systems
        apply_all_physics_laws(&mut self.world, &self.law_specifications);
    }

    fn show_status(&mut self) {
        let entity_counts = self.count_entities_by_component();
        let total_entities = entity_counts.values().sum::<usize>();
        let system_stats = get_comprehensive_system_stats(&self.world, &self.law_specifications);

        self.entity_count_history.push(total_entities);
        if self.entity_count_history.len() > 10 {
            self.entity_count_history.remove(0);
        }

        println!(
            "\n{}",
            "━━━ Schema-Driven ECS Status ━━━".bright_purple().bold()
        );

        // Show physics systems status
        println!(
            "{} {} systems affecting {} entities",
            "⚖️".bright_purple(),
            system_stats.active_systems.to_string().bright_yellow(),
            system_stats
                .total_affected_entities
                .to_string()
                .bright_cyan()
        );

        // Show component counts from schema registry
        if let Some(count) = entity_counts.get("EntityType") {
            let icon = "📦";
            println!("{} EntityType: {}", icon, count.to_string().bright_yellow());
        }
        if let Some(count) = entity_counts.get("DisplayText") {
            let icon = "📦";
            println!(
                "{} DisplayText: {}",
                icon,
                count.to_string().bright_yellow()
            );
        }
        if let Some(count) = entity_counts.get("DecayComponent") {
            let icon = "⚖️";
            println!(
                "{} DecayComponent: {}",
                icon,
                count.to_string().bright_yellow()
            );
        }
        if let Some(count) = entity_counts.get("TemporalPosition") {
            let icon = "⚖️";
            println!(
                "{} TemporalPosition: {}",
                icon,
                count.to_string().bright_yellow()
            );
        }
        if let Some(count) = entity_counts.get("MemoryLayer") {
            let icon = "⚖️";
            println!(
                "{} MemoryLayer: {}",
                icon,
                count.to_string().bright_yellow()
            );
        }
        if let Some(count) = entity_counts.get("Age") {
            let icon = "⚖️";
            println!("{} Age: {}", icon, count.to_string().bright_yellow());
        }
        if let Some(count) = entity_counts.get("Mood") {
            let icon = "📦";
            println!("{} Mood: {}", icon, count.to_string().bright_yellow());
        }
        if let Some(count) = entity_counts.get("ThreadType") {
            let icon = "📦";
            println!("{} ThreadType: {}", icon, count.to_string().bright_yellow());
        }
        if let Some(count) = entity_counts.get("ThreadName") {
            let icon = "📦";
            println!("{} ThreadName: {}", icon, count.to_string().bright_yellow());
        }
        if let Some(count) = entity_counts.get("ThreadId") {
            let icon = "📦";
            println!("{} ThreadId: {}", icon, count.to_string().bright_yellow());
        }

        println!(
            "{} {}",
            "Total entities:".bright_white(),
            total_entities.to_string().bright_green().bold()
        );

        // Show law applications from schema

        println!("  📋 DecayComponent affected by: decay, resonance");

        println!("  📋 TemporalPosition affected by: temporal_drift");

        println!("  📋 MemoryLayer affected by: memory_consolidation");

        println!("  📋 Age affected by: decay");

        println!("{}", "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━".bright_purple());
    }

    fn count_entities_by_component(&self) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();

        // Use schema component registry to count entities
        let entitytype_count = self.world.query::<&EntityType>().iter().count();
        if entitytype_count > 0 {
            counts.insert("EntityType".to_string(), entitytype_count);
        }
        let displaytext_count = self.world.query::<&DisplayText>().iter().count();
        if displaytext_count > 0 {
            counts.insert("DisplayText".to_string(), displaytext_count);
        }
        let decaycomponent_count = self.world.query::<&DecayComponent>().iter().count();
        if decaycomponent_count > 0 {
            counts.insert("DecayComponent".to_string(), decaycomponent_count);
        }
        let temporalposition_count = self.world.query::<&TemporalPosition>().iter().count();
        if temporalposition_count > 0 {
            counts.insert("TemporalPosition".to_string(), temporalposition_count);
        }
        let memorylayer_count = self.world.query::<&MemoryLayer>().iter().count();
        if memorylayer_count > 0 {
            counts.insert("MemoryLayer".to_string(), memorylayer_count);
        }
        let age_count = self.world.query::<&Age>().iter().count();
        if age_count > 0 {
            counts.insert("Age".to_string(), age_count);
        }
        let mood_count = self.world.query::<&Mood>().iter().count();
        if mood_count > 0 {
            counts.insert("Mood".to_string(), mood_count);
        }
        let threadtype_count = self.world.query::<&ThreadType>().iter().count();
        if threadtype_count > 0 {
            counts.insert("ThreadType".to_string(), threadtype_count);
        }
        let threadname_count = self.world.query::<&ThreadName>().iter().count();
        if threadname_count > 0 {
            counts.insert("ThreadName".to_string(), threadname_count);
        }
        let threadid_count = self.world.query::<&ThreadId>().iter().count();
        if threadid_count > 0 {
            counts.insert("ThreadId".to_string(), threadid_count);
        }

        counts
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    // Check for different modes
    if args.len() > 1 {
        match args[1].as_str() {
            "--schema-test" => {
                schema_test_mode();
                return;
            }
            "--component-demo" => {
                component_demo_mode();
                return;
            }
            "--help" => {
                print_help();
                return;
            }
            _ => {}
        }
    }

    println!(
        "{}",
        "🧬 Schema-First ECS Memory System Starting..."
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "Generated from YAML schemas via Copier templates".bright_blue()
    );

    let mut system = EcsMemorySystem::new();
    system.run();
}

/// Schema component testing mode
fn schema_test_mode() {
    println!("{}", "🧪 Schema Component Test Mode".bright_yellow().bold());
    println!(
        "{}",
        "Testing all schema-generated components and physics laws".bright_white()
    );

    let mut world = World::new();
    let law_specs = LawSpecifications::from_schema();

    // Test creating entities with all schema components
    println!("\n📋 Testing EntityType component:");
    println!("  Version: {}", EntityType::VERSION);

    // Create test entity with default values
    let entity = world.spawn((EntityType {
        entity_type: "test".to_string(),
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing DisplayText component:");
    println!("  Version: {}", DisplayText::VERSION);

    // Create test entity with default values
    let entity = world.spawn((DisplayText {
        content: "test".to_string(),
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing DecayComponent component:");
    println!("  Version: {}", DecayComponent::VERSION);
    println!("  Mixins: {:?}", &["decayable"]);
    println!("  Affected by laws: {:?}", &["decay", "resonance"]);

    // Create test entity with default values
    let entity = world.spawn((DecayComponent {
        strength: 1.0,
        half_life: 300.0,
        last_update: 0.0,
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing TemporalPosition component:");
    println!("  Version: {}", TemporalPosition::VERSION);
    println!("  Affected by laws: {:?}", &["temporal_drift"]);

    // Create test entity with default values
    let entity = world.spawn((TemporalPosition {
        timestamp: chrono::Utc::now(),
        precision: "day".to_string(),
        temporal_coordinates: vec![1.0, 2.0, 3.0],
        time_zone_offset: 0,
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing MemoryLayer component:");
    println!("  Version: {}", MemoryLayer::VERSION);
    println!("  Affected by laws: {:?}", &["memory_consolidation"]);

    // Create test entity with default values
    let entity = world.spawn((MemoryLayer {
        layer_type: "working".to_string(),
        access_frequency: 0.0,
        last_accessed: chrono::Utc::now(),
        consolidation_status: "active".to_string(),
        redis_key: "test".to_string(),
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing Age component:");
    println!("  Version: {}", Age::VERSION);
    println!("  Mixins: {:?}", &["decayable"]);
    println!("  Affected by laws: {:?}", &["decay"]);

    // Create test entity with default values
    let entity = world.spawn((Age {
        age_days: 0,
        created_at: chrono::Utc::now(),
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing Mood component:");
    println!("  Version: {}", Mood::VERSION);

    // Create test entity with default values
    let entity = world.spawn((Mood {
        mood_state: "neutral".to_string(),
        intensity: 0.0,
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing ThreadType component:");
    println!("  Version: {}", ThreadType::VERSION);

    // Create test entity with default values
    let entity = world.spawn((ThreadType {
        thread_type: "test".to_string(),
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing ThreadName component:");
    println!("  Version: {}", ThreadName::VERSION);

    // Create test entity with default values
    let entity = world.spawn((ThreadName {
        name: "test".to_string(),
    },));
    println!("  ✅ Created test entity: {:?}", entity);
    println!("\n📋 Testing ThreadId component:");
    println!("  Version: {}", ThreadId::VERSION);

    // Create test entity with default values
    let entity = world.spawn((ThreadId {
        id: "test".to_string(),
    },));
    println!("  ✅ Created test entity: {:?}", entity);

    // Test physics systems
    println!("\n⚡ Testing physics systems:");
    apply_all_physics_laws(&mut world, &law_specs);

    let stats = get_comprehensive_system_stats(&world, &law_specs);
    println!(
        "  📊 Systems: {}, Entities: {}",
        stats.active_systems, stats.total_affected_entities
    );

    println!("\n✨ Schema test complete!");
}

/// Component demonstration mode
fn component_demo_mode() {
    println!(
        "{}",
        "🎨 Component Demonstration Mode".bright_magenta().bold()
    );

    // Show all schema-generated components
    let all_components = ComponentRegistry::get_all_components();
    println!("\n📋 All Schema Components:");
    for (name, version) in all_components {
        println!(
            "  • {} v{}",
            name.bright_white(),
            version.to_string().bright_yellow()
        );
    }

    // Show law-component relationships
    let decay_components = ComponentRegistry::get_components_for_law("decay");
    if !decay_components.is_empty() {
        println!("\n⚖️ Decay law affects:");
        for comp in decay_components {
            println!("  • {}", comp.bright_cyan());
        }
    }

    let resonance_components = ComponentRegistry::get_components_for_law("resonance");
    if !resonance_components.is_empty() {
        println!("\n⚖️ Resonance law affects:");
        for comp in resonance_components {
            println!("  • {}", comp.bright_cyan());
        }
    }

    println!("\n🎯 This demonstrates pure schema-to-code generation!");
}

/// Print help information
fn print_help() {
    println!("{}", "🧬 Schema-First ECS Familiar".bright_green().bold());
    println!();
    println!("{}", "USAGE:".bright_white().bold());
    println!("  cargo run                    # Start schema-driven ECS simulation");
    println!("  cargo run -- --schema-test   # Test all schema components and laws");
    println!("  cargo run -- --component-demo# Demo schema component registry");
    println!("  cargo run -- --help          # Show this help");
    println!();
    println!("{}", "FEATURES:".bright_white().bold());
    println!("  🧬 100% schema-generated components");
    println!("  ⚖️ Physics laws from YAML definitions");
    println!("  🎯 Type-safe Hecs ECS architecture");
    println!("  🔄 Hot path generated from cold path");
    println!();
    println!("{}", "SCHEMA GENERATION:".bright_white().bold());
    println!("  python3 generate_all.py     # Generate entire hot path from schemas");
    println!("  python3 validate_schemas.py # Validate YAML schema definitions");
}
