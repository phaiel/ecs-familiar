# 🧬 ECS Familiar - Modular Schema-First Generation

A **100% schema-generated** Entity Component System (ECS) built with Rust Hecs, featuring modular generation, quantum systems, and event-driven architecture.

## ✨ Key Features

- **🎯 Modular Generation**: Generate only what you need (ECS, quantum, DAGs, configs)
- **⚛️ Quantum Systems**: QuTiP-powered quantum calculations via Redpanda events  
- **🧭 Event-Driven**: Rust ECS → Redpanda → Windmill DAG → Python QuTiP
- **📋 100% Schema-First**: All code generated from YAML definitions
- **🔧 CI/CD Ready**: Full automation pipeline with validation

## 🚀 Quick Start

```bash
# Generate everything (recommended)
python generate.py all

# Or generate specific targets
python generate.py ecs       # Core ECS components & systems
python generate.py quantum   # Quantum systems
python generate.py dags      # Windmill DAG nodes

# Run the ECS system
cd hot_path && cargo run

# Run quantum demo
cd hot_path && cargo run -- --quantum-demo
```

## 🏗️ Architecture

```
YAML Schemas → Multiple Generators → Generated Code
                                     ↓
┌─────────────┐   🧬 ECS Generator   ┌─────────────────┐
│ Components  │ ═════════════════>   │ components.rs   │
│ Laws        │                      │ systems.rs      │  
│ Entities    │   ⚛️ Quantum Gen     │ main.rs         │
└─────────────┘ ═════════════════>   │ quantum_sys.rs  │
                                     ├─────────────────┤
                 🧭 DAG Generator     │ DAG nodes       │
               ═════════════════>     │ QuTiP scripts   │
                                     ├─────────────────┤
                 📡 Config Gen        │ redpanda.yml    │
               ═════════════════>     │ windmill.yml    │
                                     └─────────────────┘
```

## 🎯 Modular Generation

### Individual Targets

```bash
python generate.py ecs           # Components, systems, main.rs
python generate.py quantum       # Quantum event emitters
python generate.py dags          # Windmill DAG nodes  
python generate.py entities      # Entity blueprints
python generate.py redpanda      # Event stream configs (stub)
python generate.py windmill      # Workflow configs (stub)
```

### Make Integration

```bash
make ci                          # Full CI pipeline
make quantum && make demo        # Generate quantum + run demo
make ecs                         # Core ECS only
make clean && make all           # Full regeneration
```

## ⚛️ Quantum ECS System

### Hybrid Architecture
- **Rust ECS**: High-performance component queries and entity management
- **Redpanda Events**: Stream quantum calculation requests
- **Windmill DAGs**: Execute Python QuTiP quantum calculations
- **Event Results**: Flow back through stream to update components

### Example Flow
```rust
// 1. Rust ECS queries components
for (entity, component) in world.query::<&DecayComponent>().iter() {
    // 2. Emit Redpanda event for quantum calculation
    let event = QuantumDecayEvent {
        entity_id: entity.to_bits().get() as u32,
        quantum_params: get_quantum_params(),
        // ...
    };
    emit_to_redpanda("quantum_decay_events", &event)?;
}

// 3. Windmill DAG receives event and runs QuTiP calculation
// 4. Results flow back through event stream
// 5. ECS components updated with quantum results
```

### QuTiP Integration
Generated DAG nodes include full QuTiP scripts:
```python
# Auto-generated in dags/quantum_decay_dag.yml
def quantum_decay_calculation(event_data):
    # QuTiP decoherence calculation
    psi0 = qt.basis(2, 1)  # Excited state
    c_ops = [np.sqrt(gamma) * qt.sigmam()]  # Lindblad operators
    result = qt.mesolve(H, psi0, times, c_ops)
    return {'new_strength': calculate_decay(result)}
```

## 🔧 Schema Definition

### Components with Quantum Laws
```yaml
# cold/instances/component_types.yml
- name: DecayComponent
  version: 1
  mixins: ["decayable"]
  laws: ["decay", "resonance"]  # Triggers quantum system generation
  fields:
    - name: strength
      type: float
      default_value: 1.0
    - name: half_life
      type: float
      default_value: 300.0
```

### Quantum Laws
```yaml
# cold/instances/quantum_laws.yml
- name: quantum_decay
  description: "Quantum coherence decay using QuTiP decoherence models"
  applies_to: [DecayComponent, Age]
  execution:
    runtime: python_qutip
    ecs_bridge: rust_component_query
    dag_node: windmill_tick
    event_stream: redpanda
  quantum_model:
    hilbert_space_dim: 2
    operators: ["sigma_minus", "sigma_z"]
    solver: "mesolve"
```

## 📊 Generated Files

The modular system generates different file sets based on targets:

### ECS Target (`python generate.py ecs`)
- `hot_path/src/gen/components.rs` - Component definitions
- `hot_path/src/gen/systems.rs` - Physics law systems  
- `hot_path/src/main.rs` - ECS application
- `hot_path/src/lib.rs` - Module exports
- `hot_path/Cargo.toml` - Dependencies

### Quantum Target (`python generate.py quantum`)
- `hot_path/src/gen/quantum_systems.rs` - Event emitters

### DAGs Target (`python generate.py dags`) 
- `dags/quantum_decay_dag.yml` - QuTiP decoherence calculation
- `dags/quantum_resonance_dag.yml` - QuTiP driven systems

### Config Targets (Stubs)
- `redpanda/redpanda.yml` - Event stream configuration
- `windmill/windmill.yml` - Workflow orchestration

## 🎮 Demo Modes

```bash
# Schema validation only
python generate.py --validate

# Component registry demo
cargo run -- --component-demo

# Quantum system demo (see Redpanda events!)
cargo run -- --quantum-demo
```

## 🏆 Key Achievements

### ✅ 100% Schema-First
- **Zero manual ECS code** - everything generated from YAML
- **Modular targets** - generate only what changed
- **Type safety** - Pydantic validation → Rust compilation

### ✅ Quantum Integration
- **Hybrid architecture** - Rust ECS + Python QuTiP
- **Event-driven** - Redpanda streams + Windmill DAGs
- **Real quantum calculations** - Decoherence, driven systems

### ✅ Production Ready
- **CI/CD pipeline** - Validation → generation → compilation
- **Modular design** - Independent generators
- **Stub support** - Future functionality placeholders

## 🛠️ Technology Stack

- **ECS**: Rust + Hecs (high-performance)
- **Quantum**: Python + QuTiP (scientific computation)
- **Events**: Redpanda (streaming)
- **Orchestration**: Windmill (DAG execution)
- **Generation**: Jinja2 + YAML schemas
- **Validation**: Pydantic + Rust compiler

## 🌟 Advanced Features

- **Component Mixins**: Reusable behavior patterns
- **Law-Component Binding**: Automatic system generation
- **Runtime Introspection**: Component registry with metadata
- **Event Replay**: Observable and replayable quantum calculations
- **Modular Architecture**: Generate individual subsystems

## 🔄 Development Workflow

```bash
1. Edit YAML schemas (cold/instances/)
2. Run modular generation (python generate.py <target>)  
3. Test compilation (make check)
4. Run demos (make demo)
```

**Zero drift** - schemas and code stay perfectly synchronized!

## 📁 Project Structure

```
├── cold/instances/           # YAML schema definitions
│   ├── component_types.yml   # Component schemas
│   ├── quantum_laws.yml      # Quantum law definitions  
│   └── entity_blueprints.yml # Entity compositions
├── generators/               # Modular generators
│   ├── ecs.py               # Core ECS generation
│   ├── quantum.py           # Quantum systems
│   ├── dags.py              # DAG nodes
│   └── base.py              # Shared functionality
├── templates/               # Jinja2 templates
│   ├── components.rs.jinja  # Component generation
│   ├── quantum_system.rs.jinja # Quantum systems
│   └── windmill_dag.yml.jinja # DAG nodes
├── hot_path/src/gen/        # Generated Rust code
├── dags/                    # Generated DAG nodes
└── generate.py              # Modular CLI
```

## 🚀 Getting Started

1. **Clone and setup**:
   ```bash
   git clone <repo>
   cd fam_game
   pip install -r requirements.txt
   ```

2. **Generate everything**:
   ```bash
   python generate.py all
   ```

3. **Run quantum demo**:
   ```bash
   cd hot_path && cargo run -- --quantum-demo
   ```

4. **See Redpanda events** flowing with quantum calculation requests!

---

**🎯 Result**: A fully modular, schema-first ECS system with integrated quantum calculations, event streams, and 100% generated code from YAML definitions.