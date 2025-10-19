use crate::services::channels as ChannelsService;
use crate::utils::SharedState;
use crate::utils::axum::RequestId;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use core_engine_dto::{ApiResponse, Channel, Request, conversion::IntoApiResponse};
use std::sync::Arc;
use uuid::Uuid;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/channels", get(get_all))
        .route("/channels/{id}", get(get_by_id))
        .route("/channels", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Channel>> {
    let dto = Request::new(request_id, None, &state.db_pool);
    ChannelsService::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn get_by_id(
    state: State<Arc<SharedState>>,
    Path(id): Path<Uuid>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Channel> {
    let dto = Request::new(request_id, Some(id), &state.db_pool);
    ChannelsService::get_by_id(dto).await.into_api_response()
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<Channel>,
) -> ApiResponse<Channel> {
    let dto = Request::new(request_id, Some(request), &state.db_pool);
    ChannelsService::create(dto).await.into_api_response()
}
