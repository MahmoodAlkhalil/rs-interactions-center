use async_nats::Client;
use core_engine_dto::{errors::IcError, nats};
use reqwest::StatusCode;
use std::env;
use tokio::{spawn, sync::OnceCell};

static NATS_CLIENT: OnceCell<Client> = OnceCell::const_new();
pub async fn init_nats_client() -> Result<(), IcError> {
    if NATS_CLIENT.initialized() {
        return Err(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "NATS Client is already initialized".to_owned(),
        });
    }
    let nats_url = env::var("NATS_URL").unwrap();
    let nats_seed = env::var("NATS_SEED").unwrap();
    let options: async_nats::ConnectOptions = async_nats::ConnectOptions::new().nkey(nats_seed);
    let nats_client = async_nats::connect_with_options(nats_url, options).await?;
    NATS_CLIENT.set(nats_client)?;
    Ok(())
}

pub async fn register_sink_on_topic(topic_name: String) -> Result<(), IcError> {
    if !NATS_CLIENT.initialized() {
        return Err(IcError {
            status_code: StatusCode::BAD_REQUEST,
            message: "NATS Client is NOT initialized".to_owned(),
        });
    }
    NATS_CLIENT.get().unwrap().subscribe(topic_name);
    Ok(())
}

pub async fn health_heartbeater(worker_id: String) {}

async fn health_heartbeater() {}
