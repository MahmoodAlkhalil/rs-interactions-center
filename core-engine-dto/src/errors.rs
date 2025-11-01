use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::error::Error;
use strum_macros::{Display, EnumProperty, EnumString};
use tracing::error;

#[derive(Serialize, Debug)]
pub struct ApiError {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
}

impl ApiError {
    pub fn not_found(message: &str) -> Self {
        ApiError {
            status_code: StatusCode::NOT_FOUND,
            message: message.to_owned(),
        }
    }
    pub fn internal_error(message: &str) -> Self {
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_owned(),
        }
    }
    pub fn bad_request(message: &str) -> Self {
        ApiError {
            status_code: StatusCode::BAD_REQUEST,
            message: message.to_owned(),
        }
    }
}

impl<T> From<T> for ApiError
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
        ApiError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Error occured, please check logs".to_string(),
        }
    }
}

impl Default for ApiError {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct NoType {}
