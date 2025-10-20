use crate::{services, utils::axum::RequestId};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use core_engine_dto::{ApiResponse, Request, SharedState, Skill, conversion::IntoApiResponse};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/skills", get(get_all))
        .route("/skills", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Skill>> {
    let dto = Request::new(request_id, None, Arc::clone(&state));
    services::skills::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<Skill>,
) -> ApiResponse<Skill> {
    let dto = Request::new(request_id, Some(request), Arc::clone(&state));
    services::skills::create(dto).await.into_api_response()
}
