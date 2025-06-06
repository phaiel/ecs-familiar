// This module provides ECS components using generated types from the cold path schemas
// Main types are generated from JSON Schema in generated.rs

// Re-export all generated types as components  
pub use crate::generated::*;

// --- Visualization-specific wrapper components (not in schema) ---
#[derive(Debug)]
pub struct DisplayText(pub String);

#[derive(Debug)]
pub struct EntityType(pub String);

#[derive(Debug)]
pub struct ThreadType(pub String);

#[derive(Debug)]
pub struct ThreadName(pub String);

// --- For visualization: thread positioning ---
#[derive(Debug)]
pub struct ThreadId(pub String);

// --- Decay Component for implementing time-based laws ---
#[derive(Debug)]
pub struct DecayComponent {
    pub strength: f32,
    pub half_life: f32, // Time in seconds for strength to halve
    pub last_update: f64, // Timestamp of last update
}

 