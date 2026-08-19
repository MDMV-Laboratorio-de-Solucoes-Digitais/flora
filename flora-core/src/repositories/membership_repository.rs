//! Membership repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::Membership;
use crate::traits::MembershipRepository;

/// `PostgreSQL` implementation of the `MembershipRepository` trait.
pub struct PgMembershipRepository {
    pool: PgPool,
}

impl PgMembershipRepository {
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MembershipRepository for PgMembershipRepository {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<Membership>> {
        let memberships = sqlx::query_as::<_, Membership>(
            "SELECT user_id, organization_id, role_id, joined_at, metadata
             FROM memberships
             WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(memberships)
    }

    async fn find_by_organization_id(&self, organization_id: Uuid) -> Result<Vec<Membership>> {
        let memberships = sqlx::query_as::<_, Membership>(
            "SELECT user_id, organization_id, role_id, joined_at, metadata
             FROM memberships
             WHERE organization_id = $1",
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(memberships)
    }

    async fn find_by_user_and_organization(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
    ) -> Result<Option<Membership>> {
        let membership = sqlx::query_as::<_, Membership>(
            "SELECT user_id, organization_id, role_id, joined_at, metadata
             FROM memberships
             WHERE user_id = $1 AND organization_id = $2",
        )
        .bind(user_id)
        .bind(organization_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(membership)
    }

    async fn create(&self, membership: Membership) -> Result<Membership> {
        let created = sqlx::query_as::<_, Membership>(
            "INSERT INTO memberships (user_id, organization_id, role_id, metadata)
             VALUES ($1, $2, $3, $4)
             RETURNING user_id, organization_id, role_id, joined_at, metadata",
        )
        .bind(membership.user_id)
        .bind(membership.organization_id)
        .bind(membership.role_id)
        .bind(&membership.metadata)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn delete(&self, user_id: Uuid, organization_id: Uuid) -> Result<()> {
        let result =
            sqlx::query("DELETE FROM memberships WHERE user_id = $1 AND organization_id = $2")
                .bind(user_id)
                .bind(organization_id)
                .execute(&self.pool)
                .await
                .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::MembershipNotFound);
        }
        Ok(())
    }
}
