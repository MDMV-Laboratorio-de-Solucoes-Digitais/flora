//! Session repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::Session;
use crate::traits::SessionRepository;

/// `PostgreSQL` implementation of the `SessionRepository` trait.
pub struct PgSessionRepository {
    pool: PgPool,
}

impl PgSessionRepository {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SessionRepository for PgSessionRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT id, user_id, organization_id, jti, refresh_token_id, client_ip,
                    user_agent, expires_at, last_activity_at, is_active, created_at, updated_at
             FROM sessions
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(session)
    }

    async fn find_by_jti(&self, jti: &str) -> Result<Option<Session>> {
        let session = sqlx::query_as::<_, Session>(
            "SELECT id, user_id, organization_id, jti, refresh_token_id, client_ip,
                    user_agent, expires_at, last_activity_at, is_active, created_at, updated_at
             FROM sessions
             WHERE jti = $1",
        )
        .bind(jti)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(session)
    }

    async fn find_active_by_user_id(&self, user_id: Uuid) -> Result<Vec<Session>> {
        let sessions = sqlx::query_as::<_, Session>(
            "SELECT id, user_id, organization_id, jti, refresh_token_id, client_ip,
                    user_agent, expires_at, last_activity_at, is_active, created_at, updated_at
             FROM sessions
             WHERE user_id = $1 AND is_active = true AND expires_at > NOW()",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(sessions)
    }

    async fn create(&self, session: Session) -> Result<Session> {
        let created = sqlx::query_as::<_, Session>(
            "INSERT INTO sessions (id, user_id, organization_id, jti, refresh_token_id,
                                  client_ip, user_agent, expires_at, last_activity_at, is_active)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
             RETURNING id, user_id, organization_id, jti, refresh_token_id, client_ip,
                       user_agent, expires_at, last_activity_at, is_active, created_at, updated_at",
        )
        .bind(session.id)
        .bind(session.user_id)
        .bind(session.organization_id)
        .bind(&session.jti)
        .bind(&session.refresh_token_id)
        .bind(&session.client_ip)
        .bind(&session.user_agent)
        .bind(session.expires_at)
        .bind(session.last_activity_at)
        .bind(session.is_active)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn update_last_activity(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE sessions SET last_activity_at = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::Database("session not found".to_owned()));
        }
        Ok(())
    }

    async fn revoke(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE sessions SET is_active = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::Database("session not found".to_owned()));
        }
        Ok(())
    }

    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<()> {
        sqlx::query("UPDATE sessions SET is_active = false WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        Ok(())
    }

    async fn clean_expired(&self) -> Result<usize> {
        let result = sqlx::query("DELETE FROM sessions WHERE expires_at < NOW()")
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        Ok(usize::try_from(result.rows_affected()).unwrap_or_else(|_| usize::try_from(result.rows_affected()).unwrap_or(0)))
    }
}
