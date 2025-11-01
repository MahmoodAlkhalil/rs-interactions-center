use crate::{
    services::channels as ChannelsService,
    utils::axum::{InnerRequest, SharedState, TokenClaims},
};

use crate::utils::axum::RequestId;
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use axum_jwks::Claims;
use core_engine_dto::{
    ApiResponse, Channel, ChannelWorker, MessagingJwt, conversion::IntoApiResponse,
};
use std::sync::Arc;
use uuid::Uuid;

pub fn routes(api_shared_data: SharedState) -> Router {
    Router::new()
        .route("/nats/auth/jwt", post(validate_jwt))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn validate_jwt(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Channel>> {
    let dto = InnerRequest::new(request_id, None, state.0, claims);
    ChannelsService::get_all(dto).await.into_api_response()
}
