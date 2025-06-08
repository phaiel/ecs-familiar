#!/usr/bin/env python3
"""
Base functionality for schema-first generation.
Shared validation, loading, and utility functions.
"""

import os
import subprocess
import yaml
from pathlib import Path
from jinja2 import Environment, FileSystemLoader
from copier_setup import rust_typemap, rust_default, load_yaml


class SchemaLoader:
    """Handles loading and validation of YAML schemas."""
    
    def __init__(self):
        self.schema_data = {}
    
    def load_all_schemas(self):
        """Load all YAML schemas."""
        print("🔍 Loading YAML schemas...")
        
        # Load components (required)
        components_path = Path("cold/instances/component_types.yml")
        if not components_path.exists():
            raise FileNotFoundError(f"Component schema not found: {components_path}")
        self.schema_data['components'] = load_yaml(str(components_path))
        
        # Load laws (optional)
        laws_path = Path("cold/instances/laws.yml") 
        self.schema_data['laws'] = load_yaml(str(laws_path)) if laws_path.exists() else []
        
        # Load quantum laws (optional)
        quantum_laws_path = Path("cold/instances/quantum_laws.yml")
        self.schema_data['quantum_laws'] = load_yaml(str(quantum_laws_path)) if quantum_laws_path.exists() else []
        
        # Load DAG (optional)
        dag_path = Path("cold/instances/dag.yml")
        self.schema_data['dag_nodes'] = load_yaml(str(dag_path)) if dag_path.exists() else []
        
        # Load entity blueprints (optional)
        blueprints_path = Path("cold/instances/entity_blueprints.yml")
        self.schema_data['entity_blueprints'] = load_yaml(str(blueprints_path)) if blueprints_path.exists() else []
        
        # Load commands (optional) 
        commands_path = Path("cold/instances/commands.yml")
        self.schema_data['commands'] = load_yaml(str(commands_path)) if commands_path.exists() else []
        
        # Load configuration (optional)
        config_path = Path("cold/instances/configuration.yml")
        self.schema_data['configurations'] = load_yaml(str(config_path)) if config_path.exists() else []
        
        print(f"  ✅ Loaded {len(self.schema_data['components'])} components, {len(self.schema_data['laws'])} laws")
        print(f"  ✅ Loaded {len(self.schema_data['dag_nodes'])} DAG nodes, {len(self.schema_data['entity_blueprints'])} blueprints")
        print(f"  ✅ Loaded {len(self.schema_data['commands'])} commands, {len(self.schema_data['configurations'])} configs")
        print(f"  ✅ Loaded {len(self.schema_data['quantum_laws'])} quantum laws")
        
        return self.schema_data
    
    def validate_schemas(self):
        """Validate loaded schemas."""
        print("🔍 Validating schemas...")
        
        try:
            # Validate components
            for comp in self.schema_data['components']:
                if 'name' not in comp or 'fields' not in comp:
                    raise ValueError(f"Invalid component: {comp}")
            
            print("  ✅ Schema validation passed")
            return True
            
        except Exception as e:
            print(f"  ❌ Schema validation failed: {e}")
            raise


class TemplateRenderer:
    """Handles Jinja2 template rendering with custom filters."""
    
    def __init__(self):
        self.env = Environment(loader=FileSystemLoader('templates'))
        self.env.filters['rust_typemap'] = rust_typemap
        self.env.filters['rust_default'] = rust_default
    
    def render_template(self, template_name, **kwargs):
        """Render a template with given context."""
        template = self.env.get_template(template_name)
        return template.render(**kwargs)


class FileGenerator:
    """Handles file creation and directory management."""
    
    @staticmethod
    def ensure_directories():
        """Create necessary directory structure."""
        hot_path = Path("hot_path/src")
        gen_path = Path("hot_path/src/gen")
        dags_path = Path("dags")
        
        hot_path.mkdir(parents=True, exist_ok=True)
        gen_path.mkdir(parents=True, exist_ok=True)
        dags_path.mkdir(parents=True, exist_ok=True)
        
        return hot_path, gen_path, dags_path
    
    @staticmethod
    def write_file(file_path, content):
        """Write content to file and return size info."""
        Path(file_path).parent.mkdir(parents=True, exist_ok=True)
        with open(file_path, 'w') as f:
            f.write(content)
        
        size = len(content.splitlines())
        print(f"    ✅ Generated {file_path} ({size} lines)")
        return size


class CompilationTester:
    """Handles compilation testing."""
    
    @staticmethod
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
                return True
            else:
                print("  ❌ Compilation failed:")
                print(result.stderr)
                return False
                
        except FileNotFoundError:
            print("  ⚠️ Cargo not found, skipping compilation test")
            return True


class GenerationStats:
    """Handles generation statistics and summaries."""
    
    @staticmethod
    def show_summary(schema_data, generated_files):
        """Show generation summary."""
        print("\n" + "="*50)
        print("🧬 SCHEMA-FIRST GENERATION COMPLETE")
        print("="*50)
        
        components = schema_data.get('components', [])
        
        print(f"\n📊 Statistics:")
        print(f"  • Components generated: {len(components)}")
        
        # Count law relationships
        law_components = [c for c in components if c.get('laws')]
        print(f"  • Components with laws: {len(law_components)}")
        
        # Count mixins
        mixin_components = [c for c in components if c.get('mixins')]
        print(f"  • Components with mixins: {len(mixin_components)}")
        
        print(f"\n📁 Generated Files:")
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


def create_cargo_toml():
    """Create/update Cargo.toml with necessary dependencies."""
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
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[[bin]]
name = "ecs-familiar"
path = "src/main.rs"
"""
    
    with open("hot_path/Cargo.toml", 'w') as f:
        f.write(cargo_toml)
    print("  ✅ Updated Cargo.toml") 