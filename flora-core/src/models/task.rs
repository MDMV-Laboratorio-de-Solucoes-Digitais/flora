//! Task model — an actionable item in a workspace.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;
use validator::Validate;

/// A task assigned to a user in a workspace.
///
/// Tasks support status tracking and soft-deletion.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Task {
    /// Unique identifier.
    pub id: Uuid,
    /// The workspace this task belongs to.
    pub workspace_id: Uuid,
    /// Denormalized for fast tenant isolation checks.
    pub organization_id: Uuid,
    /// The user who created the task.
    pub creator_id: Uuid,
    /// Optional assignee (nullable = unassigned).
    pub assignee_id: Option<Uuid>,
    /// Task title.
    pub title: String,
    /// Optional longer description.
    pub description: Option<String>,
    /// Current status of the task.
    pub status: TaskStatus,
    /// Soft-delete flag.
    pub is_deleted: bool,
    /// When the task was created.
    pub created_at: DateTime<Utc>,
    /// When the task was last updated.
    pub updated_at: DateTime<Utc>,
}

impl Task {
    /// Creates a new task.
    #[must_use]
    pub fn new(workspace_id: Uuid, organization_id: Uuid, creator_id: Uuid, title: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            workspace_id,
            organization_id,
            creator_id,
            assignee_id: None,
            title: title.to_owned(),
            description: None,
            status: TaskStatus::Todo,
            is_deleted: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Input for creating a task.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateTaskInput {
    #[validate(length(min = 1, max = 255, message = "title must be 1-255 characters"))]
    /// Task title.
    pub title: String,
    /// Optional longer description.
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    /// The workspace this task belongs to.
    pub workspace_id: Uuid,
    /// Optional assignee.
    pub assignee_id: Option<Uuid>,
}

/// Input for updating a task.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateTaskInput {
    #[validate(length(min = 1, max = 255))]
    /// Updated task title.
    pub title: Option<String>,
    /// Updated description.
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    /// Updated task status.
    pub status: Option<TaskStatus>,
    /// Updated assignee.
    pub assignee_id: Option<Uuid>,
}

/// Task status lifecycle.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum TaskStatus {
    /// Task has not been started.
    Todo,
    /// Task is actively being worked on.
    InProgress,
    /// Task has been completed.
    Done,
    /// Task is archived and no longer active.
    Archived,
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Todo => "Todo",
            Self::InProgress => "InProgress",
            Self::Done => "Done",
            Self::Archived => "Archived",
        };
        write!(f, "{s}")
    }
}

impl std::str::FromStr for TaskStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Todo" => Ok(Self::Todo),
            "InProgress" => Ok(Self::InProgress),
            "Done" => Ok(Self::Done),
            "Archived" => Ok(Self::Archived),
            _ => Err(format!("invalid task status: {s}")),
        }
    }
}
