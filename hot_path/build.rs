use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema_path = "../assets/sample_schema.json";

    // Check if schema file exists
    if !Path::new(schema_path).exists() {
        panic!(
            "Schema file not found at {}. \
            Please run: cd ../cold_path && python cli.py schema-dump",
            schema_path
        );
    }

    println!("🔧 Generating types from schema: {}", schema_path);

    // Generate simple, direct Rust types
    let generated_code = generate_rust_types()?;

    // Write to src/generated.rs
    let output_path = "src/generated.rs";
    std::fs::write(output_path, generated_code)?;

    println!("✅ Generated types written to: {}", output_path);

    // Tell cargo to rerun if schema changes
    println!("cargo:rerun-if-changed={}", schema_path);

    Ok(())
}

fn generate_rust_types() -> Result<String, Box<dyn std::error::Error>> {
    let code = r#"// Generated types for Familiar Engine Hot Path
// DO NOT EDIT - regenerated on every build

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Core enums from cold path
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessScope {
    View,
    Edit,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Visibility {
    Private,
    Household,
    Org,
    Public,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Cardinality {
    Actor,
    Observer,
    Recipient,
    CoParticipant,
    Target,
}

// Base entity with versioning and soft deletion
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BaseEntity {
    #[serde(default)]
    pub id: Option<Uuid>,
    pub org_id: Uuid,
    pub owner_id: Uuid,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub deleted_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub component_ids: Vec<Uuid>,
    #[serde(default)]
    pub sub_type: Option<String>,
    #[serde(default)]
    pub visibility: Visibility,
    #[serde(default)]
    pub security_level: i64,
    #[serde(default)]
    pub access_scope: Vec<AccessScope>,
    #[serde(default = "default_version")]
    pub version: i64,
    #[serde(default)]
    pub parent_version: Option<i64>,
}

// Domain entities from cold path
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Moment {
    // BaseEntity fields (flattened)
    #[serde(flatten)]
    pub base: BaseEntity,
    
    // Moment-specific fields
    pub thread_id: Uuid,
    pub author_id: Uuid,
    #[serde(default)]
    pub binding_hint: Option<i64>,
    #[serde(default)]
    pub binding_id: Option<Uuid>,
    #[serde(default)]
    pub cardinality: Option<Cardinality>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BindingPoint {
    pub thread_id: Uuid,
    pub moment_id: Uuid,
    pub cardinality: Cardinality,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Binding {
    // BaseEntity fields (flattened)
    #[serde(flatten)]
    pub base: BaseEntity,
    
    // Binding-specific fields
    #[serde(default)]
    pub points: Vec<BindingPoint>,
    #[serde(default)]
    pub thread_ids: Vec<Uuid>, // Query optimization index
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thread {
    // BaseEntity fields (flattened)
    #[serde(flatten)]
    pub base: BaseEntity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bond {
    // BaseEntity fields (flattened)
    #[serde(flatten)]
    pub base: BaseEntity,
    
    // Bond-specific fields
    pub affinity_score: f64,
    pub bond_strength: f64,
    #[serde(default)]
    pub component_context: Vec<Uuid>,
    #[serde(default)]
    pub thread_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Filament {
    // BaseEntity fields (flattened)
    #[serde(flatten)]
    pub base: BaseEntity,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Motif {
    // BaseEntity fields (flattened)
    #[serde(flatten)]
    pub base: BaseEntity,
}

// Law system from cold path - with mathematical specifications
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Law {
    pub name: String,  // e.g. 'decay', 'resonance', 'binding'
    pub trigger: LawTrigger,
    pub applies_to: Vec<String>,  // e.g. ['filament', 'moment']
    
    // Mathematical specification - the "physics" of this law
    pub formula: String,  // Mathematical expression from cold path
    pub variables: Vec<String>,  // Component fields this law reads/writes
    pub constants: serde_json::Map<String, serde_json::Value>,  // Universal constants
    #[serde(default)]
    pub constraints: Option<serde_json::Map<String, serde_json::Value>>,  // Min/max bounds
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LawTrigger {
    OnObservation,
    OnTick,
    OnAffinityMatch,
}

// Helper functions
fn default_version() -> i64 {
    1
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Private
    }
}

// Implementations for generated types
impl Moment {
    /// Check if this moment was created recently
    pub fn is_recent(&self) -> bool {
        if let Some(created_at) = &self.base.created_at {
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(*created_at);
            duration.num_minutes() < 30 // Recent if created within 30 minutes
        } else {
            false
        }
    }
    
    /// Get the thread UUID (guaranteed present in Moment)
    pub fn thread_uuid(&self) -> Uuid {
        self.thread_id
    }
    
    /// Get the author UUID (guaranteed present in Moment)
    pub fn author_uuid(&self) -> Uuid {
        self.author_id
    }
}

impl Binding {
    /// Get all unique thread IDs from binding points
    pub fn get_thread_ids(&self) -> Vec<Uuid> {
        let mut thread_ids: Vec<Uuid> = self.points
            .iter()
            .map(|point| point.thread_id)
            .collect();
        thread_ids.sort();
        thread_ids.dedup();
        thread_ids
    }
    
    /// Get all unique moment IDs from binding points  
    pub fn get_moment_ids(&self) -> Vec<Uuid> {
        let mut moment_ids: Vec<Uuid> = self.points
            .iter()
            .map(|point| point.moment_id)
            .collect();
        moment_ids.sort();
        moment_ids.dedup();
        moment_ids
    }
    
    /// Check if this binding involves a specific thread
    pub fn involves_thread(&self, thread_id: Uuid) -> bool {
        // Check both the semantic points and the optimization index
        self.points.iter().any(|point| point.thread_id == thread_id) ||
        self.thread_ids.contains(&thread_id)
    }
}

impl BindingPoint {
    /// Create a new binding point
    pub fn new(thread_id: Uuid, moment_id: Uuid, cardinality: Cardinality) -> Self {
        Self {
            thread_id,
            moment_id,
            cardinality,
        }
    }
}
"#;

    Ok(code.to_string())
}
