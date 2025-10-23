use strum_macros::{Display, EnumProperty, EnumString, VariantArray};
#[derive(
    Clone, Copy, VariantArray, Debug, PartialEq, Eq, EnumString, EnumProperty, Display, Hash,
)]
pub enum ChannelWorkerStates {
    New,
    Offline,
    Starting,
    Degraded,
    Online,
}
