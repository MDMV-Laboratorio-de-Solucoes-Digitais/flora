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
    pub id: Uuid,
    pub workspace_id: Uuid,
    /// Denormalized for fast tenant isolation checks.
    pub organization_id: Uuid,
    pub creator_id: Uuid,
    /// Optional assignee (nullable = unassigned).
    pub assignee_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    /// Soft-delete flag.
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
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
    pub title: String,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    pub workspace_id: Uuid,
    pub assignee_id: Option<Uuid>,
}

/// Input for updating a task.
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateTaskInput {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    #[validate(length(max = 5000))]
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
    pub assignee_id: Option<Uuid>,
}

/// Task status lifecycle.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
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

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Todo" => Ok(TaskStatus::Todo),
            "InProgress" => Ok(TaskStatus::InProgress),
            "Done" => Ok(TaskStatus::Done),
            "Archived" => Ok(TaskStatus::Archived),
            _ => Err(format!("invalid task status: {s}")),
        }
    }
}
