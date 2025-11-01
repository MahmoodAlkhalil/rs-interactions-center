use crate::utils::axum::{InnerRequest, SharedState, TokenClaims};
use crate::{services::users as UsersService, utils::axum::RequestId};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use axum_jwks::Claims;
use core_engine_dto::{ApiResponse, MessagingJwt, User, UserState, conversion::IntoApiResponse};
use std::sync::Arc;
use uuid::Uuid;

pub fn routes(state: SharedState) -> Router {
    Router::new()
        .route("/users", get(get_all))
        .route("/users", post(create))
        .route("/users/states", get(get_all_states))
        .route("/users/messaging-keys", get(get_messaging_key))
        .with_state(state)
}
#[debug_handler]
async fn get_all(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<User>> {
    let dto = InnerRequest::new(request_id, None, state.0, claims);
    UsersService::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn create(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
    Json(request): Json<User>,
) -> ApiResponse<User> {
    let dto = InnerRequest::new(request_id, Some(request), state.0, claims);
    UsersService::create(dto).await.into_api_response()
}

#[debug_handler]
async fn get_all_states(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<UserState>> {
    let dto = InnerRequest::new(request_id, None, state.0, claims);
    UsersService::get_all_states(dto).await.into_api_response()
}

#[debug_handler]
async fn get_messaging_key(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
) -> ApiResponse<MessagingJwt> {
    let dto = InnerRequest::new(request_id, None, state.0, claims);
    UsersService::get_messaging_token(dto)
        .await
        .into_api_response()
}
