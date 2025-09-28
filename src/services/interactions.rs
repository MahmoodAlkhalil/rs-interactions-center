use crate::db::entities::interactions::*;
use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::interactions::responses::Interaction as InteractionDto;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::shared::{IcError, NoType, WithMetadata};
use axum::Json;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<B>(
    request: RequestDto<'_, NoType, B>,
) -> Result<Json<Vec<InteractionDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let interactions = Entity::find()
        .all(request.db)
        .await
        .with_metadata(request.id)?;
    let mut response = vec![];
    for interaction in interactions {
        response.push(InteractionDto {
            id: interaction.id,
            created_at: interaction.created_at.to_utc(),
        })
    }
    Ok(Json(response))
}

pub async fn create<B>(
    request: RequestDto<'_, CreateInteraction, B>,
) -> Result<Json<ApiResponse<InteractionDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut interaction = ActiveModel::new();
    interaction.id = sea_orm::Set(Uuid::now_v7());
    let interaction = interaction
        .insert(request.db)
        .await
        .with_metadata(request.id)?;
    let response = InteractionDto {
        id: interaction.id,
        created_at: interaction.created_at.to_utc(),
    };
    Ok(Json(ApiResponse {
        id: request.id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: Some(response),
    }))
}
