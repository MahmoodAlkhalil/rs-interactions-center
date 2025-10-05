pub mod errors;
pub mod interaction_states;
pub mod user_states;
pub mod axum;
pub mod common;

use async_nats::Client;
use sea_orm::DatabaseConnection;

pub struct SharedState {
    pub db_pool: DatabaseConnection,
    pub nats_client: Client,
}

