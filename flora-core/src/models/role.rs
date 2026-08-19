//! Role model — customizable RBAC within an organization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A role defines a set of permissions within an organization.
///
/// Roles are scoped to an organization. The `permissions` field is a JSONB
/// array of permission strings (e. g., [`channel:read`, `task:write`]).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    /// Unique identifier.
    pub id: Uuid,
    /// The organization this role belongs to.
    pub organization_id: Uuid,
    /// Role name (e. g., "Owner", "Admin", "Member").
    pub name: String,
    /// JSONB array of permission strings.
    pub permissions: serde_json::Value,
    /// Description for UI display.
    pub description: Option<String>,
    /// Whether this is a built-in role (cannot be deleted).
    pub is_builtin: bool,
    /// When the role was created.
    pub created_at: DateTime<Utc>,
    /// When the role was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Role {
    /// Creates a new role.
    #[must_use]
    pub fn new(organization_id: Uuid, name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            organization_id,
            name: name.to_owned(),
            permissions: serde_json::json!([]),
            description: None,
            is_builtin: false,
            created_at: now,
            updated_at: now,
        }
    }

    /// Adds a permission to the role.
    pub fn add_permission(&mut self, permission: Permission) {
        let perm_str = permission.to_string();
        if let Some(arr) = self.permissions.as_array_mut()
            && !arr.iter().any(|v| v.as_str() == Some(&perm_str))
        {
            arr.push(serde_json::json!(perm_str));
        }
        self.updated_at = Utc::now();
    }

    /// Checks if this role has a specific permission.
    #[must_use]
    pub fn has_permission(&self, permission: Permission) -> bool {
        self.permissions.as_array().is_some_and(|arr| {
            arr.iter()
                .any(|v| v.as_str() == Some("*") || v.as_str() == Some(&permission.to_string()))
        })
    }
}

/// Well-known permissions in Flora.
///
/// These are the granular permissions that can be assigned to roles.
/// Wildcard (`*`) means all permissions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    // Organization
    /// Read organization-level data.
    OrgRead,
    /// Write organization-level data.
    OrgWrite,
    /// Full organization administration.
    OrgAdmin,
    // Workspace
    /// Read workspace data.
    WorkspaceRead,
    /// Write workspace data.
    WorkspaceWrite,
    /// Full workspace administration.
    WorkspaceAdmin,
    // Channels & Messaging
    /// Read messages in channels.
    ChannelRead,
    /// Post messages to channels.
    ChannelWrite,
    /// Delete messages or channels.
    ChannelDelete,
    /// Read messages.
    MessageRead,
    /// Write messages.
    MessageWrite,
    /// Edit messages.
    MessageEdit,
    /// Delete messages.
    MessageDelete,
    // Tasks
    /// Read tasks.
    TaskRead,
    /// Write tasks.
    TaskWrite,
    /// Assign tasks to users.
    TaskAssign,
    /// Delete tasks.
    TaskDelete,
    // Files
    /// Read files.
    FileRead,
    /// Upload and modify files.
    FileWrite,
    /// Delete files.
    FileDelete,
    // Search
    /// Perform global searches.
    SearchGlobal,
    // Notifications
    /// Read notifications.
    NotificationRead,
    // RBAC
    /// Read roles.
    RoleRead,
    /// Create and modify roles.
    RoleWrite,
    /// Read memberships.
    MemberRead,
    /// Invite members.
    MemberInvite,
    /// Remove members.
    MemberRemove,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::OrgRead => "org:read",
            Self::OrgWrite => "org:write",
            Self::OrgAdmin => "org:admin",
            Self::WorkspaceRead => "workspace:read",
            Self::WorkspaceWrite => "workspace:write",
            Self::WorkspaceAdmin => "workspace:admin",
            Self::ChannelRead => "channel:read",
            Self::ChannelWrite => "channel:write",
            Self::ChannelDelete => "channel:delete",
            Self::MessageRead => "message:read",
            Self::MessageWrite => "message:write",
            Self::MessageEdit => "message:edit",
            Self::MessageDelete => "message:delete",
            Self::TaskRead => "task:read",
            Self::TaskWrite => "task:write",
            Self::TaskAssign => "task:assign",
            Self::TaskDelete => "task:delete",
            Self::FileRead => "file:read",
            Self::FileWrite => "file:write",
            Self::FileDelete => "file:delete",
            Self::SearchGlobal => "search:global",
            Self::NotificationRead => "notification:read",
            Self::RoleRead => "role:read",
            Self::RoleWrite => "role:write",
            Self::MemberRead => "member:read",
            Self::MemberInvite => "member:invite",
            Self::MemberRemove => "member:remove",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "org:read" => Ok(Self::OrgRead),
            "org:write" => Ok(Self::OrgWrite),
            "org:admin" => Ok(Self::OrgAdmin),
            "workspace:read" => Ok(Self::WorkspaceRead),
            "workspace:write" => Ok(Self::WorkspaceWrite),
            "workspace:admin" => Ok(Self::WorkspaceAdmin),
            "channel:read" => Ok(Self::ChannelRead),
            "channel:write" => Ok(Self::ChannelWrite),
            "channel:delete" => Ok(Self::ChannelDelete),
            "message:read" => Ok(Self::MessageRead),
            "message:write" => Ok(Self::MessageWrite),
            "message:edit" => Ok(Self::MessageEdit),
            "message:delete" => Ok(Self::MessageDelete),
            "task:read" => Ok(Self::TaskRead),
            "task:write" => Ok(Self::TaskWrite),
            "task:assign" => Ok(Self::TaskAssign),
            "task:delete" => Ok(Self::TaskDelete),
            "file:read" => Ok(Self::FileRead),
            "file:write" => Ok(Self::FileWrite),
            "file:delete" => Ok(Self::FileDelete),
            "search:global" => Ok(Self::SearchGlobal),
            "notification:read" => Ok(Self::NotificationRead),
            "role:read" => Ok(Self::RoleRead),
            "role:write" => Ok(Self::RoleWrite),
            "member:read" => Ok(Self::MemberRead),
            "member:invite" => Ok(Self::MemberInvite),
            "member:remove" => Ok(Self::MemberRemove),
            _ => Err(format!("unknown permission: {s}")),
        }
    }
}
