use crossbeam_channel::Receiver;
use hecs::World;

pub struct EcsWorld(pub World);

pub struct CommandReceiver(pub Receiver<GqlCommand>);

#[derive(Debug)]
pub enum GqlCommand {
    CreateMoment { text: String, thread_id: String },
    CreateThread { name: String, thread_type: String },
    CreateFilament { content: String, thread_name: String },
    CreateMotif { pattern: String, strength: f32 },
    CreateBond { thread1: String, thread2: String, affinity: f32 },
    CreateBinding { moment_id: String, thread_id: String },
    UpdateStrength { entity_id: String, new_strength: f32 },
    UpdateDisplayText { entity_id: String, new_text: String },
    AddEntityTag { entity_id: String, tag: String },
    SoftDeleteEntity { entity_id: String },
} 