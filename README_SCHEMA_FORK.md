# ECS Familiar - Schema-First Fork

🧬 **Schema-driven ECS component generation with YAML + Copier templates**

This is a fork of [ecs-familiar](https://github.com/phaiel/ecs-familiar) that implements a **schema-first approach** using YAML component definitions and Copier templates to generate type-safe Rust Hecs components.

## 🌟 **Key Innovations**

### **Schema-First Design**
- Component definitions start in **YAML schemas**, not code
- **Pydantic validation** ensures schema correctness
- **Jinja2 templates** generate clean Rust code
- **Type safety** from schema to runtime

### **Dual Architecture** 
- **Existing System**: Python Pydantic → JSON → build.rs → Rust
- **New System**: YAML → Copier → Rust components
- **Both coexist** during migration/comparison

### **Component Features**
- **Versioning**: Track component schema evolution
- **Mixins**: Reusable component behaviors
- **Law Binding**: Explicit physics law relationships  
- **Runtime Introspection**: Component registry and metadata
- **Type Safety**: Full Rust type checking

## 🚀 **Quick Start**

### **1. Schema Definition**
```yaml
# cold/instances/component_types.yml
- name: DecayComponent
  version: 1
  mixins: ["decayable"]
  laws: ["decay"]
  fields:
    - name: strength
      type: float
      default: 1.0
    - name: half_life
      type: float
      default: 300.0
    - name: last_update
      type: float
```

### **2. Generate Rust Components**
```bash
python3 generate_components.py
```

### **3. Generated Rust Code**
```rust
// GENERATED CODE — DO NOT EDIT
#[derive(Debug, Clone)]
pub struct SchemaDecayComponent {
    pub strength: f32,      // Default: 1.0
    pub half_life: f32,     // Default: 300.0  
    pub last_update: f32,
}

impl SchemaDecayComponent {
    pub const VERSION: i32 = 1;
    pub const MIXINS: &'static [&'static str] = &["decayable"];
    pub const AFFECTED_BY_LAWS: &'static [&'static str] = &["decay"];
    
    pub fn new(last_update: f32) -> Self { /* ... */ }
}
```

### **4. Use in ECS**
```rust
use crate::components_generated::*;

// Create entities with schema components
world.spawn((
    SchemaEntityType::new("memory".to_string()),
    SchemaDecayComponent::new(0.0),
    SchemaDisplayText::new("A vivid memory".to_string()),
));

// Runtime introspection
let components = SchemaComponentRegistry::get_all_components();
let decay_affected = SchemaComponentRegistry::get_components_for_law("decay");
```

## 🏗️ **Architecture**

### **Files Structure**
```
ecs-familiar/
├── cold/
│   ├── components.yaml           # Schema definition
│   └── instances/
│       └── component_types.yml   # Component instances
├── templates/
│   └── component.rs.jinja       # Rust generation template
├── cold_path/
│   └── schema_integration.py    # Pydantic validation
├── hot_path/src/
│   ├── components_generated.rs  # Generated components
│   └── schema_demo.rs           # Demo system
├── copier.yaml                  # Copier configuration
├── copier_setup.py             # Custom Jinja filters
└── generate_components.py       # Direct generation script
```

### **Type Mapping**
| YAML Type | Rust Type | Example |
|-----------|-----------|---------|
| `int` | `i32` | `42` |
| `float` | `f32` | `3.14_f32` |  
| `str` | `String` | `"hello".to_string()` |
| `datetime` | `DateTime<Utc>` | `Utc::now()` |
| `bool` | `bool` | `true` |
| `vector` | `Vec<f32>` | `vec![1.0, 2.0]` |
| `enum` | `String` | `"active".to_string()` |

## 🔧 **Usage Examples**

### **Demo Mode**
```bash
cargo run -- --schema-demo
```

### **Component Creation**
```yaml
- name: Mood
  version: 1
  mixins: []
  laws: []
  fields:
    - name: mood_state
      type: enum
    - name: intensity
      type: float
      default: 0.0
```

### **Physics Law Binding**
```yaml
- name: Age
  version: 1
  mixins: ["decayable"]  
  laws: ["decay"]        # This component affected by decay law
  fields:
    - name: age_days
      type: int
      default: 0
```

### **Runtime Queries**
```rust
// Find all components affected by decay
let decay_components = SchemaComponentRegistry::get_components_for_law("decay");

// Get component metadata  
println!("Mixins: {:?}", SchemaAge::MIXINS);
println!("Laws: {:?}", SchemaAge::AFFECTED_BY_LAWS);
println!("Version: {}", SchemaAge::VERSION);
```

## 🧪 **Testing**

### **Schema Validation**
```bash
python3 cold_path/schema_integration.py
```

### **Type Generation** 
```bash
python3 copier_setup.py
```

### **ECS Integration**
```bash
cargo run -- --schema-demo
cargo run -- --debug
cargo run -- --help
```

## ⚖️ **Physics Laws Integration**

Components explicitly declare which physics laws affect them:

```yaml
# Component declares law relationship
- name: DecayComponent
  laws: ["decay"]

# System uses this metadata
for (entity, decay) in world.query_mut::<&mut SchemaDecayComponent>() {
    if SchemaDecayComponent::AFFECTED_BY_LAWS.contains(&"decay") {
        apply_decay_law(decay);
    }
}
```

## 🔄 **Migration Path**

1. **Parallel Development**: New schema components coexist with existing system
2. **Gradual Migration**: Move components one-by-one to schema-first  
3. **Validation**: Compare both systems during transition
4. **Full Adoption**: Eventually replace old generation system

## 🏆 **Benefits Over Original**

| Feature | Original | Schema-First |
|---------|----------|-------------|
| **Type Safety** | ✅ | ✅ |
| **Schema Validation** | ✅ | ✅ |
| **Component Versioning** | ❌ | ✅ |
| **Law Relationships** | ❌ | ✅ |
| **Runtime Introspection** | ❌ | ✅ |
| **Mixins** | ❌ | ✅ |
| **Clean Components** | ❌ | ✅ |
| **Template Flexibility** | ❌ | ✅ |

## 🔮 **Future Enhancements**

- **Proper Enum Types**: Generate Rust enums from schema
- **Validation Rules**: Schema-defined component validation
- **Relationship Mapping**: Component dependency graphs
- **Auto-Documentation**: Generate docs from schemas
- **Migration Tools**: Automated old → new conversion
- **Performance Optimization**: Component packing/alignment

## 🤝 **Contributing**

1. **Add Components**: Update `cold/instances/component_types.yml`
2. **Modify Templates**: Edit `templates/component.rs.jinja`
3. **Extend Types**: Update `copier_setup.py` type mappings
4. **Test Changes**: Run schema validation and demo modes

## 📚 **Related**

- **Original**: [ecs-familiar](https://github.com/phaiel/ecs-familiar)
- **Hecs ECS**: [https://github.com/Ralith/hecs](https://github.com/Ralith/hecs)
- **Pydantic**: [https://docs.pydantic.dev/](https://docs.pydantic.dev/)
- **Copier**: [https://copier.readthedocs.io/](https://copier.readthedocs.io/)

---

**Schema-First ECS** • **Type-Safe Components** • **Runtime Introspection** • **Physics Law Binding** 