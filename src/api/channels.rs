use crate::dtos::channels::requests::CreateChannel;
use crate::dtos::channels::responses::Channel as ChannelDto;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services;
use crate::utils::axum::RequestId;
use crate::utils::common::ToApiResponse;
use crate::utils::SharedState;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
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
) -> ApiResponse<Vec<ChannelDto>> {
    let dto = RequestDto::new(None, &state.db_pool);
    let response = services::channels::get_all(&dto).await;
    response.to_api_response(request_id)
}

#[debug_handler]
async fn get_by_id(
    state: State<Arc<SharedState>>,
    Path(id): Path<Uuid>,
    RequestId(request_id): RequestId,
) -> ApiResponse<ChannelDto> {
    let dto = RequestDto::new(Some(id), &state.db_pool);
    let response = services::channels::get_by_id(&dto).await;
    response.to_api_response(request_id)
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<CreateChannel>,
) -> ApiResponse<ChannelDto> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = services::channels::create(&dto).await;
    response.to_api_response(request_id)
}
