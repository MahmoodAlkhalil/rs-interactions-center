use core_engine_db::entities::skills::*;
use core_engine_dto::{Request, Skill, errors::IcError};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all(request: Request<()>) -> Result<Vec<Skill>, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let skills = Entity::find()
        .filter(Column::ParentId.is_null())
        .all(&shared_state.db_pool)
        .await?;
    let skills = skills.into_iter().map(|i| i.into()).collect();
    Ok(skills)
}

pub async fn create(request: Request<Skill>) -> Result<Skill, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let mut skill = ActiveModel::new();
    skill.id = Set(Uuid::now_v7());
    skill.name = Set(data.unwrap().name.unwrap());
    let skill = skill.insert(&shared_state.db_pool).await?;
    Ok(skill.into())
}
