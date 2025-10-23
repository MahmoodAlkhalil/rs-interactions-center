use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use strum_macros::{Display, EnumProperty, EnumString};
use tracing::error;

#[derive(Serialize, Debug)]
pub struct IcError {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
}

impl IcError {
    pub fn not_found(message: &str) -> Self {
        IcError {
            status_code: StatusCode::NOT_FOUND,
            message: message.to_owned(),
        }
    }
}

impl Default for IcError {
    fn default() -> Self {
        Self {
            status_code: Default::default(),
            message: String::from("Success"),
        }
    }
}

#[derive(Debug, EnumString, EnumProperty, Display)]
pub enum StateValidationError {
    InteractionStateChange,
    UserStateChange,
    QueueStateChange,
}

impl Error for StateValidationError {}

impl<T> From<T> for IcError
where
    T: std::error::Error,
{
    fn from(error: T) -> Self {
        match error.source() {
            Some(err) => {
                error!("error, source  [{:?}], content [{}]", err, err);
            }
            None => error!("error, content [{}]", error),
        }
        IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Error occured, please check logs".to_string(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NoType {}
