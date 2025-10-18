use std::string::FromUtf8Error;

use async_nats::ConnectError;
use axum::http::StatusCode;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Serialize, Debug)]
pub struct IcError {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
}

#[derive(Debug)]
pub enum StateValidationError {
    InteractionStateChange,
    UserStateChange,
    QueueStateChange,
}

impl From<StateValidationError> for IcError {
    fn from(value: StateValidationError) -> Self {
        let mut error = IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "invalid state".to_string(),
        };
        match value {
            StateValidationError::InteractionStateChange => {
                error.message = "Interaction new state is not valid".to_string()
            }
            StateValidationError::UserStateChange => {
                error.message = "User new state is not valid".to_string()
            }
            StateValidationError::QueueStateChange => {
                error.message = "Queue new state is not valid".to_string()
            }
        }
        error
    }
}

impl From<DbErr> for IcError {
    fn from(value: DbErr) -> Self {
        error!("{}", value.to_string());
        IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Database Error".to_string(),
        }
    }
}

impl From<ConnectError> for IcError {
    fn from(value: ConnectError) -> Self {
        IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "NATS client connection error".to_owned(),
        }
    }
}

impl From<std::io::Error> for IcError {
    fn from(value: std::io::Error) -> Self {
        error!("std::io::Error {}", value);
        IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "io error occured, check logs".to_string(),
        }
    }
}

impl From<FromUtf8Error> for IcError {
    fn from(value: FromUtf8Error) -> Self {
        error!("error of type [FromUtf8Error], error {}", value);
        IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "parsing error, check logs".to_string(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NoType {}
