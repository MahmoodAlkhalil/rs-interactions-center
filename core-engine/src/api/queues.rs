use crate::services::queues as QueuesService;
use crate::utils::SharedState;
use crate::utils::axum::RequestId;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router, debug_handler};
use axum::{
    extract::{Path, State},
    http::request,
};
use core_engine_dto::{ApiResponse, Interaction, Queue, Request, conversion::IntoApiResponse};
use std::sync::Arc;
use uuid::Uuid;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/queues", get(get_all))
        .route("/queues", post(create))
        .route(
            "/queues/{queue_id}/interactions/{interaction_id}",
            put(enqueue_interaction),
        )
        .route(
            "/queues/interactions/{interaction_id}",
            delete(dequeue_interaction),
        )
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Queue>> {
    let dto = Request::new(request_id, None, &state.db_pool);
    QueuesService::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<Queue>,
) -> ApiResponse<Queue> {
    let dto = Request::new(request_id, Some(request), &state.db_pool);
    QueuesService::create(dto).await.into_api_response()
}

#[debug_handler]
async fn enqueue_interaction(
    state: State<Arc<SharedState>>,
    Path((queue_id, interaction_id)): Path<(Uuid, Uuid)>,
    RequestId(request_id): RequestId,
    Json(request): Json<Interaction>,
) -> ApiResponse<Interaction> {
    let mut request = request;
    request.queue = Some(Queue {
        id: Some(queue_id),
        ..Default::default()
    });
    request.id = Some(interaction_id);
    let dto = Request::new(request_id, Some(request), &state.db_pool);
    QueuesService::enqueue_interaction(dto)
        .await
        .into_api_response()
}

#[debug_handler]
async fn dequeue_interaction(
    state: State<Arc<SharedState>>,
    Path(interaction_id): Path<Uuid>,
    RequestId(request_id): RequestId,
    Json(request): Json<Interaction>,
) -> ApiResponse<Interaction> {
    let mut request = request;
    request.id = Some(interaction_id);
    let dto = Request::new(request_id, Some(request), &state.db_pool);
    QueuesService::dequeue_interaction(dto)
        .await
        .into_api_response()
}
