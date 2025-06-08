#!/usr/bin/env python3
"""
Entities Generator: Entity Blueprints and Compositions.
Generates entity blueprints that define common component combinations.
"""

from pathlib import Path
from generators.base import TemplateRenderer, FileGenerator


class EntitiesGenerator:
    """Generates entity blueprints and composition definitions."""
    
    def __init__(self, schema_data):
        self.schema_data = schema_data
        self.renderer = TemplateRenderer()
        self.generated_files = []
    
    def generate_all(self):
        """Generate entity blueprints if they exist."""
        entity_blueprints = self.schema_data.get('entity_blueprints', [])
        
        if not entity_blueprints:
            print("⚠️ No entity blueprints found, skipping entity generation")
            return []
        
        print("🏗️ Generating entity blueprints...")
        
        # Ensure directory structure
        _, gen_path, _ = FileGenerator.ensure_directories()
        
        # Generate entity blueprints
        self.generate_entity_blueprints(gen_path, entity_blueprints)
        
        print(f"  🎯 Generated {len(self.generated_files)} entity files")
        return self.generated_files
    
    def generate_entity_blueprints(self, gen_path, entity_blueprints):
        """Generate entity_blueprints.rs with blueprint definitions."""
        print("  📋 Generating entity_blueprints.rs...")
        
        # Check if we have a template for entity blueprints
        try:
            content = self.renderer.render_template(
                'entity_blueprints.rs.jinja',
                entity_blueprints=entity_blueprints
            )
            
            file_path = gen_path / "entity_blueprints.rs"
            FileGenerator.write_file(file_path, content)
            self.generated_files.append(str(file_path))
            
        except Exception as e:
            print(f"    ⚠️ No entity_blueprints template found, creating stub: {e}")
            self.generate_stub_blueprints(gen_path, entity_blueprints)
    
    def generate_stub_blueprints(self, gen_path, entity_blueprints):
        """Generate a stub entity blueprints file."""
        print("  📋 Generating stub entity_blueprints.rs...")
        
        stub_content = """// GENERATED CODE — DO NOT EDIT
// Entity blueprints for common component combinations
use hecs::World;
use crate::gen::components::*;

/// Entity blueprints for spawning pre-configured entities
pub struct EntityBlueprints;

impl EntityBlueprints {
"""
        
        for blueprint in entity_blueprints:
            name = blueprint.get('name', 'Unknown')
            components = blueprint.get('components', [])
            
            # Extract component type names from the component list
            component_names = []
            for comp in components:
                if isinstance(comp, dict) and 'type' in comp:
                    component_names.append(comp['type'])
                elif isinstance(comp, str):
                    component_names.append(comp)
            
            stub_content += f"""
    /// Create a {name} entity - {blueprint.get('description', 'No description')}
    pub fn spawn_{name.lower()}(world: &mut World) -> hecs::Entity {{
        // TODO: Implement {name} blueprint with components: {', '.join(component_names)}
        world.spawn(())
    }}
"""
        
        stub_content += """
}
"""
        
        file_path = gen_path / "entity_blueprints.rs"
        FileGenerator.write_file(file_path, stub_content)
        self.generated_files.append(str(file_path))


def generate_entities(schema_data):
    """Main entry point for entity blueprints generation."""
    generator = EntitiesGenerator(schema_data)
    return generator.generate_all() 