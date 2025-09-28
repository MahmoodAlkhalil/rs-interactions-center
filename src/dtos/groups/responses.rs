use serde::Serialize;

#[derive(Serialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
