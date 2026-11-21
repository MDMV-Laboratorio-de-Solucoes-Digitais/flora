//! User model.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// A user account in Flora.
///
/// Users are global — they can belong to multiple organizations via Membership.
/// Their identity is linked to an OIDC subject (sub claim).
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Primary key.
    pub id: Uuid,
    /// Unique email address.
    pub email: String,
    /// OIDC subject identifier (e. g., from Zitadel).
    pub oidc_subject: Option<String>,
    /// Argon2id PHC hash for local password login; `None` for OIDC-only users.
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    /// Last time the local password was set or rotated.
    #[serde(skip_serializing)]
    pub password_updated_at: Option<DateTime<Utc>>,
    /// Display name (from OIDC profile or manual).
    pub display_name: String,
    /// Avatar URL (from OIDC or uploaded).
    pub avatar_url: Option<String>,
    /// Optional metadata (JSONB) — timezone, preferences, etc.
    pub profile: serde_json::Value,
    /// Whether the account is active.
    pub is_active: bool,
    /// When the user account was created.
    pub created_at: DateTime<Utc>,
    /// When the user account was last updated.
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Creates a new user with generated UUID and timestamps.
    #[must_use]
    pub fn new(email: &str, display_name: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            email: email.to_lowercase(),
            oidc_subject: None,
            display_name: display_name.to_owned(),
            avatar_url: None,
            profile: serde_json::json!({}),
            is_active: true,
            created_at: now,
            updated_at: now,
            password_hash: None,
            password_updated_at: None,
        }
    }

    /// Creates a user from an OIDC claims payload.
    #[must_use]
    pub fn from_oidc(subject: &str, email: &str, name: &str, picture: Option<String>) -> Self {
        let mut user = Self::new(email, name);
        user.oidc_subject = Some(subject.to_string());
        user.avatar_url = picture;
        user
    }
}

/// Input for creating a new user.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateUserInput {
    #[validate(email(message = "invalid email address"))]
    /// User email address.
    pub email: String,
    #[validate(length(min = 1, max = 255, message = "display name must be 1-255 characters"))]
    /// Display name.
    pub display_name: String,
}

/// Input for updating a user profile.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateUserInput {
    #[validate(length(min = 1, max = 255))]
    /// Updated display name.
    pub display_name: Option<String>,
    /// Updated avatar URL.
    pub avatar_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::User;

    #[test]
    fn password_fields_are_not_serialized() {
        let user = User::new("admin@example.test", "Admin");
        let value = serde_json::to_value(user).unwrap_or_default();
        assert!(value.get("password_hash").is_none());
        assert!(value.get("password_updated_at").is_none());
    }
}
