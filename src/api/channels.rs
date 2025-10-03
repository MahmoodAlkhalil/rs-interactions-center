use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelDto;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services;
use crate::shared::errors::IcError;
use crate::shared::SharedState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
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
    let dto = RequestDto::new(None, &state.db_pool);
    Ok(Json(services::channels::get_all(&dto).await?))
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    Json(request): axum::extract::Json<CreateChannel>,
) -> Result<axum::Json<ApiResponse<ChannelDto>>, IcError> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    Ok(Json(services::channels::create(&dto).await?))
}
