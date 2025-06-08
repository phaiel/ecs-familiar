pub mod common;
pub mod components;
pub mod components_generated;  // Generated from YAML schema
pub mod ecs;
pub mod generated;   // Direct code generation from JSON schema  
pub mod systems;     // ECS systems implementing physics laws
pub mod schemas;
pub mod config;
pub mod graphql;
pub mod persistence;
pub mod schema_demo; // Demo of schema-first system 