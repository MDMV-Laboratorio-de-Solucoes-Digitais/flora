//! Organization repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{Organization, Pagination, UpdateOrganizationInput};
use crate::traits::OrganizationRepository;

/// `PostgreSQL` implementation of the `OrganizationRepository` trait.
#[derive(Debug)]
pub struct PgOrganizationRepository {
    pool: PgPool,
}

impl PgOrganizationRepository {
    /// Creates a new `PgOrganizationRepository`.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationRepository for PgOrganizationRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Organization>> {
        let org = sqlx::query_as::<_, Organization>(
            "SELECT id, name, slug, settings, created_at, updated_at
             FROM organizations
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(org)
    }

    async fn find_by_slug(&self, slug: &str) -> Result<Option<Organization>> {
        let org = sqlx::query_as::<_, Organization>(
            "SELECT id, name, slug, settings, created_at, updated_at
             FROM organizations
             WHERE slug = $1",
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(org)
    }

    async fn list(&self, pagination: Pagination) -> Result<crate::models::Page<Organization>> {
        let limit = pagination.resolved_limit();
        let orgs = sqlx::query_as::<_, Organization>(
            "SELECT id, name, slug, settings, created_at, updated_at
             FROM organizations
             ORDER BY created_at DESC
             LIMIT $1",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(crate::models::Page::new(orgs, None))
    }

    async fn create(&self, org: Organization) -> Result<Organization> {
        let created = sqlx::query_as::<_, Organization>(
            "INSERT INTO organizations (id, name, slug, settings)
             VALUES ($1, $2, $3, $4)
             RETURNING id, name, slug, settings, created_at, updated_at",
        )
        .bind(org.id)
        .bind(&org.name)
        .bind(&org.slug)
        .bind(&org.settings)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn update(&self, id: Uuid, updates: UpdateOrganizationInput) -> Result<Organization> {
        let org = sqlx::query_as::<_, Organization>(
            "UPDATE organizations
             SET name = COALESCE($2, name),
                 settings = COALESCE($3, settings)
             WHERE id = $1
             RETURNING id, name, slug, settings, created_at, updated_at",
        )
        .bind(id)
        .bind(updates.name)
        .bind(updates.settings)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?
        .ok_or_else(|| Error::OrganizationNotFound(id.to_string()))?;
        Ok(org)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM organizations WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::OrganizationNotFound(id.to_string()));
        }
        Ok(())
    }
}
