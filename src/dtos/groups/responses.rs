use crate::db::entities::groups::Model as GroupsModel;
use serde::Serialize;
#[derive(Serialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<GroupsModel> for Group {
    fn from(value: GroupsModel) -> Self {
        Group {
            id: value.id,
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&GroupsModel> for Group {
    fn from(value: &GroupsModel) -> Self {
        Group {
            id: value.id,
            created_at: value.created_at.to_utc(),
        }
    }
}
