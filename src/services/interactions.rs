use crate::db::entities::interactions::*;
use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::interactions::responses::Interaction as InteractionDto;
use crate::dtos::shared::{ApiResponse, ServiceDto};
use crate::shared::helpers::extract;
use crate::shared::{IcError, NONE};
use axum::Json;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ConnectionTrait, EntityTrait, TransactionTrait,
};
use uuid::Uuid;

pub async fn get_all<B>(
    request: ServiceDto<'_, NONE, B>,
) -> Result<Json<Vec<InteractionDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let interactions = extract(request.request_id, Entity::find().all(request.db).await)?;
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
    request: ServiceDto<'_, CreateInteraction, B>,
) -> Result<Json<ApiResponse<InteractionDto>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let mut interaction = ActiveModel::new();
    interaction.id = sea_orm::Set(Uuid::now_v7());
    let interaction = extract(request.request_id, interaction.insert(request.db).await)?;
    let response = InteractionDto {
        id: interaction.id,
        created_at: interaction.created_at.to_utc(),
    };
    Ok(Json(ApiResponse {
        request_id: request.request_id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: Some(response),
    }))
}
