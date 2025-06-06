from pydantic import BaseModel, Field
from typing import List, Literal, Optional
from uuid import UUID

from .schema import BaseEntity, BaseComponent, Cardinality


class BindingPoint(BaseModel):
    """A binding point representing thread ↔ moment ↔ role (1-to-1-to-1 mapping)"""
    thread_id: UUID
    moment_id: UUID
    cardinality: Literal['actor', 'observer', 'recipient', 'co-participant', 'target']


class Binding(BaseEntity):
    """Cross-thread connection as a composite of binding points"""
    sub_type: Literal['binding'] = 'binding'
    points: List[BindingPoint] = []  # stable, semantic, per-thread per-moment
    thread_ids: List[UUID] = Field(default_factory=list)  # optional index optimization


class Moment(BaseEntity):
    """Individual memory moment with explicit authorship and thread binding"""
    sub_type: Literal['moment'] = 'moment'
    thread_id: UUID               # Thread this moment belongs to
    author_id: UUID               # Author (user or agent)
    binding_hint: Optional[int] = None  # Used by DAG to group moments (not persisted long-term)
    binding_id: Optional[UUID] = None   # Set by DAG if included in a binding
    cardinality: Optional[Cardinality] = None


# --- Additional Domain Entities ---
class Thread(BaseEntity):
    """Memory thread organizing related moments"""
    sub_type: Literal['thread'] = 'thread'


class Filament(BaseEntity):
    """Temporal memory filament"""
    sub_type: Literal['filament'] = 'filament'


class Motif(BaseEntity):
    """Recurring memory pattern"""
    sub_type: Literal['motif'] = 'motif'


class Bond(BaseEntity):
    """Cross-thread affinity bond"""
    sub_type: Literal['bond'] = 'bond'
    thread_ids: List[UUID] = []
    affinity_score: float
    bond_strength: float
    component_context: List[UUID] = [] 