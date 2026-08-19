//! Workspace repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::Workspace;
use crate::traits::WorkspaceRepository;

/// `PostgreSQL` implementation of the `WorkspaceRepository` trait.
#[derive(Debug)]
pub struct PgWorkspaceRepository {
    pool: PgPool,
}

impl PgWorkspaceRepository {
    /// Creates a new `PgWorkspaceRepository`.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorkspaceRepository for PgWorkspaceRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Workspace>> {
        let workspace = sqlx::query_as::<_, Workspace>(
            "SELECT id, organization_id, name, description, created_at, updated_at
             FROM workspaces
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(workspace)
    }

    async fn find_by_organization_id(&self, organization_id: Uuid) -> Result<Vec<Workspace>> {
        let workspaces = sqlx::query_as::<_, Workspace>(
            "SELECT id, organization_id, name, description, created_at, updated_at
             FROM workspaces
             WHERE organization_id = $1
             ORDER BY name",
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(workspaces)
    }

    async fn create(&self, workspace: Workspace) -> Result<Workspace> {
        let created = sqlx::query_as::<_, Workspace>(
            "INSERT INTO workspaces (id, organization_id, name, description)
             VALUES ($1, $2, $3, $4)
             RETURNING id, organization_id, name, description, created_at, updated_at",
        )
        .bind(workspace.id)
        .bind(workspace.organization_id)
        .bind(&workspace.name)
        .bind(&workspace.description)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn update(&self, id: Uuid, workspace: Workspace) -> Result<Workspace> {
        let updated = sqlx::query_as::<_, Workspace>(
            "UPDATE workspaces
             SET name = $2, description = $3
             WHERE id = $1
             RETURNING id, organization_id, name, description, created_at, updated_at",
        )
        .bind(id)
        .bind(&workspace.name)
        .bind(&workspace.description)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?
        .ok_or_else(|| Error::WorkspaceNotFound(id.to_string()))?;
        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM workspaces WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::WorkspaceNotFound(id.to_string()));
        }
        Ok(())
    }
}
