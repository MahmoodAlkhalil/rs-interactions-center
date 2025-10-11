use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::{Display, Formatter};
use strum::VariantArray as _;
use strum_macros::{Display, EnumString, VariantArray};

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
)]
#[repr(i32)]
pub enum InteractionStates {
    New = 0,
    Enqueued = 1000,
    Dequeued = 2000,
    Presenting = 3000,
    Active = 4000,
    Disconnected = 5000,
    WrappingUp = 6000,
    Closed = 7000,
    Unknown = -1,
}
