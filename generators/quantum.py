#!/usr/bin/env python3
"""
Quantum Generator: Quantum Systems and Event Emitters.
Generates quantum law implementations that emit events to Redpanda.
"""

from pathlib import Path
from generators.base import TemplateRenderer, FileGenerator


class QuantumGenerator:
    """Generates quantum systems for ECS quantum law implementations."""
    
    def __init__(self, schema_data):
        self.schema_data = schema_data
        self.renderer = TemplateRenderer()
        self.generated_files = []
    
    def generate_all(self):
        """Generate quantum systems if quantum laws exist."""
        quantum_laws = self.schema_data.get('quantum_laws', [])
        
        if not quantum_laws:
            print("⚠️ No quantum laws found, skipping quantum generation")
            return []
        
        print("🔬 Generating quantum systems...")
        
        # Ensure directory structure
        _, gen_path, _ = FileGenerator.ensure_directories()
        
        # Generate quantum systems
        self.generate_quantum_systems(gen_path, quantum_laws)
        
        print(f"  🎯 Generated {len(self.generated_files)} quantum files")
        return self.generated_files
    
    def generate_quantum_systems(self, gen_path, quantum_laws):
        """Generate quantum_systems.rs with QuTiP event emitters."""
        print("  ⚛️ Generating quantum_systems.rs...")
        
        content = self.renderer.render_template(
            'quantum_system.rs.jinja',
            quantum_laws=quantum_laws
        )
        
        file_path = gen_path / "quantum_systems.rs"
        FileGenerator.write_file(file_path, content)
        self.generated_files.append(str(file_path))


def generate_quantum(schema_data):
    """Main entry point for quantum systems generation."""
    generator = QuantumGenerator(schema_data)
    return generator.generate_all() 