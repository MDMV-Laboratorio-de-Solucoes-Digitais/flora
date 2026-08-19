//! Role repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::Role;
use crate::traits::RoleRepository;

/// `PostgreSQL` implementation of the `RoleRepository` trait.
pub struct PgRoleRepository {
    pool: PgPool,
}

impl PgRoleRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for PgRoleRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            "SELECT id, organization_id, name, permissions, description, is_builtin, created_at, updated_at
             FROM roles
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(role)
    }

    async fn find_by_organization_id(&self, organization_id: Uuid) -> Result<Vec<Role>> {
        let roles = sqlx::query_as::<_, Role>(
            "SELECT id, organization_id, name, permissions, description, is_builtin, created_at, updated_at
             FROM roles
             WHERE organization_id = $1
             ORDER BY name",
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(roles)
    }

    async fn find_by_name_and_organization(
        &self,
        name: &str,
        organization_id: Uuid,
    ) -> Result<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            "SELECT id, organization_id, name, permissions, description, is_builtin, created_at, updated_at
             FROM roles
             WHERE name = $1 AND organization_id = $2",
        )
        .bind(name)
        .bind(organization_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(role)
    }

    async fn create(&self, role: Role) -> Result<Role> {
        let created = sqlx::query_as::<_, Role>(
            "INSERT INTO roles (id, organization_id, name, permissions, description, is_builtin)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, organization_id, name, permissions, description, is_builtin, created_at, updated_at",
        )
        .bind(role.id)
        .bind(role.organization_id)
        .bind(&role.name)
        .bind(&role.permissions)
        .bind(&role.description)
        .bind(role.is_builtin)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn update(&self, id: Uuid, role: Role) -> Result<Role> {
        let updated = sqlx::query_as::<_, Role>(
            "UPDATE roles
             SET name = $2, permissions = $3, description = $4
             WHERE id = $1
             RETURNING id, organization_id, name, permissions, description, is_builtin, created_at, updated_at",
        )
        .bind(id)
        .bind(&role.name)
        .bind(&role.permissions)
        .bind(&role.description)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?
        .ok_or_else(|| Error::RoleNotFound(id.to_string()))?;
        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM roles WHERE id = $1 AND is_builtin = false")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::RoleNotFound(id.to_string()));
        }
        Ok(())
    }
}
