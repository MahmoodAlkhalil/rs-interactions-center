use axum::http::StatusCode;
use core_engine_const::interaction_states::InteractionStates;
use core_engine_dto::errors::IcError;
use uuid::Uuid;

use crate::utils::local_caches::{INTERACTION_STATE_TO_UUID, UUID_TO_INTERACTION_STATE};

pub async fn uuid_to_interaction_state(val: &Uuid) -> Result<InteractionStates, IcError> {
    UUID_TO_INTERACTION_STATE
        .read()
        .await
        .get(val)
        .cloned()
        .ok_or(IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!(
                "interaction state of UUID [{}] doesn't map to any interaction state in the enum",
                val
            ),
        })
}

pub async fn interaction_state_to_uuid(state: InteractionStates) -> Result<Uuid, IcError> {
    INTERACTION_STATE_TO_UUID
        .read()
        .await
        .get(&state)
        .cloned()
        .ok_or(IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: format!(
                "interaction state of enum variant [{}] doesn't map to any interaction state UUID",
                state.to_string()
            ),
        })
}
