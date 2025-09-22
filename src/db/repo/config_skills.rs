use crate::db::entities::config_skills;
use crate::dtos::shared::ServiceDto;
use crate::dtos::skills::requests::CreateSkill;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, DbErr, EntityTrait, Set,
    TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<T>(db: &T) -> Result<Vec<config_skills::Model>, DbErr>
where
    T: ConnectionTrait + TransactionTrait,
{
    config_skills::Entity::find().all(db).await
}

pub async fn create<B>(dto: ServiceDto<'_, CreateSkill, B>) -> Result<config_skills::Model, DbErr>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut queue = config_skills::ActiveModel::new();
    queue.id = Set(Uuid::new_v4());
    queue.name = Set(dto.request.name);
    let result = queue.insert(dto.db).await?;
    Ok(result)
}
