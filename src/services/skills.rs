use crate::db::entities::skills::*;
use crate::dtos::shared::RequestDto;
use crate::dtos::skills::requests::CreateSkill;
use crate::dtos::skills::responses::Skill as SkillDto;
use crate::utils::errors::{IcError, NoType};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<SkillDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let skills = Entity::find()
        .filter(Column::ParentSkillId.is_null())
        .all(request.db)
        .await?;
    let skills = skills.iter().map(|i| i.into()).collect();
    Ok(skills)
}

pub async fn create<B>(request: &RequestDto<'_, CreateSkill, B>) -> Result<SkillDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut skill = ActiveModel::new();
    skill.id = Set(Uuid::now_v7());
    skill.name = Set(request.data.as_ref().unwrap().name.to_owned());
    let skill = skill.insert(request.db).await?;
    Ok(skill.into())
}
