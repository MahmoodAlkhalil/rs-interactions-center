use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateQueue {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct QueueToChannelsMapping {
    pub queue_id: Uuid,
    pub channels: Vec<Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct EnqueueInteraction {
    pub interaction_id: Uuid,
    pub queue_id: Uuid,
    pub priority: Option<i32>,
}

#[derive(Deserialize, Debug)]
pub struct DequeueInteraction {
    pub interaction_id: Uuid,
}
