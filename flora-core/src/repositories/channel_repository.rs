//! Channel repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::Channel;
use crate::traits::ChannelRepository;

/// `PostgreSQL` implementation of the `ChannelRepository` trait.
pub struct PgChannelRepository {
    pool: PgPool,
}

impl PgChannelRepository {
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ChannelRepository for PgChannelRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Channel>> {
        let channel = sqlx::query_as::<_, Channel>(
            "SELECT id, workspace_id, organization_id, name, type, created_at, updated_at
             FROM channels
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(channel)
    }

    async fn find_by_workspace_id(&self, workspace_id: Uuid) -> Result<Vec<Channel>> {
        let channels = sqlx::query_as::<_, Channel>(
            "SELECT id, workspace_id, organization_id, name, type, created_at, updated_at
             FROM channels
             WHERE workspace_id = $1
             ORDER BY name",
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(channels)
    }

    async fn create(&self, channel: Channel) -> Result<Channel> {
        let created = sqlx::query_as::<_, Channel>(
            "INSERT INTO channels (id, workspace_id, organization_id, name, type)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, workspace_id, organization_id, name, type, created_at, updated_at",
        )
        .bind(channel.id)
        .bind(channel.workspace_id)
        .bind(channel.organization_id)
        .bind(&channel.name)
        .bind(channel.channel_type.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn update(&self, id: Uuid, channel: Channel) -> Result<Channel> {
        let updated = sqlx::query_as::<_, Channel>(
            "UPDATE channels
             SET name = $2, type = $3
             WHERE id = $1
             RETURNING id, workspace_id, organization_id, name, type, created_at, updated_at",
        )
        .bind(id)
        .bind(&channel.name)
        .bind(channel.channel_type.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?
        .ok_or_else(|| Error::ChannelNotFound(id.to_string()))?;
        Ok(updated)
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("DELETE FROM channels WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::ChannelNotFound(id.to_string()));
        }
        Ok(())
    }
}
