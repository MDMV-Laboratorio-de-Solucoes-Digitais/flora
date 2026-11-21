//! One-time bootstrap route — creates the first organization without an identity provider.
//!
//! `POST /bootstrap` is public and may succeed only once per installation.
//! The guard is a PostgreSQL advisory transaction lock plus a count of
//! non-system organizations; both run inside a single transaction together
//! with every insert, so concurrent or repeated calls cannot create partial
//! state.

use axum::{
    Router,
    extract::State,
    http::{HeaderMap, StatusCode, header},
    response::Json,
    routing::{get, post},
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Executor, Postgres, Transaction};
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::models::{Membership, Organization, Role, Session, User, Workspace};
use flora_core::services::PasswordService;
use flora_core::utils::jwt::{FloraClaims, encode_token};

use super::AppState;

/// Fixed key for the advisory transaction lock guarding one-time bootstrap.
const BOOTSTRAP_LOCK_KEY: i64 = 8_531_971_270_216_327_161;

/// Payload of `POST /bootstrap`.
#[derive(Debug, Deserialize)]
pub struct BootstrapRequest {
    /// Display name of the first organization.
    pub organization_name: String,
    /// E-mail of the initial administrator (normalized to lowercase).
    pub admin_email: String,
    /// Password for the initial administrator (15..1024 characters).
    pub admin_password: String,
}

/// Public, read-only signal consumed by the login screen.
#[derive(Debug, Serialize)]
pub struct BootstrapStatusResponse {
    /// `true` only while no non-system organization exists.
    pub available: bool,
}

/// Successful response of `POST /bootstrap`.
#[derive(Debug, Serialize)]
pub struct BootstrapResponse {
    /// JWT carrying the already organization-scoped session.
    pub session_token: String,
    /// Token lifetime in seconds.
    pub expires_in: u64,
    /// ID of the initial administrator.
    pub user_id: String,
    /// ID of the newly created organization.
    pub organization_id: String,
    /// ID of the initial workspace.
    pub workspace_id: String,
}

/// Creates the `/bootstrap` sub-router (mounted at `/api/v1/bootstrap`).
pub fn create_bootstrap_router() -> Router<AppState> {
    Router::new()
        .route("/bootstrap", post(bootstrap))
        .route("/bootstrap/status", get(bootstrap_status))
}

/// `GET /bootstrap/status` — public UX signal, never an authorization decision.
///
/// The authoritative guard remains the advisory transaction lock plus the
/// atomic count inside `POST /bootstrap`. `Cache-Control: no-store` prevents
/// proxies and browsers from serving a stale answer; the body intentionally
/// carries only the boolean (no counts, e-mails or other installation data).
async fn bootstrap_status(
    State(state): State<AppState>,
) -> Result<(HeaderMap, Json<BootstrapStatusResponse>)> {
    let available = installation_is_empty(state.db_pool.as_ref()).await?;
    let mut headers = HeaderMap::new();
    let _ = headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store"),
    );
    Ok((headers, Json(BootstrapStatusResponse { available })))
}

/// Validates the payload and returns the normalized name, slug and e-mail.
fn validate_request(payload: &BootstrapRequest) -> Result<(String, String, String)> {
    let name = payload.organization_name.trim();
    if name.is_empty() || name.chars().count() > 255 {
        return Err(Error::Validation {
            field: "organization_name".to_owned(),
            message: "organization_name must be 1-255 characters".to_owned(),
        });
    }

    let email = payload.admin_email.trim().to_lowercase();
    let email_valid = email.len() <= 255
        && !email.chars().any(char::is_whitespace)
        && email.split_once('@').is_some_and(|(local, domain)| {
            !local.is_empty()
                && !local.starts_with('.')
                && !local.ends_with('.')
                && !local.contains("..")
                && !domain.is_empty()
                && domain.contains('.')
                && !domain.starts_with('.')
                && !domain.ends_with('.')
                && domain.split('.').all(|label| !label.is_empty())
                && !domain.contains('@')
        });
    if !email_valid {
        return Err(Error::Validation {
            field: "admin_email".to_owned(),
            message: "admin_email must be a valid e-mail address".to_owned(),
        });
    }

    let mut slug = String::with_capacity(name.len());
    for ch in name.to_lowercase().chars() {
        if ch.is_alphanumeric() {
            slug.push(ch);
        } else if !slug.is_empty() && !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let trimmed_len = slug.trim_end_matches('-').len();
    slug.truncate(trimmed_len);
    if slug.is_empty() {
        slug = format!("org-{}", Uuid::now_v7().simple());
    }

    Ok((name.to_owned(), slug, email))
}

/// Extracts the client IP from proxy headers, when present and parseable.
fn extract_client_ip(headers: &HeaderMap) -> Option<String> {
    headers
        .get("x-forwarded-for")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .or_else(|| {
            headers
                .get("x-real-ip")
                .and_then(|value| value.to_str().ok())
        })
        .and_then(|value| value.parse::<std::net::IpAddr>().ok())
        .map(|ip| ip.to_string())
}

/// Inserts the organization and its initial identity objects.
async fn insert_tenant(
    tx: &mut Transaction<'_, Postgres>,
    org: &Organization,
    user: &User,
    password_hash: &str,
    role: &Role,
    membership: &Membership,
    workspace: &Workspace,
) -> Result<()> {
    let _ = sqlx::query(
        "INSERT INTO organizations (id, name, slug, settings, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(org.id)
    .bind(&org.name)
    .bind(&org.slug)
    .bind(&org.settings)
    .bind(org.created_at)
    .bind(org.updated_at)
    .execute(&mut **tx)
    .await?;

    let _ = sqlx::query(
        "INSERT INTO users (id, email, oidc_subject, password_hash, password_updated_at,
                            display_name, avatar_url, profile, is_active, created_at, updated_at)
         VALUES ($1, $2, $3, $4, NOW(), $5, $6, $7, $8, $9, $10)",
    )
    .bind(user.id)
    .bind(&user.email)
    .bind(&user.oidc_subject)
    .bind(password_hash)
    .bind(&user.display_name)
    .bind(&user.avatar_url)
    .bind(&user.profile)
    .bind(user.is_active)
    .bind(user.created_at)
    .bind(user.updated_at)
    .execute(&mut **tx)
    .await?;

    let _ = sqlx::query(
        "INSERT INTO roles (id, organization_id, name, permissions, description,
                            is_builtin, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(role.id)
    .bind(role.organization_id)
    .bind(&role.name)
    .bind(&role.permissions)
    .bind(&role.description)
    .bind(role.is_builtin)
    .bind(role.created_at)
    .bind(role.updated_at)
    .execute(&mut **tx)
    .await?;

    let _ = sqlx::query(
        "INSERT INTO memberships (user_id, organization_id, role_id, joined_at, metadata)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(membership.user_id)
    .bind(membership.organization_id)
    .bind(membership.role_id)
    .bind(membership.joined_at)
    .bind(&membership.metadata)
    .execute(&mut **tx)
    .await?;

    let _ = sqlx::query(
        "INSERT INTO workspaces (id, organization_id, name, description, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(workspace.id)
    .bind(workspace.organization_id)
    .bind(&workspace.name)
    .bind(&workspace.description)
    .bind(workspace.created_at)
    .bind(workspace.updated_at)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Inserts the active session row inside the bootstrap transaction.
async fn insert_session(tx: &mut Transaction<'_, Postgres>, session: &Session) -> Result<()> {
    let _ = sqlx::query(
        "INSERT INTO sessions (id, user_id, organization_id, jti, refresh_token_id,
                               client_ip, user_agent, expires_at, last_activity_at,
                               is_active, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
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
    .bind(session.created_at)
    .bind(session.updated_at)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// Checks the same criterion as the authoritative `POST /bootstrap` guard:
/// zero organizations other than the built-in system organization. Orphan
/// users/workspaces do not change the answer. Callers decide whether the
/// query must run inside a transaction under the advisory lock.
async fn installation_is_empty<'e, E>(db: E) -> Result<bool>
where
    E: Executor<'e, Database = Postgres>,
{
    let org_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM organizations \
         WHERE id <> '00000000-0000-0000-0000-000000000000'",
    )
    .fetch_one(db)
    .await?;
    Ok(org_count == 0)
}

/// `POST /bootstrap` — initializes an empty installation atomically.
///
/// # Errors
///
/// Returns 422 for invalid input, 409 when the installation is already
/// initialized (or the e-mail/slug is taken), and 500 on database failure.
async fn bootstrap(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<BootstrapRequest>,
) -> Result<(StatusCode, Json<BootstrapResponse>)> {
    let (name, slug, email) = validate_request(&payload)?;
    PasswordService::validate_password(&payload.admin_password, "admin_password")?;
    let password_hash = PasswordService::shared().hash(&payload.admin_password)?;
    let ttl = state.config.app.session_ttl_secs;
    let jti = Uuid::now_v7().to_string();
    let expires_at = Utc::now() + Duration::seconds(i64::try_from(ttl).unwrap_or(3_600));

    let org = Organization::new(&name, &slug);
    let user = User::new(&email, "Admin");
    let mut role = Role::new(org.id, "Admin");
    role.permissions = serde_json::json!(["*"]);
    role.is_builtin = true;
    let membership = Membership::new(user.id, org.id, role.id);
    let workspace = Workspace::new(org.id, "Main Workspace", None);
    let session = Session::new(
        user.id,
        org.id,
        jti.clone(),
        expires_at,
        extract_client_ip(&headers),
        headers
            .get("user-agent")
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned),
    );

    let mut tx = state.db_pool.begin().await?;
    let _ = sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(BOOTSTRAP_LOCK_KEY)
        .execute(&mut *tx)
        .await?;
    if !installation_is_empty(&mut *tx).await? {
        return Err(Error::DuplicateEntry(
            "bootstrap already completed".to_owned(),
        ));
    }

    insert_tenant(
        &mut tx,
        &org,
        &user,
        &password_hash,
        &role,
        &membership,
        &workspace,
    )
    .await?;
    insert_session(&mut tx, &session).await?;
    tx.commit().await?;

    let claims = FloraClaims::new(user.id, jti, org.id, &email, ttl);
    let token = encode_token(&claims, &state.config.app.jwt_secret);

    tracing::info!(
        organization_id = %org.id,
        user_id = %user.id,
        "Bootstrap completed: first organization and administrator created"
    );

    Ok((
        StatusCode::CREATED,
        Json(BootstrapResponse {
            session_token: token,
            expires_in: ttl,
            user_id: user.id.to_string(),
            organization_id: org.id.to_string(),
            workspace_id: workspace.id.to_string(),
        }),
    ))
}
