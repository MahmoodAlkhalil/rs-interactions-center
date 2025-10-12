use crate::db::entities::users::ActiveModel as UsersActiveModel;
use crate::db::entities::users::Entity as UsersEntity;
use crate::db::external_entities::user_states_tree_mv::Entity as UserStatesTreeE;
use crate::dtos::nats::CreateNatsUser;
use crate::dtos::shared::RequestDto;
use crate::dtos::users::requests::CreateUser;
use crate::dtos::users::responses::{User as UserDto, UserState};
use crate::services::nats as NatsServices;
use crate::utils::errors::IcError;
use crate::utils::errors::NoType;
use axum::http::StatusCode;
use sea_orm::TransactionSession;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};
use tokio::task;
use tracing::error;

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
    let tx = request.db.begin().await?;
    let nats_key_pairs = NatsServices::generate_nkeys()?;
    let mut user = UsersActiveModel::new();
    user.id = Set(Uuid::now_v7());
    user.username = Set(request.data.as_ref().unwrap().username.clone());
    user.name = Set(request.data.as_ref().unwrap().name.clone());
    user.nkey_seed = Set(nats_key_pairs.seed);
    user.nkey_pub = Set(nats_key_pairs.pub_key);
    let user = user.insert(&tx).await?;
    let user_id = user.id.clone().to_string();
    let user_pub_key = user.nkey_pub.clone();

    let join_handle = task::spawn_blocking(move || {
        NatsServices::append_to_users_conf(CreateNatsUser {
            user_id: user_id.as_str(),
            pub_key: user_pub_key.as_str(),
        })
        .unwrap();
    });
    match join_handle.await {
        Ok(_) => {}
        Err(e) => {
            error!(
                "blocking task NatsServices::append_to_users_conf() failed. {}",
                e
            );
            return Err(IcError {
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
                message: "server error, check logs".to_string(),
            });
        }
    };
    tx.commit().await?;
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
