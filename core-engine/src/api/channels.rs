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
        .route("/channels", get(get_all))
        .route("/channels/{id}", get(get_by_id))
        .route("/channels", post(create))
        .route("/channels/{id}/workers", post(register_channel_worker))
        .route("/channels/{id}/messaging-keys", get(get_messaging_key))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Channel>> {
    let dto = InnerRequest::new(request_id, None, state.0, claims);
    ChannelsService::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn get_by_id(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    Path(id): Path<Uuid>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Channel> {
    let dto = InnerRequest::new(request_id, Some(id), state.0, claims);
    ChannelsService::get_by_id(dto).await.into_api_response()
}

#[debug_handler]
async fn create(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
    Json(request): Json<Channel>,
) -> ApiResponse<Channel> {
    let dto = InnerRequest::new(request_id, Some(request), state.0, claims);
    ChannelsService::create(dto).await.into_api_response()
}

#[debug_handler]
async fn register_channel_worker(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
    Path(id): Path<Uuid>,
) -> ApiResponse<ChannelWorker> {
    let request = ChannelWorker {
        channel: Some(Channel {
            id: Some(id),
            ..Default::default()
        }),
        ..Default::default()
    };
    let dto = InnerRequest::new(request_id, Some(request), state.0, claims);
    ChannelsService::register_channel_worker(dto)
        .await
        .into_api_response()
}

#[debug_handler]
async fn get_messaging_key(
    Claims(claims): Claims<TokenClaims>,
    state: State<SharedState>,
    RequestId(request_id): RequestId,
    Path(id): Path<Uuid>,
) -> ApiResponse<MessagingJwt> {
    let dto = InnerRequest::new(request_id, Some(id), state.0, claims);
    ChannelsService::get_messaging_key(dto)
        .await
        .into_api_response()
}
