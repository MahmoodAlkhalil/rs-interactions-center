use crate::utils::SharedState;
use crate::{services, utils::axum::RequestId};

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use core_engine_dto::{
    ApiResponse, Interaction, Request, conversion::IntoApiResponse, errors::IcError,
};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/interactions", get(get_all))
        .route("/interactions", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Interaction>> {
    let dto = Request::new(request_id, None, &state.db_pool);
    services::interactions::get_all(dto)
        .await
        .into_api_response()
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<Interaction>,
) -> ApiResponse<Interaction> {
    let dto = Request::new(request_id, Some(request), &state.db_pool);
    services::interactions::create(dto)
        .await
        .into_api_response()
}
