use crate::db::entities::channels::Model as ChannelsM;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<ChannelsM> for Channel {
    fn from(value: ChannelsM) -> Self {
        Channel {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
        }
    }
}
impl From<&ChannelsM> for Channel {
    fn from(value: &ChannelsM) -> Self {
        Channel {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
        }
    }
}
