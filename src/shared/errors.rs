use crate::dtos::shared::ApiResponse;
use async_nats::ConnectError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use log::error;
use sea_orm::DbErr;
use uuid::Uuid;

#[derive(Debug)]
pub struct IcError {
    pub id: Uuid,
    pub message: String,
}

#[derive(Debug)]
pub struct DbErrWithId {
    pub id: Uuid,
    pub source: DbErr,
}

impl From<&'static str> for IcError {
    fn from(s: &'static str) -> Self {
        IcError {
            id: Default::default(),
            message: "".to_string(),
        }
    }
}

impl From<(Uuid, &'static str)> for IcError {
    fn from(val: (Uuid, &'static str)) -> Self {
        IcError {
            id: val.0,
            message: val.1.to_string(),
        }
    }
}

impl From<ConnectError> for IcError {
    fn from(value: ConnectError) -> Self {
        IcError {
            id: Uuid::new_v4(),
            message: "NATS client connection error".to_owned(),
        }
    }
}

pub trait WithMetadata<T> {
    fn with_metadata(self, id: Uuid) -> Result<T, IcError>;
}

impl<T> WithMetadata<T> for Result<T, DbErr> {
    fn with_metadata(self, id: Uuid) -> Result<T, IcError> {
        let err = match self.as_ref().err() {
            None => {
                return self.map_err(|e| {
                    IcError {
                        id,
                        message: "Database Error, check logs".to_string(),
                    }
                    .into()
                });
            }
            Some(error) => {
                error!("Database Error: {:?}", error);
                error
            }
        };

        match err {
            DbErr::Custom(err) => Err(IcError {
                id,
                message: self.err().unwrap().to_string(),
            }),
            _ => self.map_err(|e| {
                IcError {
                    id,
                    message: "Database Error, check logs".to_string(),
                }
                .into()
            }),
        }
    }
}
impl<T> WithMetadata<T> for Result<T, String> {
    fn with_metadata(self, id: Uuid) -> Result<T, IcError> {
        self.map_err(|e| IcError { id, message: e }.into())
    }
}

impl<T> WithMetadata<T> for Result<T, &str> {
    fn with_metadata(self, id: Uuid) -> Result<T, IcError> {
        self.map_err(|e| {
            IcError {
                id,
                message: e.parse().unwrap(),
            }
            .into()
        })
    }
}

impl IntoResponse for IcError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::<String>::new_error(
                self.id,
                -1,
                &*format!("Error: {}", self.message),
                None,
            )),
        )
            .into_response()
    }
}

pub struct NoType {}
