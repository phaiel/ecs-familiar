#!/usr/bin/env python3
"""
Pydantic integration for schema-first component definitions.
Bridges YAML schemas with Python validation and cold path processing.
"""

from typing import List, Dict, Any, Optional, Union
from pydantic import BaseModel, Field, validator
from enum import Enum
import yaml
from pathlib import Path
from datetime import datetime
import uuid


class FieldType(str, Enum):
    """Supported field types in component schemas."""
    INT = "int"
    FLOAT = "float"
    STR = "str"
    DATETIME = "datetime"
    BOOL = "bool"
    VECTOR = "vector"
    ENUM = "enum"


class ComponentField(BaseModel):
    """Individual field definition within a component."""
    name: str = Field(..., description="Field name")
    type: FieldType = Field(..., description="Field type")
    default: Optional[Any] = Field(None, description="Default value")
    
    @validator('default')
    def validate_default(cls, v, values):
        """Validate default value matches field type."""
        if v is None:
            return v
        
        field_type = values.get('type')
        if field_type == FieldType.INT and not isinstance(v, int):
            raise ValueError(f"Default value must be int, got {type(v)}")
        elif field_type == FieldType.FLOAT and not isinstance(v, (int, float)):
            raise ValueError(f"Default value must be float, got {type(v)}")
        elif field_type == FieldType.STR and not isinstance(v, str):
            raise ValueError(f"Default value must be string, got {type(v)}")
        elif field_type == FieldType.BOOL and not isinstance(v, bool):
            raise ValueError(f"Default value must be bool, got {type(v)}")
        
        return v


class ComponentType(BaseModel):
    """Schema-first component type definition."""
    name: str = Field(..., description="Component type identifier")
    version: int = Field(1, description="Component schema version")
    mixins: List[str] = Field(default_factory=list, description="Component mixins")
    laws: List[str] = Field(default_factory=list, description="Physics laws affecting this component")
    fields: List[ComponentField] = Field(..., description="Component fields")
    
    @validator('name')
    def validate_name(cls, v):
        """Ensure component name is valid Rust identifier."""
        if not v.isidentifier():
            raise ValueError(f"Component name must be valid identifier: {v}")
        if not v[0].isupper():
            raise ValueError(f"Component name must start with uppercase: {v}")
        return v


class LawSpecification(BaseModel):
    """Physics law specification for the schema-driven system."""
    name: str = Field(..., description="Law name")
    formula: str = Field(..., description="Mathematical formula")
    variables: List[str] = Field(..., description="Required variables")
    constants: Dict[str, Any] = Field(default_factory=dict, description="Law constants")
    constraints: Optional[Dict[str, Dict[str, Any]]] = Field(None, description="Variable constraints")
    applies_to: List[str] = Field(..., description="Component types this law affects")


class ComponentRegistry(BaseModel):
    """Registry of all component types and their relationships."""
    components: List[ComponentType] = Field(..., description="All component types")
    laws: List[LawSpecification] = Field(default_factory=list, description="Physics laws")
    
    def get_component(self, name: str) -> Optional[ComponentType]:
        """Get component by name."""
        for comp in self.components:
            if comp.name == name:
                return comp
        return None
    
    def get_components_for_law(self, law_name: str) -> List[ComponentType]:
        """Get all components affected by a specific law."""
        return [comp for comp in self.components if law_name in comp.laws]
    
    def get_components_with_mixin(self, mixin: str) -> List[ComponentType]:
        """Get all components with a specific mixin."""
        return [comp for comp in self.components if mixin in comp.mixins]
    
    def validate_law_consistency(self) -> List[str]:
        """Validate that all referenced laws exist and components are consistent."""
        errors = []
        
        # Check that all component laws reference existing law specs
        law_names = {law.name for law in self.laws}
        for comp in self.components:
            for law in comp.laws:
                if law not in law_names:
                    errors.append(f"Component {comp.name} references unknown law: {law}")
        
        # Check that law applies_to references existing components
        comp_names = {comp.name for comp in self.components}
        for law in self.laws:
            for comp_name in law.applies_to:
                if comp_name not in comp_names:
                    errors.append(f"Law {law.name} applies to unknown component: {comp_name}")
        
        return errors


def load_component_schema(yaml_path: str = "cold/instances/component_types.yml") -> ComponentRegistry:
    """Load and validate component schema from YAML file."""
    path = Path(yaml_path)
    if not path.exists():
        raise FileNotFoundError(f"Schema file not found: {yaml_path}")
    
    with open(path, 'r') as f:
        raw_data = yaml.safe_load(f)
    
    # Validate and parse components
    components = [ComponentType(**comp_data) for comp_data in raw_data]
    
    # Load default laws
    default_laws = [
        LawSpecification(
            name="decay",
            formula="strength = strength * pow(0.5, time_elapsed / half_life)",
            variables=["strength", "half_life", "last_update"],
            constants={},
            constraints={"strength": {"min": 0.1}},
            applies_to=["DecayComponent", "Age"]
        ),
        LawSpecification(
            name="resonance",
            formula="strength = min(strength * multiplier, max_strength) if strength > threshold else strength",
            variables=["strength"],
            constants={"threshold": 0.85, "multiplier": 1.2, "max_strength": 1.0},
            constraints={},
            applies_to=["DecayComponent"]
        )
    ]
    
    registry = ComponentRegistry(components=components, laws=default_laws)
    
    # Validate consistency
    errors = registry.validate_law_consistency()
    if errors:
        raise ValueError(f"Schema validation errors: {errors}")
    
    return registry


def generate_pydantic_models(registry: ComponentRegistry) -> str:
    """Generate Pydantic models for runtime validation from component schema."""
    models = []
    
    for comp in registry.components:
        fields = []
        for field in comp.fields:
            if field.type == FieldType.INT:
                field_type = "int"
            elif field.type == FieldType.FLOAT:
                field_type = "float"
            elif field.type == FieldType.STR:
                field_type = "str"
            elif field.type == FieldType.DATETIME:
                field_type = "datetime"
            elif field.type == FieldType.BOOL:
                field_type = "bool"
            elif field.type == FieldType.VECTOR:
                field_type = "List[float]"
            else:
                field_type = "str"  # fallback
            
            default_val = f" = {field.default!r}" if field.default is not None else ""
            fields.append(f"    {field.name}: {field_type}{default_val}")
        
        model = f"""
class {comp.name}Data(BaseModel):
    \"\"\"Pydantic model for {comp.name} component validation.\"\"\"
{chr(10).join(fields)}
    
    class Config:
        schema_version = {comp.version}
        mixins = {comp.mixins!r}
        affected_by_laws = {comp.laws!r}
"""
        models.append(model)
    
    return "\n".join(models)


# CLI for testing
if __name__ == "__main__":
    print("🔧 Testing Pydantic Schema Integration")
    
    try:
        # Load and validate schema
        registry = load_component_schema()
        print(f"✅ Loaded {len(registry.components)} components and {len(registry.laws)} laws")
        
        # Show components
        print("\n📋 Components:")
        for comp in registry.components:
            laws_str = f" (laws: {', '.join(comp.laws)})" if comp.laws else ""
            print(f"  • {comp.name} v{comp.version}{laws_str}")
        
        # Show law application
        print("\n⚖️ Law Applications:")
        for law in registry.laws:
            affected = registry.get_components_for_law(law.name)
            comp_names = [c.name for c in affected]
            print(f"  • {law.name} → {', '.join(comp_names)}")
        
        # Generate Pydantic models
        print("\n🐍 Generated Pydantic Models:")
        pydantic_code = generate_pydantic_models(registry)
        print(pydantic_code[:500] + "..." if len(pydantic_code) > 500 else pydantic_code)
        
    except Exception as e:
        print(f"❌ Error: {e}") 