use crate::api::interactions::IcError;
use crate::db::entities::interactions::Entity;
use crate::db::entities::interactions::Model;
use crate::dtos::shared::RequestDto;
use crate::shared::errors::{NoType, WithMetadata};
use sea_orm::{ConnectionTrait, EntityTrait, TransactionTrait};
use uuid::Uuid;

pub async fn find_by_id<B>(request: &RequestDto<'_, Uuid, B>) -> Result<Model, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find_by_id(request.data.unwrap())
        .one(request.db)
        .await
        .with_metadata(request.id)?
        .ok_or(IcError {
            id: request.id,
            message: "interaction not found".to_string(),
        })
}

pub async fn find_all<B>(request: &RequestDto<'_, NoType, B>) -> Result<Vec<Model>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    Entity::find()
        .all(request.db)
        .await
        .with_metadata(request.id)
}
