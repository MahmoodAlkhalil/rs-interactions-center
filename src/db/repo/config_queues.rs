use crate::db::entities::config_queues;
use crate::db::entities::config_queues::Model;
use crate::dtos::queues::requests::CreateQueue;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait,
    QueryFilter, Set, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<T>(db: &T) -> Result<Vec<config_queues::Model>, DbErr>
where
    T: ConnectionTrait + TransactionTrait,
{
    config_queues::Entity::find().all(db).await
}

pub async fn create<B>(request: CreateQueue, db: &B) -> Result<Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut queue = config_queues::ActiveModel::new();
    queue.id = Set(Uuid::new_v4());
    queue.name = Set(request.name);
    let result = queue.insert(db).await?;
    Ok(result)
}

pub async fn find_by_uuid<A>(id: Uuid, db: &A) -> Result<Option<Model>, DbErr>
where
    A: ConnectionTrait + TransactionTrait,
{
    config_queues::Entity::find()
        .filter(config_queues::Column::Id.contains(id))
        .one(db)
        .await
}
