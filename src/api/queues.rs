use crate::dtos::queues::requests::{CreateQueue, EnqueueInteraction};
use crate::dtos::queues::responses::Queue;
use crate::dtos::shared::{ApiResponse, RequestDto};
use crate::services::queues as QueuesService;
use crate::shared::errors::IcError;
use crate::shared::SharedState;
use axum::extract::State;
use axum::routing::{get, post, put};
use axum::{debug_handler, Json, Router};
use log::info;
use std::sync::Arc;
use uuid::Uuid;

pub fn routes(api_shared_data: Arc<SharedState>) -> Router {
    Router::new()
        .route("/queues", get(get_all))
        .route("/queues", post(create))
        .route("/queues/interactions", put(enqueue_interaction))
        .with_state(api_shared_data)
}
#[debug_handler]
async fn get_all(state: State<Arc<SharedState>>) -> Result<Json<ApiResponse<Vec<Queue>>>, IcError> {
    let dto = RequestDto::new(None, &state.db_pool);
    Ok(Json(QueuesService::get_all(&dto).await?))
}

#[debug_handler]
async fn create(
    state: State<Arc<SharedState>>,
    Json(request): Json<CreateQueue>,
) -> Result<Json<Queue>, IcError> {
    let request_id = Uuid::new_v4();
    info!("[{}] create queue", request_id);
    QueuesService::create(RequestDto::new(Some(request), &state.db_pool)).await
}
#[debug_handler]
async fn enqueue_interaction(
    state: State<Arc<SharedState>>,
    Json(request): Json<EnqueueInteraction>,
) -> Result<Json<ApiResponse<Queue>>, IcError> {
    let dto = RequestDto::new(Some(request), &state.db_pool);
    QueuesService::enqueue_interaction(&dto).await?;
    Ok(Json(ApiResponse {
        id: dto.id,
        message: "SUCCESS".to_string(),
        code: 0,
        data: None,
    }))
}
