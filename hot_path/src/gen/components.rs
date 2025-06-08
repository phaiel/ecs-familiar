// GENERATED CODE — DO NOT EDIT
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct EntityType {
    pub entity_type: String,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl EntityType {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct DisplayText {
    pub content: String,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl DisplayText {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct DecayComponent {
    pub strength: f32,
    pub half_life: f32,
    pub last_update: f32,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl DecayComponent {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct TemporalPosition {
    pub timestamp: DateTime<Utc>,
    pub precision: String,
    pub temporal_coordinates: Vec<f32>,
    pub time_zone_offset: i32,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl TemporalPosition {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct MemoryLayer {
    pub layer_type: String,
    pub access_frequency: f32,
    pub last_accessed: DateTime<Utc>,
    pub consolidation_status: String,
    pub redis_key: String,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl MemoryLayer {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct Age {
    pub age_days: i32,
    pub created_at: DateTime<Utc>,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl Age {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct Mood {
    pub mood_state: String,
    pub intensity: f32,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl Mood {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct ThreadType {
    pub thread_type: String,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl ThreadType {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct ThreadName {
    pub name: String,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl ThreadName {
    pub const VERSION: i32 = 1;
}

#[derive(Debug, Clone)]
pub struct ThreadId {
    pub id: String,
}

// Hecs automatically implements Component for Send + Sync + 'static types

impl ThreadId {
    pub const VERSION: i32 = 1;
}

/// Component registry for runtime introspection
pub struct ComponentRegistry;

impl ComponentRegistry {
    /// Get all component names and versions
    pub fn get_all_components() -> Vec<(&'static str, i32)> {
        vec![
            ("EntityType", EntityType::VERSION),
            ("DisplayText", DisplayText::VERSION),
            ("DecayComponent", DecayComponent::VERSION),
            ("TemporalPosition", TemporalPosition::VERSION),
            ("MemoryLayer", MemoryLayer::VERSION),
            ("Age", Age::VERSION),
            ("Mood", Mood::VERSION),
            ("ThreadType", ThreadType::VERSION),
            ("ThreadName", ThreadName::VERSION),
            ("ThreadId", ThreadId::VERSION),
        ]
    }

    /// Get components affected by a specific law
    pub fn get_components_for_law(law_name: &str) -> Vec<&'static str> {
        let mut components = Vec::new();
        // Schema-based law component mapping

        if "decay" == law_name {
            components.push("DecayComponent");
        }
        if "resonance" == law_name {
            components.push("DecayComponent");
        }

        if "temporal_drift" == law_name {
            components.push("TemporalPosition");
        }

        if "memory_consolidation" == law_name {
            components.push("MemoryLayer");
        }

        if "decay" == law_name {
            components.push("Age");
        }

        components
    }
}
