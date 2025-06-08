#!/usr/bin/env python3
"""
Master generation script: Generate entire hot path from cold path YAML schemas.
Uses Copier templates to create 100% schema-driven ECS code.
"""

import os
import subprocess
import yaml
from pathlib import Path
from jinja2 import Environment, FileSystemLoader
from copier_setup import rust_typemap, rust_default, load_yaml


def validate_schemas():
    """Validate YAML schemas before generation."""
    print("🔍 Validating YAML schemas...")
    
    schema_data = {}
    
    # Load components
    components_path = Path("cold/instances/component_types.yml")
    if not components_path.exists():
        raise FileNotFoundError(f"Component schema not found: {components_path}")
    schema_data['components'] = load_yaml(str(components_path))
    
    # Load laws (optional)
    laws_path = Path("cold/instances/laws.yml") 
    schema_data['laws'] = load_yaml(str(laws_path)) if laws_path.exists() else []
    
    # Load DAG (optional)
    dag_path = Path("cold/instances/dag.yml")
    schema_data['dag_nodes'] = load_yaml(str(dag_path)) if dag_path.exists() else []
    
    # Load entity blueprints (optional)
    blueprints_path = Path("cold/instances/entity_blueprints.yml")
    schema_data['entity_blueprints'] = load_yaml(str(blueprints_path)) if blueprints_path.exists() else []
    
    # Load commands (optional) 
    commands_path = Path("cold/instances/commands.yml")
    schema_data['commands'] = load_yaml(str(commands_path)) if commands_path.exists() else []
    
    # Load configuration (optional)
    config_path = Path("cold/instances/configuration.yml")
    schema_data['configurations'] = load_yaml(str(config_path)) if config_path.exists() else []
    
    print(f"  ✅ Loaded {len(schema_data['components'])} components, {len(schema_data['laws'])} laws")
    print(f"  ✅ Loaded {len(schema_data['dag_nodes'])} DAG nodes, {len(schema_data['entity_blueprints'])} blueprints")
    print(f"  ✅ Loaded {len(schema_data['commands'])} commands, {len(schema_data['configurations'])} configs")
    
    try:
        # Validate components
        for comp in schema_data['components']:
            if 'name' not in comp or 'fields' not in comp:
                raise ValueError(f"Invalid component: {comp}")
        
        print("  ✅ Schema validation passed")
        return schema_data
        
    except Exception as e:
        print(f"  ❌ Schema validation failed: {e}")
        raise


def generate_hot_path(schema_data):
    """Generate entire hot path using Copier templates."""
    print("🏗️ Generating hot path from schemas...")
    
    # Set up Jinja2 environment with custom filters
    env = Environment(loader=FileSystemLoader('templates'))
    env.filters['rust_typemap'] = rust_typemap
    env.filters['rust_default'] = rust_default
    
    # Create proper directory structure following best practices
    hot_path = Path("hot_path/src")
    gen_path = Path("hot_path/src/gen")  # Generated code goes in src/gen/
    hot_path.mkdir(parents=True, exist_ok=True)
    gen_path.mkdir(parents=True, exist_ok=True)
    
    # Generate into src/gen/ following best practices
    print("  📦 Generating components.rs...")
    component_template = env.get_template('component.rs.jinja')
    component_code = component_template.render(component_types=schema_data['components'])
    
    with open(gen_path / "components.rs", 'w') as f:
        f.write(component_code)
    print(f"    ✅ Generated src/gen/components.rs ({len(component_code.splitlines())} lines)")
    
    # Generate systems.rs
    print("  ⚖️ Generating systems.rs...")
    systems_template = env.get_template('systems.rs.jinja')
    systems_code = systems_template.render(component_types=schema_data['components'])
    
    with open(gen_path / "systems.rs", 'w') as f:
        f.write(systems_code)
    print(f"    ✅ Generated src/gen/systems.rs ({len(systems_code.splitlines())} lines)")
    
    # Generate main.rs
    print("  🚀 Generating main.rs...")
    main_template = env.get_template('main.rs.jinja')
    main_code = main_template.render(component_types=schema_data['components'])
    
    with open(hot_path / "main.rs", 'w') as f:
        f.write(main_code)
    print(f"    ✅ Generated main.rs ({len(main_code.splitlines())} lines)")
    
    # Generate lib.rs with proper module references
    print("  📚 Generating lib.rs...")
    lib_content = """// GENERATED CODE — DO NOT EDIT
// Generated via Copier from cold path YAML schemas

pub mod gen {
    pub mod components;
    pub mod systems;
}

// Re-export for convenience
pub use gen::components::*;
pub use gen::systems::*;
"""
    
    with open(hot_path / "lib.rs", 'w') as f:
        f.write(lib_content)
    print(f"    ✅ Generated lib.rs")


def update_cargo_toml():
    """Update Cargo.toml with necessary dependencies."""
    print("📦 Updating Cargo.toml...")
    
    cargo_toml = """[package]
name = "familiar_hot_path"
version = "0.1.0"
edition = "2021"

[dependencies]
hecs = "0.10.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
colored = "2.0"

[[bin]]
name = "ecs-familiar"
path = "src/main.rs"
"""
    
    with open("hot_path/Cargo.toml", 'w') as f:
        f.write(cargo_toml)
    print("  ✅ Updated Cargo.toml with Hecs dependencies")


def test_compilation():
    """Test that generated code compiles."""
    print("🔧 Testing compilation...")
    
    try:
        result = subprocess.run(
            ["cargo", "check", "--manifest-path", "hot_path/Cargo.toml"],
            capture_output=True,
            text=True,
            cwd="."
        )
        
        if result.returncode == 0:
            print("  ✅ Compilation successful!")
        else:
            print("  ❌ Compilation failed:")
            print(result.stderr)
            return False
            
    except FileNotFoundError:
        print("  ⚠️ Cargo not found, skipping compilation test")
        return True
    
    return True


def show_generation_summary(components):
    """Show summary of what was generated."""
    print("\n" + "="*50)
    print("🧬 SCHEMA-FIRST GENERATION COMPLETE")
    print("="*50)
    
    print(f"\n📊 Statistics:")
    print(f"  • Components generated: {len(components)}")
    
    # Count law relationships
    law_components = [c for c in components if c.get('laws')]
    print(f"  • Components with laws: {len(law_components)}")
    
    # Count mixins
    mixin_components = [c for c in components if c.get('mixins')]
    print(f"  • Components with mixins: {len(mixin_components)}")
    
    print(f"\n📁 Generated Files:")
    generated_files = [
        "hot_path/src/gen/components.rs",
        "hot_path/src/gen/systems.rs", 
        "hot_path/src/main.rs",
        "hot_path/src/lib.rs",
        "hot_path/Cargo.toml"
    ]
    
    for file_path in generated_files:
        if Path(file_path).exists():
            size = Path(file_path).stat().st_size
            print(f"  ✅ {file_path} ({size} bytes)")
        else:
            print(f"  ❌ {file_path} (missing)")
    
    print(f"\n⚖️ Physics Laws:")
    for comp in components:
        if comp.get('laws'):
            print(f"  • {comp['name']}: {', '.join(comp['laws'])}")
    
    print(f"\n🚀 Usage:")
    print(f"  cd hot_path && cargo run --bin ecs-familiar")
    print(f"  cd hot_path && cargo run --bin ecs-familiar -- --schema-test")
    print(f"  cd hot_path && cargo run --bin ecs-familiar -- --component-demo")


def main():
    """Main generation pipeline."""
    print("🧬 ECS Familiar - Schema-First Generation Pipeline")
    print("=" * 50)
    
    try:
        # Step 1: Validate schemas
        schema_data = validate_schemas()
        
        # Step 2: Generate hot path
        generate_hot_path(schema_data)
        
        # Step 3: Update Cargo.toml
        update_cargo_toml()
        
        # Step 4: Test compilation
        if test_compilation():
            # Step 5: Show summary
            show_generation_summary(schema_data['components'])
            
            print(f"\n✨ Schema-first generation completed successfully!")
            print(f"🎯 Hot path is now 100% generated from cold path YAML schemas")
            
        else:
            print(f"\n❌ Generation completed but compilation failed")
            return 1
            
    except Exception as e:
        print(f"\n❌ Generation failed: {e}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0


if __name__ == "__main__":
    exit(main()) 