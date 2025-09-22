use crate::dtos::queues::requests::CreateQueue;
use crate::dtos::shared::ServiceDto;
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
        db::repo::config_queues::get_all(request.db).await,
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
        db::repo::config_queues::create(&request).await,
    )?;
    let response = dtos::queues::responses::Queue {
        id: new.id,
        name: new.name,
        created_at: new.created_at.to_utc(),
    };
    Ok(Json(response))
}
