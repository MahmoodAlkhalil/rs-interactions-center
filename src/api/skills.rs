use crate::dtos::shared::ServiceDto;
use crate::dtos::skills::requests::CreateSkill;
use crate::dtos::skills::responses::Skill;
use crate::services;
use crate::shared::{ApiSharedData, IcError};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Router};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<ApiSharedData>) -> Router {
    Router::new()
        .route("/skills", get(get_all))
        .route("/skills", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(state: State<Arc<ApiSharedData>>) -> Result<axum::Json<Vec<Skill>>, IcError> {
    services::config_skills::get_all(&state.db_pool).await
}

#[debug_handler]
async fn create(
    state: State<Arc<ApiSharedData>>,
    axum::extract::Json(request): axum::extract::Json<CreateSkill>,
) -> Result<axum::Json<Skill>, IcError> {
    services::config_skills::create(ServiceDto {
        request,
        db: &state.db_pool,
    })
    .await
}
