use crate::db::entities::users::Model as UsersModel;
use serde::Serialize;
use uuid::Uuid;
#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<UsersModel> for User {
    type Error = ();

    fn try_from(value: UsersModel) -> Result<Self, Self::Error> {
        Ok(User {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
        })
    }
}

impl TryFrom<&UsersModel> for User {
    type Error = ();

    fn try_from(value: &UsersModel) -> Result<Self, Self::Error> {
        Ok(User {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
        })
    }
}
