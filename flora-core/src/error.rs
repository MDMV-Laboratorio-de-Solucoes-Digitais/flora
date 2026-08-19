//! Central error types for Flora Workspace.

use thiserror::Error;

/// Root error type for Flora operations.
#[derive(Debug, Error)]
pub enum Error {
    // --- Authentication & Authorization ---
    #[error("authentication required")]
    Unauthorized,

    #[error("insufficient permissions: {0}")]
    Forbidden(String),

    #[error("invalid credentials")]
    InvalidCredentials,

    #[error("session expired")]
    SessionExpired,

    #[error("OIDC provider error: {0}")]
    OidcProvider(String),

    // --- Resource Not Found ---
    #[error("user not found: {0}")]
    UserNotFound(String),

    #[error("organization not found: {0}")]
    OrganizationNotFound(String),

    #[error("workspace not found: {0}")]
    WorkspaceNotFound(String),

    #[error("channel not found: {0}")]
    ChannelNotFound(String),

    #[error("message not found: {0}")]
    MessageNotFound(String),

    #[error("task not found: {0}")]
    TaskNotFound(String),

    #[error("file not found: {0}")]
    FileNotFound(String),

    #[error("notification not found: {0}")]
    NotificationNotFound(String),

    #[error("role not found: {0}")]
    RoleNotFound(String),

    #[error("membership not found")]
    MembershipNotFound,

    // --- Validation ---
    #[error("validation error: {field} — {message}")]
    Validation { field: String, message: String },

    #[error("duplicate entry: {0}")]
    DuplicateEntry(String),

    #[error("invalid input: {0}")]
    InvalidInput(String),

    // --- Storage ---
    #[error("file too large: {size} bytes (max: {max} bytes)")]
    FileTooLarge { size: u64, max: u64 },

    #[error("file type not allowed: {0}")]
    FileTypeNotAllowed(String),

    #[error("storage error: {0}")]
    Storage(String),

    // --- Database ---
    #[error("database error: {0}")]
    Database(String),

    #[error("migration error: {0}")]
    Migration(String),

    // --- Search ---
    #[error("search error: {0}")]
    Search(String),

    #[error("search index not found: {0}")]
    SearchIndexNotFound(String),

    // --- Messaging ---
    #[error("messaging error: {0}")]
    Messaging(String),

    #[error("channel not accessible: {0}")]
    ChannelNotAccessible(String),

    // --- Notifications ---
    #[error("notification dispatch error: {0}")]
    NotificationDispatch(String),

    // --- Multi-tenancy ---
    #[error("organization context required")]
    OrganizationContextRequired,

    #[error("workspace context required")]
    WorkspaceContextRequired,

    #[error("cross-organization access forbidden")]
    CrossOrganizationAccess,

    // --- Quota ---
    #[error("quota exceeded: {resource} ({used}/{limit})")]
    QuotaExceeded {
        resource: String,
        used: u64,
        limit: u64,
    },

    // --- Rate Limiting ---
    #[error("rate limit exceeded")]
    RateLimitExceeded,

    // --- Configuration ---
    #[error("configuration error: {0}")]
    Configuration(String),

    // --- External Services ---
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("circuit breaker open: {0}")]
    CircuitBreakerOpen(String),

    // --- Internal ---
    #[error("internal error: {0}")]
    Internal(String),
}

/// Result alias for Flora operations.
pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    /// Returns `true` if the error indicates a client mistake (4xx).
    #[must_use]
    pub fn is_client_error(&self) -> bool {
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
    pub fn is_not_found(&self) -> bool {
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
