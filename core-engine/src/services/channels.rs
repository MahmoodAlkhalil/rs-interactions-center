use crate::db::entities::channels::{ActiveModel as ChannelsAM, Entity as ChannelsE};
use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelDto;
use crate::dtos::shared::RequestDto;
use crate::utils::errors::{IcError, NoType};
use axum::http::StatusCode;

use sea_orm::{ActiveModelBehavior, ConnectionTrait, EntityTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<ChannelDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let channels = ChannelsE::find().all(request.db).await?;
    let response: Vec<ChannelDto> = channels.iter().map(|c| c.into()).collect();
    Ok(response)
}

pub async fn get_by_id<B>(request: &RequestDto<'_, Uuid, B>) -> Result<ChannelDto, IcError>
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

pub async fn create<B>(request: &RequestDto<'_, CreateChannel, B>) -> Result<ChannelDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut channel = ChannelsAM::new();
    channel.id = Set(Uuid::now_v7());
    channel.name = Set(request.data.as_ref().unwrap().name.clone());
    let channel = ChannelsE::insert(channel)
        .exec_with_returning(request.db)
        .await?;
    Ok(channel.into())
}


