use crate::{
    services,
    utils::axum::{InnerRequest, RequestId, SharedState, TokenClaims},
};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use axum_jwks::Claims;
use core_engine_dto::{ApiResponse, Skill, conversion::IntoApiResponse};
use std::sync::Arc;

pub fn routes(api_shared_data: SharedState) -> Router {
    Router::new()
        .route("/skills", get(get_all))
        .route("/skills", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Skill>> {
    let dto = InnerRequest::new(request_id, None, state.0, claims);
    services::skills::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn create(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
    Json(request): Json<Skill>,
) -> ApiResponse<Skill> {
    let dto = InnerRequest::new(request_id, Some(request), state.0, claims);
    services::skills::create(dto).await.into_api_response()
}
