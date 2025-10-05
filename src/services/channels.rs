use crate::db::entities::channels::ActiveModel as ChannelsActiveModel;
use crate::db::repo::channels as ChannelsRepo;
use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelDto;
use crate::dtos::shared::RequestDto;
use crate::shared::errors::{IcError, NoType, WithMetadata};

use sea_orm::{ActiveModelBehavior, ConnectionTrait, Set, TransactionTrait};
use uuid::Uuid;

pub async fn get_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<ChannelDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let channels = ChannelsRepo::find_all(request.db)
        .await
        .with_metadata(request.id)?;
    let response: Vec<ChannelDto> = channels.iter().map(|c| c.into()).collect();
    Ok(response)
}

pub async fn get_by_id<B>(request: &RequestDto<'_, Uuid, B>) -> Result<ChannelDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Ok(ChannelsRepo::find_by_id(request.data.unwrap(), request.db)
        .await
        .with_metadata(request.id)?
        .ok_or(IcError::from((request.id, "Channel not found")))?
        .into())
}

pub async fn create<B>(request: &RequestDto<'_, CreateChannel, B>) -> Result<ChannelDto, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut channel = ChannelsActiveModel::new();
    channel.id = Set(Uuid::now_v7());
    channel.name = Set(request.data.as_ref().unwrap().name.clone());
    let channel = ChannelsRepo::insert(channel, request.db)
        .await
        .with_metadata(request.id)?;
    Ok(channel.into())
}
