use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;
use strum_macros::{Display, EnumProperty, EnumString, VariantArray};

#[derive(
    Clone,
    Copy,
    VariantArray,
    Debug,
    PartialEq,
    Eq,
    EnumString,
    Display,
    IntoPrimitive,
    TryFromPrimitive,
    EnumProperty,
)]
#[repr(i32)]
#[allow(dead_code)]
pub enum UserStates {
    Offline = 0,
    NotReady = 1000,
    #[strum(props(Parent = 1000))]
    TechnicalIssues = 1001,
    Ready = 2000,
    Presenting = 3000,
    Interacting = 4000,
    FullyOccupied = 5000,
    Unknown = -1,
}
