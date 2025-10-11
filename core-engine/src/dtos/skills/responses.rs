use crate::db::entities::skills::Model as SkillsModel;
use serde::Serialize;
#[derive(Serialize)]
pub struct Skill {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<SkillsModel> for Skill {
    fn from(value: SkillsModel) -> Self {
        Skill {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
        }
    }
}

impl From<&SkillsModel> for Skill {
    fn from(value: &SkillsModel) -> Self {
        Skill {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
        }
    }
}
