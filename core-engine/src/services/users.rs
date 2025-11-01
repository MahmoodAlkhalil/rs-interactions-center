use crate::{
    services::{self, nats as NatsServices},
    utils::{
        axum::InnerRequest,
        conversions::user_state_to_uuid,
        local_caches::{USER_STATE_TO_UUID, UUID_TO_USER_STATE},
    },
};
use axum::http::StatusCode;
use core_engine_const::user_states::UserStates;
use core_engine_db::entities::user_states::Entity as UserStatesE;
use core_engine_db::entities::users::ActiveModel as UsersActiveModel;
use core_engine_db::entities::users::Column as UsersC;
use core_engine_db::entities::users::Entity as UsersE;
use core_engine_db::external_entities::user_states_tree_mv::Entity as UserStatesTreeE;
use core_engine_dto::{MessagingJwt, User, UserState, errors::ApiError};
use sea_orm::TransactionSession;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};
use strum::VariantArray;
use tokio::task;
use tracing::{error, info};

pub async fn get_all(request: InnerRequest<()>) -> Result<Vec<User>, ApiError> {
    info!("getting all users");
    let users = UsersE::find()
        .all(request.shared_state.db_pool.as_ref())
        .await?;
    let users: Vec<User> = users.into_iter().map(|e| e.into()).collect();
    Ok(users)
}

pub async fn create(request: InnerRequest<User>) -> Result<User, ApiError> {
    let InnerRequest {
        data, shared_state, ..
    } = request;
    let data = data.unwrap();
    let name = data.name.unwrap();
    let username = data.username.unwrap();
    let tx = shared_state.db_pool.begin().await?;
    let user_id = Uuid::now_v7();
    let mut user = UsersActiveModel::new();
    let nats_user = services::nats::generate_user(user_id, &tx).await?;
    user.id = Set(user_id);
    user.username = Set(username);
    user.name = Set(name);
    user.nkey_pub = Set(nats_user.0.public_key());
    user.nkey_seed = Set(nats_user.0.seed()?);
    user.nats_jwt = Set(nats_user.1);
    user.state = Set(user_state_to_uuid(UserStates::Offline).await?);
    let user = user.insert(&tx).await?;
    tx.commit().await?;
    Ok(user.into())
}

pub async fn get_all_states(request: InnerRequest<()>) -> Result<Vec<UserState>, ApiError> {
    let states = UserStatesTreeE::find()
        .all(request.shared_state.db_pool.as_ref())
        .await?;
    Ok(states.into_iter().map(|state| state.into()).collect())
}

pub async fn get_messaging_token(request: InnerRequest<Uuid>) -> Result<MessagingJwt, ApiError> {
    let username = request
        .claims
        .preferred_username
        .ok_or(ApiError::bad_request("no preferred_username in token"))?;
    let user = UsersE::find()
        .filter(UsersC::Username.eq(username))
        .one(&*request.shared_state.db_pool)
        .await?
        .ok_or(ApiError::not_found("user not found"))?;

    if request.claims.sub != request.data.unwrap().to_string() {
        return Err(ApiError::bad_request("user IDs are not matching"));
    }
    Ok(MessagingJwt {
        seed: user.nkey_seed,
        jwt: user.nats_jwt,
    })
}

pub async fn init_user_states_local_cache(db: &DatabaseConnection) -> Result<(), ApiError> {
    let db_interaction_states = UserStatesE::find().all(db).await?;
    let enum_interaction_states = UserStates::VARIANTS;

    for db_state in db_interaction_states.into_iter() {
        for enum_state in enum_interaction_states.iter() {
            if db_state.name.as_str() == enum_state.to_string() {
                {
                    USER_STATE_TO_UUID
                        .write()
                        .await
                        .insert(enum_state.clone(), db_state.id.clone());
                }

                {
                    UUID_TO_USER_STATE
                        .write()
                        .await
                        .insert(db_state.id.clone(), enum_state.clone());
                }
            }
        }
    }
    info!(
        "prepared interaction states to UUID local hasmap {:?}",
        USER_STATE_TO_UUID
    );
    info!(
        "prepared UUID to interaction states local hasmap {:?}",
        UUID_TO_USER_STATE
    );
    Ok(())
}
