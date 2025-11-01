use axum::http::StatusCode;
use core_engine_const::{interaction_states::InteractionStates, user_states::UserStates};

use core_engine_dto::errors::ApiError;
use uuid::Uuid;

use crate::utils::local_caches::{
    INTERACTION_STATE_TO_UUID, USER_STATE_TO_UUID, UUID_TO_INTERACTION_STATE, UUID_TO_USER_STATE,
};

pub async fn uuid_to_interaction_state(val: &Uuid) -> Result<InteractionStates, ApiError> {
    UUID_TO_INTERACTION_STATE
        .read()
        .await
        .get(val)
        .ok_or(ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!(
                "interaction state of UUID [{}] doesn't map to any interaction state in the enum",
                val
            ),
        })
        .cloned()
}

pub async fn interaction_state_to_uuid(state: InteractionStates) -> Result<Uuid, ApiError> {
    INTERACTION_STATE_TO_UUID
        .read()
        .await
        .get(&state)
        .ok_or(ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!(
                "interaction state of enum variant [{}] doesn't map to any interaction state UUID",
                state.to_string()
            ),
        })
        .cloned()
}

pub async fn uuid_to_user_state(val: &Uuid) -> Result<UserStates, ApiError> {
    UUID_TO_USER_STATE
        .read()
        .await
        .get(val)
        .ok_or(ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!(
                "user state of UUID [{}] doesn't map to any user state enum",
                val
            ),
        })
        .cloned()
}

pub async fn user_state_to_uuid(state: UserStates) -> Result<Uuid, ApiError> {
    USER_STATE_TO_UUID
        .read()
        .await
        .get(&state)
        .ok_or(ApiError::internal_error(
            format!(
                "user state of enum variant [{}] doesn't map to any user state UUID",
                state.to_string()
            )
            .as_str(),
        ))
        .cloned()
}
