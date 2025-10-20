use axum::http::StatusCode;
use core_engine_db::entities::channels::{ActiveModel as ChannelsAM, Entity as ChannelsE};
use core_engine_dto::{Channel, Request, errors::IcError};

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
