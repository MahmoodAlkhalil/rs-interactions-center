use crate::dtos::shared::ApiResponse;
use crate::utils::errors::IcError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub trait ToApiResponse<T> {
    fn to_api_response(self) -> ApiResponse<T>;
}

impl<T> ToApiResponse<T> for Result<T, IcError> {
    fn to_api_response(self) -> ApiResponse<T> {
        match self {
            Ok(data) => ApiResponse {
                message: "SUCCESS".to_string(),
                code: StatusCode::OK,
                data: Some(data),
            },
            Err(error) => ApiResponse {
                message: error.message,
                code: error.status_code,
                data: None,
            },
        }
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        (self.code, Json(self)).into_response()
    }
}
