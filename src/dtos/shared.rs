use axum::http::StatusCode;
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct RequestDto<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub data: Option<A>,
    pub db: &'a B,
}

impl<'a, A, B> RequestDto<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub fn new(data: Option<A>, db: &'a B) -> Self {
        Self { data, db }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<A> {
    pub id: Uuid,
    pub message: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub code: StatusCode,
    pub data: Option<A>,
}
