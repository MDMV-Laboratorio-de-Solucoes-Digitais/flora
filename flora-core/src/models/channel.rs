//! Channel model — represents a chat channel within a workspace.
//!
//! Every channel belongs to exactly one workspace and organization.
//! Channels can be public (open to all members) or private (invitation-only).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

/// The type of channel — public or private.
#[derive(Debug, Clone, Default, Serialize, Deserialize, sqlx::Type)]
#[sqlx(rename_all = "snake_case")]
pub enum ChannelType {
    /// Public channel, visible to all workspace members.
    #[default]
    Public,
    /// Private channel, invitation-only.
    Private,
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Public => write!(f, "public"),
            Self::Private => write!(f, "private"),
        }
    }
}

/// A channel in Flora — the top-level scope for real-time messaging.
///
/// Every channel belongs to exactly one workspace and organization.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Channel {
    /// Unique identifier.
    pub id: uuid::Uuid,
    /// ID of the workspace this channel belongs to.
    pub workspace_id: uuid::Uuid,
    /// ID of the organization (denormalized for tenant isolation checks).
    pub organization_id: uuid::Uuid,
    /// Display name of the channel.
    pub name: String,
    /// Channel type (public or private).
    #[sqlx(rename = "type")]
    pub channel_type: ChannelType,
    /// When the channel was created.
    pub created_at: DateTime<Utc>,
    /// When the channel was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Channel {
    /// Creates a new channel with a generated UUID.
    #[must_use]
    pub fn new(
        workspace_id: uuid::Uuid,
        organization_id: uuid::Uuid,
        name: &str,
        channel_type: ChannelType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: uuid::Uuid::now_v7(),
            workspace_id,
            organization_id,
            name: name.to_owned(),
            channel_type,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Input for creating a new channel.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateChannelInput {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters"))]
    /// Channel display name.
    pub name: String,
    /// Channel type.
    pub channel_type: ChannelType,
}

/// Input for updating a channel.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateChannelInput {
    #[validate(length(min = 1, max = 255))]
    /// Updated channel name.
    pub name: Option<String>,
    /// Updated channel type.
    pub channel_type: Option<ChannelType>,
}
