use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelDto;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services;
use crate::shared::{IcError, SharedState};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Router};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/channels", get(get_all))
        .route("/channels", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
) -> Result<axum::Json<ApiResponse<Vec<ChannelDto>>>, IcError> {
    services::channels::get_all(RequestDto::new(None, &state.db_pool)).await
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    axum::extract::Json(request): axum::extract::Json<CreateChannel>,
) -> Result<axum::Json<ChannelDto>, IcError> {
    services::channels::create(RequestDto::new(Some(request), &state.db_pool)).await
}
