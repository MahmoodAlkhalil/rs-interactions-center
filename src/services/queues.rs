use crate::db::repo::config_queues as config_queues_repo;
use crate::db::repo::config_queues_to_channels_map as config_queues_to_channels_map_repo;
use crate::dtos::queues::requests::{CreateQueue, QueueToChannelsMapping};
use crate::dtos::shared::{ApiResponse, ServiceDto};
use crate::shared::helpers::extract;
use crate::shared::{IcError, NONE};
use crate::{db, dtos};
use axum::Json;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn get_all<B>(
    request: ServiceDto<'_, NONE, B>,
) -> Result<Json<Vec<dtos::queues::responses::Queue>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let queues = extract(
        request.request_id,
        config_queues_repo::get_all(request.db).await,
    )?;
    let mut response: Vec<dtos::queues::responses::Queue> = vec![];
    for queue in queues {
        response.push(dtos::queues::responses::Queue {
            id: queue.id,
            name: queue.name,
            created_at: queue.created_at.to_utc(),
        })
    }
    Ok(Json(response))
}

pub async fn create<B>(
    request: ServiceDto<'_, CreateQueue, B>,
) -> Result<Json<dtos::queues::responses::Queue>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let new = extract(
        request.request_id,
        config_queues_repo::create(request.data.unwrap(), request.db).await,
    )?;
    let response = dtos::queues::responses::Queue {
        id: new.id,
        name: new.name,
        created_at: new.created_at.to_utc(),
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
        config_queues_repo::find_by_uuid(request.data.as_ref().unwrap().queue_id, &transaction)
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
        .unwrap()
        .channels
        .iter()
        .map(|u| u.to_string())
        .collect();
    let db_channels = extract(
        request.request_id,
        db::repo::config_channels::find_all_by_uuid(&req_channels[..], &transaction).await,
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
        config_queues_to_channels_map_repo::delete_all_by_queue_internal_id(
            queue.internal_id,
            &transaction,
        )
        .await,
    )?;
    Ok(ApiResponse::new_success(request.request_id, None))
}
