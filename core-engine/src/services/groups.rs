use axum::http::StatusCode;
use core_engine_db::entities::groups::{ActiveModel as GroupsAM, Entity as GroupsE};
use core_engine_dto::{Group, Request, errors::IcError};
use sea_orm::{ConnectionTrait, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn create<B>(request: Request<'_, Group, B>) -> Result<Group, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let Request { id, db, data } = request;
    let data = data.unwrap();
    let group = GroupsAM {
        id: Set(Uuid::now_v7()),
        name: Set(data.name.unwrap()),
        ..Default::default()
    };
    let group = GroupsE::insert(group).exec_with_returning(db).await?;
    Ok(group.into())
}

pub async fn get_all<B>(request: Request<'_, (), B>) -> Result<Vec<Group>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let groups = GroupsE::find().all(request.db).await?;
    Ok(groups.into_iter().map(|c| c.into()).collect())
}

pub async fn get_by_id<B>(request: Request<'_, Uuid, B>) -> Result<Group, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let group = GroupsE::find_by_id(request.data.unwrap())
        .one(request.db)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Group not found".to_string(),
        })?;
    Ok(group.into())
}
