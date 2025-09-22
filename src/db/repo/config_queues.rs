use crate::db;
use db::entities;
use sea_orm::{ConnectionTrait, DbErr, EntityTrait, TransactionTrait};

pub async fn get_queues<T>(db: &T) -> Result<Vec<entities::config_queues::Model>, DbErr>
where
    T: ConnectionTrait + TransactionTrait,
{
    entities::config_queues::Entity::find().all(db).await
}
