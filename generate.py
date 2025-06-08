#!/usr/bin/env python3
"""
Modular Schema-First Generation CLI.
Generates ECS code individually or all together from YAML schemas.
"""

import sys
import argparse
from pathlib import Path

# Import all generators
from generators.base import SchemaLoader, CompilationTester, GenerationStats
from generators.ecs import generate_ecs
from generators.quantum import generate_quantum
from generators.dags import generate_dags
from generators.redpanda import generate_redpanda
from generators.windmill import generate_windmill
from generators.entities import generate_entities


def main():
    """Main CLI entry point."""
    parser = argparse.ArgumentParser(
        description="Schema-First ECS Generation CLI",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python generate.py ecs           # Generate ECS components, systems, main.rs
  python generate.py quantum       # Generate quantum systems
  python generate.py dags          # Generate Windmill DAG nodes
  python generate.py redpanda     # Generate Redpanda configs (stub)
  python generate.py windmill     # Generate Windmill configs (stub) 
  python generate.py entities     # Generate entity blueprints
  python generate.py all          # Generate everything (default)
  python generate.py --validate   # Only validate schemas
        """
    )
    
    parser.add_argument(
        'target',
        nargs='?',
        default='all',
        choices=['ecs', 'quantum', 'dags', 'redpanda', 'windmill', 'entities', 'all'],
        help='What to generate (default: all)'
    )
    
    parser.add_argument(
        '--validate',
        action='store_true',
        help='Only validate schemas, do not generate'
    )
    
    parser.add_argument(
        '--no-compile-test',
        action='store_true',
        help='Skip compilation testing'
    )
    
    args = parser.parse_args()
    
    print("🧬 Schema-First ECS Generation CLI")
    print("=" * 50)
    
    try:
        # Load and validate schemas
        loader = SchemaLoader()
        schema_data = loader.load_all_schemas()
        loader.validate_schemas()
        
        if args.validate:
            print("✅ Schema validation complete!")
            return 0
        
        # Track all generated files
        all_generated_files = []
        
        # Generate based on target
        if args.target == 'all':
            print("\n🎯 Generating all targets...")
            all_generated_files.extend(generate_ecs(schema_data))
            all_generated_files.extend(generate_quantum(schema_data))
            all_generated_files.extend(generate_dags(schema_data))
            all_generated_files.extend(generate_redpanda(schema_data))
            all_generated_files.extend(generate_windmill(schema_data))
            all_generated_files.extend(generate_entities(schema_data))
            
        elif args.target == 'ecs':
            print("\n🎯 Generating ECS code...")
            all_generated_files.extend(generate_ecs(schema_data))
            
        elif args.target == 'quantum':
            print("\n🎯 Generating quantum systems...")
            all_generated_files.extend(generate_quantum(schema_data))
            
        elif args.target == 'dags':
            print("\n🎯 Generating DAG nodes...")
            all_generated_files.extend(generate_dags(schema_data))
            
        elif args.target == 'redpanda':
            print("\n🎯 Generating Redpanda configs...")
            all_generated_files.extend(generate_redpanda(schema_data))
            
        elif args.target == 'windmill':
            print("\n🎯 Generating Windmill configs...")
            all_generated_files.extend(generate_windmill(schema_data))
            
        elif args.target == 'entities':
            print("\n🎯 Generating entity blueprints...")
            all_generated_files.extend(generate_entities(schema_data))
        
        # Test compilation for ECS targets
        compilation_success = True
        if not args.no_compile_test and args.target in ['all', 'ecs', 'quantum']:
            compilation_success = CompilationTester.test_compilation()
        
        # Show summary
        if all_generated_files:
            GenerationStats.show_summary(schema_data, all_generated_files)
            
            if compilation_success:
                print(f"\n✨ {args.target.title()} generation completed successfully!")
                if args.target in ['all', 'ecs']:
                    print(f"🎯 Hot path is now 100% generated from cold path YAML schemas")
            else:
                print(f"\n❌ Generation completed but compilation failed")
                return 1
        else:
            print(f"\n⚠️ No files generated for target: {args.target}")
            
    except Exception as e:
        print(f"\n❌ Generation failed: {e}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0


if __name__ == "__main__":
    sys.exit(main()) 