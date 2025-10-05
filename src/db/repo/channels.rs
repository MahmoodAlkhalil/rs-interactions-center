use crate::db::entities::channels::{ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, DeleteResult, EntityTrait, QueryFilter,
    TransactionTrait,
};
use uuid::Uuid;

pub async fn find_by_id<B>(id: &Uuid, db: &B) -> Result<Option<Model>, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find_by_id(*id).one(db).await
}

pub async fn find_all<B>(db: &B) -> Result<Vec<Model>, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find().all(db).await
}

pub async fn find_all_by_ids<B>(ids: Vec<Uuid>, db: &B) -> Result<Vec<Model>, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find().filter(Column::Id.is_in(ids)).all(db).await
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
