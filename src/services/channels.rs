use crate::db::entities::channels::ActiveModel as ChannelsActiveModel;
use crate::db::entities::channels::Entity as ChannelsRepo;
use crate::dtos;
use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelDto;
use crate::dtos::shared::{ApiResponse, RequestDto};

use crate::shared::{IcError, WithMetadata, NoType};
use axum::Json;

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<B>(
    request: RequestDto<'_, NoType, B>,
) -> Result<Json<ApiResponse<Vec<ChannelDto>>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let channels = ChannelsRepo::find()
        .all(request.db)
        .await
        .with_metadata(request.id)?;
    let mut response: Vec<ChannelDto> = vec![];
    for channel in channels {
        response.push(ChannelDto {
            id: channel.id,
            name: channel.name,
            created_at: channel.created_at.to_utc(),
        })
    }
    Ok(Json(ApiResponse::new_success(request.id, None)))
}

pub async fn create<B>(
    request: RequestDto<'_, CreateChannel, B>,
) -> Result<Json<ChannelDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut new_channel = ChannelsActiveModel::new();
    new_channel.id = Set(Uuid::now_v7());
    new_channel.name = Set(request.data.unwrap().name);
    let new_queue = new_channel.insert(request.db).await.with_metadata(request.id)?;
    Ok(Json(ChannelDto {
        id: new_queue.id,
        name: new_queue.name,
        created_at: new_queue.created_at.to_utc(),
    }))
}
