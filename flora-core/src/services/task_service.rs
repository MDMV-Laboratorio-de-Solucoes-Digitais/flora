//! Task service — business logic for task management.

use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::error::{Error, Result};
use crate::models::{CreateTaskInput, Page, Pagination, Task, TaskStatus, UpdateTaskInput};
use crate::traits::TaskRepository;

/// Configuration for task retention policy.
#[derive(Debug, Clone)]
pub struct TaskRetentionPolicy {
    /// Number of days to keep soft-deleted tasks before permanent deletion.
    /// Must be between 30 and 365. Default is 90.
    pub days_to_keep: u32,
}

impl Default for TaskRetentionPolicy {
    fn default() -> Self {
        Self { days_to_keep: 90 }
    }
}

impl TaskRetentionPolicy {
    /// Creates a new retention policy, clamping days to the valid range [30, 365].
    #[must_use]
    pub fn new(days: u32) -> Self {
        Self {
            days_to_keep: days.clamp(30, 365),
        }
    }
}

/// Task management service.
#[derive(Debug, Clone)]
pub struct TaskService {
    task_repo: Arc<dyn TaskRepository + Send + Sync>,
    retention_policy: TaskRetentionPolicy,
}

impl TaskService {
    /// Creates a new `TaskService`.
    #[must_use]
    pub fn new(task_repo: Arc<dyn TaskRepository + Send + Sync>) -> Self {
        Self {
            task_repo,
            retention_policy: TaskRetentionPolicy::default(),
        }
    }

    /// Creates a new `TaskService` with a specific retention policy.
    #[must_use]
    pub fn with_policy(
        task_repo: Arc<dyn TaskRepository + Send + Sync>,
        retention_policy: TaskRetentionPolicy,
    ) -> Self {
        Self {
            task_repo,
            retention_policy,
        }
    }

    /// Creates a new task.
    ///
    /// # Errors
    /// Returns an error if the input is invalid or database operations fail.
    pub async fn create_task(
        &self,
        organization_id: Uuid,
        creator_id: Uuid,
        input: CreateTaskInput,
    ) -> Result<Task> {
        input.validate().map_err(Error::from)?;

        let mut task = Task::new(input.workspace_id, organization_id, creator_id, &input.title);
        task.description = input.description;
        task.assignee_id = input.assignee_id;

        let created = self.task_repo.create(task).await?;
        tracing::info!(task_id = %created.id, "Task created");
        Ok(created)
    }

    /// Lists tasks in a workspace.
    ///
    /// # Errors
    /// Returns an error if the database query fails.
    pub async fn list_tasks(
        &self,
        workspace_id: Uuid,
        pagination: Pagination,
        status: Option<TaskStatus>,
        assignee_id: Option<Uuid>,
    ) -> Result<Page<Task>> {
        self.task_repo
            .find_by_workspace_id(workspace_id, pagination, status, assignee_id)
            .await
    }

    /// Gets a task by ID.
    ///
    /// # Errors
    /// Returns an error if the task is not found or database query fails.
    pub async fn get_task(&self, task_id: Uuid) -> Result<Task> {
        self.task_repo
            .find_by_id(task_id)
            .await?
            .ok_or_else(|| Error::TaskNotFound(task_id.to_string()))
    }

    /// Updates a task.
    ///
    /// # Errors
    /// Returns an error if the input is invalid, task not found, or database query fails.
    pub async fn update_task(&self, task_id: Uuid, input: UpdateTaskInput) -> Result<Task> {
        input.validate().map_err(Error::from)?;

        let task = self.get_task(task_id).await?;
        
        let status = input.status.unwrap_or(task.status);
        
        let updated = Task {
            title: input.title.unwrap_or(task.title),
            description: input.description.or(task.description),
            assignee_id: input.assignee_id.or(task.assignee_id),
            status,
            updated_at: chrono::Utc::now(),
            ..task
        };

        let result = self.task_repo.update(task_id, updated).await?;
        tracing::debug!(task_id = %task_id, "Task updated");
        Ok(result)
    }

    /// Assigns a task to a user.
    ///
    /// # Errors
    /// Returns an error if the task is not found or database query fails.
    pub async fn assign_task(&self, task_id: Uuid, assignee_id: Option<Uuid>) -> Result<Task> {
        let task = self.get_task(task_id).await?;
        let updated = Task {
            assignee_id,
            updated_at: chrono::Utc::now(),
            ..task
        };

        let result = self.task_repo.update(task_id, updated).await?;
        tracing::debug!(task_id = %task_id, ?assignee_id, "Task assignee updated");
        Ok(result)
    }

    /// Updates the status of a task.
    ///
    /// # Errors
    /// Returns an error if the task is not found or database query fails.
    pub async fn update_status(&self, task_id: Uuid, status: TaskStatus) -> Result<Task> {
        let task = self.get_task(task_id).await?;
        let updated = Task {
            status,
            updated_at: chrono::Utc::now(),
            ..task
        };

        let result = self.task_repo.update(task_id, updated).await?;
        tracing::debug!(task_id = %task_id, ?status, "Task status updated");
        Ok(result)
    }

    /// Soft-deletes a task.
    ///
    /// # Errors
    /// Returns an error if the task is not found or database query fails.
    pub async fn delete_task(&self, task_id: Uuid) -> Result<()> {
        self.task_repo.soft_delete(task_id).await?;
        tracing::info!(task_id = %task_id, "Task soft-deleted");
        Ok(())
    }

    /// Purges tasks that have been soft-deleted longer than the retention policy.
    ///
    /// # Errors
    /// Returns an error if the database query fails.
    pub async fn purge_tasks(&self) -> Result<usize> {
        let threshold = chrono::Utc::now() - chrono::Duration::days(i64::from(self.retention_policy.days_to_keep));
        let purged = self.task_repo.purge_old(threshold).await?;
        if purged > 0 {
            tracing::info!(purged_count = purged, days = self.retention_policy.days_to_keep, "Purged old soft-deleted tasks");
        }
        Ok(purged)
    }
}
