use serde::Serialize;

#[derive(Serialize)]
pub struct Queue {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
