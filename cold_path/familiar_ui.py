#!/usr/bin/env python3
"""
Familiar Memory System - Pydantic UI Setup
Using the proper pydantic-ui library for schema visualization and editing.
"""

import sys
from pathlib import Path
from typing import List, Set
from datetime import datetime
from uuid import uuid4

# Add the src directory to Python path
sys.path.insert(0, str(Path(__file__).parent / "src"))

# Import our actual schemas
from familiar.schema import BaseEntity, BaseComponent, Metadata, Visibility, AccessScope, Cardinality
from familiar.entities import Moment, Binding, BindingPoint, Thread
from familiar.components import TimeComponent, EmotionComponent, ThreadLinkComponent
from familiar.laws import Law
from familiar.config import Settings
from familiar.dag import Dag, DagTask

# Import pydantic-ui components
from pydantic_ui.datatables import DataTableModel
from pydantic_ui.models import UIModel, PydanticUI


class BaseEntityUI(UIModel, DataTableModel):
    """UI for BaseEntity - the core entity in the system"""
    kind = BaseEntity

    @classmethod
    def id_field(cls) -> str:
        return "id"

    @classmethod
    def get_all_data(cls) -> List[BaseEntity]:
        # Generate some sample data for demonstration
        return [
            BaseEntity(
                org_id=uuid4(),
                owner_id=uuid4(),
                tags=["memory", "test"],
                sub_type="test_entity",
                visibility=Visibility.PRIVATE,
                security_level=1,
                access_scope=[AccessScope.VIEW, AccessScope.EDIT],
                version=1
            ),
            BaseEntity(
                org_id=uuid4(),
                owner_id=uuid4(),
                tags=["system", "core"],
                sub_type="system_entity",
                visibility=Visibility.HOUSEHOLD,
                security_level=0,
                access_scope=[AccessScope.VIEW, AccessScope.ADMIN],
                version=3,
                parent_version=2
            ),
            BaseEntity(
                org_id=uuid4(),
                owner_id=uuid4(),
                tags=["archived", "old"],
                sub_type="archived_entity",
                visibility=Visibility.PRIVATE,
                security_level=0,
                deleted_at=datetime.now(),  # Example of soft-deleted entity
                access_scope=[AccessScope.VIEW],
                version=1
            )
        ]


class BaseComponentUI(UIModel, DataTableModel):
    """UI for BaseComponent - base for all components"""
    kind = BaseComponent

    @classmethod
    def id_field(cls) -> str:
        return "id"

    @classmethod
    def get_all_data(cls) -> List[BaseComponent]:
        return [
            BaseComponent(
                name="sample_component",
                metadata=[
                    Metadata(key="type", value="test", type="str"),
                    Metadata(key="version", value=1.0, type="float")
                ]
            )
        ]


class TimeComponentUI(UIModel, DataTableModel):
    """UI for TimeComponent - handles time-based data"""
    kind = TimeComponent

    @classmethod
    def id_field(cls) -> str:
        return "id"

    @classmethod
    def get_all_data(cls) -> List[TimeComponent]:
        return [
            TimeComponent(
                metadata=[
                    Metadata(key="timestamp", value=datetime.now(), type="datetime")
                ]
            )
        ]


class EmotionComponentUI(UIModel, DataTableModel):
    """UI for EmotionComponent - handles emotional data"""
    kind = EmotionComponent

    @classmethod
    def id_field(cls) -> str:
        return "id"

    @classmethod
    def get_all_data(cls) -> List[EmotionComponent]:
        return [
            EmotionComponent(emotion="joy", confidence=0.85),
            EmotionComponent(emotion="contemplation", confidence=0.72),
            EmotionComponent(emotion="nostalgia", confidence=0.94)
        ]


class ThreadLinkComponentUI(UIModel, DataTableModel):
    """UI for ThreadLinkComponent - links between threads"""
    kind = ThreadLinkComponent

    @classmethod
    def id_field(cls) -> str:
        return "id"

    @classmethod
    def get_all_data(cls) -> List[ThreadLinkComponent]:
        return [
            ThreadLinkComponent(),
            ThreadLinkComponent(metadata=[
                Metadata(key="strength", value=0.75, type="float")
            ])
        ]


class MomentUI(UIModel, DataTableModel):
    """UI for Moment - individual memory moments with ECS-valid structure"""
    kind = Moment

    @classmethod
    def id_field(cls) -> str:
        return "id"

    @classmethod
    def get_all_data(cls) -> List[Moment]:
        author_id = uuid4()
        thread_id = uuid4()
        binding_id = uuid4()
        
        return [
            Moment(
                org_id=uuid4(),
                owner_id=uuid4(),
                thread_id=thread_id,
                author_id=author_id,
                binding_hint=1,  # DAG grouping hint
                cardinality=Cardinality.ACTOR,
                tags=["conversation", "memory"]
            ),
            Moment(
                org_id=uuid4(),
                owner_id=uuid4(),
                thread_id=thread_id,
                author_id=author_id,
                binding_hint=1,  # Same group
                binding_id=binding_id,
                cardinality=Cardinality.OBSERVER,
                tags=["shared", "context"]
            ),
            Moment(
                org_id=uuid4(),
                owner_id=uuid4(),
                thread_id=uuid4(),
                author_id=uuid4(),
                cardinality=Cardinality.RECIPIENT,
                tags=["emotion", "reflection"]
            )
        ]


class BindingUI(UIModel, DataTableModel):
    """UI for Binding - cross-thread connections as composite of binding points"""
    kind = Binding

    @classmethod
    def id_field(cls) -> str:
        return "id"

    @classmethod
    def get_all_data(cls) -> List[Binding]:
        thread1_id = uuid4()
        thread2_id = uuid4()
        thread3_id = uuid4()
        moment1_id = uuid4()
        moment2_id = uuid4()
        moment3_id = uuid4()
        moment4_id = uuid4()
        
        return [
            Binding(
                org_id=uuid4(),
                owner_id=uuid4(),
                points=[
                    BindingPoint(
                        thread_id=thread1_id,
                        moment_id=moment1_id,
                        cardinality="actor"
                    ),
                    BindingPoint(
                        thread_id=thread2_id,
                        moment_id=moment2_id,
                        cardinality="recipient"
                    )
                ],
                thread_ids=[thread1_id, thread2_id],  # Query optimization index
                tags=["conversation", "binding"]
            ),
            Binding(
                org_id=uuid4(),
                owner_id=uuid4(),
                points=[
                    BindingPoint(
                        thread_id=thread1_id,
                        moment_id=moment1_id,
                        cardinality="actor"
                    ),
                    BindingPoint(
                        thread_id=thread2_id,
                        moment_id=moment3_id,
                        cardinality="observer"
                    ),
                    BindingPoint(
                        thread_id=thread3_id,
                        moment_id=moment4_id,
                        cardinality="co-participant"
                    )
                ],
                thread_ids=[thread1_id, thread2_id, thread3_id],  # Query optimization index
                tags=["complex", "multi-thread"]
            )
        ]


class ThreadUI(UIModel, DataTableModel):
    """UI for Thread - memory threads organizing related moments"""
    kind = Thread

    @classmethod
    def id_field(cls) -> str:
        return "id"

    @classmethod
    def get_all_data(cls) -> List[Thread]:
        return [
            Thread(
                org_id=uuid4(),
                owner_id=uuid4(),
                tags=["conversation", "alice"],
                visibility=Visibility.PRIVATE
            ),
            Thread(
                org_id=uuid4(),
                owner_id=uuid4(),
                tags=["work", "project"],
                visibility=Visibility.HOUSEHOLD
            ),
            Thread(
                org_id=uuid4(),
                owner_id=uuid4(),
                tags=["family", "memories"],
                visibility=Visibility.PRIVATE
            )
        ]


class LawUI(UIModel, DataTableModel):
    """UI for Law - system behavior rules"""
    kind = Law

    @classmethod
    def id_field(cls) -> str:
        return "name"

    @classmethod
    def get_all_data(cls) -> List[Law]:
        return [
            Law(
                name="decay",
                trigger="on_tick",
                applies_to=["filament", "motif"],
                parameters={"half_life": 5, "min_strength": 0.1}
            ),
            Law(
                name="resonance",
                trigger="on_affinity_match",
                applies_to=["filament"],
                parameters={"resonance_strength": 0.7, "threshold": 0.85}
            ),
            Law(
                name="binding",
                trigger="on_observation",
                applies_to=["moment", "thread"],
                parameters={"binding_threshold": 0.6, "max_bindings": 10}
            )
        ]


class SettingsUI(UIModel, DataTableModel):
    """UI for Settings - application configuration"""
    kind = Settings

    @classmethod
    def id_field(cls) -> str:
        return "api_key"

    @classmethod
    def get_all_data(cls) -> List[Settings]:
        return [
            Settings(api_key="prod-key-12345"),
            Settings(api_key="dev-key-67890")
        ]


class DagUI(UIModel, DataTableModel):
    """UI for Dag - directed acyclic graphs"""
    kind = Dag

    @classmethod
    def id_field(cls) -> str:
        return "name"

    @classmethod
    def get_all_data(cls) -> List[Dag]:
        return [
            Dag(
                name="memory_processing_dag",
                tasks=[
                    DagTask(
                        name="extract_emotions",
                        inputs=["raw_memory"],
                        outputs=["emotion_data"],
                        parameters={"model": "emotion_classifier_v2"}
                    ),
                    DagTask(
                        name="create_bindings",
                        inputs=["emotion_data", "existing_threads"],
                        outputs=["new_bindings"],
                        parameters={"threshold": 0.7}
                    )
                ]
            )
        ]


class DagTaskUI(UIModel, DataTableModel):
    """UI for DagTask - individual tasks in a DAG"""
    kind = DagTask

    @classmethod
    def id_field(cls) -> str:
        return "name"

    @classmethod
    def get_all_data(cls) -> List[DagTask]:
        return [
            DagTask(
                name="emotion_analysis",
                inputs=["text", "context"],
                outputs=["emotions", "confidence"],
                parameters={"model_version": "v2.1", "threshold": 0.5}
            ),
            DagTask(
                name="thread_matching",
                inputs=["new_memory", "thread_database"],
                outputs=["matching_threads", "scores"],
                parameters={"max_matches": 5, "min_score": 0.3}
            )
        ]


# Create the main UI registry
PYDANTIC_UI = PydanticUI()

print(f"✅ Familiar Memory UI initialized with {len(PYDANTIC_UI.get_models_name())} models:")
for model_name in PYDANTIC_UI.get_models_name():
    print(f"  📊 {model_name}") 