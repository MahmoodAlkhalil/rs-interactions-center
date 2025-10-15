use core_engine_consts::interaction_states::InteractionStates;
use core_engine_dto::errors::StateValidationError;

pub fn validate_interaction_state_change(
    old_state: InteractionStates,
    new_state: InteractionStates,
) -> Result<(), StateValidationError> {
    match old_state {
        InteractionStates::New => match new_state {
            InteractionStates::New => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Enqueued => {
                return Ok(());
            }
            InteractionStates::Dequeued => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Active => return Ok(()),
            InteractionStates::Disconnected => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Closed => return Err(StateValidationError::InteractionStateChange),
            InteractionStates::Unknown => return Err(StateValidationError::InteractionStateChange),
            InteractionStates::WrappingUp => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Presenting => todo!(),
        },
        InteractionStates::Enqueued => match new_state {
            InteractionStates::New => return Err(StateValidationError::InteractionStateChange),
            InteractionStates::Enqueued => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Dequeued => return Ok(()),
            InteractionStates::Presenting => return Ok(()),
            InteractionStates::Active => return Ok(()),
            InteractionStates::Disconnected => return Ok(()),
            InteractionStates::WrappingUp => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Closed => return Ok(()),
            InteractionStates::Unknown => return Err(StateValidationError::InteractionStateChange),
        },
        InteractionStates::Dequeued => match new_state {
            InteractionStates::New => return Err(StateValidationError::InteractionStateChange),
            InteractionStates::Enqueued => return Ok(()),
            InteractionStates::Dequeued => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Presenting => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Active => return Err(StateValidationError::InteractionStateChange),
            InteractionStates::Disconnected => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::WrappingUp => {
                return Err(StateValidationError::InteractionStateChange);
            }
            InteractionStates::Closed => return Err(StateValidationError::InteractionStateChange),
            InteractionStates::Unknown => return Err(StateValidationError::InteractionStateChange),
        },
        InteractionStates::Active => todo!(),
        InteractionStates::Disconnected => todo!(),
        InteractionStates::Closed => todo!(),
        InteractionStates::Unknown => todo!(),
        InteractionStates::WrappingUp => todo!(),
        InteractionStates::Presenting => todo!(),
    }
    Ok(())
}
