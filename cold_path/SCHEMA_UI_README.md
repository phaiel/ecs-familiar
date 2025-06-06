# Familiar Memory - Pydantic Schema UI

This uses the proper [pydantic-ui](https://github.com/prismaticd/pydantic-ui) library to provide a Django-admin-like interface for viewing and editing your actual Pydantic schemas.

## 🚀 Quick Start

```bash
cd cold_path
python main.py
```

Then visit: **http://127.0.0.1:8003**

## 📊 Available UIs

The system provides interactive interfaces for all your actual Pydantic models:

- **BaseEntityUI** - Core entity with UUIDs, visibility, access controls, soft deletion, and versioning
- **BaseComponentUI** - Base component with metadata system
- **TimeComponentUI** - Time-based components with timestamps
- **EmotionComponentUI** - Emotional data with confidence scores
- **ThreadLinkComponentUI** - Thread relationship components
- **MomentUI** - Individual memory moments with ECS-valid structure  
- **BindingUI** - Cross-thread connections as composite of binding points
- **ThreadUI** - Memory threads organizing related moments
- **LawUI** - System behavior rules (decay, resonance, binding)
- **SettingsUI** - Application configuration
- **DagUI** - Directed acyclic graphs for processing pipelines
- **DagTaskUI** - Individual tasks within DAGs

## ✨ Features

- **📋 Data Tables** - View all instances of each model
- **✏️ Form Editing** - Edit model instances with proper validation
- **🔍 Schema Inspection** - View full JSON schemas
- **✅ Validation** - Real-time validation using your actual Pydantic models
- **🎯 Type Safety** - Uses your exact schema definitions, not hardcoded examples

## 🛠️ Alternative Tools

For quick schema inspection without the web UI:

```bash
# Pretty terminal output
python inspect_schemas.py

# Export JSON schemas
python inspect_schemas.py --export
```

## 🆚 Why This is Better

Unlike the previous hardcoded GraphQL docs, this:

- ✅ Uses your **actual** Pydantic schemas
- ✅ Shows **real** inheritance relationships
- ✅ Displays **proper** field types (UUIDs, Enums, Unions)
- ✅ Includes **actual** validation constraints
- ✅ Provides **live** editing and validation
- ✅ Uses an **off-the-shelf** mature library

## 📁 File Structure

```
cold_path/
├── main.py              # FastAPI app using pydantic-ui
├── familiar_ui.py       # UI model definitions for your schemas
├── inspect_schemas.py   # CLI tool for quick schema inspection
├── pydantic_ui/         # Pydantic-UI library (templates, etc.)
├── static/              # Static files for the web UI
└── src/familiar/        # Your actual Pydantic schema definitions
```

The web UI automatically discovers and provides interfaces for all your Pydantic models without any hardcoded examples or fake data.

## 🆕 Recent Schema Enhancements

### Enhanced BaseEntity
- ✅ **Explicit Enums**: `Visibility` and `AccessScope` with full Pydantic/OpenAPI compatibility
- ✅ **Soft Deletion**: `deleted_at` field for "trash" concept without immediate pruning
- ✅ **Updated Access Levels**: `AccessScope.ADMIN` instead of `CONTROL` for clearer permissions
- 🕒 **Built-in Versioning**: Direct version tracking with parent-child relationships

### New Domain Entities (in `entities.py`)
- 📝 **Moment**: ECS-valid memory moments with DAG grouping hints
- 🔗 **Binding**: Composite of binding points with thread ↔ moment ↔ role mapping
- 📍 **BindingPoint**: 1-to-1-to-1 mapping of thread, moment, and semantic role
- 🧵 **Thread**: Memory threads organizing related moments
- 🎭 **Cardinality**: Semantic roles (actor, observer, recipient, co-participant, target)

### Schema Features
```python
# BaseEntity with built-in versioning and soft deletion
class BaseEntity(BaseModel):
    deleted_at: Optional[datetime] = None  # Soft deletion
    visibility: Visibility = Visibility.PRIVATE  # Explicit enum
    access_scope: List[AccessScope] = [AccessScope.VIEW]  # Clear permissions
    version: int = Field(default=1)  # Built-in versioning
    parent_version: Optional[int] = None  # History chain

# Semantic role mapping  
class Cardinality(str, Enum):
    ACTOR = "actor"
    OBSERVER = "observer"
    RECIPIENT = "recipient"
    CO_PARTICIPANT = "co-participant"
    TARGET = "target"

# Domain entities (in entities.py)
class BindingPoint(BaseModel):
    thread_id: UUID
    moment_id: UUID
    cardinality: Literal['actor', 'observer', 'recipient', 'co-participant', 'target']

class Binding(BaseEntity):
    sub_type: Literal['binding'] = 'binding'
    points: List[BindingPoint] = []  # stable, semantic, per-thread per-moment
    thread_ids: List[UUID] = Field(default_factory=list)  # optional index optimization

class Moment(BaseEntity):
    sub_type: Literal['moment'] = 'moment'
    thread_id: UUID               # Thread this moment belongs to
    author_id: UUID               # Author (user or agent)
    binding_hint: Optional[int] = None  # Used by DAG to group moments (not persisted long-term)
    binding_id: Optional[UUID] = None   # Set by DAG if included in a binding
    cardinality: Optional[Cardinality] = None
```

### Key Design Features
- 🏗️ **ECS-Compatible**: BaseComponent/BaseEntity structure supports entity-component-system patterns
- 🔄 **Version Tracking**: Built into BaseEntity with version and parent_version fields  
- ✅ **Already-Created Moments**: Binding references existing moments, no shared context generation
- 📍 **1-to-1-to-1 Mapping**: Each BindingPoint maps exactly one thread, one moment, one cardinality role
- 🔗 **DAG Workflow**: `binding_hint` groups moments → DAG creates Binding → `binding_id` backrefs
- ⚡ **Query Optimization**: `thread_ids` denormalized index enables efficient thread-based binding queries 