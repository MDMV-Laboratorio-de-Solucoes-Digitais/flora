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
    pub id: Uuid,
    pub organization_id: Uuid,
    pub user_id: Uuid,
    pub event_type: NotificationType,
    pub target_id: Uuid,
    /// Optional deep link URL for the notification.
    pub target_url: Option<String>,
    /// Whether the notification has been viewed.
    pub is_read: bool,
    pub created_at: DateTime<Utc>,
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum NotificationType {
    Mention,
    Assignment,
    Reply,
    FileShare,
    ContentUpdate,
    TaskCompleted,
    Invitation,
    RoleChange,
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

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Mention" => Ok(NotificationType::Mention),
            "Assignment" => Ok(NotificationType::Assignment),
            "Reply" => Ok(NotificationType::Reply),
            "FileShare" => Ok(NotificationType::FileShare),
            "ContentUpdate" => Ok(NotificationType::ContentUpdate),
            "TaskCompleted" => Ok(NotificationType::TaskCompleted),
            "Invitation" => Ok(NotificationType::Invitation),
            "RoleChange" => Ok(NotificationType::RoleChange),
            "WorkspaceActivity" => Ok(NotificationType::WorkspaceActivity),
            _ => Err(format!("invalid notification type: {s}")),
        }
    }
}
