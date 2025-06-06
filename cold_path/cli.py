import typer
import json
import yaml
from uuid import uuid4
from pathlib import Path
from src.familiar import entities
from src.familiar.dag import Dag
from src.familiar.laws import Law, decay_law, resonance_law, binding_law
from src.familiar.schema import (
    BaseEntity, BaseComponent, Visibility, AccessScope, Cardinality, Metadata
)

app = typer.Typer(help="Familiar Engine Cold Path CLI")


def get_all_pydantic_models():
    """Get all Pydantic models from our schema definitions"""
    models = {}
    
    # Base models from schema.py
    models['BaseEntity'] = BaseEntity
    models['BaseComponent'] = BaseComponent
    models['Metadata'] = Metadata
    models['Visibility'] = Visibility
    models['AccessScope'] = AccessScope
    models['Cardinality'] = Cardinality
    
    # Domain entities from entities.py
    models['Moment'] = entities.Moment
    models['Binding'] = entities.Binding
    models['BindingPoint'] = entities.BindingPoint
    models['Thread'] = entities.Thread
    models['Filament'] = entities.Filament
    models['Motif'] = entities.Motif
    models['Bond'] = entities.Bond
    
    # Laws from laws.py - both the schema and the actual physical laws
    models['Law'] = Law
    
    # The actual laws that govern the universe (with their mathematical physics)
    models['DecayLaw'] = decay_law
    models['ResonanceLaw'] = resonance_law  
    models['BindingLaw'] = binding_law
    
    return models


def _clean_schema_refs(schema):
    """Remove $defs and convert $ref to simple type references."""
    if isinstance(schema, dict):
        # Remove $defs to avoid duplication
        if '$defs' in schema:
            del schema['$defs']
        
        # Convert complex $ref to simple string types for enums
        if '$ref' in schema:
            ref = schema['$ref']
            if ref.startswith('#/$defs/'):
                type_name = ref.replace('#/$defs/', '')
                if type_name in ['AccessScope', 'Visibility', 'Cardinality']:
                    # Convert enum refs to inline definitions
                    enum_values = {
                        'AccessScope': ["view", "edit", "admin"],
                        'Visibility': ["private", "household", "org", "public"],
                        'Cardinality': ["actor", "observer", "recipient", "co-participant", "target"]
                    }
                    if type_name in enum_values:
                        return {
                            "type": "string",
                            "enum": enum_values[type_name],
                            "description": f"<enum '{type_name}'>"
                        }
                # For other refs, just use string type
                return {"type": "string", "format": "uuid" if "id" in ref.lower() else None}
        
        # Recursively clean nested objects
        return {k: _clean_schema_refs(v) for k, v in schema.items()}
    elif isinstance(schema, list):
        return [_clean_schema_refs(item) for item in schema]
    else:
        return schema


@app.command()
def schema_dump(output_path: str = "assets/sample_schema.json"):
    """Export actual Pydantic schemas as JSON for the hot path."""
    models = get_all_pydantic_models()
    
    # Generate clean schemas without $defs duplication
    schemas = {}
    
    for name, model in models.items():
        try:
            if hasattr(model, 'model_json_schema'):
                # Pydantic v2 style 
                raw_schema = model.model_json_schema()
                schemas[name] = _clean_schema_refs(raw_schema)
            elif hasattr(model, 'schema'):
                # Pydantic v1 style fallback
                raw_schema = model.schema()
                schemas[name] = _clean_schema_refs(raw_schema)
            else:
                # For enums and other types
                if hasattr(model, '_member_names_'):
                    # Enum type
                    schemas[name] = {
                        "type": "string",
                        "enum": list(model._member_names_),
                        "description": f"<enum '{name}'>"
                    }
                else:
                    schemas[name] = {
                        "type": "string",
                        "description": str(model)
                    }
        except Exception as e:
            print(f"⚠️  Could not generate schema for {name}: {e}")
            schemas[name] = {"error": str(e)}
    
    # Create output directory if it doesn't exist
    Path(output_path).parent.mkdir(parents=True, exist_ok=True)
    
    # Write schemas with metadata (simplified structure)
    output = {
        "schema_version": "1.0.0",
        "generated_by": "familiar-cold-path",
        "models": schemas
    }
    
    with open(output_path, "w") as f:
        json.dump(output, f, indent=2, default=str)
    
    print(f"✅ Real Pydantic schemas exported to {output_path}")
    print(f"📊 Exported {len(schemas)} model schemas:")
    for name in sorted(schemas.keys()):
        print(f"   • {name}")


@app.command()
def schema_summary():
    """Show a summary of all available schemas."""
    models = get_all_pydantic_models()
    
    print(f"📋 Available Pydantic Models ({len(models)} total):\n")
    
    for name, model in sorted(models.items()):
        try:
            if hasattr(model, 'model_fields'):
                # Pydantic v2
                field_count = len(model.model_fields)
                fields = list(model.model_fields.keys())[:3]  # Show first 3 fields
            elif hasattr(model, '__fields__'):
                # Pydantic v1 fallback  
                field_count = len(model.__fields__)
                fields = list(model.__fields__.keys())[:3]
            else:
                field_count = 0
                fields = []
            
            field_preview = ", ".join(fields)
            if field_count > 3:
                field_preview += f", ... (+{field_count - 3} more)"
            
            print(f"  🔹 {name}")
            print(f"     Fields ({field_count}): {field_preview}")
            
            if hasattr(model, '__doc__') and model.__doc__:
                doc = model.__doc__.strip().split('\n')[0]  # First line only
                print(f"     Doc: {doc}")
            print()
            
        except Exception as e:
            print(f"  ❌ {name}: Error reading model ({e})")


@app.command()
def entity_generate():
    """Generates and prints a sample entity bundle as JSON."""
    moment = entities.Moment(org_id=uuid4(), owner_id=uuid4())
    print("--- Sample Moment ---")
    print(moment.model_dump_json(indent=2))


@app.command()
def dag_validate(dag_path: str = "assets/sample_dag.yml"):
    """Validates a DAG YAML file against the Dag schema."""
    try:
        with open(dag_path, "r") as f:
            dag_data = yaml.safe_load(f)
        Dag.model_validate(dag_data)
        print(f"✅ DAG at {dag_path} is valid.")
    except Exception as e:
        print(f"❌ Invalid DAG: {e}")


if __name__ == "__main__":
    app() 