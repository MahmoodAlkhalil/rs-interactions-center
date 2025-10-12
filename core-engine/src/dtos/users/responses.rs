use crate::db::entities::user_states::Model as UserStatesM;
use crate::db::external_entities::user_states_tree_mv::Model as UserStatesTreeM;
use crate::db::entities::users::Model as UsersM;
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize)]
pub struct UserState {
    pub id: i32,
    pub name: String,
    pub level: i32,
    pub path: Vec<i32>,
    pub path_text: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<UsersM> for User {
    fn from(value: UsersM) -> Self {
        User {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&UsersM> for User {
    fn from(value: &UsersM) -> Self {
        User {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<UserStatesTreeM> for UserState {
    fn from(value: UserStatesTreeM) -> Self {
        UserState {
            id: value.id,
            name: value.name,
            level: value.level,
            path: value.path,
            path_text: value.full_path,
            created_at: value.created_at.to_utc(),
        }
    }
}

