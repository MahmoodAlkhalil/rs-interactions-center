use crate::db::entities::users::ActiveModel as UsersActiveModel;
use crate::db::entities::users::Entity as UsersEntity;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::dtos::users::requests::CreateUser;
use crate::dtos::users::responses::User as UserDto;
use crate::shared::errors::NoType;
use crate::shared::errors::{IcError, WithMetadata};
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};

pub async fn get_all<B>(
    request: &RequestDto<'_, NoType, B>,
) -> Result<ApiResponse<Vec<UserDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let users = UsersEntity::find()
        .all(request.db)
        .await
        .with_metadata(request.id)?;
    let response: Vec<UserDto> = users.iter().map(|e| e.try_into().unwrap()).collect();
    Ok(ApiResponse {
        id: request.id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: Some(response),
    })
}

pub async fn create<B>(
    request: &RequestDto<'_, CreateUser, B>,
) -> Result<ApiResponse<UserDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut user = UsersActiveModel::new();
    user.id = Set(Uuid::now_v7());
    user.name = Set(request.data.as_ref().unwrap().name.clone());
    let user = user.insert(request.db).await.with_metadata(request.id)?;
    Ok(ApiResponse {
        id: request.id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: Some(user.try_into().unwrap()),
    })
}
