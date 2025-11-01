use core_engine_db::entities::skills::*;
use core_engine_dto::{Skill, errors::ApiError};
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, QueryFilter,
    Set, TransactionTrait,
};
use uuid::Uuid;

use crate::utils::axum::InnerRequest;

pub async fn get_all(request: InnerRequest<()>) -> Result<Vec<Skill>, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let skills = Entity::find()
        .filter(Column::ParentId.is_null())
        .all(shared_state.db_pool.as_ref())
        .await?;
    let skills = skills.into_iter().map(|i| i.into()).collect();
    Ok(skills)
}

pub async fn create(request: InnerRequest<Skill>) -> Result<Skill, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let mut skill = ActiveModel::new();
    skill.id = Set(Uuid::now_v7());
    skill.name = Set(data.unwrap().name.unwrap());
    let skill = skill.insert(shared_state.db_pool.as_ref()).await?;
    Ok(skill.into())
}
