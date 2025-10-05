use crate::db::entities::interactions::Model;
use serde::Serialize;

#[derive(Serialize)]
pub struct Interaction {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<Model> for Interaction {
    fn from(value: Model) -> Self {
        Interaction {
            id: value.id,
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&Model> for Interaction {
    fn from(value: &Model) -> Self {
        Interaction {
            id: value.id,
            created_at: value.created_at.to_utc(),
        }
    }
}
