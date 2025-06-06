from pydantic import BaseModel
from typing import List, Literal, Optional

class Law(BaseModel):
    name: str  # e.g. 'decay', 'resonance', 'binding'
    trigger: Literal['on_observation', 'on_tick', 'on_affinity_match']
    applies_to: List[str]  # e.g. ['filament', 'moment']
    
    # Mathematical specification - the "physics" of this law
    formula: str  # Mathematical expression 
    variables: List[str]  # Component fields this law reads/writes
    constants: dict  # Fixed values (like speed of light, gravitational constant)
    constraints: Optional[dict] = None  # Min/max bounds, etc.


# --- Decay Law: Exponential decay over time ---
# The fundamental physics of memory degradation
decay_law = Law(
    name="decay",
    trigger="on_tick", 
    applies_to=["filament", "motif"],
    formula="strength = strength * pow(0.5, time_elapsed / half_life)",
    variables=["strength", "half_life", "last_update"],
    constants={},  # No universal constants needed
    constraints={"strength": {"min": 0.1}}  # Minimum strength bound
)


# --- Resonance Law: Amplification for strong signals ---  
# The physics of memory reinforcement
resonance_law = Law(
    name="resonance",
    trigger="on_affinity_match",
    applies_to=["filament"], 
    formula="strength = min(strength * multiplier, max_strength) if strength > threshold else strength",
    variables=["strength"],
    constants={
        "threshold": 0.85,      # Universal resonance threshold
        "multiplier": 1.2,      # Universal amplification factor  
        "max_strength": 1.0     # Universal maximum strength
    }
)

# --- Binding Law: Spatial connection formation ---
binding_law = Law(
    name="binding", 
    trigger="on_tick",
    applies_to=["moment"],
    formula="create_binding_if(distance < proximity_threshold and affinity > affinity_threshold)",
    variables=["position", "affinity"],
    constants={
        "proximity_threshold": 5.0,   # Universal spatial binding distance
        "affinity_threshold": 0.7     # Universal affinity requirement
    }
) 