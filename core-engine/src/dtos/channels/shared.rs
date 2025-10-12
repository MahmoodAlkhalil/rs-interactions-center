use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct ChannelHealthNotificationPayload {
    pub id: Uuid,
    pub remote_timestamp: chrono::DateTime<chrono::Utc>,
}

pub struct ChannelHealth {
    pub is_online: bool,
    pub last_healthy_message: chrono::DateTime<chrono::Utc>,
}
