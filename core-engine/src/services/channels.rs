use axum::http::StatusCode;
use core_engine_db::entities::channels::{ActiveModel as ChannelsAM, Entity as ChannelsE};
use core_engine_dto::{Channel, Request, errors::IcError};

use sea_orm::{ActiveModelBehavior, ConnectionTrait, EntityTrait, Set, TransactionTrait};
use tower::util::error::optional::None;
use uuid::Uuid;

pub async fn get_all<B>(request: Request<'_, None, B>) -> Result<Vec<Channel>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let channels = ChannelsE::find().all(request.db).await?;
    let response: Vec<Channel> = channels.into_iter().map(|c| c.into()).collect();
    Ok(response)
}

pub async fn get_by_id<B>(request: Request<'_, Uuid, B>) -> Result<Channel, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let channel = ChannelsE::find_by_id(request.data.unwrap())
        .one(request.db)
        .await?
        .ok_or(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "channel not found".to_string(),
        })?;
    Ok(channel.into())
}

pub async fn create<B>(request: Request<'_, Channel, B>) -> Result<Channel, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let Request { db, data } = request;
    let data = data.unwrap();
    let mut channel = ChannelsAM::new();
    channel.id = Set(Uuid::now_v7());
    channel.name = Set(data.name.unwrap());
    let channel = ChannelsE::insert(channel).exec_with_returning(db).await?;
    Ok(channel.into())
}
