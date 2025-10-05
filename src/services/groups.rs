use crate::db::entities::groups::ActiveModel as GroupsActiveModel;
use crate::db::repo::groups as GroupsRepo;
use crate::dtos::groups::requests::CreateGroup;
use crate::dtos::groups::responses::Group as GroupDto;
use crate::dtos::shared::RequestDto;
use crate::utils::errors::{IcError, NoType};
use axum::http::StatusCode;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn create<B>(request: &RequestDto<'_, CreateGroup, B>) -> Result<GroupDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let group = GroupsRepo::insert(
        GroupsActiveModel {
            id: Set(Uuid::now_v7()),
            name: Set(request.request.as_ref().unwrap().name.clone()),
            mark_for_delete: Set(false),
            ..Default::default()
        },
        request.db,
    )
    .await?;
    Ok(group.into())
}

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<GroupDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Ok(GroupsRepo::find_all(request.db)
        .await?
        .iter()
        .map(|c| c.into())
        .collect())
}

pub async fn get_by_id<B>(request: &RequestDto<'_, Uuid, B>) -> Result<GroupDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Ok(GroupsRepo::find_by_id(request.request.unwrap(), request.db)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Group not found".to_string(),
        })?
        .into())
}
