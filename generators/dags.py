#!/usr/bin/env python3
"""
DAGs Generator: Windmill DAG Nodes for QuTiP Calculations.
Generates DAG nodes that execute Python QuTiP calculations.
"""

from pathlib import Path
from generators.base import TemplateRenderer, FileGenerator


class DAGsGenerator:
    """Generates Windmill DAG nodes for quantum calculations."""
    
    def __init__(self, schema_data):
        self.schema_data = schema_data
        self.renderer = TemplateRenderer()
        self.generated_files = []
    
    def generate_all(self):
        """Generate DAG nodes for quantum laws."""
        quantum_laws = self.schema_data.get('quantum_laws', [])
        
        if not quantum_laws:
            print("⚠️ No quantum laws found, skipping DAG generation")
            return []
        
        print("🧭 Generating Windmill DAG nodes...")
        
        # Ensure directory structure
        _, _, dags_path = FileGenerator.ensure_directories()
        
        # Generate DAG nodes
        self.generate_dag_nodes(dags_path, quantum_laws)
        
        print(f"  🎯 Generated {len(self.generated_files)} DAG files")
        return self.generated_files
    
    def generate_dag_nodes(self, dags_path, quantum_laws):
        """Generate Windmill DAG YAML files for each quantum law."""
        print("  📋 Generating DAG nodes...")
        
        for law in quantum_laws:
            content = self.renderer.render_template(
                'windmill_dag.yml.jinja',
                law=law
            )
            
            file_path = dags_path / f"{law['name']}_dag.yml"
            FileGenerator.write_file(file_path, content)
            self.generated_files.append(str(file_path))
            print(f"    ✅ Generated dags/{law['name']}_dag.yml")


def generate_dags(schema_data):
    """Main entry point for DAG nodes generation."""
    generator = DAGsGenerator(schema_data)
    return generator.generate_all() 