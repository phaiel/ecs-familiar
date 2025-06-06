from pydantic import BaseModel, Field
from typing import List, Literal, Optional, Union
from uuid import UUID, uuid4
from datetime import datetime
from enum import Enum


class Visibility(str, Enum):
    PRIVATE = "private"
    HOUSEHOLD = "household"
    ORG = "org"
    PUBLIC = "public"


class AccessScope(str, Enum):
    VIEW = "view"
    EDIT = "edit"
    ADMIN = "admin"


class Cardinality(str, Enum):
    ACTOR = "actor"
    OBSERVER = "observer"
    RECIPIENT = "recipient"
    CO_PARTICIPANT = "co-participant"
    TARGET = "target"


# --- Typed Metadata (for component bonding logic) ---
class Metadata(BaseModel):
    key: str
    value: Union[str, float, int, datetime, bool]
    type: Literal['str', 'float', 'int', 'datetime', 'bool']


# --- Base Component Schema ---
class BaseComponent(BaseModel):
    id: UUID = Field(default_factory=uuid4)
    name: str
    metadata: List[Metadata] = []


# --- Base Entity Object (used by all objects in ECS) ---
class BaseEntity(BaseModel):
    id: UUID = Field(default_factory=uuid4)
    org_id: UUID
    owner_id: UUID
    created_at: datetime = Field(default_factory=datetime.utcnow)
    updated_at: Optional[datetime] = None
    deleted_at: Optional[datetime] = None  # Soft deletion support
    tags: List[str] = []
    component_ids: List[UUID] = []  # populated component UUIDs
    sub_type: Optional[str] = None  # e.g. 'emotion', 'reflection', 'nap_event'
    visibility: Visibility = Visibility.PRIVATE
    security_level: int = 0
    access_scope: List[AccessScope] = Field(default_factory=lambda: [AccessScope.VIEW])
    version: int = Field(default=1)
    parent_version: Optional[int] = None
 