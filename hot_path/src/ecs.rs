// This module contains the core ECS systems logic.
// Systems operate on entities and their components to drive the simulation.
// The law system has replaced hardcoded behaviors like decay.

use hecs::World;

/// Generic system runner - delegates to law_executor for all dynamic behaviors
/// Laws defined in the cold path schema now handle decay, resonance, etc.
pub fn run_systems(world: &mut World) {
    // All dynamic behaviors are now handled by the law system
    // This keeps the ECS clean and makes all game rules schema-driven
    crate::law_executor::execute_laws(world, crate::components::LawTrigger::OnTick);
} 