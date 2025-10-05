use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::dtos::users::requests::CreateUser;
use crate::dtos::users::responses::User as UserDto;
use crate::services::users as UsersService;
use crate::shared::errors::IcError;
use crate::shared::SharedState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{debug_handler, Json, Router};
use std::sync::Arc;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/users", get(get_all))
        .route("/users", post(create))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(
    state: State<Arc<SharedState>>,
) -> Result<Json<ApiResponse<Vec<UserDto>>>, IcError> {
    let dto = RequestDto::new(None, &state.db_pool);
    Ok(Json(ApiResponse::new_success(
        dto.id,
        Some(UsersService::get_all(&dto).await?),
    )))
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    Json(request): Json<CreateUser>,
) -> Result<Json<ApiResponse<UserDto>>, IcError> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    Ok(Json(ApiResponse::new_success(
        dto.id,
        Some(UsersService::create(&dto).await?),
    )))
}
