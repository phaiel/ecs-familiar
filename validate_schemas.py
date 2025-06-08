#!/usr/bin/env python3
"""
Schema validation script: Use Pydantic to validate YAML schemas.
Ensures schema correctness before code generation.
"""

from cold_path.schema_integration import (
    load_component_schema, 
    ComponentRegistry, 
    generate_pydantic_models
)
from pathlib import Path


def validate_component_schemas():
    """Validate component schemas using Pydantic."""
    print("🔍 Validating Component Schemas with Pydantic")
    print("=" * 45)
    
    try:
        # Load and validate schema
        registry = load_component_schema()
        print(f"✅ Loaded {len(registry.components)} components and {len(registry.laws)} laws")
        
        # Show components
        print("\n📋 Validated Components:")
        for comp in registry.components:
            laws_str = f" (laws: {', '.join(comp.laws)})" if comp.laws else ""
            mixins_str = f" (mixins: {', '.join(comp.mixins)})" if comp.mixins else ""
            print(f"  • {comp.name} v{comp.version}{laws_str}{mixins_str}")
        
        # Show law applications
        print("\n⚖️ Law Applications:")
        for law in registry.laws:
            affected = registry.get_components_for_law(law.name)
            comp_names = [c.name for c in affected]
            print(f"  • {law.name} → {', '.join(comp_names)}")
        
        # Validate field types
        print("\n🔧 Field Type Validation:")
        all_field_types = set()
        for comp in registry.components:
            for field in comp.fields:
                all_field_types.add(field.type.value)
        
        for field_type in sorted(all_field_types):
            print(f"  ✅ {field_type}")
        
        # Generate validation report
        print("\n📊 Validation Report:")
        
        # Check for components without laws
        no_law_components = [c for c in registry.components if not c.laws]
        print(f"  • Components without laws: {len(no_law_components)}")
        
        # Check for components with mixins
        mixin_components = [c for c in registry.components if c.mixins]
        print(f"  • Components with mixins: {len(mixin_components)}")
        
        # Check for default values
        default_fields = []
        for comp in registry.components:
            for field in comp.fields:
                if field.default is not None:
                    default_fields.append(f"{comp.name}.{field.name}")
        print(f"  • Fields with defaults: {len(default_fields)}")
        
        return True
        
    except Exception as e:
        print(f"❌ Schema validation failed: {e}")
        import traceback
        traceback.print_exc()
        return False


def check_schema_consistency():
    """Check for schema consistency issues."""
    print("\n🔍 Checking Schema Consistency")
    print("-" * 30)
    
    try:
        registry = load_component_schema()
        errors = registry.validate_law_consistency()
        
        if not errors:
            print("✅ No consistency issues found")
            return True
        else:
            print("❌ Consistency issues found:")
            for error in errors:
                print(f"  • {error}")
            return False
            
    except Exception as e:
        print(f"❌ Consistency check failed: {e}")
        return False


def generate_pydantic_preview():
    """Generate preview of Pydantic models."""
    print("\n🐍 Pydantic Model Preview")
    print("-" * 25)
    
    try:
        registry = load_component_schema()
        pydantic_code = generate_pydantic_models(registry)
        
        # Show first few models as preview
        lines = pydantic_code.split('\n')
        preview_lines = lines[:30]  # First 30 lines
        
        print("Generated Pydantic validation models:")
        for i, line in enumerate(preview_lines, 1):
            print(f"{i:2d}: {line}")
        
        if len(lines) > 30:
            print(f"   ... and {len(lines) - 30} more lines")
        
        return True
        
    except Exception as e:
        print(f"❌ Pydantic generation failed: {e}")
        return False


def check_yaml_files():
    """Check that all required YAML files exist."""
    print("\n📁 Checking YAML Files")
    print("-" * 20)
    
    required_files = [
        "cold/components.yaml",
        "cold/instances/component_types.yml",
    ]
    
    all_exist = True
    for file_path in required_files:
        path = Path(file_path)
        if path.exists():
            size = path.stat().st_size
            print(f"  ✅ {file_path} ({size} bytes)")
        else:
            print(f"  ❌ {file_path} (missing)")
            all_exist = False
    
    return all_exist


def main():
    """Main validation pipeline."""
    print("🔍 ECS Familiar - Schema Validation Pipeline")
    print("=" * 45)
    print("Using Pydantic to validate YAML schemas before code generation")
    
    success = True
    
    # Step 1: Check YAML files exist
    if not check_yaml_files():
        print("\n❌ Required YAML files missing")
        return 1
    
    # Step 2: Validate schemas with Pydantic
    if not validate_component_schemas():
        print("\n❌ Schema validation failed")
        success = False
    
    # Step 3: Check consistency
    if not check_schema_consistency():
        print("\n❌ Schema consistency check failed")
        success = False
    
    # Step 4: Generate Pydantic preview
    if not generate_pydantic_preview():
        print("\n❌ Pydantic model generation failed")
        success = False
    
    # Final result
    if success:
        print("\n✨ All schema validations passed!")
        print("🎯 Schemas are ready for code generation")
        print("\nNext step: Run `python3 generate_all.py` to generate hot path")
        return 0
    else:
        print("\n❌ Schema validation failed")
        print("🔧 Fix the issues above before running code generation")
        return 1


if __name__ == "__main__":
    exit(main()) 