use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Queue {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub channels: Option<Vec<Channel>>,
    pub groups: Option<Vec<Group>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: Option<Uuid>,
    pub name: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize)]
pub struct Interaction {
    pub id: Option<Uuid>,
    pub state: Option<i32>,
    pub state_name: Option<String>,
    pub queue: Option<Queue>,
    pub created_at: Option<DateTime<Utc>>,
}

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
    pub message: String,
    #[serde(skip_serializing, skip_deserializing)]
    pub code: StatusCode,
    pub data: Option<A>,
}
