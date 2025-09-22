use sea_orm::{ConnectionTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

pub struct ServiceDto<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub request_id: Uuid,
    pub data: Option<A>,
    pub db: &'a B,
}

impl<'a, A, B> ServiceDto<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub fn new(data: Option<A>, db: &'a B) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            data,
            db,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<A> {
    pub request_id: Uuid,
    pub message: String,
    pub code: i32,
    pub data: Option<A>,
}

impl<A> ApiResponse<A> {
    pub fn new_success(id: Uuid, data: Option<A>) -> Self {
        ApiResponse {
            request_id: id,
            message: "SUCCESS".to_string(),
            code: 0,
            data,
        }
    }
    pub fn new_error(id: Uuid, code: i32, message: &str, data: Option<A>) -> Self {
        ApiResponse {
            request_id: id,
            message: message.to_string(),
            code,
            data,
        }
    }
}
