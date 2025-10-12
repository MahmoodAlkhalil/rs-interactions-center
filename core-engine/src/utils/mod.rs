pub mod axum;
pub mod common;
pub mod errors;
pub mod user_states;
pub mod validators;

use crate::dtos::channels::shared::ChannelHealth;
use sea_orm::DatabaseConnection;
use std::collections::HashMap;
use async_nats::Client;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct SharedState {
    pub db_pool: DatabaseConnection,
    pub nats_client:Client,
    pub channel_health_tracker: RwLock<HashMap<Uuid, ChannelHealth>>,
}

impl SharedState {
    pub fn new(db_pool: DatabaseConnection,nats_client:Client) -> Self {
        SharedState {
            db_pool,
            nats_client,
            channel_health_tracker: RwLock::new(HashMap::with_capacity(10)),
        }
    }
}
