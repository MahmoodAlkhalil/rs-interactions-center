use crate::db::entities::groups::{ActiveModel as GroupsAM, Entity as GroupsE};
use crate::dtos::groups::requests::CreateGroup;
use crate::dtos::groups::responses::Group as GroupDto;
use crate::dtos::shared::RequestDto;
use crate::utils::errors::{IcError, NoType};
use axum::http::StatusCode;
use sea_orm::{ConnectionTrait, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn create<B>(request: &RequestDto<'_, CreateGroup, B>) -> Result<GroupDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let group = GroupsAM {
        id: Set(Uuid::now_v7()),
        name: Set(request.data.as_ref().unwrap().name.clone()),
        ..Default::default()
    };
    let group = GroupsE::insert(group)
        .exec_with_returning(request.db)
        .await?;
    Ok(group.into())
}

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<GroupDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let groups = GroupsE::find().all(request.db).await?;
    Ok(groups.iter().map(|c| c.into()).collect())
}

pub async fn get_by_id<B>(request: &RequestDto<'_, Uuid, B>) -> Result<GroupDto, IcError>
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
