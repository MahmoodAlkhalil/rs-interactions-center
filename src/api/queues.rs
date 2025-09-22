use crate::dtos::queues::responses::Queue;
use crate::shared::{ApiSharedData, IcError};
use crate::{db, services};
use axum::extract::State;
use axum::routing::get;
use axum::{debug_handler, Json, Router};
use log::info;
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<ApiSharedData>) -> Router {
    info!("initializing queues router with a shared state");
    Router::new()
        .route("/", get(get_queues))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_queues(state: State<Arc<ApiSharedData>>) -> Result<Json<Vec<Queue>>, IcError> {
    services::config_queues::get_queues(&state.db_pool).await
}
