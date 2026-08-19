//! Message repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{Message, Pagination};
use crate::traits::MessageRepository;

/// `PostgreSQL` implementation of the `MessageRepository` trait.
pub struct PgMessageRepository {
    pool: PgPool,
}

impl PgMessageRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MessageRepository for PgMessageRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Message>> {
        let message = sqlx::query_as::<_, Message>(
            "SELECT id, channel_id, organization_id, sender_id, content, thread_id,
                    is_edited, is_deleted, created_at, updated_at
             FROM messages
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(message)
    }

    async fn find_by_channel_id(
        &self,
        channel_id: Uuid,
        pagination: Pagination,
    ) -> Result<crate::models::Page<Message>> {
        let limit = pagination.resolved_limit();
        let messages = sqlx::query_as::<_, Message>(
            "SELECT id, channel_id, organization_id, sender_id, content, thread_id,
                    is_edited, is_deleted, created_at, updated_at
             FROM messages
             WHERE channel_id = $1 AND is_deleted = false
             ORDER BY created_at DESC
             LIMIT $2",
        )
        .bind(channel_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(crate::models::Page::new(messages, None))
    }

    async fn find_by_sender_id(
        &self,
        sender_id: Uuid,
        pagination: Pagination,
    ) -> Result<crate::models::Page<Message>> {
        let limit = pagination.resolved_limit();
        let messages = sqlx::query_as::<_, Message>(
            "SELECT id, channel_id, organization_id, sender_id, content, thread_id,
                    is_edited, is_deleted, created_at, updated_at
             FROM messages
             WHERE sender_id = $1 AND is_deleted = false
             ORDER BY created_at DESC
             LIMIT $2",
        )
        .bind(sender_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(crate::models::Page::new(messages, None))
    }

    async fn create(&self, message: Message) -> Result<Message> {
        let created = sqlx::query_as::<_, Message>(
            "INSERT INTO messages (id, channel_id, organization_id, sender_id, content, thread_id)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, channel_id, organization_id, sender_id, content, thread_id,
                       is_edited, is_deleted, created_at, updated_at",
        )
        .bind(message.id)
        .bind(message.channel_id)
        .bind(message.organization_id)
        .bind(message.sender_id)
        .bind(&message.content)
        .bind(message.thread_id)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn update(&self, id: Uuid, message: Message) -> Result<Message> {
        let updated = sqlx::query_as::<_, Message>(
            "UPDATE messages
             SET content = $2, is_edited = true
             WHERE id = $1
             RETURNING id, channel_id, organization_id, sender_id, content, thread_id,
                       is_edited, is_deleted, created_at, updated_at",
        )
        .bind(id)
        .bind(&message.content)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?
        .ok_or_else(|| Error::MessageNotFound(id.to_string()))?;
        Ok(updated)
    }

    async fn soft_delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE messages SET is_deleted = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::MessageNotFound(id.to_string()));
        }
        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE messages SET is_deleted = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::MessageNotFound(id.to_string()));
        }
        Ok(())
    }
}
