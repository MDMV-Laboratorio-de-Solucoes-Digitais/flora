//! Flora Tasks - Task Management Module
//!
//! Note: Using `sqlx::query` (runtime) instead of `query!` (compile-time)
//! because this crate is compiled without DATABASE_URL in CI.

use flora_core::{models::Task, Error, Result};
use sqlx::PgPool;
use uuid::Uuid;

/// Task service for managing tasks.
pub struct TaskService {
    db: PgPool,
}

impl TaskService {
    #[must_use]
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    /// Creates a new task in the database.
    pub async fn create(&self, task: Task) -> Result<Task> {
        sqlx::query(
            r#"
            INSERT INTO tasks (id, workspace_id, organization_id, creator_id, assignee_id,
                               title, description, status, is_deleted, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(task.id)
        .bind(task.workspace_id)
        .bind(task.organization_id)
        .bind(task.creator_id)
        .bind(task.assignee_id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.status.to_string())
        .bind(task.is_deleted)
        .bind(task.created_at)
        .bind(task.updated_at)
        .execute(&self.db)
        .await
        .map_err(Error::from_sqlx)?;

        Ok(task)
    }

    /// Finds a task by ID.
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>> {
        let row = sqlx::query_as::<_, Task>(
            r#"
            SELECT id, workspace_id, organization_id, creator_id, assignee_id,
                   title, description, status, is_deleted, created_at, updated_at
            FROM tasks
            WHERE id = $1 AND is_deleted = false
            "#,
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(row)
    }
}
