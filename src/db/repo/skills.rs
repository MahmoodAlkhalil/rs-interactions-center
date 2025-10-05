use crate::db::entities::skills::ActiveModel;
use crate::db::entities::skills::Entity;
use crate::db::entities::skills::Model;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, TransactionTrait,
};
use uuid::Uuid;

pub async fn find_by_id<B>(id: Uuid, db: &B) -> Result<Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(DbErr::Custom("skill not found".to_string()))
}

pub async fn find_all<B>(db: &B) -> Result<Vec<Model>, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find().all(db).await
}

pub async fn create<B>(model: ActiveModel, db: &B) -> Result<Model, DbErr>
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
