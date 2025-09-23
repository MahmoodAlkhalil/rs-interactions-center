use crate::db;
use crate::db::entities::config_queues_to_channels_map::Model;
use db::entities::config_queues_to_channels_map::{Column, Entity};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait, ModelTrait, QueryFilter,
    TransactionTrait,
};

pub async fn find_all_by_queue_internal_id<A>(
    queue_internal_id: i64,
    db: &A,
) -> Result<Vec<Model>, DbErr>
where
    A: ConnectionTrait + TransactionTrait,
{
    Entity::find()
        .filter(Column::QueueInternalId.eq(queue_internal_id))
        .all(db)
        .await
}

pub async fn find_all_by_channel_internal_id<A>(
    channel_internal_id: i64,
    db: &A,
) -> Result<Vec<Model>, DbErr>
where
    A: ConnectionTrait + TransactionTrait,
{
    Entity::find()
        .filter(Column::ChannelInternalId.eq(channel_internal_id))
        .all(db)
        .await
}

pub async fn delete_all_by_queue_internal_id<A>(
    queue_internal_id: i64,
    db: &A,
) -> Result<DeleteResult, DbErr>
where
    A: ConnectionTrait + TransactionTrait,
{
    Entity::delete_many()
        .filter(Column::QueueInternalId.eq(queue_internal_id))
        .exec(db)
        .await
}
