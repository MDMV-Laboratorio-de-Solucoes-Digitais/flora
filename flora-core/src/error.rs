//! Central error types for Flora Workspace.

use axum::response::{IntoResponse, Json, Response};
use thiserror::Error;

/// Root error type for Flora operations.
#[derive(Debug, Error)]
pub enum Error {
    // --- Authentication & Authorization ---
    /// Authentication credentials are required to access this resource.
    #[error("authentication required")]
    Unauthorized,

    /// The authenticated user lacks permission for this operation.
    #[error("insufficient permissions: {0}")]
    Forbidden(String),

    /// The provided credentials are invalid.
    #[error("invalid credentials")]
    InvalidCredentials,

    /// The session has expired and the user must re-authenticate.
    #[error("session expired")]
    SessionExpired,

    /// An error occurred with the OIDC identity provider.
    #[error("OIDC provider error: {0}")]
    OidcProvider(String),

    // --- Resource Not Found ---
    /// The requested user does not exist.
    #[error("user not found: {0}")]
    UserNotFound(String),

    /// The requested organization does not exist.
    #[error("organization not found: {0}")]
    OrganizationNotFound(String),

    /// The requested workspace does not exist.
    #[error("workspace not found: {0}")]
    WorkspaceNotFound(String),

    /// The requested channel does not exist.
    #[error("channel not found: {0}")]
    ChannelNotFound(String),

    /// The requested message does not exist.
    #[error("message not found: {0}")]
    MessageNotFound(String),

    /// The requested task does not exist.
    #[error("task not found: {0}")]
    TaskNotFound(String),

    /// The requested file does not exist.
    #[error("file not found: {0}")]
    FileNotFound(String),

    /// The requested notification does not exist.
    #[error("notification not found: {0}")]
    NotificationNotFound(String),

    /// The requested role does not exist.
    #[error("role not found: {0}")]
    RoleNotFound(String),

    /// The membership record was not found.
    #[error("membership not found")]
    MembershipNotFound,

    // --- Validation ---
    /// A validation error occurred.
    #[error("validation error: {field} — {message}")]
    Validation {
        /// The name of the field that failed validation.
        field: String,
        /// The validation error message.
        message: String,
    },

    /// A duplicate entry already exists.
    #[error("duplicate entry: {0}")]
    DuplicateEntry(String),

    /// The provided input is invalid.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    // --- Storage ---
    /// The file exceeds the maximum allowed size.
    #[error("file too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge {
        /// The actual size of the file in bytes.
        size: u64,
        /// The maximum allowed size in bytes.
        max: u64,
    },

    /// The file type is not allowed.
    #[error("file type not allowed: {0}")]
    FileTypeNotAllowed(String),

    /// A storage backend error occurred.
    #[error("storage error: {0}")]
    Storage(String),

    // --- Database ---
    /// A database error occurred.
    #[error("database error: {0}")]
    Database(String),

    /// A migration error occurred.
    #[error("migration error: {0}")]
    Migration(String),

    // --- Search ---
    /// A search engine error occurred.
    #[error("search error: {0}")]
    Search(String),

    /// The search index does not exist.
    #[error("search index not found: {0}")]
    SearchIndexNotFound(String),

    // --- Messaging ---
    /// A messaging service error occurred.
    #[error("messaging error: {0}")]
    Messaging(String),

    /// The channel is not accessible (e.g., private channel the user is not a member of).
    #[error("channel not accessible: {0}")]
    ChannelNotAccessible(String),

    // --- Notifications ---
    /// A notification dispatch error occurred.
    #[error("notification dispatch error: {0}")]
    NotificationDispatch(String),

    // --- Multi-tenancy ---
    /// An organization context is required but none was provided.
    #[error("organization context required")]
    OrganizationContextRequired,

    /// A workspace context is required but none was provided.
    #[error("workspace context required")]
    WorkspaceContextRequired,

    /// Access across organizations is not permitted.
    #[error("cross-organization access forbidden")]
    CrossOrganizationAccess,

    // --- Quota ---
    /// A resource quota has been exceeded.
    #[error("quota exceeded: {resource} ({used}/{limit})")]
    QuotaExceeded {
        /// The resource type that exceeded its quota.
        resource: String,
        /// The current usage count.
        used: u64,
        /// The quota limit.
        limit: u64,
    },

    // --- Rate Limiting ---
    /// Too many requests have been made; rate limit exceeded.
    #[error("rate limit exceeded")]
    RateLimitExceeded,

    // --- Configuration ---
    /// A configuration value is missing or invalid.
    #[error("configuration error: {0}")]
    Configuration(String),

    // --- External Services ---
    /// An external service is unavailable.
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),

    /// A circuit breaker for a service is open (fail-fast mode).
    #[error("circuit breaker open: {0}")]
    CircuitBreakerOpen(String),

    /// An I/O error occurred.
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    // --- Internal ---
    /// An unexpected internal error occurred.
    #[error("internal error: {0}")]
    Internal(String),
}

/// Result alias for Flora operations.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Returns `true` if the error indicates a client mistake (4xx).
    #[must_use]
    pub const fn is_client_error(&self) -> bool {
        matches!(
            self,
            Self::Unauthorized
                | Self::Forbidden(_)
                | Self::InvalidCredentials
                | Self::Validation { .. }
                | Self::DuplicateEntry(_)
                | Self::InvalidInput(_)
                | Self::FileTooLarge { .. }
                | Self::FileTypeNotAllowed(_)
                | Self::OrganizationContextRequired
                | Self::WorkspaceContextRequired
                | Self::CrossOrganizationAccess
                | Self::QuotaExceeded { .. }
                | Self::RateLimitExceeded
        )
    }

    /// Returns `true` if the error indicates a not-found (404).
    #[must_use]
    pub const fn is_not_found(&self) -> bool {
        matches!(
            self,
            Self::UserNotFound(_)
                | Self::OrganizationNotFound(_)
                | Self::WorkspaceNotFound(_)
                | Self::ChannelNotFound(_)
                | Self::MessageNotFound(_)
                | Self::TaskNotFound(_)
                | Self::FileNotFound(_)
                | Self::NotificationNotFound(_)
                | Self::RoleNotFound(_)
                | Self::MembershipNotFound
        )
    }

    /// Maps sqlx errors to Flora errors.
    pub fn from_sqlx(err: sqlx::Error) -> Self {
        use sqlx::Error;
        match err {
            Error::RowNotFound => Self::Database("record not found".to_owned()),
            Error::Database(database_error) => {
                let postgresqlreturn = database_error
                    .code()
                    .map_or_else(String::new, |c| c.to_string());
                if postgresqlreturn == "23505" {
                    Self::DuplicateEntry(database_error.message().to_owned())
                } else if postgresqlreturn == "23503" {
                    Self::Database(format!(
                        "foreign key violation: {}",
                        database_error.message()
                    ))
                } else {
                    Self::Database(database_error.message().to_owned())
                }
            }
            _ => Self::Database(err.to_string()),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        Self::from_sqlx(err)
    }
}

impl From<validator::ValidationErrors> for Error {
    fn from(err: validator::ValidationErrors) -> Self {
        let first = err.field_errors().iter().next().map(|(field, errs)| {
            (
                field.to_string(),
                errs[0].message.as_ref().map_or("invalid", |m| m),
            )
        });
        match first {
            Some((field, msg)) => Self::Validation {
                field,
                message: msg.to_string(),
            },
            None => Self::Validation {
                field: "unknown".to_owned(),
                message: "validation failed".to_owned(),
            },
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (_status, body) = match &self {
            Self::Unauthorized => (
                axum::http::StatusCode::UNAUTHORIZED,
                serde_json::json!({ "error": "unauthorized", "message": "Authentication credentials are required" }),
            ),
            Self::Forbidden(_) => (
                axum::http::StatusCode::FORBIDDEN,
                serde_json::json!({ "error": "forbidden", "message": "Insufficient permissions" }),
            ),
            Self::InvalidCredentials => (
                axum::http::StatusCode::UNAUTHORIZED,
                serde_json::json!({ "error": "invalid_credentials", "message": "Invalid credentials" }),
            ),
            Self::SessionExpired => (
                axum::http::StatusCode::UNAUTHORIZED,
                serde_json::json!({ "error": "session_expired", "message": "Session has expired" }),
            ),
            Self::OidcProvider(_) => (
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                serde_json::json!({ "error": "oidc_error", "message": self.to_string() }),
            ),
            Self::UserNotFound(_)
            | Self::OrganizationNotFound(_)
            | Self::WorkspaceNotFound(_)
            | Self::ChannelNotFound(_)
            | Self::MessageNotFound(_)
            | Self::TaskNotFound(_)
            | Self::FileNotFound(_)
            | Self::NotificationNotFound(_)
            | Self::RoleNotFound(_)
            | Self::MembershipNotFound => (
                axum::http::StatusCode::NOT_FOUND,
                serde_json::json!({ "error": "not_found", "message": self.to_string() }),
            ),
            Self::Validation { field, message } => (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                serde_json::json!({ "error": "validation_error", "field": field, "message": message }),
            ),
            Self::DuplicateEntry(_) | Self::InvalidInput(_) => (
                axum::http::StatusCode::CONFLICT,
                serde_json::json!({ "error": "conflict", "message": self.to_string() }),
            ),
            Self::FileTooLarge { size, max } => (
                axum::http::StatusCode::PAYLOAD_TOO_LARGE,
                serde_json::json!({ "error": "file_too_large", "size": size, "max": max }),
            ),
            Self::FileTypeNotAllowed(_) => (
                axum::http::StatusCode::UNSUPPORTED_MEDIA_TYPE,
                serde_json::json!({ "error": "file_type_not_allowed", "message": self.to_string() }),
            ),
            Self::ChannelNotAccessible(_) => (
                axum::http::StatusCode::FORBIDDEN,
                serde_json::json!({ "error": "channel_not_accessible", "message": self.to_string() }),
            ),
            Self::OrganizationContextRequired | Self::WorkspaceContextRequired => (
                axum::http::StatusCode::BAD_REQUEST,
                serde_json::json!({ "error": "missing_context", "message": self.to_string() }),
            ),
            Self::CrossOrganizationAccess => (
                axum::http::StatusCode::FORBIDDEN,
                serde_json::json!({ "error": "cross_organization_access", "message": self.to_string() }),
            ),
            Self::QuotaExceeded { .. } | Self::RateLimitExceeded => (
                axum::http::StatusCode::TOO_MANY_REQUESTS,
                serde_json::json!({ "error": "rate_limited", "message": self.to_string() }),
            ),
            Self::Database(_)
            | Self::Migration(_)
            | Self::Storage(_)
            | Self::Search(_)
            | Self::SearchIndexNotFound(_)
            | Self::Messaging(_)
            | Self::NotificationDispatch(_)
            | Self::Configuration(_)
            | Self::ServiceUnavailable(_)
            | Self::CircuitBreakerOpen(_)
            | Self::IoError(_)
            | Self::Internal(_) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                serde_json::json!({ "error": "internal", "message": self.to_string() }),
            ),
        };
        Json(body).into_response()
    }
}
