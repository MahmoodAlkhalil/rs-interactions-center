use crate::db::entities::config_queues;
use crate::dtos::queues::requests::CreateQueue;
use crate::dtos::shared::ServiceDto;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, Set,
    TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<T>(db: &T) -> Result<Vec<config_queues::Model>, DbErr>
where
    T: ConnectionTrait + TransactionTrait,
{
    config_queues::Entity::find().all(db).await
}

pub async fn create<B>(
    dto: ServiceDto<'_, CreateQueue, B>,
) -> Result<config_queues::Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut queue = config_queues::ActiveModel::new();
    queue.id = Set(Uuid::new_v4());
    queue.name = Set(dto.request.name);
    let result = queue.insert(dto.db).await?;
    Ok(result)
}
