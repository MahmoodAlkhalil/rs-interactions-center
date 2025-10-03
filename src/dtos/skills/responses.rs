use crate::db::entities::skills::Model as SkillsModel;
use serde::Serialize;
#[derive(Serialize)]
pub struct Skill {
    pub id: uuid::Uuid,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl TryFrom<SkillsModel> for Skill {
    type Error = ();

    fn try_from(value: SkillsModel) -> Result<Self, Self::Error> {
        Ok(Skill {
            id: value.id,
            name: value.name,
            created_at: value.created_at.to_utc(),
        })
    }
}

impl TryFrom<&SkillsModel> for Skill {
    type Error = ();

    fn try_from(value: &SkillsModel) -> Result<Self, Self::Error> {
        Ok(Skill {
            id: value.id,
            name: value.name.clone(),
            created_at: value.created_at.to_utc(),
        })
    }
}
