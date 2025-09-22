use serde::Serialize;

#[derive(Serialize)]
pub struct Queue {
    pub id: uuid::Uuid,
    pub name: String,
}
