#!/usr/bin/env python3
"""
Copier setup with custom Jinja2 filters for Rust code generation.
"""

import yaml
from pathlib import Path


def rust_typemap(type: str):
    return {
        'int': 'i32',
        'float': 'f32',
        'str': 'String',
        'datetime': 'DateTime<Utc>',
        'bool': 'bool',
        'vector': 'Vec<f32>',    # Dynamic vector for compatibility
        'enum': 'String',        # or a generated enum
    }.get(type, type)


def rust_default(value, type_str: str = None) -> str:
    """Generate Rust default values."""
    if value is None:
        if type_str == 'str':
            return '"".to_string()'
        elif type_str in ['int', 'float']:
            return '0'
        elif type_str == 'bool':
            return 'false'
        elif type_str == 'datetime':
            return 'Utc::now()'
        elif type_str == 'vector':
            return 'Vec::new()'
        else:
            return "Default::default()"
    
    if type_str == 'str':
        # Escape quotes in strings
        escaped = str(value).replace('"', '\\"')
        return f'"{escaped}".to_string()'
    elif type_str == 'float':
        return f'{value}'
    elif type_str == 'int':
        return str(value)
    elif type_str == 'bool':
        return 'true' if value else 'false'
    elif type_str == 'datetime':
        return 'Utc::now()'
    elif type_str == 'vector':
        if isinstance(value, list):
            return f'vec![{", ".join(f"{v}" for v in value)}]'
        else:
            return 'Vec::new()'
    else:
        # For unknown types or direct values
        if isinstance(value, bool):
            return 'true' if value else 'false'
        elif isinstance(value, str):
            escaped = value.replace('"', '\\"')
            return f'"{escaped}".to_string()'
        else:
            return str(value)


def load_yaml(file_path: str) -> list:
    """Load YAML file and return parsed data."""
    path = Path(file_path)
    if not path.exists():
        raise FileNotFoundError(f"YAML file not found: {file_path}")
    
    with open(path, 'r') as f:
        return yaml.safe_load(f)


def rust_type_for_value(value):
    """Determine Rust type for a given value."""
    if isinstance(value, str):
        return "String"
    elif isinstance(value, bool):
        return "bool"
    elif isinstance(value, int):
        return "i32"
    elif isinstance(value, float):
        return "f64"
    else:
        return "String"


def title_filter(text):
    """Convert text to title case for Rust struct names."""
    return ''.join(word.capitalize() for word in text.split('_'))


# Register filters for Copier
def setup_jinja_filters(jinja_env):
    """Register custom filters with Jinja environment."""
    jinja_env.filters['rust_typemap'] = rust_typemap
    jinja_env.filters['rust_default'] = rust_default
    jinja_env.filters['rust_type_for_value'] = rust_type_for_value
    jinja_env.filters['title'] = title_filter
    jinja_env.globals['load_yaml'] = load_yaml


# For direct execution
if __name__ == "__main__":
    # Test the filters
    print("Testing Rust type mapping:")
    test_types = ['int', 'float', 'str', 'datetime', 'bool', 'vector', 'enum']
    for t in test_types:
        print(f"  {t} -> {rust_typemap(t)}")
    
    print("\nTesting default value generation:")
    test_defaults = [
        (42, 'int'),
        (3.14, 'float'),
        ("hello", 'str'),
        (True, 'bool'),
        ([1.0, 2.0, 3.0], 'vector'),
    ]
    for value, type_str in test_defaults:
        print(f"  {value} ({type_str}) -> {rust_default(value, type_str)}")
    
    print("\nTesting YAML loading:")
    try:
        components = load_yaml('cold/instances/component_types.yml')
        print(f"  Loaded {len(components)} component types")
        for comp in components[:3]:  # Show first 3
            print(f"    - {comp['name']} (v{comp['version']})")
    except FileNotFoundError as e:
        print(f"  {e}") 