use crate::db::entities::channels::Model;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub nats_topic_id: i64,
}

impl From<Model> for Channel {
    fn from(value: Model) -> Self {
        Channel {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
            nats_topic_id: value.nats_topic_id,
        }
    }
}
impl From<&Model> for Channel {
    fn from(value: &Model) -> Self {
        Channel {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
            nats_topic_id: value.nats_topic_id,
        }
    }
}
