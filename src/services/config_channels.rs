use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::shared::{ApiResponse, ServiceDto};
use crate::shared::helpers::extract;
use crate::shared::{IcError, NONE};
use crate::{db, dtos};
use axum::Json;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn get_all<B>(
    request: ServiceDto<'_, NONE, B>,
) -> Result<Json<ApiResponse<Vec<dtos::channels::responses::Channel>>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let queues = extract(
        request.request_id,
        db::repo::config_channels::get_all(request.db).await,
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
) -> Result<Json<dtos::queues::responses::Queue>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let new_queue = extract(
        request.request_id,
        db::repo::config_channels::create(&request).await,
    )?;
    let response = dtos::queues::responses::Queue {
        id: new_queue.id,
        name: new_queue.name,
        created_at: new_queue.created_at.to_utc(),
    };
    Ok(Json(response))
}
