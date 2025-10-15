use crate::services::users as UsersService;
use crate::utils::SharedState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use core_engine_dto::{ApiResponse, Request, User, UserState, conversion::IntoApiResponse};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/users", get(get_all))
        .route("/users", post(create))
        .route("/users/states", get(get_all_states))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(state: State<Arc<SharedState>>) -> ApiResponse<Vec<User>> {
    let dto = Request::new(None, &state.db_pool);
    UsersService::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn create(state: State<Arc<SharedState>>, Json(request): Json<User>) -> ApiResponse<User> {
    let dto = Request::new(Some(request), &state.db_pool);
    UsersService::create(dto).await.into_api_response()
}

#[debug_handler]
async fn get_all_states(state: State<Arc<SharedState>>) -> ApiResponse<Vec<UserState>> {
    let dto = Request::new(None, &state.db_pool);
    UsersService::get_all_states(dto).await.into_api_response()
}
