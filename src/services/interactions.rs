use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::shared::{ApiResponse, ServiceDto};
use crate::shared::helpers::extract;
use crate::shared::{IcError, NONE};
use crate::{db, dtos};
use axum::Json;
use sea_orm::{ConnectionTrait, TransactionTrait};

pub async fn get_all<B>(
    request: ServiceDto<'_, NONE, B>,
) -> Result<Json<Vec<dtos::interactions::responses::Interaction>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let interactions = extract(
        request.request_id,
        db::repo::interactions::get_all(request.db).await,
    )?;
    let mut response = vec![];
    for interaction in interactions {
        response.push(dtos::interactions::responses::Interaction {
            id: interaction.id,
            created_at: interaction.created_at.to_utc(),
        })
    }
    Ok(Json(response))
}

pub async fn create<B>(
    request: ServiceDto<'_, CreateInteraction, B>,
) -> Result<Json<ApiResponse<dtos::interactions::responses::Interaction>>, IcError>
where
    B: ConnectionTrait + TransactionTrait,
{
    let new_queue = extract(
        request.request_id,
        db::repo::interactions::create(&request).await,
    )?;
    let response = dtos::interactions::responses::Interaction {
        id: new_queue.id,
        created_at: new_queue.created_at.to_utc(),
    };
    Ok(Json(ApiResponse {
        request_id: request.request_id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: Some(response),
    }))
}
