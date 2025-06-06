#!/usr/bin/env python3
"""
Quick CLI tool to inspect and print all Pydantic schemas.
Perfect for getting a quick overview of your models.
"""

import sys
from pathlib import Path
from typing import Dict, Any
import json

# Add the src directory to Python path
sys.path.insert(0, str(Path(__file__).parent / "src"))

from pydantic import BaseModel
from rich.console import Console
from rich.table import Table
from rich.panel import Panel
from rich.text import Text
from rich import box

# Try to import rich for better formatting
try:
    from rich.console import Console
    from rich.table import Table
    from rich.panel import Panel
    from rich.text import Text
    from rich import box
    RICH_AVAILABLE = True
except ImportError:
    RICH_AVAILABLE = False
    print("💡 Install 'rich' for better formatting: pip install rich")

# Import the actual models
try:
    from familiar.schema import *
    from familiar.components import *
    from familiar.entities import *
    from familiar.laws import *
    from familiar.config import *
    from familiar.dag import *
    print("✅ Successfully imported all schema modules")
except ImportError as e:
    print(f"⚠️  Import error: {e}")


def get_all_pydantic_models() -> Dict[str, type]:
    """Find all Pydantic models in the current namespace."""
    models = {}
    
    # Get models from current module's globals
    for name, obj in globals().items():
        if (isinstance(obj, type) and 
            issubclass(obj, BaseModel) and 
            obj is not BaseModel):
            models[name] = obj
    
    return models


def print_model_info_rich(console: Console, name: str, model_class: type):
    """Print model information using rich formatting."""
    schema = model_class.model_json_schema()
    
    # Create main panel
    title = f"📊 {name}"
    
    # Get description
    description = schema.get('description') or getattr(model_class, '__doc__', '')
    if description:
        title += f"\n[dim]{description.strip()}[/dim]"
    
    # Show inheritance
    inheritance_info = ""
    if len(model_class.__bases__) > 1 or model_class.__bases__[0] != BaseModel:
        bases = [base.__name__ for base in model_class.__bases__ if base != BaseModel]
        if bases:
            inheritance_info = f"[yellow]📋 Inherits from: {', '.join(bases)}[/yellow]\n\n"
    
    # Create fields table
    table = Table(box=box.ROUNDED)
    table.add_column("Field", style="cyan", no_wrap=True)
    table.add_column("Type", style="magenta")
    table.add_column("Required", style="bold")
    table.add_column("Description", style="dim")
    table.add_column("Default")
    
    properties = schema.get('properties', {})
    required_fields = set(schema.get('required', []))
    
    for field_name, field_info in properties.items():
        field_type = field_info.get('type', 'unknown')
        
        # Handle complex types
        if 'anyOf' in field_info:
            types = [item.get('type', 'unknown') for item in field_info['anyOf'] if 'type' in item]
            field_type = f"Union[{', '.join(types)}]"
        elif 'items' in field_info:
            item_type = field_info['items'].get('type', 'unknown')
            field_type = f"List[{item_type}]"
        elif '$ref' in field_info:
            field_type = field_info['$ref'].split('/')[-1]
        elif 'enum' in field_info:
            field_type = f"Enum{field_info['enum']}"
        
        is_required = field_name in required_fields
        required_text = "[red]✓ Required[/red]" if is_required else "[green]○ Optional[/green]"
        
        description = field_info.get('description', '')
        default = field_info.get('default', 'N/A')
        
        table.add_row(
            field_name,
            field_type,
            required_text,
            description,
            str(default) if default != 'N/A' else 'N/A'
        )
    
    # Print inheritance info if any
    if inheritance_info:
        console.print(inheritance_info.strip())
    
    # Create and print the panel with table
    if table.rows:
        panel = Panel(
            table,
            title=title,
            border_style="blue",
            padding=(1, 2)
        )
        console.print(panel)
    else:
        panel = Panel(
            "[dim]No fields defined[/dim]",
            title=title,
            border_style="blue",
            padding=(1, 2)
        )
        console.print(panel)
    
    console.print()


def print_model_info_simple(name: str, model_class: type):
    """Print model information using simple formatting."""
    schema = model_class.model_json_schema()
    
    print(f"\n{'='*60}")
    print(f"📊 {name}")
    print(f"{'='*60}")
    
    # Description
    description = schema.get('description') or getattr(model_class, '__doc__', '')
    if description:
        print(f"Description: {description.strip()}")
        print()
    
    # Inheritance
    if len(model_class.__bases__) > 1 or model_class.__bases__[0] != BaseModel:
        bases = [base.__name__ for base in model_class.__bases__ if base != BaseModel]
        if bases:
            print(f"📋 Inherits from: {', '.join(bases)}")
            print()
    
    # Fields
    properties = schema.get('properties', {})
    required_fields = set(schema.get('required', []))
    
    if properties:
        print("🔧 Fields:")
        print(f"{'Field':<20} {'Type':<20} {'Required':<12} {'Description'}")
        print("-" * 80)
        
        for field_name, field_info in properties.items():
            field_type = field_info.get('type', 'unknown')
            
            # Handle complex types
            if 'anyOf' in field_info:
                types = [item.get('type', 'unknown') for item in field_info['anyOf'] if 'type' in item]
                field_type = f"Union[{', '.join(types)}]"
            elif 'items' in field_info:
                item_type = field_info['items'].get('type', 'unknown')
                field_type = f"List[{item_type}]"
            elif '$ref' in field_info:
                field_type = field_info['$ref'].split('/')[-1]
            
            is_required = field_name in required_fields
            required_text = "✓ Required" if is_required else "○ Optional"
            description = field_info.get('description', '')
            
            print(f"{field_name:<20} {field_type:<20} {required_text:<12} {description}")
    else:
        print("No fields defined")


def main():
    """Main function to inspect all schemas."""
    models = get_all_pydantic_models()
    
    if RICH_AVAILABLE:
        console = Console()
        
        # Header
        console.print()
        console.print(Panel.fit(
            "[bold blue]🧠 Familiar Memory System[/bold blue]\n"
            "[dim]Pydantic Schema Inspector[/dim]\n"
            f"[green]Found {len(models)} models[/green]",
            border_style="bright_blue"
        ))
        console.print()
        
        # Print each model
        for name, model_class in sorted(models.items()):
            print_model_info_rich(console, name, model_class)
            
    else:
        # Simple formatting fallback
        print("\n" + "="*60)
        print("🧠 FAMILIAR MEMORY SYSTEM - SCHEMA INSPECTOR")
        print("="*60)
        print(f"Found {len(models)} Pydantic models")
        
        for name, model_class in sorted(models.items()):
            print_model_info_simple(name, model_class)
    
    print(f"\n✅ Inspection complete! Found {len(models)} models.")
    
    # Summary
    model_names = list(models.keys())
    print(f"📋 Models: {', '.join(model_names)}")


def export_json_schemas():
    """Export all schemas as JSON files."""
    models = get_all_pydantic_models()
    output_dir = Path("../schema_exports")
    output_dir.mkdir(exist_ok=True)
    
    for name, model_class in models.items():
        schema = model_class.model_json_schema()
        output_file = output_dir / f"{name}.json"
        
        with open(output_file, 'w') as f:
            json.dump(schema, f, indent=2)
        
        print(f"✅ Exported {name} schema to {output_file}")
    
    print(f"\n📁 All schemas exported to {output_dir}")


if __name__ == "__main__":
    import argparse
    
    parser = argparse.ArgumentParser(description="Inspect Pydantic schemas in the Familiar project")
    parser.add_argument("--export", action="store_true", help="Export schemas as JSON files")
    
    args = parser.parse_args()
    
    if args.export:
        export_json_schemas()
    else:
        main() 