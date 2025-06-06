#!/usr/bin/env python3
"""
Test script to verify hot_path ↔ cold_path bridge connection
"""

import json
import subprocess
import sys
from pathlib import Path

def test_cold_path_export():
    """Test that cold path can export schemas"""
    print("🔄 Testing Cold Path Schema Export...")
    
    # Run schema export
    result = subprocess.run([
        sys.executable, "cold_path/cli.py", "schema-dump", 
        "--output-path", "bridge_test_schema.json"
    ], capture_output=True, text=True)
    
    if result.returncode != 0:
        print(f"❌ Schema export failed: {result.stderr}")
        return False
    
    # Check file was created
    schema_file = Path("bridge_test_schema.json")
    if not schema_file.exists():
        print("❌ Schema file not created")
        return False
    
    # Validate JSON structure
    try:
        with open(schema_file) as f:
            schema_data = json.load(f)
        
        required_keys = ["schema_version", "generated_by", "models"]
        if not all(key in schema_data for key in required_keys):
            print("❌ Schema JSON missing required keys")
            return False
        
        model_count = len(schema_data["models"])
        print(f"✅ Cold Path Export: {model_count} models exported")
        
        # Check for key models
        key_models = ["Binding", "Moment", "BindingPoint", "Thread"]
        missing = [m for m in key_models if m not in schema_data["models"]]
        if missing:
            print(f"⚠️  Missing key models: {missing}")
        else:
            print("✅ All key domain models present")
        
        # Cleanup
        schema_file.unlink()
        return True
        
    except json.JSONDecodeError as e:
        print(f"❌ Invalid JSON: {e}")
        return False

def test_hot_path_status():
    """Check hot path Rust project status"""
    print("\n🔄 Testing Hot Path Status...")
    
    cargo_toml = Path("hot_path/Cargo.toml")
    if not cargo_toml.exists():
        print("❌ Hot path Cargo.toml not found")
        return False
    
    # Check if hot path compiles
    result = subprocess.run([
        "cargo", "check"
    ], cwd="hot_path", capture_output=True, text=True)
    
    if result.returncode != 0:
        print(f"⚠️  Hot path compilation issues: {result.stderr}")
        return False
    else:
        print("✅ Hot Path: Rust project compiles")
        return True

def test_bridge_connection():
    """Test the overall bridge connection"""
    print("\n🌉 Testing Bridge Connection...")
    
    # Check if assets directory exists for schema transfer
    cold_assets = Path("cold_path/assets")
    hot_assets = Path("hot_path/assets")
    
    if not cold_assets.exists():
        print("❌ Cold path assets directory missing")
        return False
    
    if not hot_assets.exists():
        print("❌ Hot path assets directory missing")
        return False
    
    # Test schema export to correct location
    result = subprocess.run([
        sys.executable, "cli.py", "schema-dump", 
        "--output-path", "../assets/sample_schema.json"
    ], cwd="cold_path", capture_output=True, text=True)
    
    if result.returncode != 0:
        print(f"❌ Cross-path schema export failed: {result.stderr}")
        return False
    
    # Check file in root assets
    root_schema = Path("assets/sample_schema.json")
    if root_schema.exists():
        print("✅ Bridge: Schema exported to shared assets directory")
        return True
    else:
        print("❌ Schema not found in shared assets")
        return False

def main():
    """Run all bridge tests"""
    print("🚀 Familiar Engine - Bridge Connection Test\n")
    
    tests = [
        ("Cold Path Export", test_cold_path_export),
        ("Hot Path Status", test_hot_path_status),
        ("Bridge Connection", test_bridge_connection)
    ]
    
    results = []
    for test_name, test_func in tests:
        try:
            success = test_func()
            results.append((test_name, success))
        except Exception as e:
            print(f"❌ {test_name}: Exception - {e}")
            results.append((test_name, False))
    
    # Summary
    print(f"\n📊 Test Results:")
    passed = sum(1 for _, success in results if success)
    total = len(results)
    
    for test_name, success in results:
        status = "✅ PASS" if success else "❌ FAIL"
        print(f"  {status} - {test_name}")
    
    print(f"\n🎯 Overall: {passed}/{total} tests passed")
    
    if passed == total:
        print("🎉 Bridge connection is working!")
        print("Next: Update hot_path to consume JSON schemas dynamically")
    else:
        print("🔧 Some components need attention")
    
    return passed == total

if __name__ == "__main__":
    success = main()
    sys.exit(0 if success else 1) 