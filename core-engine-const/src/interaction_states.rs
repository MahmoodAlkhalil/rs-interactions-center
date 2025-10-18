use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use strum::{EnumProperty, VariantArray};
use strum_macros::{Display, EnumProperty, EnumString, VariantArray};
use uuid::Uuid;

pub fn interaction_state_to_uuid(variant: InteractionStates) -> Uuid {
    Uuid::from_str(variant.get_str("Id").unwrap()).unwrap()
}

pub fn uuid_to_interaction_state(val: &Uuid) -> InteractionStates {
    InteractionStates::from_str(val.to_string().as_str()).unwrap()
}

pub fn uuid_to_interaction_state_name(val: &Uuid) -> String {
    InteractionStates::from_str(val.to_string().as_str())
        .unwrap()
        .to_string()
}

#[derive(Clone, Copy, VariantArray, Debug, PartialEq, Eq, EnumString, EnumProperty, Display)]
pub enum InteractionStates {
    #[strum(
        props(Id = "0199f2ab-4d42-735c-b756-7cec5c297f8c"),
        serialize = "0199f2ab-4d42-735c-b756-7cec5c297f8c"
    )]
    New,
    #[strum(
        props(Id = "0199f2ab-4d42-721b-a554-21084f43fab6"),
        serialize = "0199f2ab-4d42-721b-a554-21084f43fab6"
    )]
    Enqueued,
    #[strum(
        props(Id = "0199f2ab-4d42-7293-a39a-dd143a1227f5"),
        serialize = "0199f2ab-4d42-7293-a39a-dd143a1227f5"
    )]
    Dequeued,
    #[strum(
        props(Id = "0199f2ab-4d42-7140-abb7-d7e168abccdc"),
        serialize = "0199f2ab-4d42-7140-abb7-d7e168abccdc"
    )]
    Presenting,
    #[strum(
        props(Id = "0199f2ab-4d42-77e6-a3ad-21c6ce6fcb38"),
        serialize = "0199f2ab-4d42-77e6-a3ad-21c6ce6fcb38"
    )]
    Active,
    #[strum(
        props(Id = "0199f2ab-4d42-75d9-90f2-153f28941843"),
        serialize = "0199f2ab-4d42-75d9-90f2-153f28941843"
    )]
    Disconnected,
    #[strum(
        props(Id = "0199f2ab-4d42-71e8-8dad-5c3cecc52f83"),
        serialize = "0199f2ab-4d42-71e8-8dad-5c3cecc52f83"
    )]
    WrappingUp,
    #[strum(
        props(Id = "0199f2ab-4d42-77e9-be55-b42a7ea13103"),
        serialize = "0199f2ab-4d42-77e9-be55-b42a7ea13103"
    )]
    Closed,
    #[strum(
        props(Id = "0199f2ab-4d42-7126-878f-51afbabeb4ea"),
        serialize = "0199f2ab-4d42-7126-878f-51afbabeb4ea"
    )]
    Unknown,
}
