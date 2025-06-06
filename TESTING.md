# Testing the Familiar Memory Visualization

## Quick Start Options

### Option 1: Automated Testing (Recommended)
Automatically starts the app and creates sample data:

```bash
./test_visualization.sh
```

This script will:
1. Start the Rust app in the background
2. Wait for the GraphQL server to be ready (up to 60 seconds)
3. Automatically create rich sample data
4. Keep the app running until you press Ctrl+C

### Option 2: Manual Testing
If you prefer to start the app manually:

```bash
# Terminal 1 - Start the app
cd hot_path
cargo run --release

# Terminal 2 - Create sample data (once app is running)
cd ..
./create_sample_data_manual.sh
```

## What You Should See

After sample data is created, look for:
- **🧵 Golden rope-like threads** with emissive glow
- **✨ Golden moment orbs** positioned along thread paths
- **🔗 Purple binding convergence effects** 
- **🎨 Dynamic animations** with time-based effects

## Controls
- **WASD** - Move camera
- **Mouse wheel** - Zoom in/out
- **Click entities** - Show info about memories
- **A/D keys** - Also moves timeline position

## Data Exploration
- Visit **http://127.0.0.1:8000** for GraphiQL interface
- Query the created memory network
- See real-time updates in the 3D visualization

## Troubleshooting

**Black/invisible threads?**
- Fixed in latest version with emissive materials and proper color handling

**Entities disappearing?**
- Extended decay half-lives: moments (5 min), threads (10 min)

**Server not starting?**
- Check port 8000 is available
- Look for compilation errors in the Rust output
- Make sure you're in the project root directory

**Sample data fails?**
- Ensure GraphQL server is running and accessible
- Check network connectivity to localhost:8000 