use serde::Serialize;
use uuid::Uuid;
#[derive(Serialize, Debug)]
pub struct Channel {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
