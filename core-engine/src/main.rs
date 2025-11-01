use crate::{
    services::nats,
    utils::axum::{RequestIdLayer, SharedState},
};
use async_nats::{Client, jetstream::new};
use axum::Router;
use axum_jwks::Jwks;
use core_engine_dto::errors::ApiError;
use futures_util::SinkExt;
use sea_orm::{ConnectOptions, Database, DatabaseConnection, sea_query::Oper};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tracing::Level;
mod api;
mod processes;
mod services;
mod utils;
use nats_io_jwt::{Account, KeyPair, Operator, Permission, SigningKeys, Token, User};

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    dotenv::dotenv().ok();
    //todo [make logging format an env variable]
    // tracing_subscriber::fmt()
    //     .json()
    //     .with_current_span(true)
    //     .with_span_list(false)
    //     .init();
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    let jwks = init_jwks().await?;
    let db_pool = init_database_pool().await?;

    init_local_caches(&db_pool).await?;
    nats::init_config(&db_pool).await?;
    let nats_client = nats::create_client(&db_pool).await?;
    let shared_state = SharedState::new(Arc::new(db_pool), Arc::new(nats_client), jwks);
    start_http_server(shared_state.clone()).await?;

    Ok(())
}

async fn init_database_pool() -> Result<DatabaseConnection, ApiError> {
    let mut opt = ConnectOptions::new(env::var("DATABASE_URL").unwrap().to_owned());
    opt.max_connections(50)
        .min_connections(5)
        .connect_lazy(false)
        .connect_timeout(Duration::from_secs(30))
        .acquire_timeout(Duration::from_secs(30))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false);
    let pool = Database::connect(opt).await?;
    Ok(pool)
}

async fn init_local_caches(db: &DatabaseConnection) -> Result<(), ApiError> {
    services::interactions::init_interaction_states_local_cache(db).await?;
    services::users::init_user_states_local_cache(db).await?;
    Ok(())
}

async fn start_http_server(shared_state: SharedState) -> Result<(), ApiError> {
    let api_v1 = Router::new()
        .merge(api::queues::routes(shared_state.clone()))
        .merge(api::channels::routes(shared_state.clone()))
        .merge(api::skills::routes(shared_state.clone()))
        .merge(api::interactions::routes(shared_state.clone()))
        .merge(api::users::routes(shared_state.clone()))
        .layer(RequestIdLayer);

    let router = Router::new().nest("/api/v1", api_v1);
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:8080").await {
        Ok(data) => data,
        Err(e) => {
            panic!("failed to bind tcp listener. {}", e);
        }
    };
    match axum::serve(listener, router).await {
        Ok(data) => data,
        Err(e) => {
            panic!("Failed to start http server. {}", e);
        }
    };
    Ok(())
}

async fn init_jwks() -> Result<Jwks, ApiError> {
    let jwks = Jwks::from_oidc_url(
        "http://localhost:8081/realms/rsic/.well-known/openid-configuration",
        None,
    )
    .await?;
    Ok(jwks)
}
