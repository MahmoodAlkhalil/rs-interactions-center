use crate::db::entities::channels::ActiveModel as ChannelsActiveModel;
use crate::db::entities::channels::Entity as ChannelsRepo;
use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelDto;
use crate::dtos::shared::{ApiResponse, RequestDto};

use crate::shared::errors::{IcError, NoType, WithMetadata};

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<B>(
    request: &RequestDto<'_, NoType, B>,
) -> Result<ApiResponse<Vec<ChannelDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let channels = ChannelsRepo::find()
        .all(request.db)
        .await
        .with_metadata(request.id)?;
    let response: Vec<ChannelDto> = channels.iter().map(|c| c.try_into().unwrap()).collect();
    Ok(ApiResponse::new_success(request.id, Some(response)))
}

pub async fn create<B>(
    request: &RequestDto<'_, CreateChannel, B>,
) -> Result<ApiResponse<ChannelDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut channel = ChannelsActiveModel::new();
    channel.id = Set(Uuid::now_v7());
    channel.name = Set(request.data.as_ref().unwrap().name.clone());
    let channel = channel.insert(request.db).await.with_metadata(request.id)?;
    Ok(ApiResponse::new_success(
        request.id,
        Some(channel.try_into().unwrap()),
    ))
}
