// This module provides ECS components using generated types from the cold path schemas
// Main types are generated from JSON Schema in generated.rs

// Re-export all generated types as components  
pub use crate::generated::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

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

/// Temporal positioning component - treats time as a spatial dimension
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalPosition {
    pub timestamp: DateTime<Utc>,
    pub precision: TemporalPrecision,
    pub temporal_coordinates: (f64, f64, f64), // (past_distance, present_distance, future_distance)
    pub time_zone_offset: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalPrecision {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Decade,
}

/// Component tracking temporal relationships and causal links
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemporalLink {
    pub relation_type: TemporalRelationType,
    pub target_entity: Uuid,
    pub temporal_distance: f64, // In seconds
    pub causality_strength: f32, // 0.0 to 1.0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemporalRelationType {
    Before,
    After,
    Simultaneous,
    CausallyLinked,
    TemporallyAdjacent,
    Recurrent,
}

/// Working memory vs primary memory distinction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLayer {
    pub layer_type: MemoryLayerType,
    pub access_frequency: f32,
    pub last_accessed: DateTime<Utc>,
    pub consolidation_status: ConsolidationStatus,
    pub redis_key: Option<String>, // For working memory Redis integration
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemoryLayerType {
    WorkingMemory,   // Hot, fast access, temporary
    PrimaryMemory,   // Cold, persistent, archived
    TransitionalMemory, // Being moved between layers
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsolidationStatus {
    Fresh,           // Just created
    Consolidating,   // Being moved to primary memory
    Consolidated,    // Stored in primary memory
    Archived,        // Long-term storage
}

 