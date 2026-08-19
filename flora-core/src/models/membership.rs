//! Membership model — the join table between users and organizations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Maps a user to an organization with a specific role.
///
/// This is the "User ↔ Organization" join with per-organization role assignment.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Membership {
    /// The user ID.
    pub user_id: Uuid,
    /// The organization ID.
    pub organization_id: Uuid,
    /// The role ID within this organization.
    pub role_id: Uuid,
    /// Timestamp when the membership was created.
    pub joined_at: DateTime<Utc>,
    /// Optional metadata (e. g., invitation source).
    pub metadata: serde_json::Value,
}

impl Membership {
    /// Creates a new membership.
    #[must_use]
    pub fn new(user_id: Uuid, organization_id: Uuid, role_id: Uuid) -> Self {
        Self {
            user_id,
            organization_id,
            role_id,
            joined_at: Utc::now(),
            metadata: serde_json::json!({}),
        }
    }
}

/// Role name constants for built-in roles.
pub mod role_names {
    /// Owner role name.
    pub const OWNER: &str = "Owner";
    /// Admin role name.
    pub const ADMIN: &str = "Admin";
    /// Member role name.
    pub const MEMBER: &str = "Member";
    /// Guest role name.
    pub const GUEST: &str = "Guest";
}
