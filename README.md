# ECS Familiar

A schema-driven Entity-Component-System (ECS) memory simulation engine with a novel **hot/cold path architecture**. The cold path (Python) defines the "laws of physics" using mathematical formulas, while the hot path (Rust) executes those laws as a pure physics simulator.

## 🌌 Core Philosophy 

Like living on Earth where gravity affects everything naturally through physics, entities in this system are influenced by laws defined in the schema - but the runtime has no awareness of the specific mathematics, it just experiences the effects.

## 🏗️ Architecture

**Cold Path (Universe Creator)** - Python
- Defines all schemas, entities, components, and physical laws
- Mathematical specifications: formulas, variables, constants, constraints  
- Exports complete universe definition to JSON schema

**Hot Path (Physics Simulator)** - Rust  
- High-performance ECS runtime using Hecs
- Pure physics simulator that executes mathematical laws from schema
- No hardcoded behavior - laws apply naturally based on component values
- GraphQL API for real-time interaction

## ⚖️ Schema-Driven Physics

Laws are defined mathematically in the cold path:

```python
decay_law = Law(
    name="decay",
    formula="strength = strength * pow(0.5, time_elapsed / half_life)",
    variables=["strength", "half_life", "last_update"], 
    constants={},
    constraints={"strength": {"min": 0.1}}
)
```

The hot path reads these specifications and executes them as pure physics:

```rust
// Physics simulator executes whatever laws exist in schema
fn apply_law(formula: &str, entity: Entity, world: &mut World) {
    // Parse and execute mathematical formula on entity components
}
```

## 🚀 Getting Started

### Prerequisites
- Python 3.11+ with Poetry
- Rust 1.70+

### Cold Path Setup

```bash
cd cold_path
poetry install
poetry run python cli.py schema-dump
```

This generates `assets/sample_schema.json` containing all entities, components, and physical laws.

### Hot Path Setup

```bash
cd hot_path
cargo build  # Auto-generates Rust types from Python schemas
cargo run    # Starts the physics simulation
```

Visit http://127.0.0.1:8000 for GraphQL API interface.

## 🧬 Core Entities

- **Moment**: Individual memory instances
- **Thread**: Sequential chains of related moments  
- **Binding**: Complex relationships between threads/moments
- **Bond**, **Filament**, **Motif**: Higher-order memory structures

## 🔗 Schema Bridge

The build process automatically generates type-safe Rust code from Python Pydantic schemas:

1. Python schemas export to JSON
2. Custom build.rs generates Rust structs 
3. Perfect type translation with UUIDs, enums, and proper validation

## 📊 Real-time Monitoring

The hot path provides colorful terminal output showing:
- Entity counts by type
- System status and trends  
- Law execution statistics
- Memory usage and performance 