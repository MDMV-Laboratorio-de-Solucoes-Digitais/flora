//! Session management service.
use async_trait::async_trait;
use chrono::{Duration, Utc};
use flora_core::{
    error::{Error, Result},
    models::{Session, UpdateSessionInput},
    repositories::{PgSessionRepository, SessionRepository},
    traits::SessionRepository,
};
use uuid::Uuid;

/// Service for managing user sessions.
#[derive(Debug)]
pub struct SessionService {
    session_repo: PgSessionRepository,
}

impl SessionService {
    /// Creates a new `SessionService`.
    #[must_use]
    pub const fn new(pool: flora_core::PgPool) -> Self {
        Self {
            session_repo: PgSessionRepository::new(pool),
        }
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
        let expires_at = now + Duration::seconds(ttl_secs as i64);

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
    pub async fn revoke_session(&self, jti: &str) -> Result<bool> {
        let session = self.session_repo.find_by_jti(jti).await?;
        if let Some(mut session) = session {
            session.is_active = false;
            session.updated_at = Utc::now();
            self.session_repo
                .update(
                    session.id,
                    UpdateSessionInput {
                        is_active: Some(false),
                        updated_at: Some(session.updated_at),
                    },
                )
                .await?;
            Ok(true)
        } else {
            Ok(false)
        }
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
    pub async fn revoke_all_sessions_for_user(&self, user_id: Uuid) -> Result<u64> {
        self.session_repo.revoke_all_for_user(user_id).await
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
    pub async fn update_last_activity(&self, jti: &str) -> Result<bool> {
        let session = self.session_repo.find_by_jti(jti).await?;
        if let Some(mut session) = session {
            session.last_activity_at = Utc::now();
            session.updated_at = Utc::now();
            self.session_repo
                .update(
                    session.id,
                    UpdateSessionInput {
                        last_activity_at: Some(session.last_activity_at),
                        updated_at: Some(session.updated_at),
                    },
                )
                .await?;
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
    #[must_use]
    pub fn should_refresh(&self, session: &Session, refresh_threshold_secs: u64) -> bool {
        let now = Utc::now();
        let time_until_expires = session.expires_at.signed_duration_since(now);
        time_until_expires.num_seconds() < refresh_threshold_secs as i64
            && time_until_expires.num_seconds() > 0
    }
}

#[async_trait]
impl super::traits::SessionService for SessionService {}
