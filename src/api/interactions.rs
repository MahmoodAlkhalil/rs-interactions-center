use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::interactions::responses::Interaction;
use crate::dtos::shared::{ApiResponse, ServiceDto};
use crate::services;
use crate::shared::{ApiSharedData, IcError};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Router};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<ApiSharedData>) -> Router {
    Router::new()
        .route("/interactions", get(get_all))
        .route("/interactions", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<ApiSharedData>>,
) -> Result<axum::Json<Vec<Interaction>>, IcError> {
    services::interactions::get_all(ServiceDto::new(None, &state.db_pool)).await
}

#[debug_handler]
async fn create(
    state: State<Arc<ApiSharedData>>,
    axum::extract::Json(request): axum::extract::Json<CreateInteraction>,
) -> Result<axum::Json<ApiResponse<Interaction>>, IcError> {
    services::interactions::create(ServiceDto::new(Some(request), &state.db_pool)).await
}
