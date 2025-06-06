# 🎨 Familiar Memory Visualization Guide

This guide shows you all the ways to visualize and explore your memory simulation schema and data.

## 📊 Visualization Options

### 1. **3D Memory Space** (Real-time Visualization)
**File**: The main Rust application
**Access**: Run `./test_visualization.sh` or `cargo run --release` in `hot_path/`
**Features**:
- ✨ Interactive 3D space with golden threads and moment orbs
- 🎮 WASD movement, mouse wheel zoom, click for entity info
- 🔗 Real-time convergence effects showing thread relationships
- ⚡ Dynamic animations and decay systems

### 2. **Schema Structure Viewer** (Static Schema)
**File**: `schema_visualizer.html`
**Access**: Open directly in your browser
**Features**:
- 📋 Visual cards showing all entity types (Moment, Thread, Binding, etc.)
- 🔍 Field details with types and relationships
- 🎨 Color-coded entity categories
- 🔗 Quick link to GraphiQL when server is running

### 3. **GraphiQL Explorer** (Interactive API)
**File**: Built into the Rust app
**Access**: http://127.0.0.1:8000 when app is running
**Features**:
- 📖 Built-in schema documentation browser
- ⚡ Auto-completion for queries and mutations
- 🔄 Real-time query execution
- 📝 Query history and saved queries

### 4. **Query Explorer** (Custom Interface)
**File**: `query_explorer.html`  
**Access**: Open in browser while app is running
**Features**:
- 🚀 Pre-built common queries (List Threads, Moments, Bonds, etc.)
- 📊 Live data statistics dashboard
- 🔧 Custom query builder with syntax highlighting
- ⌨️ Keyboard shortcuts (Ctrl+Enter to execute)

## 🚀 Quick Start

1. **Start the Memory Simulation**:
   ```bash
   ./test_visualization.sh
   ```

2. **Open Visualization Tools**:
   - **3D View**: Automatically opens with the app
   - **Schema Reference**: Open `schema_visualizer.html` in browser
   - **GraphiQL**: Visit http://127.0.0.1:8000
   - **Query Explorer**: Open `query_explorer.html` in browser

## 📚 Common Tasks

### View All Memory Threads
```graphql
query {
  threads {
    name
    thread_type
    creation_time
  }
}
```

### Explore Moments on a Thread
```graphql
query {
  momentsByThread(threadId: "Alice") {
    text
    timestamp
    emotional_charge
  }
}
```

### See Thread Relationships
```graphql
query {
  bonds {
    thread_ids
    affinity_score
    bond_strength
  }
}
```

### Create New Memory Data
```graphql
mutation {
  createThread(name: "NewMemory", threadType: "experience") {
    success
    message
  }
  createMoment(text: "A new memory", threadId: "NewMemory") {
    success
    message
  }
}
```

## 🎯 Understanding the Data

### **Entities Overview**:
- **🧵 Threads**: Memory categories (people, places, events)
- **✨ Moments**: Individual memories scoped to threads
- **🔗 Bindings**: Cross-thread connections
- **🔗 Bonds**: Relationship strength between threads
- **🎨 Motifs**: Emerging patterns from multiple memories
- **🌱 Filaments**: Contextual interpretations

### **Relationships**:
- Moments belong to exactly one Thread
- Bindings create convergence between Moments and Threads
- Bonds connect Threads with affinity scores
- Motifs emerge from patterns across multiple Moments

## 🛠️ Troubleshooting

**Schema tools show "Server Offline"?**
- Make sure the Rust app is running: `cd hot_path && cargo run --release`
- Check that port 8000 is available

**3D visualization shows no entities?**
- Run the sample data script: `./create_sample_data_manual.sh`
- Or use the Query Explorer to create test data

**GraphiQL not loading?**
- Verify the app started successfully (look for "GraphiQL IDE listening" message)
- Try refreshing the browser or clearing cache

## 🎨 Customization

**Want different sample data?**
- Edit `create_sample_data.sh` to create your own memory scenarios
- Use the Query Explorer's "Create Sample Data" button for quick tests

**Need different queries?**
- Modify the `queries` object in `query_explorer.html`
- Use GraphiQL's query builder for complex explorations

**Customize 3D visualization?**
- Edit materials and shaders in `hot_path/src/materials.rs`
- Adjust entity sizes and positioning in `hot_path/src/main.rs` 