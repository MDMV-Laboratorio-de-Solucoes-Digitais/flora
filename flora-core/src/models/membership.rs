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
    pub user_id: Uuid,
    pub organization_id: Uuid,
    pub role_id: Uuid,
    pub joined_at: DateTime<Utc>,
    /// Optional metadata (e.g., invitation source).
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
    pub const OWNER: &str = "Owner";
    pub const ADMIN: &str = "Admin";
    pub const MEMBER: &str = "Member";
    pub const GUEST: &str = "Guest";
}
