//! Session model — tracks authenticated user sessions.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A user session in Flora.
///
/// Sessions are short-lived and tied to a specific organization context.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    /// The organization this session is scoped to.
    pub organization_id: Uuid,
    /// JWT token identifier (jti claim).
    pub jti: String,
    /// Optional refresh token identifier.
    pub refresh_token_id: Option<String>,
    /// IP address of the client that created the session.
    pub client_ip: Option<String>,
    /// User agent of the client that created the session.
    pub user_agent: Option<String>,
    /// When the session expires.
    pub expires_at: DateTime<Utc>,
    /// Last activity timestamp.
    pub last_activity_at: DateTime<Utc>,
    /// Whether the session is active (not revoked).
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    /// Creates a new session.
    #[must_use]
    pub fn new(
        user_id: Uuid,
        organization_id: Uuid,
        jti: String,
        expires_at: DateTime<Utc>,
        client_ip: Option<String>,
        user_agent: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            user_id,
            organization_id,
            jti,
            refresh_token_id: None,
            client_ip,
            user_agent,
            expires_at,
            last_activity_at: now,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}
