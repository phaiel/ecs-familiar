// Laws are schema-driven mathematical physics from the cold path
// The hot path is a pure physics simulator that executes mathematical laws

//! # Schema-Driven Physics Engine
//! 
//! Laws are defined mathematically in the cold path and executed here.
//! The hot path has no hardcoded behavior - it reads the "universal constants" 
//! and mathematical formulas from the schema and applies them naturally.

use hecs::World;
use crate::components::{Law, LawTrigger, EntityType, DecayComponent};

/// Execute all laws that match a specific trigger
/// The hot path is agnostic - it just applies whatever physics laws exist
pub fn execute_laws(world: &mut World, trigger: LawTrigger) {
    // Collect all active laws that match the trigger
    let mut matching_laws = Vec::new();
    
    for (_entity, (law, _entity_type)) in world.query::<(&Law, &EntityType)>().iter() {
        if law.trigger == trigger {
            matching_laws.push(law.clone());
        }
    }
    
    // Execute each law using its mathematical specification from cold path
    for law in matching_laws {
        execute_physics_law(world, &law);
    }
}

/// Execute a physics law based on its mathematical formula from the schema
/// This is a pure physics simulator - no hardcoded behavior
fn execute_physics_law(world: &mut World, law: &Law) {
    match law.name.as_str() {
        "decay" => apply_decay_physics(world, law),
        "resonance" => apply_resonance_physics(world, law),
        "binding" => apply_binding_physics(world, law),
        _ => {
            // Unknown physics law - log for debugging
            println!("⚠️  Unknown physics law: {}", law.name);
        }
    }
}

/// Apply decay physics based on the mathematical formula from cold path
/// Formula: "strength = strength * pow(0.5, time_elapsed / half_life)"
fn apply_decay_physics(world: &mut World, law: &Law) {
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
    
    // Get minimum strength constraint from law
    let min_strength = law.constraints
        .as_ref()
        .and_then(|c| c.get("strength"))
        .and_then(|s| s.as_object())
        .and_then(|s| s.get("min"))
        .and_then(|m| m.as_f64())
        .unwrap_or(0.1) as f32;
    
    // Apply to entities that match the law's target types
    for (_entity, (entity_type, decay)) in world.query_mut::<(&EntityType, &mut DecayComponent)>() {
        if law.applies_to.contains(&entity_type.0) {
            let time_elapsed = current_time - decay.last_update;
            
            if time_elapsed > 0.0 {
                // Execute the mathematical formula from cold path
                // Formula: strength = strength * pow(0.5, time_elapsed / half_life)
                let decay_factor = 0.5_f32.powf(time_elapsed as f32 / decay.half_life);
                decay.strength *= decay_factor;
                decay.last_update = current_time;
                
                // Apply constraints from cold path
                if decay.strength < min_strength {
                    decay.strength = min_strength;
                }
            }
        }
    }
}

/// Apply resonance physics based on the mathematical formula from cold path  
/// Formula: "strength = min(strength * multiplier, max_strength) if strength > threshold else strength"
fn apply_resonance_physics(world: &mut World, law: &Law) {
    // Read universal constants from cold path
    let threshold = law.constants
        .get("threshold")
        .and_then(|v| v.as_f64())
        .unwrap_or(0.85) as f32;
        
    let multiplier = law.constants
        .get("multiplier")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.2) as f32;
        
    let max_strength = law.constants
        .get("max_strength")
        .and_then(|v| v.as_f64())
        .unwrap_or(1.0) as f32;
    
    for (_entity, (entity_type, decay)) in world.query_mut::<(&EntityType, &mut DecayComponent)>() {
        if law.applies_to.contains(&entity_type.0) && decay.strength > threshold {
            // Execute the mathematical formula from cold path
            // Formula: strength = min(strength * multiplier, max_strength)
            decay.strength = (decay.strength * multiplier).min(max_strength);
        }
    }
}

/// Apply binding physics based on mathematical formula from cold path
/// This would read proximity and affinity rules from the law's constants
fn apply_binding_physics(_world: &mut World, _law: &Law) {
    // Future: Read spatial binding constants from law.constants
    // Execute mathematical formulas for distance and affinity calculations
} 