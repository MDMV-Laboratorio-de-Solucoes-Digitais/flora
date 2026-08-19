//! Domain models for Flora Workspace.
//!
//! Every tenant-scoped entity includes `organization_id` for multi-tenant isolation.

pub mod channel;
pub mod file;
pub mod membership;
pub mod message;
pub mod notification;
pub mod organization;
pub mod role;
pub mod session;
pub mod task;
pub mod user;
pub mod workspace;

pub use channel::{Channel, ChannelType};
pub use file::File;
pub use membership::Membership;
pub use message::Message;
pub use notification::{Notification, NotificationType};
pub use organization::{CreateOrganizationInput, Organization, UpdateOrganizationInput};
pub use role::{Permission, Role};
pub use session::Session;
pub use task::{Task, TaskStatus};
pub use user::{CreateUserInput, UpdateUserInput, User};
pub use workspace::Workspace;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A UUID-based identifier for all entities.
pub type EntityId = Uuid;

/// Common fields shared by all entities.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Entity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Common fields for tenant-scoped entities.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TenantEntity {
    #[sqlx(flatten)]
    pub entity: Entity,
    pub organization_id: Uuid,
}

/// Pagination request parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    pub limit: Option<i64>,
    pub cursor: Option<Uuid>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: Some(50),
            cursor: None,
        }
    }
}

impl Pagination {
    #[must_use]
    pub fn resolved_limit(&self) -> i64 {
        self.limit.unwrap_or(50).min(100)
    }
}

/// A paginated list response.
#[derive(Debug, Clone, Serialize)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub next_cursor: Option<Uuid>,
    pub total: Option<i64>,
}

impl<T> Page<T> {
    #[must_use]
    pub const fn new(data: Vec<T>, next_cursor: Option<Uuid>) -> Self {
        Self {
            data,
            next_cursor,
            total: None,
        }
    }

    #[must_use]
    pub const fn with_total(data: Vec<T>, next_cursor: Option<Uuid>, total: i64) -> Self {
        Self {
            data,
            next_cursor,
            total: Some(total),
        }
    }
}

/// API response wrapper.
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub timestamp: DateTime<Utc>,
}

impl<T> ApiResponse<T> {
    pub fn new(data: T) -> Self {
        Self {
            data,
            timestamp: Utc::now(),
        }
    }
}
