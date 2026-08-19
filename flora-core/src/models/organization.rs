//! Organization model — the top-level tenant.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// An organization is the top-level tenant in Flora.
///
/// Every resource belongs to exactly one organization, enforced by `organization_id`.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Organization {
    pub id: Uuid,
    pub name: String,
    /// URL-safe slug, unique across the system.
    pub slug: String,
    /// JSONB settings: `retention_days`, `file_upload_limit_mb`, etc.
    pub settings: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Organization {
    /// Creates a new organization with a generated UUID.
    #[must_use]
    pub fn new(name: &str, slug: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            name: name.to_owned(),
            slug: slug.to_lowercase(),
            settings: serde_json::json!({
                "file_upload_limit_mb": 100,
                "retention_days": 90,
                "org_quota_bytes": 10u64 * 1024 * 1024 * 1024,
                "workspace_quota_bytes": 2u64 * 1024 * 1024 * 1024,
            }),
            created_at: now,
            updated_at: now,
        }
    }

    /// Gets a setting value by key.
    #[must_use]
    pub fn setting<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
        serde_json::from_value(serde_json::json!(&self.settings[key])).ok()
    }
}

/// Input for creating an organization.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateOrganizationInput {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters"))]
    pub name: String,
    #[validate(length(min = 1, max = 63))]
    pub slug: String,
    /// Optional custom settings.
    pub settings: Option<serde_json::Value>,
}

/// Input for updating an organization.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateOrganizationInput {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub settings: Option<serde_json::Value>,
}
