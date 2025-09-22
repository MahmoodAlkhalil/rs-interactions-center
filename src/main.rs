use crate::shared::{ApiSharedData, IcError};
use axum::routing::get;
use axum::Router;
use log::{error, info};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use std::time::Duration;
use thiserror::__private::AsDynError;

mod api;
mod db;
mod dtos;
mod services;
mod shared;

#[tokio::main]
async fn main() -> Result<(), IcError> {
    dotenv::dotenv().ok();
    env_logger::init();
    let db_pool = init_database_pool().await?;
    let api_shared_data = ApiSharedData { db_pool };
    let shared_state = Arc::new(api_shared_data);
    start_http_server(Arc::clone(&shared_state)).await?;
    Ok(())
}

async fn init_database_pool() -> Result<DatabaseConnection, IcError> {
    info!("initializing db pool");
    let mut opt = ConnectOptions::new(
        "postgres://rsic:Hala_1994@localhost:5432/rs-interactions-center?schemaName=core"
            .to_owned(),
    );
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    let pool = Database::connect(opt).await?;
    Ok(pool)
}

async fn start_http_server(api_shared_data: Arc<ApiSharedData>) -> Result<(), IcError> {
    let api_v1 = Router::new()
        .merge(api::queues::routes(Arc::clone(&api_shared_data)))
        .merge(api::channels::routes(Arc::clone(&api_shared_data)))
        .merge(api::skills::routes(Arc::clone(&api_shared_data)));
    let router = Router::new().nest("/api/v1", api_v1);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
