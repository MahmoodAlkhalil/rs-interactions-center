use core_engine_dto::errors::ApiError;
use tokio_util::sync::CancellationToken;
use tracing::{Level, warn};

use crate::{
    api::ApiClient,
    nats::create_client,
    worker::{Worker, start},
};

mod api;
mod nats;
mod worker;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    let env_file = if cfg!(debug_assertions) {
        ".env.core-voice"
    } else {
        ".env"
    };
    dotenv::from_filename(env_file).ok();
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .init();

    let api_client = ApiClient::new().await?;
    let nats_client = create_client().await?;
    let cancelation_token = CancellationToken::new();
    let channel = api_client.get_channel().await?;
    let worker = Worker::new(nats_client, api_client, cancelation_token.clone(), channel);

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            warn!("ctl+c!");
            cancelation_token.cancel();
        },
        _ = start(worker) => {
            warn!("worker loop exited");
        },
    }
    Ok(())
}
