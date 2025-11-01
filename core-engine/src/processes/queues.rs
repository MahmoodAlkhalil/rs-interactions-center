use core_engine_db::entities::channels::Model as ChannelsM;
use std::{sync::Arc, time::Duration};
use tokio::time::timeout;

use crate::utils::axum::SharedState;

pub async fn notify_channel_enqueued_interaction(
    shared_state: SharedState,
    channel: ChannelsM,
    payload: String,
) {
    let result = timeout(
        Duration::from_secs(10),
        shared_state
            .nats_client
            .request(format!("channel.{}", channel.name), payload.into()),
    )
    .await;
}
