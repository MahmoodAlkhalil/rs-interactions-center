use crate::db::entities::config_channels;
use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::shared::ServiceDto;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, Set,
    TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<T>(db: &T) -> Result<Vec<config_channels::Model>, DbErr>
where
    T: ConnectionTrait + TransactionTrait,
{
    config_channels::Entity::find().all(db).await
}

pub async fn create<B>(
    dto: &ServiceDto<'_, CreateChannel, B>,
) -> Result<config_channels::Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut channel = config_channels::ActiveModel::new();
    channel.id = Set(Uuid::new_v4());
    channel.name = Set(dto.data.as_ref().unwrap().name.clone());
    let result = channel.insert(dto.db).await?;
    Ok(result)
}
