use crate::dtos::queues::requests::{CreateQueue, DequeueInteraction, EnqueueInteraction};
use crate::dtos::queues::responses::Queue;
use crate::dtos::shared::ToApiResponse;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services::queues as QueuesService;
use crate::utils::axum::RequestId;
use crate::utils::SharedState;
use axum::extract::{Path, State};
use axum::routing::{delete, get, post, put};
use axum::{debug_handler, Json, Router};
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
            "/queues/{queue_id}/interactions/{interaction_id}",
            delete(dequeue_interaction_from_queue),
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
    let dto = RequestDto::new(None, &state.db_pool);
    let response = QueuesService::get_all(&dto).await;
    response.to_api_response()
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<CreateQueue>,
) -> ApiResponse<Queue> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = QueuesService::create(&dto).await;
    response.to_api_response()
}

#[debug_handler]
async fn enqueue_interaction(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Path((queue_id, interaction_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<EnqueueInteraction>,
) -> ApiResponse<()> {
    let mut request = request;
    request.queue_id = queue_id;
    request.interaction_id = interaction_id;
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = QueuesService::enqueue_interaction(&dto).await;
    response.to_api_response()
}

#[debug_handler]
async fn dequeue_interaction(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Path(interaction_id): Path<Uuid>,
    Json(request): Json<DequeueInteraction>,
) -> ApiResponse<()> {
    let mut request = request;
    request.interaction_id = interaction_id;
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = QueuesService::dequeue_interaction(&dto).await;
    response.to_api_response()
}

#[debug_handler]
async fn dequeue_interaction_from_queue(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Path((queue_id, interaction_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<DequeueInteraction>,
) -> ApiResponse<()> {
    let mut request = request;
    request.interaction_id = interaction_id;
    request.queue_id = Some(queue_id);
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = QueuesService::dequeue_interaction(&dto).await;
    response.to_api_response()
}
