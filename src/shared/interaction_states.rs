use crate::api::interactions::IcError;
use log::info;
use std::fmt::{Display, Formatter};

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
) -> Result<(), String> {
    info!(
        "validating interaction state change from {} to {}",
        prev_state, next_state
    );
    match prev_state {
        InteractionStates::New => match next_state {
            InteractionStates::New => {
                return Err(format!(
                    "Cannot change from {} to {}",
                    prev_state, next_state
                ));
            }
            InteractionStates::Enqueued => {
                return Ok(());
            }
            InteractionStates::Dequeued => {
                return Err(format!(
                    "Cannot change from {} to {}",
                    prev_state, next_state
                ));
            }
            InteractionStates::Active => return Ok(()),
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
            return Err("old state is Unknown, system hiccup..".to_owned());
        }
    }
    Ok(())
}
