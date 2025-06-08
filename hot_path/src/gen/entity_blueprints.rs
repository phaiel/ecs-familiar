// GENERATED CODE — DO NOT EDIT
// Entity blueprints for common component combinations
use hecs::World;
use crate::gen::components::*;

/// Entity blueprints for spawning pre-configured entities
pub struct EntityBlueprints;

impl EntityBlueprints {

    /// Create a Memory entity - A memory entity with decay and temporal positioning
    pub fn spawn_memory(world: &mut World) -> hecs::Entity {
        // TODO: Implement Memory blueprint with components: EntityType, DisplayText, DecayComponent, TemporalPosition
        world.spawn(())
    }

    /// Create a Thread entity - A thread entity for organizing memories
    pub fn spawn_thread(world: &mut World) -> hecs::Entity {
        // TODO: Implement Thread blueprint with components: EntityType, DisplayText, ThreadType, ThreadName, ThreadId
        world.spawn(())
    }

    /// Create a Moment entity - A specific moment within a thread
    pub fn spawn_moment(world: &mut World) -> hecs::Entity {
        // TODO: Implement Moment blueprint with components: EntityType, DisplayText, DecayComponent, ThreadId
        world.spawn(())
    }

    /// Create a AgedEntity entity - An entity that ages over time
    pub fn spawn_agedentity(world: &mut World) -> hecs::Entity {
        // TODO: Implement AgedEntity blueprint with components: EntityType, DisplayText, Age, Mood
        world.spawn(())
    }

    /// Create a MemoryLayer entity - A memory layer for consolidation
    pub fn spawn_memorylayer(world: &mut World) -> hecs::Entity {
        // TODO: Implement MemoryLayer blueprint with components: EntityType, DisplayText, MemoryLayer
        world.spawn(())
    }

}
