# 🔐 Familiar System SOC – Secure, Self-Organizing Architecture
This document defines the full system architecture for Familiar, integrating ECS-driven memory, AI agent orchestration, and a robust security framework. The architecture prioritizes temporal fidelity, self-organizing behavior, and scalable, schema-first memory execution.

## 1. ⚙️ Cold Path – Authoring & Schema Definition (Python)
* Language: Python (Pydantic v2 for schema)
* Schema Strategy:
    * Typed metadata (e.g. time, emotion:str, confidence:float)
    * Reusable field mixins
    * DAG and law schema validation via templates
    * Copier-based template generation
* Security Base Components:
    * org_id, owner_id, access_scope, security_level
    * components_present, object_type, sub_type, tags

## 2. 🔥 Hot Path – Execution (Rust)
### a. ECS Engine (hecs; Bevy for visualization only)
* Entities: Threads, Moments, Filaments, Motifs, Bonds
* Components:
    * System: UUID, timestamps, schema tags
    * Access: org, user, role, visibility, privacy_policy
    * Dynamic metadata: emotion, time, type, etc.
* Laws (executed on tick or observation):
    * decay: Time-based weakening of component presence
    * resonance: Reinforcement of repeated or harmonically aligned values
    * bonding: Tertiary relationship formation based on component affinity
    * consolidation: Motif and filament collapse at time intervals
### b. Memory
* Redis: Working memory (volatile, fast decay)
* ChronicleDB: Canonical, append-only object history
* OpenSearch: Vector + semantic retrieval (loci-aware)
* ECS executes laws lazily on observation or tick

## 3. 🧠 AI Agent Layer
* Orchestration: DAG-driven agent pipelines, minimal improvisation
* Agents:
    * Penates: Classifier (tree walk)
    * Decima: Pattern/Draft hydration and stitching
    * Nona: Weave synthesis, consolidation
* Input: Moirai (weave, units, draft/pattern)
* Output: Object creation (via GraphQL API to ECS/Chronicle)

## 4. 🔐 Security Framework (ECS-Law Driven)
### a. Security Component Types
| Component      | Description                                          |
|----------------|------------------------------------------------------|
| `org_id`       | Hard boundary for all security                       |
| `owner_id`     | User responsible for object                          |
| `visibility`   | Enum: private, household, org, public                |
| `security_level` | Numeric/enum control layer                           |
| `access_scope` | View/edit/control – field-level enforceable        |

### b. Enforcement Strategy
* System Law: `security_enforcement`
    * Runs before render or agent access
    * Filters access based on `org_id`, `visibility`, user-role
    * Optional policies for dynamic content (e.g., parent-only)
* Observed Entities:
    * Observation never cascades – observing a bond doesn't access threads
    * Security law guards retrieval and render boundaries
* Visual Security (Optional Bevy):
    * Rendered as veil/light-filter metaphor (e.g., blurred threads)

## 5. 🧬 Objects
| Object         | Description                         | Versioning      |
|----------------|-------------------------------------|-----------------|
| **Thread**     | Person/place/thing, stable          | Append-only     |
| **Moment**     | Discrete event, calendar-anchored   | Append-only     |
| **Filament**   | Condensed insight about a thread    | Mutable + GBU   |
| **Motif**      | Condensed insight about moments     | Mutable + GBU   |
| **Bond**       | Passive, emergent thread relation   | GC + capped     |
| **Binding**    | Explicit link to a moment           | Append-only     |

  * Observations are non-recursive, memory-efficient
* Filament/Motif consolidation occurs on ticks (daily → yearly)
* ECS observation reveals nearby context by component affinity

## 6. 🎨 Visualization Layer (Bevy)
* Render Metaphor: Weaving of light/threads
* Scope: Fully read-only, works from cached motif/filament/bond data
* Execution: iOS/WebGL capable, local or edge compute
* Visualization Strategy:
    * Highlight motifs (weight = brightness)
    * Display bonds as curved threads, tension = bond strength
    * Time scroll (GBU compliant)

## 7. 🧪 Execution Principles
* Laws are schema-driven: Cold path authors logic, hot path runs it
* Lazy ECS: Ticks or observation only; no constant polling
* Motif/Filament consolidation offloaded to agent layer
* Agent computation is separate from ECS/laws
* Bonds are conservative: Depth-capped, pruned via async GC
* Append-only GBU: Full replay via ChronicleDB

## 8. 🧭 MVP Guidance
* MVP uses `sample_obj1`, `sample_obj2`... until Familiar data is layered in
* ECS engine: `hecs` for MVP (no render)
* DAGs: YAML schema validated + cached
* Redis + ChronicleDB + OpenSearch integrated
* GraphQL API scaffolded via Rust (async interface for agents)

## 9. 🔍 Open Work / Deferred Scope
* Rule authoring for DAG laws
* Motif/Filament locus-based cache preloading
* Security tier hierarchy expansion (e.g. child-only, parent-only)
* Visual debugger (timeline + ECS log replay)
* Agent tools for schema auto-completion and component recommendation 