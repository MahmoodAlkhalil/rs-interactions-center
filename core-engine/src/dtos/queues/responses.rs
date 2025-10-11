use crate::db::entities::channels::Model as ChannelsM;
use crate::db::entities::queues::Model as QueuesM;
use crate::db::entities::queues_channels_assignment::Model as QueuesChannelsAssignmentM;
use crate::dtos::channels::responses::Channel;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct Queue {
    pub id: Uuid,
    pub name: String,
    pub channels: Vec<Channel>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Debug)]
pub struct QueuesChannelsAssignment {
    pub id: Uuid,
    pub channels: Vec<Uuid>,
}

impl From<QueuesM> for Queue {
    fn from(value: QueuesM) -> Self {
        Queue {
            id: value.id,
            name: value.name,
            channels: vec![],
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&QueuesM> for Queue {
    fn from(value: &QueuesM) -> Self {
        Queue {
            id: value.id,
            name: value.name.clone(),
            channels: vec![],
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&(QueuesM, Vec<ChannelsM>)> for Queue {
    fn from(value: &(QueuesM, Vec<ChannelsM>)) -> Self {
        Queue {
            id: value.0.id,
            name: value.0.name.clone(),
            channels: value.1.iter().map(|channel| channel.into()).collect(),
            created_at: value.0.created_at.to_utc(),
        }
    }
}
