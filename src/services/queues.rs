use crate::db::entities::channels::{Column as ChannelsColumn, Entity as ChannelsEntity};
use crate::db::entities::queues::*;
use crate::db::entities::queues_channels_assignment::{
    Column as QueuesChannelsAssignmentColumn, Entity as QueuesChannelsAssignmentEntity,
};
use crate::dtos::queues::requests::{CreateQueue, QueueToChannelsMapping};
use crate::dtos::queues::responses::Queue as QueueDto;
use crate::dtos::shared::{ApiResponse, ServiceDto};
use crate::shared::helpers::extract;
use crate::shared::{IcError, NONE};
use axum::Json;
use sea_orm::prelude::*;
use sea_orm::{ConnectionTrait, Set, TransactionTrait};

pub async fn get_all<B>(request: ServiceDto<'_, NONE, B>) -> Result<Json<Vec<QueueDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let queues = extract(request.request_id, Entity::find().all(request.db).await)?;
    let mut response: Vec<QueueDto> = vec![];
    for queue in queues {
        response.push(QueueDto {
            id: queue.id,
            name: queue.name,
            created_at: queue.created_at.to_utc(),
        })
    }
    Ok(Json(response))
}

pub async fn create<B>(request: ServiceDto<'_, CreateQueue, B>) -> Result<Json<QueueDto>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut queue = ActiveModel::new();
    queue.id = Set(Uuid::now_v7());
    queue.name = Set(request.data.unwrap().name);
    let queue = extract(request.request_id, queue.insert(request.db).await)?;
    let response = QueueDto {
        id: queue.id,
        name: queue.name,
        created_at: queue.created_at.to_utc(),
    };
    Ok(Json(response))
}

pub async fn replace_channels_mappings<B>(
    request: ServiceDto<'_, QueueToChannelsMapping, B>,
) -> Result<ApiResponse<QueueToChannelsMapping>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let transaction = extract(request.request_id, request.db.begin().await)?;
    let data = extract(
        request.request_id,
        Entity::find_by_id(request.data.as_ref().unwrap().queue_id)
            .one(request.db)
            .await,
    )?;
    let queue = match data {
        None => {
            return Err(IcError {
                id: request.request_id,
                message: "queue not found".to_string(),
            });
        }
        Some(data) => data,
    };
    let req_channels: Vec<String> = request
        .data
        .as_ref()
        .unwrap()
        .channels
        .iter()
        .map(|u| u.to_string())
        .collect();
    let db_channels = extract(
        request.request_id,
        ChannelsEntity::find()
            .filter(ChannelsColumn::Id.is_in(&req_channels[..]))
            .all(request.db)
            .await,
    )?;
    for req_channel in &req_channels[..] {
        for db_channel in &db_channels {
            if (!db_channel.id.to_string().eq(req_channel)) {
                return Err(IcError {
                    id: request.request_id,
                    message: format!("channel {} not found", req_channel),
                });
            }
        }
    }
    let delete_existing_mappings_result = extract(
        request.request_id,
        QueuesChannelsAssignmentEntity::delete_many()
            .filter(
                QueuesChannelsAssignmentColumn::QueueId.contains(request.data.unwrap().queue_id),
            )
            .exec(request.db)
            .await,
    )?;
    Ok(ApiResponse::new_success(request.request_id, None))
}
