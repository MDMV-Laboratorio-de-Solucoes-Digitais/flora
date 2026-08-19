//! Core domain models for Flora Workspace.

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

// Re-export commonly used types
pub use channel::{Channel, ChannelType, CreateChannelInput, UpdateChannelInput};
pub use file::{CreateFileInput, File};
pub use membership::Membership;
pub use message::{CreateMessageInput, Message, UpdateMessageInput};
pub use notification::{Notification, NotificationType};
pub use organization::{CreateOrganizationInput, Organization, UpdateOrganizationInput};
pub use role::{Permission, Role};
pub use session::Session;
pub use task::{CreateTaskInput, Task, TaskStatus, UpdateTaskInput};
pub use user::{CreateUserInput, UpdateUserInput, User};
pub use workspace::{CreateWorkspaceInput, UpdateWorkspaceInput, Workspace};

// ============================================================================
// Shared types defined in this module (not in separate files)
// ============================================================================

use serde::{Deserialize, Serialize};

// ============================================================================
// Shared types defined in this module (not in separate files)
// ============================================================================

/// Pagination helper for list queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Maximum number of items to return.
    pub limit: usize,
    /// Number of items to skip (offset).
    pub offset: usize,
}

impl Pagination {
    /// Maximum allowed limit for pagination.
    const MAX_LIMIT: usize = 100;

    /// Returns the resolved limit, capped at a maximum value.
    #[must_use]
    pub fn resolved_limit(&self) -> i64 {
        self.min(Self::MAX_LIMIT)
            .limit
            .try_into()
            .unwrap_or_else(|_| i64::try_from(Self::MAX_LIMIT).unwrap_or(100))
    }

    /// Returns a new pagination with the minimum of the current limit and a max.
    fn min(&self, max: usize) -> Self {
        Self {
            limit: self.limit.min(max),
            offset: self.offset,
        }
    }
}

/// Page model representing a paginated list of items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    /// The items on the current page.
    pub items: Vec<T>,
    /// Pagination information for the page.
    pub pagination: Option<Pagination>,
}

impl<T> Page<T> {
    /// Creates a new page with items and pagination.
    #[must_use]
    pub const fn new(items: Vec<T>, pagination: Option<Pagination>) -> Self {
        Self { items, pagination }
    }
}
