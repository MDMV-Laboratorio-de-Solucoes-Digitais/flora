//! In-process realtime event hub for single-node delivery (cycle 015-A).
//!
//! Postgres remains authoritative; this hub provides best-effort fanout to
//! connected WebSocket clients on the same node.

use serde::Serialize;
use tokio::sync::broadcast;
use uuid::Uuid;

/// Capacity of the broadcast channel. Slow/lagging receivers may miss events and
/// must recover via REST catch-up.
pub const HUB_CAPACITY: usize = 256;

/// Canonical realtime event kinds for persisted messages.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum RealtimeEvent {
    /// A new message was persisted.
    NewMessage {
        /// Channel where the message was created.
        channel_id: Uuid,
        /// Canonical message payload.
        message: MessageEventPayload,
    },
    /// An existing message was edited.
    MessageUpdated {
        /// Channel where the message was edited.
        channel_id: Uuid,
        /// Canonical message payload.
        message: MessageEventPayload,
    },
    /// A message was soft-deleted.
    MessageDeleted {
        /// Channel where the message was deleted.
        channel_id: Uuid,
        /// Canonical message payload.
        message: MessageEventPayload,
    },
}

/// Canonical message payload carried by realtime events (aligned with REST).
#[derive(Debug, Clone, Serialize, serde::Deserialize)]
pub struct MessageEventPayload {
    /// Message ID.
    pub id: String,
    /// Channel ID.
    pub channel_id: String,
    /// Sender user ID.
    pub sender_id: String,
    /// Sender public display name (or email fallback).
    #[serde(default)]
    pub sender_name: Option<String>,
    /// Sender avatar URL.
    #[serde(default)]
    pub sender_avatar_url: Option<String>,
    /// Message content.
    pub content: String,
    /// Thread/reply parent ID.
    pub thread_id: Option<String>,
    /// Whether edited.
    pub is_edited: bool,
    /// RFC3339 creation timestamp.
    pub created_at: String,
    /// RFC3339 update timestamp.
    pub updated_at: String,
}

/// Resolve the public display name for a message author.
///
/// Prefers trimmed `display_name`, falls back to trimmed `email`, and returns
/// `None` when both are blank.
#[must_use]
pub fn resolve_sender_name(display_name: &str, email: &str) -> Option<String> {
    let name = display_name.trim();
    if !name.is_empty() {
        return Some(name.to_string());
    }
    let mail = email.trim();
    if !mail.is_empty() {
        return Some(mail.to_string());
    }
    None
}

/// Envelope published through the in-process hub.
#[derive(Debug, Clone)]
pub struct HubEnvelope {
    /// Organization scope for tenant isolation.
    pub organization_id: Uuid,
    /// Channel scope.
    pub channel_id: Uuid,
    /// Event payload.
    pub event: RealtimeEvent,
}

impl HubEnvelope {
    /// Creates a new hub envelope.
    #[must_use]
    pub const fn new(organization_id: Uuid, channel_id: Uuid, event: RealtimeEvent) -> Self {
        Self {
            organization_id,
            channel_id,
            event,
        }
    }
}

/// Creates a new realtime hub sender.
#[must_use]
pub fn create_hub() -> broadcast::Sender<HubEnvelope> {
    let (tx, _rx) = broadcast::channel(HUB_CAPACITY);
    tx
}

/// Publishes an event to the hub (best-effort; no subscribers is not an error).
pub fn publish(tx: &broadcast::Sender<HubEnvelope>, envelope: HubEnvelope) {
    let _ = tx.send(envelope);
}

#[cfg(test)]
mod tests {
    use super::resolve_sender_name;

    #[test]
    fn resolve_sender_name_prefers_trimmed_display_name() {
        assert_eq!(
            resolve_sender_name("  Alice Admin  ", "alice@acme.test"),
            Some("Alice Admin".to_string())
        );
    }

    #[test]
    fn resolve_sender_name_falls_back_to_email_or_none() {
        assert_eq!(
            resolve_sender_name("   ", "  bob@acme.test  "),
            Some("bob@acme.test".to_string())
        );
        assert_eq!(resolve_sender_name("   ", "  "), None);
    }
}
