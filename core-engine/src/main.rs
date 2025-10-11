use crate::utils::axum::RequestIdLayer;
use crate::utils::errors::IcError;
use crate::utils::SharedState;
use async_nats::Client;
use axum::Router;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, Level};
use uuid::Uuid;
use CoreEngineDbMigration::Migrator;
mod api;
mod db;
mod dtos;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), IcError> {
    dotenv::dotenv().ok();
    //todo [make logging format an env variable]
    // tracing_subscriber::fmt()
    //     .json()
    //     .with_current_span(true)
    //     .with_span_list(false)
    //     .init();
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();
    let db_pool = init_database_pool().await?;
    let nats_client = init_nats_client().await?;
    let shared_state = Arc::new(SharedState {
        db_pool,
        nats_client,
    });
    Migrator::up(&shared_state.db_pool, None).await?;
    start_http_server(Arc::clone(&shared_state)).await?;
    Ok(())
}

async fn init_database_pool() -> Result<DatabaseConnection, IcError> {
    info!("initializing db pool");
    let mut opt = ConnectOptions::new(env::var("DATABASE_URL").unwrap().to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    let pool = Database::connect(opt).await?;
    Ok(pool)
}

async fn init_nats_client() -> Result<Client, IcError> {
    let nats_url = env::var("NATS_URL").unwrap();
    let client = async_nats::connect(nats_url).await?;
    Ok(client)
}

async fn start_http_server(shared_state: Arc<SharedState>) -> Result<(), IcError> {
    let api_v1 = Router::new()
        .merge(api::queues::routes(Arc::clone(&shared_state)))
        .merge(api::channels::routes(Arc::clone(&shared_state)))
        .merge(api::skills::routes(Arc::clone(&shared_state)))
        .merge(api::interactions::routes(Arc::clone(&shared_state)))
        .merge(api::users::routes(Arc::clone(&shared_state)))
        .layer(RequestIdLayer);

    let router = Router::new().nest("/api/v1", api_v1);
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
        Ok(data) => data,
        Err(_) => {
            panic!("failed to bind tcp listener");
        }
    };
    match axum::serve(listener, router).await {
        Ok(data) => data,
        Err(e) => {
            panic!("Failed to start http server");
        }
    };
    Ok(())
}
