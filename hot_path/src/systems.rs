use hecs::World;
use crate::components::{DecayComponent, EntityType};
use std::time::{SystemTime, UNIX_EPOCH};

/// Mathematical law specifications loaded from schema
pub struct LawSpecifications {
    pub decay_law: DecayLawSpec,
    pub resonance_law: ResonanceLawSpec,
}

#[derive(Debug, Clone)]
pub struct DecayLawSpec {
    pub formula: String,
    pub applies_to: Vec<String>,
    pub min_strength: f32,
}

#[derive(Debug, Clone)]
pub struct ResonanceLawSpec {
    pub formula: String,
    pub threshold: f32,
    pub multiplier: f32,
    pub max_strength: f32,
}

impl LawSpecifications {
    /// Load law specifications from schema (replaces law entities)
    pub fn from_schema() -> Self {
        Self {
            decay_law: DecayLawSpec {
                formula: "strength = strength * pow(0.5, time_elapsed / half_life)".to_string(),
                applies_to: vec!["filament".to_string(), "motif".to_string(), "moment".to_string()],
                min_strength: 0.1,
            },
            resonance_law: ResonanceLawSpec {
                formula: "strength = min(strength * multiplier, max_strength) if strength > threshold else strength".to_string(),
                threshold: 0.85,
                multiplier: 1.2,
                max_strength: 1.0,
            },
        }
    }
}

/// Decay System - applies exponential decay law to entities
pub fn decay_system(world: &mut World, law_spec: &DecayLawSpec) {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    for (_entity, (entity_type, decay)) in world.query_mut::<(&EntityType, &mut DecayComponent)>() {
        // Only apply to entity types specified in law
        if law_spec.applies_to.contains(&entity_type.0) {
            let time_elapsed = current_time - decay.last_update;
            
            // Apply mathematical formula from schema
            decay.strength = decay.strength * (0.5_f32).powf(time_elapsed as f32 / decay.half_life);
            
            // Apply constraints from schema  
            if decay.strength < law_spec.min_strength {
                decay.strength = law_spec.min_strength;
            }
            
            decay.last_update = current_time;
        }
    }
}

/// Resonance System - applies resonance amplification law
pub fn resonance_system(world: &mut World, law_spec: &ResonanceLawSpec) {
    for (_entity, (entity_type, decay)) in world.query_mut::<(&EntityType, &mut DecayComponent)>() {
        // Apply resonance to filaments when strength is high
        if entity_type.0 == "filament" && decay.strength > law_spec.threshold {
            // Apply mathematical formula from schema
            let new_strength = decay.strength * law_spec.multiplier;
            decay.strength = new_strength.min(law_spec.max_strength);
        }
    }
}

/// System runner that applies all physical laws
pub fn run_physics_systems(world: &mut World, law_specs: &LawSpecifications) {
    // Apply each law as a proper ECS system
    decay_system(world, &law_specs.decay_law);
    resonance_system(world, &law_specs.resonance_law);
}

/// Get system statistics for monitoring
pub fn get_system_stats(world: &World, law_specs: &LawSpecifications) -> SystemStats {
    let mut affected_entities = 0;
    
    for (_entity, (entity_type, _decay)) in world.query::<(&EntityType, &DecayComponent)>().iter() {
        if law_specs.decay_law.applies_to.contains(&entity_type.0) {
            affected_entities += 1;
        }
    }
    
    SystemStats {
        active_systems: 2, // decay + resonance
        affected_entities,
        law_specifications: format!("decay: {}, resonance: {}", 
            law_specs.decay_law.applies_to.len(),
            if law_specs.resonance_law.threshold > 0.0 { 1 } else { 0 }
        ),
    }
}

#[derive(Debug, Clone)]
pub struct SystemStats {
    pub active_systems: usize,
    pub affected_entities: usize,
    pub law_specifications: String,
} 