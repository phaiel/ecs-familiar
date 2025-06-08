#!/usr/bin/env python3
"""
Direct component generation using Jinja2 templates.
Generates Rust Hecs components from YAML schema definitions.
"""

import yaml
from pathlib import Path
from jinja2 import Environment, FileSystemLoader
from copier_setup import rust_typemap, rust_default, load_yaml


def generate_rust_components(output_path: str = "hot_path/src/components_generated.rs"):
    """Generate Rust components from YAML schema."""
    
    # Load component data
    component_types = load_yaml("cold/instances/component_types.yml")
    print(f"📋 Loaded {len(component_types)} component types")
    
    # Set up Jinja2 environment
    env = Environment(loader=FileSystemLoader('templates'))
    env.filters['rust_typemap'] = rust_typemap
    env.filters['rust_default'] = rust_default
    
    # Load template
    template = env.get_template('component.rs.jinja')
    
    # Generate Rust code
    rust_code = template.render(component_types=component_types)
    
    # Write to output file
    output_file = Path(output_path)
    output_file.parent.mkdir(parents=True, exist_ok=True)
    
    with open(output_file, 'w') as f:
        f.write(rust_code)
    
    print(f"✅ Generated Rust components: {output_path}")
    print(f"📏 Generated {len(rust_code.splitlines())} lines of code")
    
    # Show some statistics
    print(f"\n📊 Component Statistics:")
    laws_components = [c for c in component_types if c.get('laws')]
    mixin_components = [c for c in component_types if c.get('mixins')]
    
    print(f"  • Total components: {len(component_types)}")
    print(f"  • Components with laws: {len(laws_components)}")
    print(f"  • Components with mixins: {len(mixin_components)}")
    
    # Show components by law
    law_map = {}
    for comp in component_types:
        for law in comp.get('laws', []):
            if law not in law_map:
                law_map[law] = []
            law_map[law].append(comp['name'])
    
    if law_map:
        print(f"\n⚖️ Law Applications:")
        for law, comps in law_map.items():
            print(f"  • {law}: {', '.join(comps)}")
    
    return output_file


if __name__ == "__main__":
    print("🚀 Generating Rust Components from YAML Schema")
    print("="*50)
    
    try:
        output_file = generate_rust_components()
        
        # Show first few lines of generated code
        print(f"\n📝 Preview of generated code:")
        with open(output_file, 'r') as f:
            lines = f.readlines()[:20]
            for i, line in enumerate(lines, 1):
                print(f"{i:2d}: {line.rstrip()}")
            if len(lines) == 20:
                print("   ...")
                
    except Exception as e:
        print(f"❌ Error: {e}")
        import traceback
        traceback.print_exc() 