use crate::utils::errors::{NoType, StateValidationError};
use std::fmt::{Display, Formatter};
use tracing::info;

pub enum InteractionStates {
    New,
    Enqueued,
    Dequeued,
    Active,
    Disconnected,
    Closed,
    Unknown,
}

impl Display for InteractionStates {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self {
            InteractionStates::New => write!(f, "({})", "New"),
            InteractionStates::Enqueued => write!(f, "({})", "Enqueued"),
            InteractionStates::Dequeued => write!(f, "({})", "Dequeued"),
            InteractionStates::Active => write!(f, "({})", "Active"),
            InteractionStates::Disconnected => write!(f, "({})", "Disconnected"),
            InteractionStates::Closed => write!(f, "({})", "Closed"),
            InteractionStates::Unknown => write!(f, "({})", "Unknown"),
        }
    }
}

//NEVER RETURN AN ERROR FROM HERE
impl TryFrom<i32> for InteractionStates {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(InteractionStates::New),
            1000 => Ok(InteractionStates::Enqueued),
            2000 => Ok(InteractionStates::Dequeued),
            3000 => Ok(InteractionStates::Active),
            4000 => Ok(InteractionStates::Disconnected),
            5000 => Ok(InteractionStates::Closed),
            _ => Ok(InteractionStates::Unknown),
        }
    }
}

pub fn validate_state_change(
    prev_state: InteractionStates,
    next_state: InteractionStates,
) -> Result<NoType, StateValidationError> {
    info!(
        "validating interaction state change from {} to {}",
        prev_state, next_state
    );
    match prev_state {
        InteractionStates::New => match next_state {
            InteractionStates::New => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Enqueued => {
                return Ok(NoType {});
            }
            InteractionStates::Dequeued => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Active => return Ok(NoType {}),
            InteractionStates::Disconnected => {}
            InteractionStates::Closed => {}
            InteractionStates::Unknown => {}
        },
        InteractionStates::Enqueued => {}
        InteractionStates::Dequeued => {}
        InteractionStates::Active => {}
        InteractionStates::Disconnected => {}
        InteractionStates::Closed => {}
        InteractionStates::Unknown => {
            return Err(StateValidationError::InteractionStateChange);
        }
    }
    Ok(NoType {})
}
