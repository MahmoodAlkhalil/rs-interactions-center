use axum::http::StatusCode;
use core_engine_const::channel_worker_states::ChannelWorkerStates;
use core_engine_db::entities::channels::{ActiveModel as ChannelsAM, Entity as ChannelsE};
use core_engine_db::entities::channels_workers::ActiveModel as ChannelsWorkersAM;
use core_engine_db::entities::channels_workers::Entity as ChannelsWorkersE;
use core_engine_dto::{Channel, ChannelWorker, Request, errors::IcError};
use nanoid::nanoid;
use sea_orm::{ActiveModelBehavior, ConnectionTrait, EntityTrait, Set, TransactionTrait};
use tower::util::error::optional::None;
use uuid::Uuid;

pub async fn get_all(request: Request<()>) -> Result<Vec<Channel>, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let channels = ChannelsE::find().all(&shared_state.db_pool).await?;
    let response: Vec<Channel> = channels.into_iter().map(|c| c.into()).collect();
    Ok(response)
}

pub async fn get_by_id(request: Request<Uuid>) -> Result<Channel, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let channel = ChannelsE::find_by_id(request.data.unwrap())
        .one(&shared_state.db_pool)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "channel not found".to_string(),
        })?;
    Ok(channel.into())
}

pub async fn create(request: Request<Channel>) -> Result<Channel, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let data = data.unwrap();
    let mut channel = ChannelsAM::new();
    channel.id = Set(Uuid::now_v7());
    channel.name = Set(data.name.unwrap());
    let channel = ChannelsE::insert(channel)
        .exec_with_returning(&shared_state.db_pool)
        .await?;
    Ok(channel.into())
}

pub async fn register_channel_worker(
    request: Request<ChannelWorker>,
) -> Result<ChannelWorker, IcError> {
    let Request {
        id,
        shared_state,
        data,
    } = request;
    let data = data.unwrap();
    let channel_id = data.channel.unwrap().id.unwrap();
    let tx = shared_state.db_pool.begin().await?;
    ChannelsE::find_by_id(channel_id)
        .one(&tx)
        .await?
        .ok_or(IcError::not_found("channel not found"))?;
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
