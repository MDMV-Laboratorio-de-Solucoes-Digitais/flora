//! Notification repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{Notification, Pagination};
use crate::traits::NotificationRepository;

/// `PostgreSQL` implementation of the `NotificationRepository` trait.
#[derive(Debug)]
pub struct PgNotificationRepository {
    pool: PgPool,
}

impl PgNotificationRepository {
    /// Creates a new `PgNotificationRepository`.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl NotificationRepository for PgNotificationRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Notification>> {
        let notification = sqlx::query_as::<_, Notification>(
            "SELECT id, organization_id, user_id, event_type, target_id, target_url, is_read, created_at, updated_at
             FROM notifications
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(notification)
    }

    async fn find_unread_by_user_id(
        &self,
        user_id: Uuid,
        pagination: Pagination,
    ) -> Result<crate::models::Page<Notification>> {
        let limit = pagination.resolved_limit();
        let notifications = sqlx::query_as::<_, Notification>(
            "SELECT id, organization_id, user_id, event_type, target_id, target_url, is_read, created_at, updated_at
             FROM notifications
             WHERE user_id = $1 AND is_read = false
             ORDER BY created_at DESC
             LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(crate::models::Page::new(notifications, None))
    }

    async fn find_by_user_id(
        &self,
        user_id: Uuid,
        pagination: Pagination,
    ) -> Result<crate::models::Page<Notification>> {
        let limit = pagination.resolved_limit();
        let notifications = sqlx::query_as::<_, Notification>(
            "SELECT id, organization_id, user_id, event_type, target_id, target_url, is_read, created_at, updated_at
             FROM notifications
             WHERE user_id = $1
             ORDER BY created_at DESC
             LIMIT $2",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(crate::models::Page::new(notifications, None))
    }

    async fn create(&self, notification: Notification) -> Result<Notification> {
        let created = sqlx::query_as::<_, Notification>(
            "INSERT INTO notifications (id, organization_id, user_id, event_type, target_id, target_url)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING id, organization_id, user_id, event_type, target_id, target_url, is_read, created_at, updated_at",
        )
        .bind(notification.id)
        .bind(notification.organization_id)
        .bind(notification.user_id)
        .bind(notification.event_type)
        .bind(notification.target_id)
        .bind(&notification.target_url)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn mark_as_read(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE notifications SET is_read = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::NotificationNotFound(id.to_string()));
        }
        Ok(())
    }

    async fn mark_all_as_read(&self, user_id: Uuid) -> Result<()> {
        let _ = sqlx::query("UPDATE notifications SET is_read = true WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        Ok(())
    }

    async fn delete_old(&self, older_than: chrono::DateTime<Utc>) -> Result<usize> {
        let result = sqlx::query("DELETE FROM notifications WHERE created_at < $1")
            .bind(older_than)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        Ok(usize::try_from(result.rows_affected())
            .unwrap_or_else(|_| usize::try_from(result.rows_affected()).unwrap_or(0)))
    }
}
