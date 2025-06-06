from pydantic import BaseModel
from typing import List, Dict, Any

class DagTask(BaseModel):
    name: str
    inputs: List[str]
    outputs: List[str]
    parameters: Dict[str, Any] = {}

class Dag(BaseModel):
    name: str
    tasks: List[DagTask] 