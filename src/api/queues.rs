use crate::dtos::queues::requests::CreateQueue;
use crate::dtos::queues::responses::Queue;
use crate::dtos::shared::ServiceDto;
use crate::services;
use crate::shared::{ApiSharedData, IcError};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Router};
use log::info;
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<ApiSharedData>) -> Router {
    info!("initializing queues router with a shared state");
    Router::new()
        .route("/queues", get(get_queues))
        .route("/queues", post(create_queue))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_queues(state: State<Arc<ApiSharedData>>) -> Result<axum::Json<Vec<Queue>>, IcError> {
    services::config_queues::get_all(&state.db_pool).await
}

#[debug_handler]
async fn create_queue(
    state: State<Arc<ApiSharedData>>,
    axum::extract::Json(request): axum::extract::Json<CreateQueue>,
) -> Result<axum::Json<Queue>, IcError> {
    services::config_queues::create(ServiceDto {
        request,
        db: &state.db_pool,
    })
    .await
}
