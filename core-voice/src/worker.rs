use std::sync::Arc;

use crate::api::ApiClient;
use async_nats::Client as NatsClient;
use bytes::Bytes;
use core_engine_const::channel_worker_states::ChannelWorkerStates;
use core_engine_dto::{Channel, errors::ApiError};
use futures_util::StreamExt;
use tokio::sync::{OnceCell, RwLock};
use tokio_util::sync::CancellationToken;
use tracing::warn;
use tracing_subscriber::fmt::format;

pub struct Worker {
    state: RwLock<ChannelWorkerStates>,
    nats_client: NatsClient,
    api_client: ApiClient,
    cancelation_token: CancellationToken,
    channel: Channel,
}

impl Worker {
    pub fn new(
        nats_client: NatsClient,
        api_client: ApiClient,
        cancelation_token: CancellationToken,
        channel: Channel,
    ) -> Self {
        Worker {
            state: RwLock::new(ChannelWorkerStates::Offline),
            nats_client,
            api_client,
            cancelation_token,
            channel,
        }
    }

    pub async fn get_state(self: &Self) -> ChannelWorkerStates {
        self.state.read().await.clone()
    }
}

pub async fn start(worker: Worker) {
    let cancelation_token = worker.cancelation_token.clone();
    tokio::select! {
        _ = cancelation_token.cancelled() => {
           warn!("cancelation token for the worker was triggered");
        }
        _ = background_run(worker) => {
            warn!("worker loop exited!");
        }
    }
}

async fn background_run(worker: Worker) {
    {
        let mut state = worker.state.write().await;
        *state = ChannelWorkerStates::Starting;
    }

    let events_topic = worker
        .nats_client
        .subscribe(format!(
            "channel.{}.events",
            worker.channel.name.clone().unwrap()
        ))
        .await
        .unwrap();
}
