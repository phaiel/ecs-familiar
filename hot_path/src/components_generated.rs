// GENERATED CODE — DO NOT EDIT  
// Generated from cold/instances/component_types.yml
use chrono::prelude::*;
use serde::{Deserialize, Serialize};


/// EntityType component
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaEntityType {
    pub entity_type: String,
}

impl SchemaEntityType {
    /// Component schema version
    pub const VERSION: i32 = 1;
    
    /// Create new SchemaEntityType with default values
    pub fn new(
        entity_type: String,
    ) -> Self {
        Self {
            entity_type,
        }
    }
}


/// DisplayText component
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaDisplayText {
    pub content: String,
}

impl SchemaDisplayText {
    /// Component schema version
    pub const VERSION: i32 = 1;
    
    /// Create new SchemaDisplayText with default values
    pub fn new(
        content: String,
    ) -> Self {
        Self {
            content,
        }
    }
}


/// DecayComponent component (affects: decay)
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaDecayComponent {
    /// Default: 1.0
    pub strength: f32,
    /// Default: 300.0
    pub half_life: f32,
    pub last_update: f32,
}

impl SchemaDecayComponent {
    /// Component schema version
    pub const VERSION: i32 = 1;
    /// Mixins: decayable
    pub const MIXINS: &'static [&'static str] = &["decayable"];
    /// Physics laws that affect this component
    pub const AFFECTED_BY_LAWS: &'static [&'static str] = &["decay"];
    
    /// Create new SchemaDecayComponent with default values
    pub fn new(
        last_update: f32,
    ) -> Self {
        Self {
            strength: 1.0_f32,
            half_life: 300.0_f32,
            last_update,
        }
    }
}


/// TemporalPosition component
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaTemporalPosition {
    pub timestamp: DateTime<Utc>,
    pub precision: String,
    pub temporal_coordinates: Vec<f32>,
    pub time_zone_offset: i32,
}

impl SchemaTemporalPosition {
    /// Component schema version
    pub const VERSION: i32 = 1;
    
    /// Create new SchemaTemporalPosition with default values
    pub fn new(
        timestamp: DateTime<Utc>,
        precision: String,
        temporal_coordinates: Vec<f32>,
        time_zone_offset: i32,
    ) -> Self {
        Self {
            timestamp,
            precision,
            temporal_coordinates,
            time_zone_offset,
        }
    }
}


/// MemoryLayer component
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaMemoryLayer {
    pub layer_type: String,
    /// Default: 0.0
    pub access_frequency: f32,
    pub last_accessed: DateTime<Utc>,
    pub consolidation_status: String,
    pub redis_key: String,
}

impl SchemaMemoryLayer {
    /// Component schema version
    pub const VERSION: i32 = 1;
    
    /// Create new SchemaMemoryLayer with default values
    pub fn new(
        layer_type: String,
        last_accessed: DateTime<Utc>,
        consolidation_status: String,
        redis_key: String,
    ) -> Self {
        Self {
            layer_type,
            access_frequency: 0.0_f32,
            last_accessed,
            consolidation_status,
            redis_key,
        }
    }
}


/// Age component (affects: decay)
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaAge {
    /// Default: 0
    pub age_days: i32,
    pub created_at: DateTime<Utc>,
}

impl SchemaAge {
    /// Component schema version
    pub const VERSION: i32 = 1;
    /// Mixins: decayable
    pub const MIXINS: &'static [&'static str] = &["decayable"];
    /// Physics laws that affect this component
    pub const AFFECTED_BY_LAWS: &'static [&'static str] = &["decay"];
    
    /// Create new SchemaAge with default values
    pub fn new(
        created_at: DateTime<Utc>,
    ) -> Self {
        Self {
            age_days: 0,
            created_at,
        }
    }
}


/// Mood component
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaMood {
    pub mood_state: String,
    /// Default: 0.0
    pub intensity: f32,
}

impl SchemaMood {
    /// Component schema version
    pub const VERSION: i32 = 1;
    
    /// Create new SchemaMood with default values
    pub fn new(
        mood_state: String,
    ) -> Self {
        Self {
            mood_state,
            intensity: 0.0_f32,
        }
    }
}


/// ThreadType component
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaThreadType {
    pub thread_type: String,
}

impl SchemaThreadType {
    /// Component schema version
    pub const VERSION: i32 = 1;
    
    /// Create new SchemaThreadType with default values
    pub fn new(
        thread_type: String,
    ) -> Self {
        Self {
            thread_type,
        }
    }
}


/// ThreadName component
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaThreadName {
    pub name: String,
}

impl SchemaThreadName {
    /// Component schema version
    pub const VERSION: i32 = 1;
    
    /// Create new SchemaThreadName with default values
    pub fn new(
        name: String,
    ) -> Self {
        Self {
            name,
        }
    }
}


/// ThreadId component
/// 
/// This is a Hecs component - automatically implements Component trait
/// for any type that is Send + Sync + 'static.
#[derive(Debug, Clone)]
pub struct SchemaThreadId {
    pub id: String,
}

impl SchemaThreadId {
    /// Component schema version
    pub const VERSION: i32 = 1;
    
    /// Create new SchemaThreadId with default values
    pub fn new(
        id: String,
    ) -> Self {
        Self {
            id,
        }
    }
}



/// Component registry for runtime introspection
pub struct SchemaComponentRegistry;

impl SchemaComponentRegistry {
    /// Get all component names and versions
    pub fn get_all_components() -> Vec<(&'static str, i32)> {
        vec![
            ("SchemaEntityType", SchemaEntityType::VERSION),
            ("SchemaDisplayText", SchemaDisplayText::VERSION),
            ("SchemaDecayComponent", SchemaDecayComponent::VERSION),
            ("SchemaTemporalPosition", SchemaTemporalPosition::VERSION),
            ("SchemaMemoryLayer", SchemaMemoryLayer::VERSION),
            ("SchemaAge", SchemaAge::VERSION),
            ("SchemaMood", SchemaMood::VERSION),
            ("SchemaThreadType", SchemaThreadType::VERSION),
            ("SchemaThreadName", SchemaThreadName::VERSION),
            ("SchemaThreadId", SchemaThreadId::VERSION),
        ]
    }
    
    /// Get components affected by a specific law
    pub fn get_components_for_law(law_name: &str) -> Vec<&'static str> {
        let mut components = Vec::new();
        if SchemaDecayComponent::AFFECTED_BY_LAWS.contains(&law_name) {
            components.push("SchemaDecayComponent");
        }
        if SchemaAge::AFFECTED_BY_LAWS.contains(&law_name) {
            components.push("SchemaAge");
        }
        components
    }
} 