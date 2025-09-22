use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::shared::ServiceDto;
use crate::shared::IcError;
use crate::{db, dtos};
use axum::Json;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn get_all<T>(db: &T) -> Result<Json<Vec<dtos::channels::responses::Channel>>, IcError>
where
    T: ConnectionTrait + TransactionTrait,
{
    let queues = db::repo::config_channels::get_all(db).await?;
    let mut response: Vec<dtos::channels::responses::Channel> = vec![];
    for queue in queues {
        response.push(dtos::channels::responses::Channel {
            id: queue.id,
            name: queue.name,
            created_at: queue.created_at.to_utc(),
        })
    }
    Ok(Json(response))
}

pub async fn create<B>(
    request: ServiceDto<'_, CreateChannel, B>,
) -> Result<Json<dtos::queues::responses::Queue>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let new_queue = db::repo::config_channels::create(request).await?;
    let response = dtos::queues::responses::Queue {
        id: new_queue.id,
        name: new_queue.name,
        created_at: new_queue.created_at.to_utc(),
    };
    Ok(Json(response))
}
