pub mod helpers;

use crate::dtos::shared::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use log::error;
use sea_orm::{DatabaseConnection, DbErr};
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

impl From<DbErrWithId> for IcError {
    fn from(err: DbErrWithId) -> Self {
        error!("[{}] database error [{}]", err.id, err.source.to_string());
        IcError {
            id: err.id,
            message: "database error".to_string(),
        }
    }
}
pub trait WithMetadata<T> {
    fn with_metadata(self, id: Uuid) -> Result<T, IcError>;
}

impl<T> WithMetadata<T> for Result<T, DbErr> {
    fn with_metadata(self, id: Uuid) -> Result<T, IcError> {
        self.map_err(|e| DbErrWithId { id, source: e }.into())
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

pub struct SharedState {
    pub db_pool: DatabaseConnection,
}

//Dummy Struct to be used as placeholder for Generic types with Option set to None
pub struct NoType {}
