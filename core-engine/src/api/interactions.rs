use crate::{
    services,
    utils::axum::{InnerRequest, RequestId, SharedState, TokenClaims},
};

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use axum_jwks::Claims;
use core_engine_dto::{ApiResponse, Interaction, conversion::IntoApiResponse, errors::ApiError};
use std::sync::Arc;

pub fn routes(api_shared_data: SharedState) -> Router {
    Router::new()
        .route("/interactions", get(get_all))
        .route("/interactions", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Interaction>> {
    let dto = InnerRequest::new(request_id, None, state.0, claims);
    services::interactions::get_all(dto)
        .await
        .into_api_response()
}

#[debug_handler]
async fn create(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
    Json(request): Json<Interaction>,
) -> ApiResponse<Interaction> {
    let dto = InnerRequest::new(request_id, Some(request), state.0, claims);
    services::interactions::create(dto)
        .await
        .into_api_response()
}
