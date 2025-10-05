use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelDto;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services;
use crate::shared::errors::IcError;
use crate::shared::SharedState;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use std::sync::Arc;
use uuid::Uuid;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/channels", get(get_all))
        .route("/channels/{id}", get(get_by_id))
        .route("/channels", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
) -> Result<axum::Json<ApiResponse<Vec<ChannelDto>>>, IcError> {
    let dto = RequestDto::new(None, &state.db_pool);
    let response = services::channels::get_all(&dto).await?;
    Ok(Json(ApiResponse::new_success(dto.id, Some(response))))
}

#[debug_handler]
async fn get_by_id(
    state: State<Arc<SharedState>>,
    Path(id): Path<Uuid>,
) -> Result<axum::Json<ApiResponse<ChannelDto>>, IcError> {
    let dto = RequestDto::new(Some(id), &state.db_pool);
    let response = services::channels::get_by_id(&dto).await?;
    Ok(Json(ApiResponse::new_success(dto.id, Some(response))))
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    Json(request): axum::extract::Json<CreateChannel>,
) -> Result<axum::Json<ApiResponse<ChannelDto>>, IcError> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = services::channels::create(&dto).await?;
    Ok(Json(ApiResponse::new_success(dto.id, Some(response))))
}
