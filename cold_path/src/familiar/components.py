from datetime import datetime
from typing import List, Literal, Optional
from pydantic import Field

from .schema import BaseComponent, Metadata

class TimeComponent(BaseComponent):
    name: Literal['time'] = 'time'
    metadata: List[Metadata] = [
        Metadata(key='timestamp', value=datetime.utcnow(), type='datetime')
    ]

class EmotionComponent(BaseComponent):
    name: Literal['emotion'] = 'emotion'
    emotion: str  # e.g., 'joy', 'sadness'
    confidence: float = Field(..., ge=0.0, le=1.0)
    metadata: List[Metadata] = []

class ThreadLinkComponent(BaseComponent):
    name: Literal['thread_link'] = 'thread_link'
    metadata: List[Metadata] = []