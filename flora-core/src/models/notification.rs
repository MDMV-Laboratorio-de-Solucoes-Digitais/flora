//! Notification model — an alert for a relevant event.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

/// A notification sent to a user about an event.
///
/// Notifications are soft-deleted when marked as read (or via retention policy).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Notification {
    /// Unique identifier.
    pub id: Uuid,
    /// The organization this notification belongs to.
    pub organization_id: Uuid,
    /// The user who receives this notification.
    pub user_id: Uuid,
    /// Type of event that triggered the notification.
    pub event_type: NotificationType,
    /// ID of the related entity (message, task, file, etc.).
    pub target_id: Uuid,
    /// Optional deep link URL for the notification.
    pub target_url: Option<String>,
    /// Whether the notification has been viewed.
    pub is_read: bool,
    /// When the notification was created.
    pub created_at: DateTime<Utc>,
    /// When the notification was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Notification {
    /// Creates a new notification.
    #[must_use]
    pub fn new(
        organization_id: Uuid,
        user_id: Uuid,
        event_type: NotificationType,
        target_id: Uuid,
        target_url: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            organization_id,
            user_id,
            event_type,
            target_id,
            target_url,
            is_read: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Types of notification events.
///
/// These correspond to actions that can trigger notifications in Flora.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Type, Default)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum NotificationType {
    /// User was mentioned in a message.
    #[default]
    Mention,
    /// Task was assigned to the user.
    Assignment,
    /// Reply to the user's message.
    Reply,
    /// File was shared with the user.
    FileShare,
    /// Content the user follows was updated.
    ContentUpdate,
    /// A task assigned to the user was completed.
    TaskCompleted,
    /// User received an invitation.
    Invitation,
    /// User's role was changed.
    RoleChange,
    /// Activity in a workspace the user is a member of.
    WorkspaceActivity,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Mention => "Mention",
            Self::Assignment => "Assignment",
            Self::Reply => "Reply",
            Self::FileShare => "FileShare",
            Self::ContentUpdate => "ContentUpdate",
            Self::TaskCompleted => "TaskCompleted",
            Self::Invitation => "Invitation",
            Self::RoleChange => "RoleChange",
            Self::WorkspaceActivity => "WorkspaceActivity",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for NotificationType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Mention" => Ok(Self::Mention),
            "Assignment" => Ok(Self::Assignment),
            "Reply" => Ok(Self::Reply),
            "FileShare" => Ok(Self::FileShare),
            "ContentUpdate" => Ok(Self::ContentUpdate),
            "TaskCompleted" => Ok(Self::TaskCompleted),
            "Invitation" => Ok(Self::Invitation),
            "RoleChange" => Ok(Self::RoleChange),
            "WorkspaceActivity" => Ok(Self::WorkspaceActivity),
            _ => Err(format!("invalid notification type: {s}")),
        }
    }
}
