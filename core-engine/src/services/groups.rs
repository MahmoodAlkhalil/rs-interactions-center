use axum::http::StatusCode;
use core_engine_db::entities::groups::{ActiveModel as GroupsAM, Entity as GroupsE};
use core_engine_dto::{Group, errors::ApiError};
use sea_orm::{ConnectionTrait, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

use crate::utils::axum::InnerRequest;

pub async fn create(request: InnerRequest<Group>) -> Result<Group, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let data = data.unwrap();
    let group = GroupsAM {
        id: Set(Uuid::now_v7()),
        name: Set(data.name.unwrap()),
        ..Default::default()
    };
    let group = GroupsE::insert(group)
        .exec_with_returning(shared_state.db_pool.as_ref())
        .await?;
    Ok(group.into())
}

pub async fn get_all(request: InnerRequest<()>) -> Result<Vec<Group>, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let groups = GroupsE::find().all(shared_state.db_pool.as_ref()).await?;
    Ok(groups.into_iter().map(|c| c.into()).collect())
}

pub async fn get_by_id(request: InnerRequest<Uuid>) -> Result<Group, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let group = GroupsE::find_by_id(request.data.unwrap())
        .one(shared_state.db_pool.as_ref())
        .await?
        .ok_or(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Group not found".to_string(),
        })?;
    Ok(group.into())
}
