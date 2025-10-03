use crate::db::entities::queues::Model as QueuesModel;
use serde::Serialize;
use uuid::Uuid;
#[derive(Serialize)]
pub struct Queue {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<QueuesModel> for Queue {
    type Error = ();

    fn try_from(value: QueuesModel) -> Result<Self, Self::Error> {
        Ok(Queue {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
        })
    }
}

impl TryFrom<&QueuesModel> for Queue {
    type Error = ();

    fn try_from(value: &QueuesModel) -> Result<Self, Self::Error> {
        Ok(Queue {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
        })
    }
}
