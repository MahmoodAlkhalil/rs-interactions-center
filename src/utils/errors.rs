use async_nats::ConnectError;
use axum::http::StatusCode;
use sea_orm::DbErr;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

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

pub trait WithMetadata<T> {
    fn with_metadata(self, id: Uuid) -> Result<T, IcError>;
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NoType {}
