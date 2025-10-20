use axum::http::StatusCode;
use core_engine_db::entities::groups::{ActiveModel as GroupsAM, Entity as GroupsE};
use core_engine_dto::{Group, Request, errors::IcError};
use sea_orm::{ConnectionTrait, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn create(request: Request<Group>) -> Result<Group, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let data = data.unwrap();
    let group = GroupsAM {
        id: Set(Uuid::now_v7()),
        name: Set(data.name.unwrap()),
        ..Default::default()
    };
    let group = GroupsE::insert(group)
        .exec_with_returning(&shared_state.db_pool)
        .await?;
    Ok(group.into())
}

pub async fn get_all(request: Request<()>) -> Result<Vec<Group>, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let groups = GroupsE::find().all(&shared_state.db_pool).await?;
    Ok(groups.into_iter().map(|c| c.into()).collect())
}

pub async fn get_by_id(request: Request<Uuid>) -> Result<Group, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let group = GroupsE::find_by_id(request.data.unwrap())
        .one(&shared_state.db_pool)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Group not found".to_string(),
        })?;
    Ok(group.into())
}
