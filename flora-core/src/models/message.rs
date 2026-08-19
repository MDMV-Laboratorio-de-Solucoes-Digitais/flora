//! Message model — a unit of communication in a channel.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// A message posted in a channel.
///
/// Messages support threading via self-referential `thread_id`.
/// Soft-deletion is supported via `is_deleted` flag.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Message {
    /// Unique identifier.
    pub id: Uuid,
    /// The channel this message belongs to.
    pub channel_id: Uuid,
    /// Denormalized for fast tenant isolation checks.
    pub organization_id: Uuid,
    /// The user who sent the message.
    pub sender_id: Uuid,
    /// Text content of the message.
    pub content: String,
    /// Optional self-referential FK for replies (threading).
    pub thread_id: Option<Uuid>,
    /// Whether the message has been edited.
    pub is_edited: bool,
    /// Soft-delete flag (true = deleted, false = active).
    pub is_deleted: bool,
    /// When the message was created.
    pub created_at: DateTime<Utc>,
    /// When the message was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Message {
    /// Creates a new message.
    #[must_use]
    pub fn new(channel_id: Uuid, organization_id: Uuid, sender_id: Uuid, content: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            channel_id,
            organization_id,
            sender_id,
            content: content.to_owned(),
            thread_id: None,
            is_edited: false,
            is_deleted: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Input for creating a message.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateMessageInput {
    #[validate(length(min = 1, max = 10000, message = "content must be 1-10000 characters"))]
    /// Text content of the message.
    pub content: String,
    /// Thread parent message ID (for replies).
    pub thread_id: Option<Uuid>,
}

/// Input for updating a message (content only).
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateMessageInput {
    #[validate(length(min = 1, max = 10000))]
    /// Updated text content.
    pub content: String,
}
