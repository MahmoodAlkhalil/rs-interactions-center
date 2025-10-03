use crate::dtos::shared::RequestDto;
use crate::dtos::skills::requests::CreateSkill;
use crate::dtos::skills::responses::Skill;
use crate::services;
use crate::shared::errors::{ IcError};
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Router};
use std::sync::Arc;
use crate::shared::SharedState;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/skills", get(get_all))
        .route("/skills", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(state: State<Arc<SharedState>>) -> Result<axum::Json<Vec<Skill>>, IcError> {
    services::skills::get_all(RequestDto::new(None, &state.db_pool)).await
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    axum::extract::Json(request): axum::extract::Json<CreateSkill>,
) -> Result<axum::Json<Skill>, IcError> {
    services::skills::create(RequestDto::new(Some(request), &state.db_pool)).await
}
