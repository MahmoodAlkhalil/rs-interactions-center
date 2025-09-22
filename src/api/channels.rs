use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel;
use crate::dtos::queues::responses::Queue;
use crate::dtos::shared::ServiceDto;
use crate::services;
use crate::shared::{ApiSharedData, IcError};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Router};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<ApiSharedData>) -> Router {
    Router::new()
        .route("/channels", get(get_all))
        .route("/channels", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(state: State<Arc<ApiSharedData>>) -> Result<axum::Json<Vec<Channel>>, IcError> {
    services::config_channels::get_all(&state.db_pool).await
}

#[debug_handler]
async fn create(
    state: State<Arc<ApiSharedData>>,
    axum::extract::Json(request): axum::extract::Json<CreateChannel>,
) -> Result<axum::Json<Queue>, IcError> {
    services::config_channels::create(ServiceDto {
        request,
        db: &state.db_pool,
    })
    .await
}
