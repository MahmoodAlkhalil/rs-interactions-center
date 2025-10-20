use crate::{services::users as UsersService, utils::axum::RequestId};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use core_engine_dto::{
    ApiResponse, Request, SharedState, User, UserState, conversion::IntoApiResponse,
};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/users", get(get_all))
        .route("/users", post(create))
        .route("/users/states", get(get_all_states))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<User>> {
    let dto = Request::new(request_id, None, Arc::clone(&state));
    UsersService::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<User>,
) -> ApiResponse<User> {
    let dto = Request::new(request_id, Some(request), Arc::clone(&state));
    UsersService::create(dto).await.into_api_response()
}

#[debug_handler]
async fn get_all_states(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<UserState>> {
    let dto = Request::new(request_id, None, Arc::clone(&state));
    UsersService::get_all_states(dto).await.into_api_response()
}
