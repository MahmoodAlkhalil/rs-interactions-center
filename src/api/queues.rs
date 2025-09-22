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
use uuid::Uuid;

pub fn routes(api_shared_data: Arc<ApiSharedData>) -> Router {
    Router::new()
        .route("/queues", get(get_queues))
        .route("/queues", post(create_queue))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_queues(state: State<Arc<ApiSharedData>>) -> Result<axum::Json<Vec<Queue>>, IcError> {
    let request_id = Uuid::new_v4();
    info!("[{}] get queues", request_id);
    services::config_queues::get_all(ServiceDto::new(None, &state.db_pool)).await
}

#[debug_handler]
async fn create_queue(
    state: State<Arc<ApiSharedData>>,
    axum::extract::Json(request): axum::extract::Json<CreateQueue>,
) -> Result<axum::Json<Queue>, IcError> {
    let request_id = Uuid::new_v4();
    info!("[{}] create queue", request_id);
    services::config_queues::create(ServiceDto::new(Some(request), &state.db_pool)).await
}
