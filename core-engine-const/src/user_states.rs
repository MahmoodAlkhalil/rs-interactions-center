use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::fmt::Display;
use strum_macros::{Display, EnumProperty, EnumString, VariantArray};

#[derive(Clone, Copy, VariantArray, Debug, PartialEq, Eq, EnumString, Display, EnumProperty)]
#[repr(i32)]
#[allow(dead_code)]
pub enum UserStates {
    #[strum(props(Id = "0199f2aa-430d-73ea-92ef-1b87cd43ff0d"))]
    Offline,
    #[strum(props(Id = "0199f2aa-6e1c-7f1e-84f7-f7209a5f1187"))]
    NotReady,
    #[strum(props(Id = "0199f2aa-958a-7c39-ac8a-a00c7b026825"))]
    Ready,
    #[strum(props(Id = "0199f2aa-958a-78a6-b97b-0107d4a36c21"))]
    Presenting,
    #[strum(props(Id = "0199f2aa-958a-7db1-9707-d74157771619"))]
    Interacting,
    #[strum(props(Id = "0199f2aa-958a-70da-8fd8-1af360e018d2"))]
    FullyOccupied,
    #[strum(props(Id = "0199f2aa-958a-7579-bd35-c34a73be8e68"))]
    Unknown,
    #[strum(props(
        Id = "0199f2aa-958a-7221-9eb7-085fbe54c10b",
        Parent = "0199f2aa-6e1c-7f1e-84f7-f7209a5f1187"
    ))]
    TechnicalIssue,
}
