use crate::db::entities::channels::ActiveModel as ChannelsActiveModel;
use crate::db::entities::channels::Entity as ChannelsRepo;
use crate::dtos;
use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelResponse;
use crate::dtos::shared::{ApiResponse, ServiceDto};
use crate::shared::helpers::extract;
use crate::shared::{IcError, NONE};
use axum::Json;

use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<B>(
    request: ServiceDto<'_, NONE, B>,
) -> Result<Json<ApiResponse<Vec<dtos::channels::responses::Channel>>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let queues = extract(
        request.request_id,
        ChannelsRepo::find().all(request.db).await,
    )?;
    let mut response: Vec<dtos::channels::responses::Channel> = vec![];
    for queue in queues {
        response.push(dtos::channels::responses::Channel {
            id: queue.id,
            name: queue.name,
            created_at: queue.created_at.to_utc(),
        })
    }
    Ok(Json(ApiResponse::new_success(request.request_id, None)))
}

pub async fn create<B>(
    request: ServiceDto<'_, CreateChannel, B>,
) -> Result<Json<ChannelResponse>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut new_channel = ChannelsActiveModel::new();
    new_channel.id = Set(Uuid::now_v7());
    new_channel.name = Set(request.data.unwrap().name);
    let new_queue = extract(request.request_id, new_channel.insert(request.db).await)?;
    Ok(Json(ChannelResponse {
        id: new_queue.id,
        name: new_queue.name,
        created_at: new_queue.created_at.to_utc(),
    }))
}
