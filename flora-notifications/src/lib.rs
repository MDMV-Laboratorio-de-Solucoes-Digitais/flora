//! Flora Notifications - Notification System with PGMQ + Valkey Streams
//!
//! Note: Using `sqlx::query` (runtime) instead of `query!` (compile-time)
//! because this crate is compiled without `DATABASE_URL` in CI.
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive dependency overrides"
)]
use flora_core::{
    Result,
    error::Error,
    models::{Notification, NotificationType},
};
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::fmt;
use std::str::FromStr;

/// Notification service for managing notifications.
pub struct NotificationService {
    db: PgPool,
    redis: ConnectionManager,
}

impl fmt::Debug for NotificationService {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NotificationService")
            .field("db", &"<PgPool>")
            .field("redis", &"<ConnectionManager>")
            .finish()
    }
}

impl NotificationService {
    /// Creates a new `NotificationService` with the given database pool and Valkey connection.
    #[must_use]
    pub const fn new(db: PgPool, redis: ConnectionManager) -> Self {
        Self { db, redis }
    }

    /// Creates a new notification in the database and publishes to Valkey stream.
    ///
    /// # Errors
    ///
    /// Returns an error if the database insert or Valkey publish fails.
    pub async fn create(&self, notification: Notification) -> Result<Notification> {
        let event_type = notification.event_type;
        let _count = sqlx::query(
            r"
            INSERT INTO notifications (id, organization_id, user_id, event_type, target_id,
                                       target_url, is_read, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ",
        )
        .bind(notification.id)
        .bind(notification.organization_id)
        .bind(notification.user_id)
        .bind(event_type)
        .bind(notification.target_id)
        .bind(&notification.target_url)
        .bind(notification.is_read)
        .bind(notification.created_at)
        .bind(notification.updated_at)
        .execute(&self.db)
        .await
        .map_err(Error::from_sqlx)?;

        // Publish to Valkey stream for real-time delivery.
        let mut conn = self.redis.clone();
        let stream_key = format!("notifications:org:{}", notification.organization_id);
        let payload =
            serde_json::to_string(&notification).map_err(|e| Error::Internal(e.to_string()))?;

        let _: String = redis::cmd("XADD")
            .arg(&stream_key)
            .arg("*")
            .arg("user_id")
            .arg(notification.user_id.to_string())
            .arg("event_type")
            .arg(event_type.to_string())
            .arg("payload")
            .arg(&payload)
            .query_async(&mut conn)
            .await
            .map_err(|e| Error::Messaging(e.to_string()))?;

        Ok(notification)
    }

    /// Gets unread notifications for a user.
    ///
    /// # Errors
    ///
    /// Returns an error if the query fails.
    pub async fn get_unread(&self, user_id: uuid::Uuid, limit: i64) -> Result<Vec<Notification>> {
        let rows = sqlx::query_as::<_, NotificationRow>(
            r"
            SELECT id, organization_id, user_id, event_type, target_id, target_url,
                   is_read, created_at, updated_at
            FROM notifications
            WHERE user_id = $1 AND is_read = false
            ORDER BY created_at DESC
            LIMIT $2
            ",
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.db)
        .await
        .map_err(Error::from_sqlx)?;

        Ok(rows.into_iter().map(NotificationRow::into_model).collect())
    }

    /// Marks a notification as read.
    ///
    /// # Errors
    ///
    /// Returns an error if the update fails.
    pub async fn mark_read(&self, id: uuid::Uuid) -> Result<()> {
        let _count = sqlx::query("UPDATE notifications SET is_read = true WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(Error::from_sqlx)?;
        Ok(())
    }
}

/// Temporary row type for query results without `query!` macro.
#[derive(Debug, sqlx::FromRow)]
struct NotificationRow {
    id: uuid::Uuid,
    organization_id: uuid::Uuid,
    user_id: uuid::Uuid,
    event_type: String,
    target_id: uuid::Uuid,
    target_url: Option<String>,
    is_read: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl NotificationRow {
    #[must_use]
    fn into_model(self) -> Notification {
        Notification {
            id: self.id,
            organization_id: self.organization_id,
            user_id: self.user_id,
            event_type: NotificationType::from_str(&self.event_type).unwrap_or_default(),
            target_id: self.target_id,
            target_url: self.target_url,
            is_read: self.is_read,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}
