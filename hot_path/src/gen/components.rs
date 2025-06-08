// GENERATED CODE — DO NOT EDIT
// Generated from cold/instances/component_types.yml via copier
use chrono::prelude::*;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

// Vector type for embeddings (768-dimensional by default)
pub type Vector = [f32; 768];


/// EntityType Component (v1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityType {
    pub entity_type: String,
}

impl EntityType {
    pub const VERSION: i32 = 1;

    pub fn new() -> Self {
        Self {
            entity_type: "".to_string(),
        }
    }
}


/// DisplayText Component (v1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayText {
    pub content: String,
}

impl DisplayText {
    pub const VERSION: i32 = 1;

    pub fn new() -> Self {
        Self {
            content: "".to_string(),
        }
    }
}


/// DecayComponent Component (v1)
/// Mixins: decayable
/// Governed by laws: decay, resonance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecayComponent {
    pub strength: f32,
    pub half_life: f32,
    pub last_update: f32,
}

impl DecayComponent {
    pub const VERSION: i32 = 1;
    pub const MIXINS: &'static [&'static str] = &["decayable"
    ];
    pub const LAWS: &'static [&'static str] = &["decay", "resonance"
    ];

    pub fn new() -> Self {
        Self {
            strength: 1.0,
            half_life: 300.0,
            last_update: 0.0,
        }
    }
}


/// TemporalPosition Component (v1)
/// Governed by laws: temporal_drift
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPosition {
    pub timestamp: DateTime<Utc>,
    pub precision: String,
    pub temporal_coordinates: Vec<f32>,
    pub time_zone_offset: i32,
}

impl TemporalPosition {
    pub const VERSION: i32 = 1;
    pub const LAWS: &'static [&'static str] = &["temporal_drift"
    ];

    pub fn new() -> Self {
        Self {
            timestamp: Utc::now(),
            precision: "day".to_string(),
            temporal_coordinates: Vec::new(),
            time_zone_offset: 0,
        }
    }
}


/// MemoryLayer Component (v1)
/// Governed by laws: memory_consolidation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLayer {
    pub layer_type: String,
    pub access_frequency: f32,
    pub last_accessed: DateTime<Utc>,
    pub consolidation_status: String,
    pub redis_key: String,
}

impl MemoryLayer {
    pub const VERSION: i32 = 1;
    pub const LAWS: &'static [&'static str] = &["memory_consolidation"
    ];

    pub fn new() -> Self {
        Self {
            layer_type: "working".to_string(),
            access_frequency: 0.0,
            last_accessed: Utc::now(),
            consolidation_status: "active".to_string(),
            redis_key: "".to_string(),
        }
    }
}


/// Age Component (v1)
/// Mixins: decayable
/// Governed by laws: decay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Age {
    pub age_days: i32,
    pub created_at: DateTime<Utc>,
}

impl Age {
    pub const VERSION: i32 = 1;
    pub const MIXINS: &'static [&'static str] = &["decayable"
    ];
    pub const LAWS: &'static [&'static str] = &["decay"
    ];

    pub fn new() -> Self {
        Self {
            age_days: 0,
            created_at: Utc::now(),
        }
    }
}


/// Mood Component (v1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mood {
    pub mood_state: String,
    pub intensity: f32,
}

impl Mood {
    pub const VERSION: i32 = 1;

    pub fn new() -> Self {
        Self {
            mood_state: "neutral".to_string(),
            intensity: 0.0,
        }
    }
}


/// ThreadType Component (v1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadType {
    pub thread_type: String,
}

impl ThreadType {
    pub const VERSION: i32 = 1;

    pub fn new() -> Self {
        Self {
            thread_type: "".to_string(),
        }
    }
}


/// ThreadName Component (v1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadName {
    pub name: String,
}

impl ThreadName {
    pub const VERSION: i32 = 1;

    pub fn new() -> Self {
        Self {
            name: "".to_string(),
        }
    }
}


/// ThreadId Component (v1)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadId {
    pub id: String,
}

impl ThreadId {
    pub const VERSION: i32 = 1;

    pub fn new() -> Self {
        Self {
            id: "".to_string(),
        }
    }
}



// Component registry for runtime introspection
pub struct ComponentRegistry;

impl ComponentRegistry {
    pub fn get_component_info(name: &str) -> Option<ComponentInfo> {
        match name {
            "EntityType" => Some(ComponentInfo {
                name: "EntityType",
                version: 1,
                mixins: vec![],
                laws: vec![],
            }),
            "DisplayText" => Some(ComponentInfo {
                name: "DisplayText",
                version: 1,
                mixins: vec![],
                laws: vec![],
            }),
            "DecayComponent" => Some(ComponentInfo {
                name: "DecayComponent",
                version: 1,
                mixins: vec!["decayable".to_string()],
                laws: vec!["decay".to_string(), "resonance".to_string()],
            }),
            "TemporalPosition" => Some(ComponentInfo {
                name: "TemporalPosition",
                version: 1,
                mixins: vec![],
                laws: vec!["temporal_drift".to_string()],
            }),
            "MemoryLayer" => Some(ComponentInfo {
                name: "MemoryLayer",
                version: 1,
                mixins: vec![],
                laws: vec!["memory_consolidation".to_string()],
            }),
            "Age" => Some(ComponentInfo {
                name: "Age",
                version: 1,
                mixins: vec!["decayable".to_string()],
                laws: vec!["decay".to_string()],
            }),
            "Mood" => Some(ComponentInfo {
                name: "Mood",
                version: 1,
                mixins: vec![],
                laws: vec![],
            }),
            "ThreadType" => Some(ComponentInfo {
                name: "ThreadType",
                version: 1,
                mixins: vec![],
                laws: vec![],
            }),
            "ThreadName" => Some(ComponentInfo {
                name: "ThreadName",
                version: 1,
                mixins: vec![],
                laws: vec![],
            }),
            "ThreadId" => Some(ComponentInfo {
                name: "ThreadId",
                version: 1,
                mixins: vec![],
                laws: vec![],
            }),
            _ => None,
        }
    }
    
    pub fn all_components() -> Vec<&'static str> {
        vec![
            "EntityType",
            "DisplayText",
            "DecayComponent",
            "TemporalPosition",
            "MemoryLayer",
            "Age",
            "Mood",
            "ThreadType",
            "ThreadName",
            "ThreadId"
        ]
    }
    
    pub fn get_all_components() -> Vec<(&'static str, i32)> {
        vec![
            ("EntityType", 1),
            ("DisplayText", 1),
            ("DecayComponent", 1),
            ("TemporalPosition", 1),
            ("MemoryLayer", 1),
            ("Age", 1),
            ("Mood", 1),
            ("ThreadType", 1),
            ("ThreadName", 1),
            ("ThreadId", 1)
        ]
    }
    
    pub fn get_components_for_law(law_name: &str) -> Vec<&'static str> {
        let mut components = Vec::new();
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

#[derive(Debug, Clone)]
pub struct ComponentInfo {
    pub name: &'static str,
    pub version: i32,
    pub mixins: Vec<String>,
    pub laws: Vec<String>,
} 