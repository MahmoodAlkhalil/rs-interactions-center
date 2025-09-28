use crate::shared::{IcError, SharedState, WithMetadata};
use axum::Router;
use env_logger::Builder;
use log::info;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

mod api;
mod db;
mod dtos;
mod services;
mod shared;

#[tokio::main]
async fn main() -> Result<(), IcError> {
    dotenv::dotenv().ok();
    Builder::new()
        .format_source_path(Some(std::env::current_dir().unwrap().as_path()).is_some())
        .init();
    let db_pool = init_database_pool().await?;
    let shared_state = Arc::new(SharedState { db_pool });
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
    let pool = Database::connect(opt).await.with_metadata(Uuid::new_v4())?;
    Ok(pool)
}

async fn start_http_server(shared_state: Arc<SharedState>) -> Result<(), IcError> {
    let api_v1 = Router::new()
        .merge(api::queues::routes(Arc::clone(&shared_state)))
        .merge(api::channels::routes(Arc::clone(&shared_state)))
        .merge(api::skills::routes(Arc::clone(&shared_state)));
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
