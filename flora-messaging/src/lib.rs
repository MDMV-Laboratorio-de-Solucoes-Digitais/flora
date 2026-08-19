//! Flora Messaging - Real-time messaging with Valkey Pub/Sub
//! Uses Valkey Pub/Sub for real-time channel communication.

use flora_core::{Error, Result};
use redis::aio::ConnectionManager;
use uuid::Uuid;

/// Messaging service for real-time communication via Valkey Pub/Sub.
pub struct MessagingService {
    redis: ConnectionManager,
}

/// A message published to a channel.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Message {
    pub channel_id: Uuid,
    pub organization_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub thread_id: Option<Uuid>,
}

impl MessagingService {
    #[must_use]
    pub fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Publishes a message to the channel's pub/sub topic via Valkey.
    pub async fn publish(&self, msg: Message) -> Result<()> {
        let payload = serde_json::to_string(&msg).map_err(|e| Error::Internal(e.to_string()))?;
        let channel = format!("org:{}:channel:{}", msg.organization_id, msg.channel_id);

        let mut conn = self.redis.clone();
        let _subscribers: i32 = redis::cmd("PUBLISH")
            .arg(&channel)
            .arg(&payload)
            .query_async(&mut conn)
            .await
            .map_err(|e| Error::Messaging(e.to_string()))?;

        Ok(())
    }
}
