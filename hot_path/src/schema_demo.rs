use hecs::World;
use crate::components_generated::{SchemaDecayComponent, SchemaEntityType, SchemaDisplayText, SchemaAge, SchemaComponentRegistry};
use crate::systems::LawSpecifications;
use chrono::Utc;

/// Demonstrate the new schema-first component system
pub fn demo_schema_components() {
    println!("🧪 Schema-First Component System Demo");
    println!("=====================================");
    
    // Create a world with schema-generated components
    let mut world = World::new();
    
    // Demo 1: Create entities with generated components
    println!("\n📋 Creating entities with schema-generated components:");
    
    // Create a memory entity using generated components
    let memory_entity = world.spawn((
        SchemaEntityType::new("memory".to_string()),
        SchemaDisplayText::new("A vivid childhood memory".to_string()),
        SchemaDecayComponent::new(0.0), // last_update will be set by system
    ));
    println!("  ✅ Created memory entity: {:?}", memory_entity);
    
    // Create an age tracker using generated components
    let age_entity = world.spawn((
        SchemaEntityType::new("age_tracker".to_string()),
        SchemaAge::new(Utc::now()),
        SchemaDisplayText::new("Person's age".to_string()),
    ));
    println!("  ✅ Created age entity: {:?}", age_entity);
    
    // Demo 2: Component introspection
    println!("\n🔍 Component Registry Information:");
    let all_components = SchemaComponentRegistry::get_all_components();
    for (name, version) in all_components {
        println!("  • {} v{}", name, version);
    }
    
    // Demo 3: Law-component relationships
    println!("\n⚖️ Law-Component Relationships:");
    let decay_components = SchemaComponentRegistry::get_components_for_law("decay");
    println!("  • decay law affects: {:?}", decay_components);
    
    // Demo 4: Component constants and metadata
    println!("\n📊 Component Metadata:");
    println!("  • SchemaDecayComponent mixins: {:?}", SchemaDecayComponent::MIXINS);
    println!("  • SchemaDecayComponent laws: {:?}", SchemaDecayComponent::AFFECTED_BY_LAWS);
    println!("  • SchemaAge mixins: {:?}", SchemaAge::MIXINS);
    println!("  • SchemaAge laws: {:?}", SchemaAge::AFFECTED_BY_LAWS);
    
    // Demo 5: Apply physics laws to generated components
    println!("\n⚡ Applying Physics Laws:");
    let law_specs = LawSpecifications::from_schema();
    
    // Apply decay to all compatible components
    let mut decay_applied = 0;
    for (_entity, (entity_type, decay)) in world.query_mut::<(&SchemaEntityType, &mut SchemaDecayComponent)>() {
        if law_specs.decay_law.applies_to.contains(&entity_type.entity_type) {
            println!("  • Applying decay to {} entity", entity_type.entity_type);
            decay.last_update = chrono::Utc::now().timestamp() as f32;
            decay_applied += 1;
        }
    }
    println!("  ✅ Applied decay to {} entities", decay_applied);
    
    // Demo 6: Query entities by schema-generated components
    println!("\n🔎 Querying Schema Entities:");
    for (entity, (entity_type, display_text)) in world.query::<(&SchemaEntityType, &SchemaDisplayText)>().iter() {
        println!("  • {:?}: {} - '{}'", entity, entity_type.entity_type, display_text.content);
    }
    
    println!("\n✨ Schema-first demo complete!");
}

/// Compare old vs new component systems
pub fn compare_component_systems() {
    println!("\n🔄 Component System Comparison");
    println!("==============================");
    
    // Old system (existing generated.rs)
    println!("\n📜 Old System (generated.rs):");
    println!("  • Generated from Python Pydantic schemas");
    println!("  • JSON intermediate format");
    println!("  • build.rs code generation");
    println!("  • Complex entity structures");
    
    // New system (components_generated.rs)
    println!("\n🆕 New System (YAML + Copier):");
    println!("  • Schema-first YAML definitions");
    println!("  • Jinja2 template generation");
    println!("  • Clean Hecs components");
    println!("  • Law-component relationships");
    println!("  • Component versioning");
    println!("  • Runtime introspection");
    
    println!("\n🏆 Benefits of New System:");
    println!("  ✅ Type-safe Hecs components");
    println!("  ✅ Clear law-component mapping");
    println!("  ✅ Version tracking");
    println!("  ✅ Mixins and metadata");
    println!("  ✅ Runtime component registry");
    println!("  ✅ Schema validation with Pydantic");
    
    println!("\n🔮 Both systems can coexist during migration!");
} 