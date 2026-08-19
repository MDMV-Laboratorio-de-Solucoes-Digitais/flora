//! Task repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{Pagination, Task, TaskStatus};
use crate::traits::TaskRepository;

/// `PostgreSQL` implementation of the `TaskRepository` trait.
pub struct PgTaskRepository {
    pool: PgPool,
}

impl PgTaskRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>> {
        let task = sqlx::query_as::<_, Task>(
            "SELECT id, workspace_id, organization_id, creator_id, assignee_id,
                    title, description, status, is_deleted, created_at, updated_at
             FROM tasks
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(task)
    }

    async fn find_by_workspace_id(
        &self,
        workspace_id: Uuid,
        pagination: Pagination,
        status: Option<TaskStatus>,
        assignee_id: Option<Uuid>,
    ) -> Result<crate::models::Page<Task>> {
        let limit = pagination.resolved_limit();

        let tasks = match (status, assignee_id) {
            (None, None) => {
                sqlx::query_as::<_, Task>(
                    "SELECT id, workspace_id, organization_id, creator_id, assignee_id,
                            title, description, status, is_deleted, created_at, updated_at
                     FROM tasks
                     WHERE workspace_id = $1 AND is_deleted = false
                     ORDER BY created_at DESC
                     LIMIT $2",
                )
                .bind(workspace_id)
                .bind(limit)
                .fetch_all(&self.pool)
                .await
            }
            (Some(s), None) => {
                sqlx::query_as::<_, Task>(
                    "SELECT id, workspace_id, organization_id, creator_id, assignee_id,
                            title, description, status, is_deleted, created_at, updated_at
                     FROM tasks
                     WHERE workspace_id = $1 AND status = $2 AND is_deleted = false
                     ORDER BY created_at DESC
                     LIMIT $3",
                )
                .bind(workspace_id)
                .bind(s.to_string())
                .bind(limit)
                .fetch_all(&self.pool)
                .await
            }
            (None, Some(a)) => {
                sqlx::query_as::<_, Task>(
                    "SELECT id, workspace_id, organization_id, creator_id, assignee_id,
                            title, description, status, is_deleted, created_at, updated_at
                     FROM tasks
                     WHERE workspace_id = $1 AND assignee_id = $2 AND is_deleted = false
                     ORDER BY created_at DESC
                     LIMIT $3",
                )
                .bind(workspace_id)
                .bind(a)
                .bind(limit)
                .fetch_all(&self.pool)
                .await
            }
            (Some(s), Some(a)) => {
                sqlx::query_as::<_, Task>(
                    "SELECT id, workspace_id, organization_id, creator_id, assignee_id,
                            title, description, status, is_deleted, created_at, updated_at
                     FROM tasks
                     WHERE workspace_id = $1 AND status = $2 AND assignee_id = $3 AND is_deleted = false
                     ORDER BY created_at DESC
                     LIMIT $4",
                )
                .bind(workspace_id)
                .bind(s.to_string())
                .bind(a)
                .bind(limit)
                .fetch_all(&self.pool)
                .await
            }
        }
        .map_err(Error::from_sqlx)?;

        Ok(crate::models::Page::new(tasks, None))
    }

    async fn find_by_assignee_id(
        &self,
        assignee_id: Uuid,
        pagination: Pagination,
    ) -> Result<crate::models::Page<Task>> {
        let limit = pagination.resolved_limit();
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT id, workspace_id, organization_id, creator_id, assignee_id,
                    title, description, status, is_deleted, created_at, updated_at
             FROM tasks
             WHERE assignee_id = $1 AND is_deleted = false
             ORDER BY created_at DESC
             LIMIT $2",
        )
        .bind(assignee_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(crate::models::Page::new(tasks, None))
    }

    async fn create(&self, task: Task) -> Result<Task> {
        let created = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (id, workspace_id, organization_id, creator_id, assignee_id,
                                 title, description, status)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             RETURNING id, workspace_id, organization_id, creator_id, assignee_id,
                       title, description, status, is_deleted, created_at, updated_at",
        )
        .bind(task.id)
        .bind(task.workspace_id)
        .bind(task.organization_id)
        .bind(task.creator_id)
        .bind(task.assignee_id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.status.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn update(&self, id: Uuid, task: Task) -> Result<Task> {
        let updated = sqlx::query_as::<_, Task>(
            "UPDATE tasks
             SET workspace_id = $2, organization_id = $3, creator_id = $4, assignee_id = $5,
                 title = $6, description = $7, status = $8
             WHERE id = $1
             RETURNING id, workspace_id, organization_id, creator_id, assignee_id,
                       title, description, status, is_deleted, created_at, updated_at",
        )
        .bind(id)
        .bind(task.workspace_id)
        .bind(task.organization_id)
        .bind(task.creator_id)
        .bind(task.assignee_id)
        .bind(&task.title)
        .bind(&task.description)
        .bind(task.status.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?
        .ok_or_else(|| Error::TaskNotFound(id.to_string()))?;
        Ok(updated)
    }

    async fn soft_delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE tasks SET is_deleted = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::TaskNotFound(id.to_string()));
        }
        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE tasks SET is_deleted = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::TaskNotFound(id.to_string()));
        }
        Ok(())
    }
}
