#!/usr/bin/env python3
"""
Familiar Memory System - Main UI Application
Uses the proper pydantic-ui library for schema visualization and editing.
"""

from fastapi import FastAPI, HTTPException, Request
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates

# Import our UI setup
from familiar_ui import PYDANTIC_UI

app = FastAPI(
    title="Familiar Memory System - Schema UI",
    description="Interactive UI for viewing and editing Pydantic schemas in the Familiar Memory System",
    version="1.0.0"
)

# Mount static files and templates
app.mount("/static", StaticFiles(directory="static"), name="static")
templates = Jinja2Templates(directory="pydantic_ui/templates")


@app.get("/")
async def available_models(request: Request):
    """List all the available model pages"""
    model_names = PYDANTIC_UI.get_models_name()
    return templates.TemplateResponse(
        "index.html", 
        {
            "request": request, 
            "models": model_names,
            "title": "Familiar Memory System - Schema UI"
        }
    )


@app.get("/{model}")
async def model_datatable(request: Request, model: str):
    """Datatable page for a given model"""
    return templates.TemplateResponse(
        "datatable_view.html", 
        {
            "request": request, 
            "model": model,
            "title": f"Familiar Memory - {model}"
        }
    )


@app.get("/{model}/list")
async def model_list(model: str):
    """JSON list of all instances of model"""
    klass = PYDANTIC_UI.get_model_by_name(model)
    if not klass:
        raise HTTPException(404, f"Model {model} not found")
    
    return klass.to_datatables(klass.get_all_data())


@app.get("/{model}/{obj_id}/")
async def model_detail(request: Request, model: str, obj_id: str):
    """JSON model instance"""
    klass = PYDANTIC_UI.get_model_by_name(model)
    if not klass:
        raise HTTPException(404, f"Model {model} not found")
    
    for current_obj in klass.get_all_data():
        current_obj_id = getattr(current_obj, klass.id_field())
        if str(current_obj_id) == obj_id:
            return templates.TemplateResponse(
                "detail_view.html",
                {
                    "request": request,
                    "id": obj_id,
                    "class_name": klass.__name__,
                    "current": current_obj.model_dump_json(indent=2),
                    "schema": klass.to_json_editor_representation(indent=2),
                    "autocomplete": klass.all_js_autocomplete_function_paths(),
                },
            )

    raise HTTPException(404, "No such id")


@app.post("/{model}/{obj_id}/")
async def edit_item(request: Request, model: str, obj_id: str, data: dict):
    """Edit a model instance"""
    klass = PYDANTIC_UI.get_model_by_name(model)
    if not klass:
        raise HTTPException(404, f"Model {model} not found")
    
    try:
        obj = klass.kind(**data)
        return obj.model_dump()
    except Exception as e:
        raise HTTPException(400, f"Validation error: {str(e)}")


@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {
        "status": "healthy",
        "models_available": len(PYDANTIC_UI.get_models_name()),
        "models": PYDANTIC_UI.get_models_name()
    }


if __name__ == "__main__":
    import uvicorn

    print("🚀 Starting Familiar Memory Schema UI...")
    print("📊 Available models:")
    for model_name in PYDANTIC_UI.get_models_name():
        print(f"  - {model_name}")
    print("\n🌐 UI available at: http://127.0.0.1:8003")
    print("📖 This uses the proper pydantic-ui library for Django-admin-like interface")
    
    uvicorn.run("main:app", host="127.0.0.1", port=8003, reload=True) 