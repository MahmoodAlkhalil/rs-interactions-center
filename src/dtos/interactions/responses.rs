use crate::db::entities::interactions::Model;
use serde::Serialize;

#[derive(Serialize)]
pub struct Interaction {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<&Model> for Interaction {
    type Error = ();

    fn try_from(value: &Model) -> Result<Self, Self::Error> {
        Ok(Interaction {
            id: value.id,
            created_at: value.created_at.to_utc(),
        })
    }
}
