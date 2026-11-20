//! Flora Messaging - Real-time messaging with Valkey Pub/Sub
//! Uses Valkey Pub/Sub for real-time channel communication.
//!
//! ## Performance Targets
//!
//! - WebSocket delivery: < 500ms (p95)
//! - Fan-out per message: < 500ms (p95)
//!
//! When targets are exceeded, warning-level tracing events are emitted.
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive dependency overrides"
)]
use flora_core::{Error, Result};
use redis::aio::ConnectionManager;
use std::fmt;
use uuid::Uuid;

/// Target latency for WebSocket message delivery in milliseconds (p95).
pub const WEBSOCKET_LATENCY_TARGET_MS: u64 = 500;

/// Target latency for fan-out per message in milliseconds (p95).
pub const FANOUT_LATENCY_TARGET_MS: u64 = 500;

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

    /// Publishes a message to the channel's Pub/Sub topic via Valkey.
    ///
    /// Measures publish latency and emits a warning if the operation
    /// exceeds [`FANOUT_LATENCY_TARGET_MS`].
    ///
    /// # Errors
    ///
    /// Returns an error if the message could not be serialized or published.
    pub async fn publish(&self, msg: Message) -> Result<()> {
        let start = tokio::time::Instant::now();

        let payload = serde_json::to_string(&msg).map_err(|e| Error::Internal(e.to_string()))?;
        let channel = format!("org:{}:channel:{}", msg.organization_id, msg.channel_id);

        let mut conn = self.redis.clone();
        let _count: i32 = redis::cmd("PUBLISH")
            .arg(&channel)
            .arg(&payload)
            .query_async(&mut conn)
            .await
            .map_err(|e| Error::Messaging(e.to_string()))?;

        let elapsed = start.elapsed();
        let elapsed_ms = elapsed.as_millis();

        tracing::debug!(
            channel_id = %msg.channel_id,
            org_id = %msg.organization_id,
            sender_id = %msg.sender_id,
            elapsed_ms = %elapsed_ms,
            "Message published to Valkey"
        );

        if elapsed_ms > u128::from(FANOUT_LATENCY_TARGET_MS) {
            tracing::warn!(
                channel_id = %msg.channel_id,
                org_id = %msg.organization_id,
                elapsed_ms = %elapsed_ms,
                target_ms = FANOUT_LATENCY_TARGET_MS,
                "Fan-out latency exceeded target"
            );
        }

        if elapsed_ms > u128::from(WEBSOCKET_LATENCY_TARGET_MS) {
            tracing::warn!(
                channel_id = %msg.channel_id,
                org_id = %msg.organization_id,
                elapsed_ms = %elapsed_ms,
                target_ms = WEBSOCKET_LATENCY_TARGET_MS,
                "WebSocket delivery latency exceeded target"
            );
        }

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
