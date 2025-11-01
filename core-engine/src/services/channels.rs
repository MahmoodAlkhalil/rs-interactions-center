use axum::http::StatusCode;
use core_engine_const::channel_worker_states::ChannelWorkerStates;
use core_engine_db::entities::channels::{
    ActiveModel as ChannelsAM, Column as ChannelsC, Entity as ChannelsE,
};
use core_engine_db::entities::channels_workers::ActiveModel as ChannelsWorkersAM;
use core_engine_db::entities::channels_workers::Entity as ChannelsWorkersE;
use core_engine_dto::{Channel, ChannelWorker, MessagingJwt, errors::ApiError};
use nanoid::nanoid;
use sea_orm::ColumnTrait;
use sea_orm::{
    ActiveModelBehavior, ConnectionTrait, EntityTrait, QueryFilter, Set, TransactionTrait,
};
use std::str::FromStr;
use tower::util::error::optional::None;
use uuid::Uuid;

use crate::{services::nats, utils::axum::InnerRequest};

pub async fn get_all(request: InnerRequest<()>) -> Result<Vec<Channel>, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let channels = ChannelsE::find().all(shared_state.db_pool.as_ref()).await?;
    let response: Vec<Channel> = channels.into_iter().map(|c| c.into()).collect();
    Ok(response)
}

pub async fn get_by_id(request: InnerRequest<Uuid>) -> Result<Channel, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let channel = ChannelsE::find_by_id(request.data.unwrap())
        .one(shared_state.db_pool.as_ref())
        .await?
        .ok_or(ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: "channel not found".to_string(),
        })?;
    Ok(channel.into())
}

pub async fn create(request: InnerRequest<Channel>) -> Result<Channel, ApiError> {
    let InnerRequest {
        shared_state, data, ..
    } = request;
    let data = data.unwrap();
    let name = data.name.unwrap();
    let exist_by_name = ChannelsE::find()
        .filter(ChannelsC::Name.eq(&name))
        .one(shared_state.db_pool.as_ref())
        .await?
        .is_some();
    if exist_by_name {
        return Err(ApiError::bad_request(
            "channel with same name already exists",
        ));
    }
    let nats_account = nats::generate_channel(&name, &*shared_state.db_pool).await?;
    let mut channel = ChannelsAM::new();
    let id = Uuid::now_v7();
    channel.id = Set(id);
    channel.name = Set(name);
    channel.nkey_pub = Set(nats_account.0.public_key());
    channel.nkey_seed = Set(nats_account.0.seed()?);
    channel.nats_jwt = Set(nats_account.1);
    channel.client_id = Set(Uuid::new_v4());
    let channel = ChannelsE::insert(channel)
        .exec_with_returning(shared_state.db_pool.as_ref())
        .await?;
    Ok(channel.into())
}

pub async fn get_messaging_key(request: InnerRequest<Uuid>) -> Result<(MessagingJwt), ApiError> {
    let client_id = request
        .claims
        .client_id
        .ok_or(ApiError::bad_request("client_id is not in token"))?;
    let channel = ChannelsE::find_by_id(request.data.unwrap())
        .one(&*request.shared_state.db_pool)
        .await?
        .ok_or(ApiError::not_found(
            "channel not found by client_id in token",
        ))?;
    if channel.client_id.to_string() != client_id {
        return Err(ApiError::bad_request("bad client_id"));
    }

    Ok(MessagingJwt {
        seed: channel.nkey_seed,
        jwt: channel.nats_jwt,
    })
}
pub async fn register_channel_worker(
    request: InnerRequest<ChannelWorker>,
) -> Result<ChannelWorker, ApiError> {
    let InnerRequest {
        id,
        shared_state,
        data,
        claims,
        ..
    } = request;
    let data = data.unwrap();
    let channel_id = data.channel.unwrap().id.unwrap();
    let tx = shared_state.db_pool.begin().await?;
    ChannelsE::find_by_id(channel_id)
        .one(&tx)
        .await?
        .ok_or(ApiError::not_found("channel not found"))?;
    let channel_worker = ChannelsWorkersAM {
        id: Set(nanoid!(10)),
        state: Set(ChannelWorkerStates::New.to_string()),
        channel_id: Set(channel_id),
        ..Default::default()
    };
    let channel_worker = ChannelsWorkersE::insert(channel_worker)
        .exec_with_returning(&tx)
        .await?;
    tx.commit().await?;
    Ok(channel_worker.into())
}
