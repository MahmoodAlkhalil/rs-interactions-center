use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::interactions::responses::Interaction;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services;
pub use crate::shared::errors::IcError;
use crate::shared::SharedState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/interactions", get(get_all))
        .route("/interactions", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
) -> Result<Json<ApiResponse<Vec<Interaction>>>, IcError> {
    let dto = RequestDto::new(None, &state.db_pool);
    let response = services::interactions::get_all(&dto).await?;
    Ok(Json(ApiResponse::new_success(dto.id, Some(response))))
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    Json(request): Json<CreateInteraction>,
) -> Result<axum::Json<ApiResponse<Interaction>>, IcError> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = services::interactions::create(&dto).await?;
    Ok(Json(ApiResponse::new_success(dto.id, Some(response))))
}
