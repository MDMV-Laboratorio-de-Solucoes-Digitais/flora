//! Flora Tasks - Task Management Module
//!
//! Note: Using `sqlx::query` (runtime) instead of `query!` (compile-time)
//! because this crate is compiled without `DATABASE_URL` in CI.
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive dependency overrides"
)]
use flora_core::{Result, error::Error, models::Task};
use sqlx::PgPool;
use std::fmt;
use uuid::Uuid;

/// Task service for managing tasks.
pub struct TaskService {
    db: PgPool,
}

impl fmt::Debug for TaskService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TaskService")
            .field("db", &"<PgPool>")
            .finish()
    }
}

impl TaskService {
    /// Creates a new `TaskService` with the given database pool.
    #[must_use]
    pub const fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Creates a new task in the database.
    ///
    /// # Errors
    ///
    /// Returns an error if the task fails to be created.
    pub async fn create(&self, task: Task) -> Result<Task> {
        let _count = sqlx::query(
            r"
            INSERT INTO tasks (id, workspace_id, organization_id, creator_id, assignee_id,
                               title, description, status, is_deleted, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ",
        )
        .bind(task.id)
        .bind(task.workspace_id)
        .bind(task.organization_id)
        .bind(task.creator_id)
        .bind(task.assignee_id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.status)
        .bind(task.is_deleted)
        .bind(task.created_at)
        .bind(task.updated_at)
        .execute(&self.db)
        .await
        .map_err(Error::from_sqlx)?;

        Ok(task)
    }

    /// Finds a task by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the task fails to be found.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>> {
        let row = sqlx::query_as::<_, Task>(
            r"
            SELECT id, workspace_id, organization_id, creator_id, assignee_id,
                   title, description, status, is_deleted, created_at, updated_at
            FROM tasks
            WHERE id = $1 AND is_deleted = false
            ",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(row)
    }
}
