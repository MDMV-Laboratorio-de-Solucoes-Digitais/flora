//! Role model — customizable RBAC within an organization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A role defines a set of permissions within an organization.
///
/// Roles are scoped to an organization. The `permissions` field is a JSONB
/// array of permission strings (e.g., [`channel:read`, `task:write`]).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Role {
    pub id: Uuid,
    pub organization_id: Uuid,
    /// Role name (e.g., "Owner", "Admin", "Member").
    pub name: String,
    /// JSONB array of permission strings.
    pub permissions: serde_json::Value,
    /// Description for UI display.
    pub description: Option<String>,
    /// Whether this is a built-in role (cannot be deleted).
    pub is_builtin: bool,
    pub created_at: DateTime<Utc>,
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
    OrgRead,
    OrgWrite,
    OrgAdmin,
    // Workspace
    WorkspaceRead,
    WorkspaceWrite,
    WorkspaceAdmin,
    // Channels & Messaging
    ChannelRead,
    ChannelWrite,
    ChannelDelete,
    MessageRead,
    MessageWrite,
    MessageEdit,
    MessageDelete,
    // Tasks
    TaskRead,
    TaskWrite,
    TaskAssign,
    TaskDelete,
    // Files
    FileRead,
    FileWrite,
    FileDelete,
    // Search
    SearchGlobal,
    // Notifications
    NotificationRead,
    // RBAC
    RoleRead,
    RoleWrite,
    MemberRead,
    MemberInvite,
    MemberRemove,
}

impl std::fmt::Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Permission::OrgRead => "org:read",
            Permission::OrgWrite => "org:write",
            Permission::OrgAdmin => "org:admin",
            Permission::WorkspaceRead => "workspace:read",
            Permission::WorkspaceWrite => "workspace:write",
            Permission::WorkspaceAdmin => "workspace:admin",
            Permission::ChannelRead => "channel:read",
            Permission::ChannelWrite => "channel:write",
            Permission::ChannelDelete => "channel:delete",
            Permission::MessageRead => "message:read",
            Permission::MessageWrite => "message:write",
            Permission::MessageEdit => "message:edit",
            Permission::MessageDelete => "message:delete",
            Permission::TaskRead => "task:read",
            Permission::TaskWrite => "task:write",
            Permission::TaskAssign => "task:assign",
            Permission::TaskDelete => "task:delete",
            Permission::FileRead => "file:read",
            Permission::FileWrite => "file:write",
            Permission::FileDelete => "file:delete",
            Permission::SearchGlobal => "search:global",
            Permission::NotificationRead => "notification:read",
            Permission::RoleRead => "role:read",
            Permission::RoleWrite => "role:write",
            Permission::MemberRead => "member:read",
            Permission::MemberInvite => "member:invite",
            Permission::MemberRemove => "member:remove",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for Permission {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "org:read" => Ok(Permission::OrgRead),
            "org:write" => Ok(Permission::OrgWrite),
            "org:admin" => Ok(Permission::OrgAdmin),
            "workspace:read" => Ok(Permission::WorkspaceRead),
            "workspace:write" => Ok(Permission::WorkspaceWrite),
            "workspace:admin" => Ok(Permission::WorkspaceAdmin),
            "channel:read" => Ok(Permission::ChannelRead),
            "channel:write" => Ok(Permission::ChannelWrite),
            "channel:delete" => Ok(Permission::ChannelDelete),
            "message:read" => Ok(Permission::MessageRead),
            "message:write" => Ok(Permission::MessageWrite),
            "message:edit" => Ok(Permission::MessageEdit),
            "message:delete" => Ok(Permission::MessageDelete),
            "task:read" => Ok(Permission::TaskRead),
            "task:write" => Ok(Permission::TaskWrite),
            "task:assign" => Ok(Permission::TaskAssign),
            "task:delete" => Ok(Permission::TaskDelete),
            "file:read" => Ok(Permission::FileRead),
            "file:write" => Ok(Permission::FileWrite),
            "file:delete" => Ok(Permission::FileDelete),
            "search:global" => Ok(Permission::SearchGlobal),
            "notification:read" => Ok(Permission::NotificationRead),
            "role:read" => Ok(Permission::RoleRead),
            "role:write" => Ok(Permission::RoleWrite),
            "member:read" => Ok(Permission::MemberRead),
            "member:invite" => Ok(Permission::MemberInvite),
            "member:remove" => Ok(Permission::MemberRemove),
            _ => Err(format!("unknown permission: {s}")),
        }
    }
}
