use crate::db::entities::groups::Model as GroupsModel;
use serde::Serialize;
#[derive(Serialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<GroupsModel> for Group {
    type Error = ();

    fn try_from(value: GroupsModel) -> Result<Self, Self::Error> {
        Ok(Group {
            id: value.id,
            created_at: value.created_at.to_utc(),
        })
    }
}

impl TryFrom<&GroupsModel> for Group {
    type Error = ();

    fn try_from(value: &GroupsModel) -> Result<Self, Self::Error> {
        Ok(Group {
            id: value.id,
            created_at: value.created_at.to_utc(),
        })
    }
}
