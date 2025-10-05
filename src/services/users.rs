use crate::db::entities::users::ActiveModel as UsersActiveModel;
use crate::db::entities::users::Entity as UsersEntity;
use crate::dtos::shared::RequestDto;
use crate::dtos::users::requests::CreateUser;
use crate::dtos::users::responses::User as UserDto;
use crate::utils::errors::NoType;
use crate::utils::errors::IcError;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<UserDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let users = UsersEntity::find().all(request.db).await?;
    let users: Vec<UserDto> = users.iter().map(|e| e.try_into().unwrap()).collect();
    Ok(users)
}

pub async fn create<B>(request: &RequestDto<'_, CreateUser, B>) -> Result<UserDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut user = UsersActiveModel::new();
    user.id = Set(Uuid::now_v7());
    user.name = Set(request.request.as_ref().unwrap().name.clone());
    let user = user.insert(request.db).await?;
    Ok(user.into())
}
