pub mod conversion;
pub mod errors;
pub mod nats;

use std::sync::Arc;

use axum::http::StatusCode;
use axum_jwks::Jwks;
use chrono::{DateTime, Utc};
use core_engine_const::channel_worker_states::ChannelWorkerStates;
use sea_orm::{ConnectionTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use async_nats::Client;
use sea_orm::DatabaseConnection;

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
    pub workers: Option<Vec<ChannelWorker>>,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ChannelWorker {
    pub id: Option<String>,
    pub state: Option<String>,
    pub channel: Option<Channel>,
    pub created_at: Option<DateTime<Utc>>,
    pub update_at: Option<DateTime<Utc>>,
    pub disable: Option<bool>,
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
    pub channel_worker: Option<Uuid>,
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
pub struct MessagingJwt {
    pub seed: String,
    pub jwt: String,
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

#[derive(Serialize, Deserialize)]
pub enum ChannelMessageType {
    New,
    Update,
    Delete,
    NotSpecified,
}

#[derive(Serialize, Deserialize)]
pub enum ChannelMessageTarget {
    Interaction,
    Queue,
    Channel,
    User,
    NotSpecified,
}

impl Default for ChannelMessageType {
    fn default() -> Self {
        ChannelMessageType::NotSpecified
    }
}

impl Default for ChannelMessageTarget {
    fn default() -> Self {
        ChannelMessageTarget::NotSpecified
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ChannelMessage {
    pub id: Uuid,
    pub r#type: ChannelMessageType,
    pub target: ChannelMessageTarget,
    pub interaction: Option<Interaction>,
    pub queue: Option<Queue>,
    pub channel: Option<Channel>,
    pub user: Option<User>,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelHeartbeatMessage {
    pub worker_id: String,
    pub state: ChannelWorkerStates,
    pub worker_timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct KeycloakTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_expires_in: u64,
    pub token_type: String,
    #[serde(rename = "not-before-policy")]
    pub not_before_policy: u64,
    pub scope: String,
    pub refresh_token: Option<String>,
}
