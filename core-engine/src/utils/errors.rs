use core::error;
use std::string::FromUtf8Error;

use async_nats::ConnectError;
use axum::http::StatusCode;
use core_engine_consts::interaction_states::InteractionStates;
use num_enum::TryFromPrimitiveError;
use sea_orm::{DbErr, SqlxError};
use serde::{Deserialize, Serialize};
use tracing::error;

#[derive(Debug)]
pub struct IcError {
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
                error.message = "interaction new state is not valid".to_string()
            }
            StateValidationError::UserStateChange => {
                error.message = "user new state is not valid".to_string()
            }
            StateValidationError::QueueStateChange => {
                error.message = "queue new state is not valid".to_string()
            }
        }
        error
    }
}

impl From<SqlxError> for IcError {
    fn from(value: SqlxError) -> Self {
        IcError {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "database driver error".to_string(),
        }
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

impl From<TryFromPrimitiveError<InteractionStates>> for IcError {
    fn from(_: TryFromPrimitiveError<InteractionStates>) -> Self {
        IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "Invalid interaction state value".to_string(),
        }
    }
}

impl From<ConnectError> for IcError {
    fn from(value: ConnectError) -> Self {
        IcError {
            status_code: Default::default(),
            message: "".to_string(),
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
