//! Flora Messaging - Real-time messaging with Valkey Pub/Sub
//! Uses Valkey Pub/Sub for real-time channel communication.
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive dependency overrides"
)]
use flora_core::{Error, Result};
use redis::aio::ConnectionManager;
use std::fmt;
use uuid::Uuid;

/// Messaging service for real-time communication via Valkey Pub/Sub.
pub struct MessagingService {
    redis: ConnectionManager,
}

impl fmt::Debug for MessagingService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MessagingService")
            .field("redis", &"<ConnectionManager>")
            .finish()
    }
}

impl MessagingService {
    /// Creates a new `MessagingService` with the given Valkey connection manager.
    #[must_use]
    pub const fn new(redis: ConnectionManager) -> Self {
        Self { redis }
    }

    /// Publishes a message to the channel's pub/Sub topic via Valkey.
    ///
    /// # Errors
    ///
    /// Returns an error if the message could not be serialized or published.
    pub async fn publish(&self, msg: Message) -> Result<()> {
        let payload = serde_json::to_string(&msg).map_err(|e| Error::Internal(e.to_string()))?;
        let channel = format!("org:{}:channel:{}", msg.organization_id, msg.channel_id);

        let mut conn = self.redis.clone();
        let _count: i32 = redis::cmd("PUBLISH")
            .arg(&channel)
            .arg(&payload)
            .query_async(&mut conn)
            .await
            .map_err(|e| Error::Messaging(e.to_string()))?;

        Ok(())
    }
}

/// A message published to a channel.
#[derive(Debug, Clone, serde::Serialize)]
pub struct Message {
    /// The channel this message was sent to.
    pub channel_id: Uuid,
    /// The organization that owns this channel.
    pub organization_id: Uuid,
    /// The user who sent this message.
    pub sender_id: Uuid,
    /// The text content of the message.
    pub content: String,
    /// Optional thread ID for threaded replies.
    pub thread_id: Option<Uuid>,
}
