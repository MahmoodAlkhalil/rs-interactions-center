use crate::db::entities::queues::Model as QueuesModel;
use serde::Serialize;
use uuid::Uuid;
#[derive(Serialize)]
pub struct Queue {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<QueuesModel> for Queue {
    fn from(value: QueuesModel) -> Self {
        Queue {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&QueuesModel> for Queue {
    fn from(value: &QueuesModel) -> Self {
        Queue {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
        }
    }
}
