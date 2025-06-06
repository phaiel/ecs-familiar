#!/usr/bin/env python3
"""
Interactive web UI for Pydantic schema exploration.
Provides live schema docs, example generation, and validation.
"""

from fastapi import FastAPI, Request
from fastapi.responses import HTMLResponse
from fastapi.staticfiles import StaticFiles
import uvicorn
import json
from typing import Dict, Any, List
from pathlib import Path
import importlib.util
import sys

from pydantic import BaseModel, ValidationError


# Import your actual models (adjust imports as needed)
try:
    from src.familiar.schema import *
    from src.familiar.components import *
    from src.familiar.laws import *
    from src.familiar.config import *
    from src.familiar.dag import *
except ImportError as e:
    print(f"Import warning: {e}")
    print("Some models may not be available in the UI")


app = FastAPI(title="Familiar Memory Schema Explorer", version="1.0.0")


def get_all_models() -> Dict[str, BaseModel]:
    """Collect all Pydantic models from the current module namespace."""
    models = {}
    current_module = sys.modules[__name__]
    
    for name in dir(current_module):
        obj = getattr(current_module, name)
        if (isinstance(obj, type) and 
            issubclass(obj, BaseModel) and 
            obj is not BaseModel and
            hasattr(obj, 'model_json_schema')):
            models[name] = obj
    
    return models


@app.get("/", response_class=HTMLResponse)
async def schema_explorer():
    """Main schema explorer page."""
    models = get_all_models()
    
    html = f"""
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Familiar Memory - Interactive Schema Explorer</title>
    <script src="https://cdn.jsdelivr.net/npm/vue@3/dist/vue.global.js"></script>
    <style>
        body {{
            font-family: 'Inter', -apple-system, BlinkMacSystemFont, sans-serif;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
        }}
        .container {{
            max-width: 1600px;
            margin: 0 auto;
            padding: 20px;
        }}
        .header {{
            text-align: center;
            margin-bottom: 30px;
            background: rgba(255, 255, 255, 0.1);
            padding: 30px;
            border-radius: 15px;
            backdrop-filter: blur(10px);
        }}
        .sidebar {{
            position: fixed;
            left: 20px;
            top: 200px;
            width: 300px;
            background: rgba(0, 0, 0, 0.3);
            border-radius: 12px;
            padding: 20px;
            max-height: 70vh;
            overflow-y: auto;
        }}
        .main-content {{
            margin-left: 360px;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 15px;
            padding: 30px;
            backdrop-filter: blur(10px);
        }}
        .model-list {{
            list-style: none;
            padding: 0;
        }}
        .model-item {{
            padding: 12px;
            margin: 8px 0;
            background: rgba(255, 255, 255, 0.1);
            border-radius: 8px;
            cursor: pointer;
            transition: all 0.3s ease;
            border-left: 4px solid transparent;
        }}
        .model-item:hover {{
            background: rgba(255, 255, 255, 0.2);
            transform: translateX(5px);
        }}
        .model-item.active {{
            border-left-color: #4CAF50;
            background: rgba(76, 175, 80, 0.2);
        }}
        .schema-details {{
            background: rgba(0, 0, 0, 0.2);
            border-radius: 12px;
            padding: 25px;
            margin-bottom: 20px;
        }}
        .schema-title {{
            font-size: 2em;
            color: #4CAF50;
            margin-bottom: 15px;
            display: flex;
            align-items: center;
            gap: 10px;
        }}
        .field-grid {{
            display: grid;
            gap: 15px;
            margin: 20px 0;
        }}
        .field-card {{
            background: rgba(255, 255, 255, 0.1);
            border-radius: 8px;
            padding: 15px;
            border-left: 3px solid #2196F3;
        }}
        .field-name {{
            font-weight: bold;
            color: #4ECDC4;
            font-size: 1.1em;
        }}
        .field-type {{
            font-family: 'Courier New', monospace;
            background: rgba(0, 0, 0, 0.3);
            padding: 4px 8px;
            border-radius: 4px;
            margin: 5px 0;
            display: inline-block;
        }}
        .field-description {{
            color: #E0E0E0;
            font-style: italic;
            margin-top: 8px;
        }}
        .examples-section {{
            background: rgba(255, 255, 255, 0.05);
            border-radius: 12px;
            padding: 20px;
            margin-top: 20px;
        }}
        .example-generator {{
            display: flex;
            gap: 15px;
            margin-bottom: 20px;
        }}
        .btn {{
            background: #4CAF50;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 8px;
            cursor: pointer;
            font-size: 1em;
            transition: background 0.3s ease;
        }}
        .btn:hover {{
            background: #45a049;
        }}
        .btn-secondary {{
            background: #2196F3;
        }}
        .btn-secondary:hover {{
            background: #1976D2;
        }}
        .json-display {{
            background: rgba(0, 0, 0, 0.4);
            border-radius: 8px;
            padding: 20px;
            font-family: 'Courier New', monospace;
            white-space: pre-wrap;
            overflow-x: auto;
            border: 1px solid rgba(255, 255, 255, 0.2);
        }}
        .required-badge {{
            background: #FF6B6B;
            color: white;
            font-size: 0.8em;
            padding: 2px 6px;
            border-radius: 4px;
            margin-left: 8px;
        }}
        .optional-badge {{
            background: #4ECDC4;
            color: white;
            font-size: 0.8em;
            padding: 2px 6px;
            border-radius: 4px;
            margin-left: 8px;
        }}
    </style>
</head>
<body>
    <div id="app">
        <div class="container">
            <div class="header">
                <h1>🧠 Familiar Memory Schema Explorer</h1>
                <p>Interactive documentation and validation for Pydantic models</p>
                <p>Found {{{{ models.length }}}} models</p>
            </div>
            
            <div class="sidebar">
                <h3>📋 Models</h3>
                <ul class="model-list">
                    <li v-for="model in models" 
                        :key="model.name"
                        class="model-item"
                        :class="{{active: selectedModel === model.name}}"
                        @click="selectModel(model.name)">
                        {{{{ model.name }}}}
                    </li>
                </ul>
            </div>
            
            <div class="main-content">
                <div v-if="selectedModelData" class="schema-details">
                    <div class="schema-title">
                        📊 {{{{ selectedModelData.name }}}}
                        <span v-if="selectedModelData.description" style="font-size: 0.6em; color: #E0E0E0;">
                            - {{{{ selectedModelData.description }}}}
                        </span>
                    </div>
                    
                    <div v-if="selectedModelData.inheritance" 
                         style="background: rgba(255, 193, 7, 0.2); padding: 10px; border-radius: 6px; margin-bottom: 15px;">
                        📋 Inherits from: {{{{ selectedModelData.inheritance.join(', ') }}}}
                    </div>
                    
                    <div class="field-grid">
                        <div v-for="field in selectedModelData.fields" 
                             :key="field.name" 
                             class="field-card">
                            <div class="field-name">
                                {{{{ field.name }}}}
                                <span v-if="field.required" class="required-badge">Required</span>
                                <span v-else class="optional-badge">Optional</span>
                            </div>
                            <div class="field-type">{{{{ field.type }}}}</div>
                            <div v-if="field.description" class="field-description">
                                {{{{ field.description }}}}
                            </div>
                            <div v-if="field.default !== undefined && field.default !== null" 
                                 style="color: #FFC107; margin-top: 5px;">
                                Default: {{{{ field.default }}}}
                            </div>
                        </div>
                    </div>
                </div>
                
                <div v-if="selectedModelData" class="examples-section">
                    <h3>🎯 Examples & Testing</h3>
                    <div class="example-generator">
                        <button class="btn" @click="generateExample">🎲 Generate Example</button>
                        <button class="btn btn-secondary" @click="generateSchema">📄 Show JSON Schema</button>
                        <button class="btn btn-secondary" @click="validateExample">✅ Validate</button>
                    </div>
                    
                    <div v-if="currentExample" class="json-display">{{{{ currentExample }}}}</div>
                </div>
            </div>
        </div>
    </div>

    <script>
        const {{ createApp }} = Vue;
        
        createApp({{
            data() {{
                return {{
                    models: {json.dumps([{"name": name} for name in models.keys()])},
                    selectedModel: null,
                    selectedModelData: null,
                    currentExample: null
                }}
            }},
            methods: {{
                async selectModel(modelName) {{
                    this.selectedModel = modelName;
                    try {{
                        const response = await fetch(`/model/${{modelName}}`);
                        this.selectedModelData = await response.json();
                    }} catch (error) {{
                        console.error('Error fetching model data:', error);
                    }}
                }},
                async generateExample() {{
                    if (!this.selectedModel) return;
                    try {{
                        const response = await fetch(`/example/${{this.selectedModel}}`);
                        const example = await response.json();
                        this.currentExample = JSON.stringify(example, null, 2);
                    }} catch (error) {{
                        console.error('Error generating example:', error);
                    }}
                }},
                async generateSchema() {{
                    if (!this.selectedModel) return;
                    try {{
                        const response = await fetch(`/schema/${{this.selectedModel}}`);
                        const schema = await response.json();
                        this.currentExample = JSON.stringify(schema, null, 2);
                    }} catch (error) {{
                        console.error('Error generating schema:', error);
                    }}
                }},
                async validateExample() {{
                    if (!this.currentExample || !this.selectedModel) return;
                    try {{
                        const response = await fetch(`/validate/${{this.selectedModel}}`, {{
                            method: 'POST',
                            headers: {{'Content-Type': 'application/json'}},
                            body: this.currentExample
                        }});
                        const result = await response.json();
                        if (result.valid) {{
                            alert('✅ Valid!');
                        }} else {{
                            alert('❌ Invalid: ' + result.errors.join(', '));
                        }}
                    }} catch (error) {{
                        alert('❌ Validation error: ' + error.message);
                    }}
                }}
            }},
            mounted() {{
                // Auto-select first model
                if (this.models.length > 0) {{
                    this.selectModel(this.models[0].name);
                }}
            }}
        }}).mount('#app');
    </script>
</body>
</html>
"""
    return html


@app.get("/model/{model_name}")
async def get_model_details(model_name: str):
    """Get detailed information about a specific model."""
    models = get_all_models()
    
    if model_name not in models:
        return {"error": "Model not found"}
    
    model_class = models[model_name]
    schema = model_class.model_json_schema()
    
    # Parse fields with detailed information
    fields = []
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
        
        fields.append({
            'name': field_name,
            'type': field_type,
            'required': field_name in required_fields,
            'description': field_info.get('description', ''),
            'default': field_info.get('default')
        })
    
    # Get inheritance info
    inheritance = []
    if len(model_class.__bases__) > 1 or model_class.__bases__[0] != BaseModel:
        inheritance = [base.__name__ for base in model_class.__bases__ if base != BaseModel]
    
    return {
        'name': model_name,
        'description': schema.get('description') or model_class.__doc__,
        'fields': fields,
        'inheritance': inheritance
    }


@app.get("/example/{model_name}")
async def generate_example(model_name: str):
    """Generate an example instance of the model."""
    models = get_all_models()
    
    if model_name not in models:
        return {"error": "Model not found"}
    
    model_class = models[model_name]
    
    try:
        # Create an example with minimal required fields
        schema = model_class.model_json_schema()
        properties = schema.get('properties', {})
        required_fields = set(schema.get('required', []))
        
        example_data = {}
        
        for field_name, field_info in properties.items():
            if field_name in required_fields:
                field_type = field_info.get('type', 'string')
                
                # Generate sample values based on type
                if field_type == 'string':
                    example_data[field_name] = f"sample_{field_name}"
                elif field_type == 'integer':
                    example_data[field_name] = 42
                elif field_type == 'number':
                    example_data[field_name] = 3.14
                elif field_type == 'boolean':
                    example_data[field_name] = True
                elif field_type == 'array':
                    example_data[field_name] = ["sample_item"]
                else:
                    example_data[field_name] = f"sample_{field_name}"
        
        # Try to create a valid instance
        instance = model_class(**example_data)
        return instance.model_dump()
        
    except Exception as e:
        return {"error": f"Could not generate example: {str(e)}"}


@app.get("/schema/{model_name}")
async def get_json_schema(model_name: str):
    """Get the JSON schema for a model."""
    models = get_all_models()
    
    if model_name not in models:
        return {"error": "Model not found"}
    
    model_class = models[model_name]
    return model_class.model_json_schema()


@app.post("/validate/{model_name}")
async def validate_data(model_name: str, request: Request):
    """Validate data against a model."""
    models = get_all_models()
    
    if model_name not in models:
        return {"valid": False, "errors": ["Model not found"]}
    
    try:
        data = await request.json()
        model_class = models[model_name]
        instance = model_class(**data)
        return {"valid": True, "data": instance.model_dump()}
    except ValidationError as e:
        return {"valid": False, "errors": [str(err) for err in e.errors()]}
    except Exception as e:
        return {"valid": False, "errors": [str(e)]}


if __name__ == "__main__":
    print("🚀 Starting Familiar Memory Schema Explorer...")
    print("📊 Available at: http://localhost:8001")
    print("🔧 Interactive schema documentation with live examples")
    
    uvicorn.run(app, host="0.0.0.0", port=8001, reload=True) 