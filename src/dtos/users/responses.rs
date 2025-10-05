use crate::db::entities::users::Model as UsersModel;
use serde::Serialize;
use uuid::Uuid;
#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<UsersModel> for User {
    fn from(value: UsersModel) -> Self {
        User {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&UsersModel> for User {
    fn from(value: &UsersModel) -> Self {
        User {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
        }
    }
}
