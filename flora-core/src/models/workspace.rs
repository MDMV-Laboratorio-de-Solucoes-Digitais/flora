//! Workspace model — a collaborative area within an organization.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// A workspace is a collaborative area within an organization.
///
/// Each workspace maintains isolated channels, tasks, and files.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workspace {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Workspace {
    /// Creates a new workspace.
    #[must_use]
    pub fn new(organization_id: Uuid, name: &str, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            organization_id,
            name: name.to_owned(),
            description,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Input for creating a workspace.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateWorkspaceInput {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters"))]
    pub name: String,
    #[validate(length(max = 1000))]
    pub description: Option<String>,
}

/// Input for updating a workspace.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateWorkspaceInput {
    #[validate(length(min = 1, max = 255))]
    pub name: Option<String>,
    pub description: Option<String>,
}
