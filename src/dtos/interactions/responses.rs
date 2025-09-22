use serde::Serialize;

#[derive(Serialize)]
pub struct Interaction {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
