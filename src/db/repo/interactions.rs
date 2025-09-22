use crate::db::entities::interactions;
use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::shared::ServiceDto;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, Set,
    TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<T>(db: &T) -> Result<Vec<interactions::Model>, DbErr>
where
    T: ConnectionTrait + TransactionTrait,
{
    interactions::Entity::find().all(db).await
}

pub async fn create<B>(
    dto: &ServiceDto<'_, CreateInteraction, B>,
) -> Result<interactions::Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut interaction = interactions::ActiveModel::new();
    interaction.id = Set(Uuid::new_v4());
    let result = interaction.insert(dto.db).await?;
    Ok(result)
}
