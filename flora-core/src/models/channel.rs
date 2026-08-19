//! Channel model — a communication space within a workspace.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// A channel is a communication space within a workspace.
///
/// Channels can be public (visible to all org members) or private.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Channel {
    /// Unique identifier.
    pub id: Uuid,
    /// The workspace this channel belongs to.
    pub workspace_id: Uuid,
    /// Denormalized for fast tenant isolation checks.
    pub organization_id: Uuid,
    /// Display name of the channel.
    pub name: String,
    #[serde(rename = "type")]
    #[sqlx(rename = "type")]
    /// Type of channel (public or private).
    pub channel_type: ChannelType,
    /// When the channel was created.
    pub created_at: DateTime<Utc>,
    /// When the channel was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Channel {
    /// Creates a new channel.
    #[must_use]
    pub fn new(
        workspace_id: Uuid,
        organization_id: Uuid,
        name: String,
        channel_type: ChannelType,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            workspace_id,
            organization_id,
            name,
            channel_type,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Input for creating a channel.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateChannelInput {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters"))]
    /// Channel name.
    pub name: String,
    /// Type of channel (public or private).
    pub channel_type: ChannelType,
    /// Optional description.
    #[validate(length(max = 255))]
    pub description: Option<String>,
}

/// Input for updating a channel.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateChannelInput {
    #[validate(length(min = 1, max = 255))]
    /// New name for the channel.
    pub name: Option<String>,
    /// New description.
    pub description: Option<String>,
    /// New channel type.
    pub channel_type: Option<ChannelType>,
}

/// Type of channel (public or private).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum ChannelType {
    /// Visible to all members of the organization.
    Public,
    /// Only visible to explicitly invited members.
    Private,
}

impl std::fmt::Display for ChannelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Public => "Public",
            Self::Private => "Private",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for ChannelType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "public" => Ok(Self::Public),
            "private" => Ok(Self::Private),
            _ => Err(format!(
                "invalid channel type: {s} (expected 'Public' or 'Private')"
            )),
        }
    }
}
