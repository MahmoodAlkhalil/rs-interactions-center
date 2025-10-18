pub mod conversion;
pub mod errors;
pub mod nats;

use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct EmptyResponse;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Queue {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub groups: Option<Vec<Group>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Channel {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Group {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Interaction {
    pub id: Option<Uuid>,
    pub state: Option<Uuid>,
    pub state_name: Option<String>,
    pub queue: Option<Queue>,
    pub priority: Option<i32>,
    pub channel: Option<Channel>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub username: Option<String>,
    pub groups: Option<Group>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Skill {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserState {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
    pub mark_for_delete: Option<bool>,
    pub level: Option<i32>,
    pub path: Option<Vec<Uuid>>,
    pub full_path: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct Request<'a, A, B>
where
    B: ConnectionTrait + TransactionTrait,
{
    pub data: Option<A>,
    pub db: &'a B,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub message: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub code: StatusCode,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn empty_success() -> Self {
        ApiResponse {
            message: String::from("SUCCESS"),
            code: StatusCode::OK,
            data: None,
        }
    }
}
