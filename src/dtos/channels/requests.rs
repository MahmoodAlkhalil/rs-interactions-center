use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct CreateChannel {
    pub name: String,
}