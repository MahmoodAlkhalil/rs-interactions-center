pub mod axum;
pub mod common;

pub mod user_states;
pub mod validators;

use async_nats::Client;
use sea_orm::DatabaseConnection;

pub struct SharedState {
    pub db_pool: DatabaseConnection,
    pub nats_client: Client,
}

impl SharedState {
    pub fn new(db_pool: DatabaseConnection, nats_client: Client) -> Self {
        SharedState {
            db_pool,
            nats_client,
        }
    }
}
