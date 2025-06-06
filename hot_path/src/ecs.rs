// This module contains the core ECS systems coordinator.
// The actual systems are now in the systems module following proper ECS architecture.
// Laws are systems (behavior) not entities (data).

use hecs::World;

/// Generic system runner - coordinates all ECS systems
/// Systems are the "laws of physics" that operate on entities
pub fn run_systems(world: &mut World, law_specs: &crate::systems::LawSpecifications) {
    // Run all physics systems using schema-defined specifications
    crate::systems::run_physics_systems(world, law_specs);
} 