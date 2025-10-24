use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumProperty, EnumString, VariantArray};
#[derive(
    Clone,
    Copy,
    VariantArray,
    Debug,
    PartialEq,
    Eq,
    EnumString,
    EnumProperty,
    Display,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum ChannelWorkerStates {
    New,
    Offline,
    Starting,
    Degraded,
    Online,
}
