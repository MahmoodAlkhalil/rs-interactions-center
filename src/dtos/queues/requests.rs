use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateQueue {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct QueueToChannelsMap {
    pub queue_id: Uuid,
    pub channel_id: Uuid,
}
