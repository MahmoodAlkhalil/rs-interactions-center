use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::DatabaseConnection;
use serde_json::Map;

#[derive(thiserror::Error, Debug)]
pub enum IcError {
    #[error(transparent)]
    DbError(#[from] sea_orm::DbErr),
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
}

impl IntoResponse for IcError {
    fn into_response(self) -> Response {
        let message = match self {
            IcError::DbError(e) => match e.sql_err() {
                Some(e) => &*e.to_string(),
                _ => &*e.to_string(),
            },
            IcError::StdIoError(_) => "io error",
            IcError::AxumError(_) => "axum error",
        };
        let mut body = Map::new();
        body.insert(String::from("message"), message.into());
        (StatusCode::INTERNAL_SERVER_ERROR, Json(body)).into_response()
    }
}

pub struct ApiSharedData {
    pub db_pool: DatabaseConnection,
}
