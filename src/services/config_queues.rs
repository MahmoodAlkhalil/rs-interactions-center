use crate::shared::IcError;
use crate::{db, dtos};
use axum::Json;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn get_queues<T>(db: &T) -> Result<Json<Vec<dtos::queues::responses::Queue>>, IcError>
where
    T: ConnectionTrait + TransactionTrait,
{
    let queues = db::repo::config_queues::get_queues(db).await?;
    let mut response: Vec<dtos::queues::responses::Queue> = vec![];
    for queue in queues {
        response.push(dtos::queues::responses::Queue {
            id: queue.id,
            name: queue.name,
        })
    }
    Ok(Json(response))
}
