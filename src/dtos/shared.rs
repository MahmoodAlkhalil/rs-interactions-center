use crate::utils::errors::IcError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde::{Deserialize, Serialize};

pub struct RequestDto<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub request: Option<A>,
    pub db: &'a B,
}

impl<'a, A, B> RequestDto<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub fn new(data: Option<A>, db: &'a B) -> Self {
        Self { request: data, db }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<A> {
    pub message: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub code: StatusCode,
    pub data: Option<A>,
}

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
