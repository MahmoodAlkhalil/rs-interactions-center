use core_engine_db::entities::skills::*;
use core_engine_dto::{Request, Skill, errors::IcError};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<B>(request: Request<'_, (), B>) -> Result<Vec<Skill>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let skills = Entity::find()
        .filter(Column::ParentId.is_null())
        .all(request.db)
        .await?;
    let skills = skills.into_iter().map(|i| i.into()).collect();
    Ok(skills)
}

pub async fn create<B>(request: Request<'_, Skill, B>) -> Result<Skill, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut skill = ActiveModel::new();
    skill.id = Set(Uuid::now_v7());
    skill.name = Set(request.data.unwrap().name.unwrap());
    let skill = skill.insert(request.db).await?;
    Ok(skill.into())
}
