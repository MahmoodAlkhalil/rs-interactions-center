use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::dtos::skills::requests::CreateSkill;
use crate::dtos::skills::responses::Skill;
use crate::services;
use crate::shared::SharedState;
use crate::shared::errors::IcError;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router, debug_handler};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/skills", get(get_all))
        .route("/skills", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(state: State<Arc<SharedState>>) -> Result<Json<ApiResponse<Vec<Skill>>>, IcError> {
    let dto = RequestDto::new(None, &state.db_pool);
    Ok(Json(ApiResponse::new_success(
        dto.id,
        Some(services::skills::get_all(&dto).await?),
    )))
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    Json(request): Json<CreateSkill>,
) -> Result<Json<ApiResponse<Skill>>, IcError> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    Ok(Json(ApiResponse::new_success(
        dto.id,
        Some(services::skills::create(&dto).await?),
    )))
}
