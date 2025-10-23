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

pub async fn get_all(request: Request<()>) -> Result<Vec<User>, IcError> {
    let users = UsersEntity::find()
        .all(&request.shared_state.db_pool)
        .await?;
    let users: Vec<User> = users.into_iter().map(|e| e.into()).collect();
    Ok(users)
}

pub async fn create(request: Request<User>) -> Result<User, IcError> {
    let Request {
        data,
        shared_state,
        id,
    } = request;
    let data = data.unwrap();
    let name = data.name.unwrap();
    let username = data.username.unwrap();
    let tx = shared_state.db_pool.begin().await?;
    let nats_key_pairs = nkeys::KeyPair::new_user();
    let mut user = UsersActiveModel::new();
    let user_id = Uuid::now_v7();
    user.id = Set(user_id);
    user.username = Set(username);
    user.name = Set(name);
    user.nkey_seed = Set(nats_key_pairs.seed()?);
    user.nkey_pub = Set(nats_key_pairs.public_key());
    let user = user.insert(&tx).await?;
    let join_handle = task::spawn_blocking(move || {
        NatsServices::append_to_users_conf(
            user_id.to_string().as_str(),
            &nats_key_pairs.public_key().as_str(),
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

pub async fn get_all_states(request: Request<()>) -> Result<Vec<UserState>, IcError> {
    let states = UserStatesTreeE::find()
        .all(&request.shared_state.db_pool)
        .await?;
    Ok(states.into_iter().map(|state| state.into()).collect())
}
