use crate::dtos::queues::requests::{CreateQueue, EnqueueInteraction};
use crate::dtos::queues::responses::Queue;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services::queues as QueuesService;
use crate::utils::axum::RequestId;
use crate::utils::common::ToApiResponse;
use crate::utils::errors::NoType;
use crate::utils::SharedState;
use axum::extract::State;
use axum::routing::{get, post, put};
use axum::{debug_handler, Json, Router};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/queues", get(get_all))
        .route("/queues", post(create))
        .route("/queues/interactions", put(enqueue_interaction))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Queue>> {
    let dto = RequestDto::new(None, &state.db_pool);
    let response = QueuesService::get_all(&dto).await;
    response.to_api_response(request_id)
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<CreateQueue>,
) -> ApiResponse<Queue> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = QueuesService::create(&dto).await;
    response.to_api_response(request_id)
}

#[debug_handler]
async fn enqueue_interaction(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<EnqueueInteraction>,
) -> Json<ApiResponse<NoType>> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = QueuesService::enqueue_interaction(&dto).await;
    Json(response.to_api_response(request_id))
}
