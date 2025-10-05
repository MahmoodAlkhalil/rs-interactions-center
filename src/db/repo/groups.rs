use crate::db::entities::groups::{ActiveModel, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait, TransactionTrait,
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

pub async fn insert<B>(model: ActiveModel, db: &B) -> Result<Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    model.insert(db).await
}

pub async fn update<B>(model: ActiveModel, db: &B) -> Result<Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    model.update(db).await
}

pub async fn delete_by_id<B>(id: Uuid, db: &B) -> Result<DeleteResult, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::delete_by_id(id).exec(db).await
}
