use crate::services::nats as NatsServices;
use axum::http::StatusCode;
use core_engine_db::entities::users::ActiveModel as UsersActiveModel;
use core_engine_db::entities::users::Entity as UsersEntity;
use core_engine_db::external_entities::user_states_tree_mv::Entity as UserStatesTreeE;
use core_engine_dto::{Request, User, UserState, errors::IcError};
use sea_orm::TransactionSession;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};
use tokio::task;
use tracing::error;

pub async fn get_all<B>(request: Request<'_, (), B>) -> Result<Vec<User>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let users = UsersEntity::find().all(request.db).await?;
    let users: Vec<User> = users.into_iter().map(|e| e.into()).collect();
    Ok(users)
}

pub async fn create<B>(request: Request<'_, User, B>) -> Result<User, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let Request { db, data, id } = request;
    let data = data.unwrap();
    let name = data.name.unwrap();
    let username = data.username.unwrap();
    let tx = db.begin().await?;
    let nats_key_pairs = NatsServices::generate_nkeys()?;
    let mut user = UsersActiveModel::new();
    let user_id = Uuid::now_v7();
    user.id = Set(user_id);
    user.username = Set(username);
    user.name = Set(name);
    user.nkey_seed = Set(nats_key_pairs.seed);
    user.nkey_pub = Set(nats_key_pairs.pub_key.clone());
    let user = user.insert(&tx).await?;
    let join_handle = task::spawn_blocking(move || {
        NatsServices::append_to_users_conf(
            user_id.to_string().as_str(),
            &nats_key_pairs.pub_key.as_str(),
        )
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

pub async fn get_all_states<B>(request: Request<'_, (), B>) -> Result<Vec<UserState>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let states = UserStatesTreeE::find().all(request.db).await?;
    Ok(states.into_iter().map(|state| state.into()).collect())
}
