use crate::db::entities::user_states_tree_mv::Entity as UserStatesTreeE;
use crate::db::entities::users::ActiveModel as UsersActiveModel;
use crate::db::entities::users::Entity as UsersEntity;
use crate::dtos::shared::RequestDto;
use crate::dtos::users::requests::CreateUser;
use crate::dtos::users::responses::{User as UserDto, UserState};
use crate::utils::errors::IcError;
use crate::utils::errors::NoType;
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
    user.name = Set(request.data.as_ref().unwrap().name.clone());
    let user = user.insert(request.db).await?;
    Ok(user.into())
}

pub async fn get_all_states<B>(
    request: &RequestDto<'_, NoType, B>,
) -> Result<Vec<UserState>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let states = UserStatesTreeE::find().all(request.db).await?;
    Ok(states.iter().map(|state| state.clone().into()).collect())
}
