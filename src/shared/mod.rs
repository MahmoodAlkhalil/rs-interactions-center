pub mod helpers;

use crate::dtos::shared::ApiResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::{DatabaseConnection, DbErr};
use uuid::Uuid;

#[derive(Debug)]
pub struct IcError {
    pub id: Uuid,
    pub message: String,
}

impl From<DbErr> for IcError {
    fn from(error: DbErr) -> Self {
        IcError {
            id: Default::default(),
            message: "database error, check logs".to_string(),
        }
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
pub struct NONE {}
