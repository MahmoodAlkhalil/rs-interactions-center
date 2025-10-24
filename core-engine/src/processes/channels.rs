use async_nats::Subscriber;
use axum::http::StatusCode;
use core_engine_db::entities::channels::Entity as ChannelsE;
use core_engine_dto::{ChannelHeartbeatMessage, SharedState, errors::IcError};
use futures_util::StreamExt;
use sea_orm::EntityTrait;
use sea_orm::sqlx::postgres::PgListener;
use std::sync::Arc;
use tokio::spawn;
use tracing::{error, info, warn};

pub async fn health_monitor(state: Arc<SharedState>) -> Result<(), IcError> {
    let channels = ChannelsE::find().all(&state.db_pool).await?;
    for channel in channels {
        let stream = state
            .nats_client
            .subscribe(format!("{}.heartbeat", channel.name.trim()))
            .await?;
        spawn(async move {
            handle_channel_heartbeat_messages(stream).await;
        });
    }
    Ok(())
}

pub async fn handle_channel_heartbeat_messages(mut stream: Subscriber) {
    loop {
        let message = stream.next().await;
        let message = match message {
            Some(data) => data,
            None => return,
        };
        let heartbeat_message: serde_json::Result<ChannelHeartbeatMessage> =
            serde_json::from_slice(&message.payload);
        let heartbeat_message = match (heartbeat_message) {
            Ok(message) => message,
            Err(_) => {
                error!(
                    "received invalid message on Channel heartbeat topic [{}]",
                    message.subject
                );
                continue;
            }
        };
        
    }
}
