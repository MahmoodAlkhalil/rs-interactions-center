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

impl TryFrom<Model> for Channel {
    type Error = ();

    fn try_from(value: Model) -> Result<Self, Self::Error> {
        Ok(Channel {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
            nats_topic_id: value.nats_topic_id,
        })
    }
}
impl TryFrom<&Model> for Channel {
    type Error = ();

    fn try_from(value: &Model) -> Result<Self, Self::Error> {
        Ok(Channel {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
            nats_topic_id: value.nats_topic_id,
        })
    }
}
