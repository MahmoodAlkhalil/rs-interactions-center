use strum_macros::{Display, EnumProperty, EnumString, VariantArray};
#[derive(
    Clone, Copy, VariantArray, Debug, PartialEq, Eq, EnumString, EnumProperty, Display, Hash,
)]
pub enum InteractionStates {
    #[strum()]
    New,
    #[strum()]
    Enqueued,
    #[strum()]
    Dequeued,
    #[strum()]
    Presenting,
    #[strum()]
    Active,
    #[strum()]
    Disconnected,
    #[strum(to_string = "Wrap Up")]
    WrapUp,
    #[strum()]
    Closed,
    #[strum()]
    Unknown,
}
