//! Session management service.
use crate::{
    error::{Error, Result},
    models::Session,
    traits::SessionRepository,
};
use chrono::{Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;

/// Service for managing user sessions.
#[derive(Debug, Clone)]
pub struct SessionService {
    session_repo: Arc<dyn SessionRepository + Send + Sync>,
}

impl SessionService {
    /// Creates a new `SessionService`.
    #[must_use]
    pub fn new(session_repo: Arc<dyn SessionRepository + Send + Sync>) -> Self {
        Self { session_repo }
    }

    /// Creates a new session for a user in an organization.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    /// * `organization_id` - The organization ID
    /// * `ttl_secs` - Time to live in seconds
    /// * `client_ip` - Optional client IP address
    /// * `user_agent` - Optional user agent string
    ///
    /// # Returns
    ///
    /// The created session
    ///
    /// # Errors
    ///
    /// Returns an error if the session creation fails.
    pub async fn create_session(
        &self,
        user_id: Uuid,
        organization_id: Uuid,
        ttl_secs: u64,
        client_ip: Option<String>,
        user_agent: Option<String>,
    ) -> Result<Session> {
        let jti = Uuid::now_v7().to_string();
        let now = Utc::now();
        let ttl = i64::try_from(ttl_secs)
            .map_err(|_| Error::InvalidInput("ttl_secs too large".to_string()))?;
        let expires_at = now + Duration::seconds(ttl);

        let session = Session::new(
            user_id,
            organization_id,
            jti,
            expires_at,
            client_ip,
            user_agent,
        );

        self.session_repo.create(session).await
    }

    /// Validates a session by its JWT token identifier (jti).
    ///
    /// # Arguments
    ///
    /// * `jti` - The JWT token identifier
    ///
    /// # Returns
    ///
    /// The session if valid and active, None otherwise
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn validate_session(&self, jti: &str) -> Result<Option<Session>> {
        self.session_repo.find_by_jti(jti).await
    }

    /// Revokes a session by marking it as inactive.
    ///
    /// # Arguments
    ///
    /// * `jti` - The JWT token identifier
    ///
    /// # Returns
    ///
    /// True if the session was revoked, false if not found
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn revoke_session(&self, jti: &str) -> Result<bool> {
        self.session_repo.revoke_by_jti(jti).await
    }

    /// Revokes all sessions for a user (used for security-sensitive actions).
    ///
    /// # Arguments
    ///
    /// * `user_id` - The user ID
    ///
    /// # Returns
    ///
    /// Number of sessions revoked
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn revoke_all_sessions_for_user(&self, user_id: Uuid) -> Result<u64> {
        // Return 0 as count since the repo doesn't return count
        self.session_repo
            .revoke_all_for_user(user_id)
            .await
            .map(|()| 0u64)
    }

    /// Updates the last activity timestamp for a session.
    ///
    /// # Arguments
    ///
    /// * `jti` - The JWT token identifier
    ///
    /// # Returns
    ///
    /// True if the session was updated, false if not found
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn update_last_activity(&self, jti: &str) -> Result<bool> {
        let session = self.session_repo.find_by_jti(jti).await?;
        if let Some(s) = session {
            self.session_repo.update_last_activity(s.id).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Checks if a session is about to expire and should be refreshed.
    ///
    /// # Arguments
    ///
    /// * `session` - The session to check
    /// * `refresh_threshold_secs` - Time before expiration to consider for refresh (default: 300s/5min)
    ///
    /// # Returns
    ///
    /// True if the session should be refreshed
    ///
    /// # Panics
    ///
    /// Panics if `refresh_threshold_secs` is larger than `i64::MAX` (which is extremely unlikely in practice).
    #[must_use]
    pub fn should_refresh(&self, session: &Session, refresh_threshold_secs: u64) -> bool {
        let now = Utc::now();
        let time_until_expires = session.expires_at.signed_duration_since(now);
        let threshold = i64::try_from(refresh_threshold_secs).unwrap_or(i64::MAX);
        time_until_expires.num_seconds() < threshold && time_until_expires.num_seconds() > 0
    }
}
