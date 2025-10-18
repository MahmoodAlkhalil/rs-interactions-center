use strum_macros::{Display, EnumProperty, EnumString, VariantArray};

#[derive(Clone, Copy, VariantArray, Debug, PartialEq, Eq, EnumString, Display, EnumProperty)]
#[allow(dead_code)]
pub enum UserStates {
    Offline,
    #[strum(to_string = "Not Ready")]
    NotReady,
    Ready,
    Presenting,
    Interacting,
    #[strum(to_string = "Fully Occupied")]
    FullyOccupied,
    Unknown,
}
