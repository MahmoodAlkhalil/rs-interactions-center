use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateQueue {
    pub name: String,
    pub channels: Vec<Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct AssignChannelsToQueue {
    pub id: Uuid,
    pub channels: Vec<Uuid>,
}

#[derive(Deserialize, Debug)]
pub struct EnqueueInteraction {
    #[serde(skip_deserializing)]
    pub interaction_id: Uuid,
    #[serde(skip_deserializing)]
    pub queue_id: Uuid,
    pub priority: Option<i32>,
}
#[derive(Deserialize, Debug)]
pub struct DequeueInteraction {
    #[serde(skip_deserializing)]
    pub interaction_id: Uuid,
    #[serde(skip_deserializing)]
    pub queue_id: Option<Uuid>,
}
