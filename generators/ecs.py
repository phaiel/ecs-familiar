#!/usr/bin/env python3
"""
ECS Generator: Components, Systems, and Main Rust code.
This is the core generator that creates the Hecs ECS architecture.
"""

from pathlib import Path
from generators.base import TemplateRenderer, FileGenerator, create_cargo_toml


class ECSGenerator:
    """Generates ECS components, systems, and main application code."""
    
    def __init__(self, schema_data):
        self.schema_data = schema_data
        self.renderer = TemplateRenderer()
        self.generated_files = []
    
    def generate_all(self):
        """Generate all ECS code: components, systems, main.rs, lib.rs."""
        print("🏗️ Generating ECS code from schemas...")
        
        # Ensure directory structure
        hot_path, gen_path, _ = FileGenerator.ensure_directories()
        
        # Generate core ECS files
        self.generate_components(gen_path)
        self.generate_systems(gen_path)
        self.generate_main(hot_path)
        self.generate_lib(hot_path)
        
        # Update Cargo.toml
        create_cargo_toml()
        self.generated_files.append("hot_path/Cargo.toml")
        
        print(f"  🎯 Generated {len(self.generated_files)} ECS files")
        return self.generated_files
    
    def generate_components(self, gen_path):
        """Generate components.rs with all component definitions."""
        print("  📦 Generating components.rs...")
        
        content = self.renderer.render_template(
            'components.rs.jinja',
            component_types=self.schema_data['components']
        )
        
        file_path = gen_path / "components.rs"
        FileGenerator.write_file(file_path, content)
        self.generated_files.append(str(file_path))
    
    def generate_systems(self, gen_path):
        """Generate systems.rs with physics law implementations."""
        print("  ⚖️ Generating systems.rs...")
        
        content = self.renderer.render_template(
            'systems.rs.jinja',
            component_types=self.schema_data['components']
        )
        
        file_path = gen_path / "systems.rs"
        FileGenerator.write_file(file_path, content)
        self.generated_files.append(str(file_path))
    
    def generate_main(self, hot_path):
        """Generate main.rs with ECS application."""
        print("  🚀 Generating main.rs...")
        
        content = self.renderer.render_template(
            'main.rs.jinja',
            component_types=self.schema_data['components']
        )
        
        file_path = hot_path / "main.rs"
        FileGenerator.write_file(file_path, content)
        self.generated_files.append(str(file_path))
    
    def generate_lib(self, hot_path):
        """Generate lib.rs with module exports."""
        print("  📚 Generating lib.rs...")
        
        # Check if quantum systems will be generated
        has_quantum = bool(self.schema_data.get('quantum_laws'))
        
        lib_content = """// GENERATED CODE — DO NOT EDIT
// Generated via Copier from cold path YAML schemas

pub mod gen {
    pub mod components;
    pub mod systems;"""
        
        if has_quantum:
            lib_content += """
    pub mod quantum_systems;"""
        
        lib_content += """
}

// Re-export for convenience
pub use gen::components::*;
pub use gen::systems::*;"""
        
        if has_quantum:
            lib_content += """
pub use gen::quantum_systems::*;"""
        
        lib_content += """
"""
        
        file_path = hot_path / "lib.rs"
        FileGenerator.write_file(file_path, lib_content)
        self.generated_files.append(str(file_path))


def generate_ecs(schema_data):
    """Main entry point for ECS generation."""
    generator = ECSGenerator(schema_data)
    return generator.generate_all() 