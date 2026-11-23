//! User repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{UpdateUserInput, User};
use crate::traits::UserRepository;

/// `PostgreSQL` implementation of the `UserRepository` trait.
#[derive(Debug)]
pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    /// Creates a new `PgUserRepository`.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, oidc_subject, password_hash, password_updated_at,
                    display_name, avatar_url, profile, is_active, created_at, updated_at
             FROM users
             WHERE email = $1",
        )
        .bind(email.to_lowercase())
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(user)
    }

    async fn find_by_oidc_subject(&self, subject: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, oidc_subject, password_hash, password_updated_at,
                    display_name, avatar_url, profile, is_active, created_at, updated_at
             FROM users
             WHERE oidc_subject = $1",
        )
        .bind(subject)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, oidc_subject, password_hash, password_updated_at,
                    display_name, avatar_url, profile, is_active, created_at, updated_at
             FROM users
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(user)
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let users = sqlx::query_as::<_, User>(
            "SELECT id, email, oidc_subject, password_hash, password_updated_at,
                    display_name, avatar_url, profile, is_active, created_at, updated_at
             FROM users
             WHERE id = ANY($1)",
        )
        .bind(ids)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(users)
    }

    async fn create(&self, user: User) -> Result<User> {
        let created = sqlx::query_as::<_, User>(
            "INSERT INTO users (id, email, oidc_subject, password_hash, password_updated_at,
                                display_name, avatar_url, profile, is_active)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, email, oidc_subject, password_hash, password_updated_at,
                       display_name, avatar_url, profile, is_active, created_at, updated_at",
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.oidc_subject)
        .bind(&user.password_hash)
        .bind(user.password_updated_at)
        .bind(&user.display_name)
        .bind(&user.avatar_url)
        .bind(&user.profile)
        .bind(user.is_active)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn set_password_hash_if_absent(
        &self,
        id: Uuid,
        password_hash: &str,
        updated_at: DateTime<Utc>,
    ) -> Result<bool> {
        let result = sqlx::query(
            "UPDATE users SET password_hash = $2, password_updated_at = $3
             WHERE id = $1 AND password_hash IS NULL",
        )
        .bind(id)
        .bind(password_hash)
        .bind(updated_at)
        .execute(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(result.rows_affected() > 0)
    }

    async fn update(&self, id: Uuid, updates: UpdateUserInput) -> Result<User> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users
             SET display_name = COALESCE($2, display_name),
                 avatar_url = COALESCE($3, avatar_url)
             WHERE id = $1 AND is_active = true
             RETURNING id, email, oidc_subject, password_hash, password_updated_at,
                       display_name, avatar_url, profile, is_active, created_at, updated_at",
        )
        .bind(id)
        .bind(updates.display_name)
        .bind(updates.avatar_url)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?
        .ok_or_else(|| Error::UserNotFound(id.to_string()))?;
        Ok(user)
    }

    async fn deactivate(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE users SET is_active = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::UserNotFound(id.to_string()));
        }
        Ok(())
    }
}
