use crate::db::entities::queues::{ActiveModel, Entity, Model};
use crate::db::entities::queues_channels_assignment::{
    ActiveModel as QueuesChannelsAssignmentActiveModel, Column as QueuesChannelsAssignmentColumn,
    Entity as QueuesChannelsAssignmentEntity, Model as QueuesChannelsAssignmentModel,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait, QueryFilter,
    Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn find_by_id<B>(id: Uuid, db: &B) -> Result<Option<Model>, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find_by_id(id).one(db).await
}

pub async fn find_all<B>(db: &B) -> Result<Vec<Model>, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find().all(db).await
}

pub async fn insert<B>(data: ActiveModel, db: &B) -> Result<Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    data.insert(db).await
}

pub async fn update<B>(data: ActiveModel, db: &B) -> Result<Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    data.update(db).await
}

pub async fn find_queue_with_assigned_channels<B>(
    id: Uuid,
    db: &B,
) -> Result<Vec<(Model, Vec<QueuesChannelsAssignmentModel>)>, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find_by_id(id)
        .find_with_related(QueuesChannelsAssignmentEntity)
        .all(db)
        .await
}

pub async fn delete_queue_to_channels_assignment<B>(
    data: &(Model, Vec<QueuesChannelsAssignmentModel>),
    db: &B,
) -> Result<DeleteResult, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    let channels: Vec<Uuid> = data.1.iter().map(|c| c.channel_id).collect();
    QueuesChannelsAssignmentEntity::delete_many()
        .filter(QueuesChannelsAssignmentColumn::QueueId.eq(data.0.id))
        .filter(QueuesChannelsAssignmentColumn::ChannelId.is_in(channels))
        .exec(db)
        .await
}

pub async fn insert_queue_to_channel_assignment<B>(
    queue_id: Uuid,
    channels_ids: &Vec<Uuid>,
    db: &B,
) -> Result<Vec<(Uuid, Uuid)>, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    let items: Vec<QueuesChannelsAssignmentActiveModel> = channels_ids
        .iter()
        .map(|channel_id| {
            return QueuesChannelsAssignmentActiveModel {
                queue_id: Set(queue_id.clone()),
                channel_id: Set(channel_id.clone()),
            };
        })
        .collect();
    QueuesChannelsAssignmentEntity::insert_many(items)
        .exec_with_returning_keys(db)
        .await
}
