// GENERATED CODE — DO NOT EDIT  
// Generated via Copier from cold path quantum law schemas
use hecs::World;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;


/// Redpanda event for quantum_decay
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumdecayEvent {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub law_name: String,
    pub entity_id: Option<u32>,
    pub component_data: serde_json::Value,
    pub quantum_params: HashMap<String, f64>,
}

/// Quantum coherence decay using QuTiP decoherence models
pub fn quantum_decay_system(world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
    let mut events = Vec::new();
    // Process DecayComponent entities - using manual serialization to avoid trait requirements
    for (entity, _component) in world.query::<&DecayComponent>().iter() {
        let event = QuantumdecayEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "quantum_calculation_request".to_string(),
            law_name: "quantum_decay".to_string(),
            entity_id: Some(entity.to_bits().get() as u32),
            component_data: serde_json::json!({"component_type": "DecayComponent"}),
            quantum_params: get_quantum_decay_params(),
        };
        events.push(event);
    }
    // Process Age entities - using manual serialization to avoid trait requirements
    for (entity, _component) in world.query::<&Age>().iter() {
        let event = QuantumdecayEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "quantum_calculation_request".to_string(),
            law_name: "quantum_decay".to_string(),
            entity_id: Some(entity.to_bits().get() as u32),
            component_data: serde_json::json!({"component_type": "Age"}),
            quantum_params: get_quantum_decay_params(),
        };
        events.push(event);
    }
    
    emit_to_redpanda("quantum_decay_events", &events)?;
    Ok(())
}

fn get_quantum_decay_params() -> HashMap<String, f64> {
    let mut params = HashMap::new();
    params.insert("hilbert_space_dim".to_string(), 2.0);
    params
}


/// Redpanda event for quantum_resonance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantumresonanceEvent {
    pub event_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub law_name: String,
    pub entity_id: Option<u32>,
    pub component_data: serde_json::Value,
    pub quantum_params: HashMap<String, f64>,
}

/// Quantum resonance using QuTiP driven systems
pub fn quantum_resonance_system(world: &mut World) -> Result<(), Box<dyn std::error::Error>> {
    let mut events = Vec::new();
    // Process DecayComponent entities - using manual serialization to avoid trait requirements
    for (entity, _component) in world.query::<&DecayComponent>().iter() {
        let event = QuantumresonanceEvent {
            event_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: "quantum_calculation_request".to_string(),
            law_name: "quantum_resonance".to_string(),
            entity_id: Some(entity.to_bits().get() as u32),
            component_data: serde_json::json!({"component_type": "DecayComponent"}),
            quantum_params: get_quantum_resonance_params(),
        };
        events.push(event);
    }
    
    emit_to_redpanda("quantum_resonance_events", &events)?;
    Ok(())
}

fn get_quantum_resonance_params() -> HashMap<String, f64> {
    let mut params = HashMap::new();
    params.insert("hilbert_space_dim".to_string(), 3.0);
    params
}



fn emit_to_redpanda(topic: &str, events: &[impl Serialize]) -> Result<(), Box<dyn std::error::Error>> {
    for event in events {
        println!("📡 [{}] {}", topic, serde_json::to_string(event)?);
    }
    Ok(())
}

use crate::gen::components::*;