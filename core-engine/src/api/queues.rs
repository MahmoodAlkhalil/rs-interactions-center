use crate::{
    services::queues as QueuesService,
    utils::axum::{InnerRequest, SharedState, TokenClaims},
};

use crate::utils::axum::RequestId;
use axum::routing::{delete, get, post, put};
use axum::{Json, Router, debug_handler};
use axum::{
    extract::{Path, State},
    http::request,
};
use axum_jwks::Claims;
use core_engine_dto::{ApiResponse, Interaction, Queue, conversion::IntoApiResponse};
use std::sync::Arc;
use uuid::Uuid;

pub fn routes(api_shared_data: SharedState) -> Router {
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
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Queue>> {
    let dto = InnerRequest::new(request_id, None, state.0, claims);
    QueuesService::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn create(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
    Json(request): Json<Queue>,
) -> ApiResponse<Queue> {
    let dto = InnerRequest::new(request_id, Some(request), state.0, claims);
    QueuesService::create(dto).await.into_api_response()
}

#[debug_handler]
async fn enqueue_interaction(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
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
    let dto = InnerRequest::new(request_id, Some(request), state.0, claims);
    QueuesService::enqueue_interaction(dto)
        .await
        .into_api_response()
}

#[debug_handler]
async fn dequeue_interaction(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    Path(interaction_id): Path<Uuid>,
    RequestId(request_id): RequestId,
    Json(request): Json<Interaction>,
) -> ApiResponse<Interaction> {
    let mut request = request;
    request.id = Some(interaction_id);
    let dto = InnerRequest::new(request_id, Some(request), state.0, claims);
    QueuesService::dequeue_interaction(dto)
        .await
        .into_api_response()
}
