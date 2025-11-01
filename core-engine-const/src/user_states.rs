use strum_macros::{Display, EnumProperty, EnumString, VariantArray};
#[derive(
    Clone, Copy, VariantArray, Debug, PartialEq, Eq, EnumString, EnumProperty, Display, Hash,
)]
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
