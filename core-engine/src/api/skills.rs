use crate::services;
use crate::utils::SharedState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use core_engine_dto::{ApiResponse, Request, Skill, conversion::IntoApiResponse};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/skills", get(get_all))
        .route("/skills", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(state: State<Arc<SharedState>>) -> ApiResponse<Vec<Skill>> {
    let dto = Request::new(None, &state.db_pool);
    services::skills::get_all(dto).await.into_api_response()
}

#[debug_handler]
async fn create(state: State<Arc<SharedState>>, Json(request): Json<Skill>) -> ApiResponse<Skill> {
    let dto = Request::new(Some(request), &state.db_pool);
    services::skills::create(dto).await.into_api_response()
}
