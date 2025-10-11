use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::dtos::skills::requests::CreateSkill;
use crate::dtos::skills::responses::Skill;
use crate::services;
use crate::utils::axum::RequestId;
use crate::dtos::shared::ToApiResponse;
use crate::utils::SharedState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/skills", get(get_all))
        .route("/skills", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
) -> ApiResponse<Vec<Skill>> {
    let dto = RequestDto::new(None, &state.db_pool);
    let response = services::skills::get_all(&dto).await;
    response.to_api_response()
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    RequestId(request_id): RequestId,
    Json(request): Json<CreateSkill>,
) -> ApiResponse<Skill> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    let response = services::skills::create(&dto).await;
    response.to_api_response()
}
