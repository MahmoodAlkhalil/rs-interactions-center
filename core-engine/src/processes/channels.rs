use crate::db::entities::channels::Entity as ChannelsE;
use crate::dtos::channels::shared::{ChannelHealth, ChannelHealthNotificationPayload};
use crate::utils::errors::IcError;
use crate::utils::SharedState;
use axum::http::StatusCode;
use sea_orm::sqlx::postgres::PgListener;
use sea_orm::EntityTrait;
use std::sync::Arc;
use tokio::spawn;
use tracing::{error, info, warn};

pub async fn health_monitor(state: Arc<SharedState>) -> Result<(), IcError> {
    let mut listener =
        PgListener::connect_with(&state.db_pool.get_postgres_connection_pool()).await?;
    let channels = ChannelsE::find().all(&state.db_pool).await?;
    for channel in channels {
        info!(
            "subscribing to [{}]",
            format!("channels.health.{}", channel.id)
        );
        listener
            .listen(format!("channels.health.{}", channel.id).as_str())
            .await?;
        let mut lock = state.as_ref().channel_health_tracker.write().await;
        lock.insert(
            channel.id,
            ChannelHealth {
                is_online: false,
                last_healthy_message: chrono::DateTime::UNIX_EPOCH.to_utc(),
            },
        );
    }
    loop {
        let notification = listener.recv().await;
        match notification {
            Ok(notification) => {
                let notification_content: serde_json::Result<ChannelHealthNotificationPayload> =
                    serde_json::from_str(notification.payload());
                match notification_content {
                    Ok(notification_content) => {
                        let cloned_state = Arc::clone(&state);
                        spawn(async move {
                            info!(
                                "received channel health monitoring notification on topic [{}]",
                                notification.channel()
                            );
                            let mut lock = cloned_state.channel_health_tracker.write().await;
                            let current_health = lock.get(&notification_content.id);
                            match current_health {
                                None => {
                                    warn!(
                                        "received a channel health notification from a channel [{}] that is not in the shared state on topic [{}]",
                                        notification_content.id,
                                        notification.channel()
                                    )
                                }
                                Some(data) => {
                                    info!(
                                        "updating channel health tracker for channel [{}]",
                                        notification_content.id
                                    );
                                }
                            }
                        });
                    }
                    Err(error) => {
                        error!(
                            "received invalid channel health monitoring notification on topic [{}] with the following content [{}]",
                            notification.channel(),
                            notification.payload()
                        );
                    }
                }
            }
            Err(err) => {
                error!(
                    "received an error when tried to receive a channel health monitoring notification"
                )
            }
        }
    }
    Ok(())
}
