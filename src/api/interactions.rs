use crate::dtos::interactions::requests::CreateInteraction;
use crate::dtos::interactions::responses::Interaction;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services;
use crate::shared::{SharedState, IcError};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Router};
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
) -> Result<axum::Json<Vec<Interaction>>, IcError> {
    services::interactions::get_all(RequestDto::new(None, &state.db_pool)).await
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    axum::extract::Json(request): axum::extract::Json<CreateInteraction>,
) -> Result<axum::Json<ApiResponse<Interaction>>, IcError> {
    services::interactions::create(RequestDto::new(Some(request), &state.db_pool)).await
}
