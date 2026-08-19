//! Organization management routes.
//!
//! Per T024: Organization CRUD with RBAC.

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::models::{Membership, Organization, Role};
use flora_core::repositories::{
    PgMembershipRepository, PgOrganizationRepository, PgRoleRepository,
};
use flora_core::traits::{MembershipRepository, OrganizationRepository, RoleRepository};

use super::AppState;

/// Request body for creating an organization.
#[derive(Debug, Deserialize)]
pub struct CreateOrgRequest {
    /// The organization name.
    pub name: String,
    /// The organization slug (URL-friendly identifier).
    pub slug: String,
    /// Optional settings for the organization.
    pub settings: Option<serde_json::Value>,
}

/// Response body for an organization.
#[derive(Debug, Serialize)]
pub struct OrgResponse {
    /// The organization ID.
    pub id: String,
    /// The organization name.
    pub name: String,
    /// The organization slug.
    pub slug: String,
    /// The organization settings.
    pub settings: serde_json::Value,
    /// The creation timestamp.
    pub created_at: String,
}

impl From<Organization> for OrgResponse {
    fn from(org: Organization) -> Self {
        Self {
            id: org.id.to_string(),
            name: org.name,
            slug: org.slug,
            settings: org.settings,
            created_at: org.created_at.to_rfc3339(),
        }
    }
}

/// Response body for listing organizations the user belongs to.
#[derive(Debug, Serialize)]
pub struct OrgListItem {
    /// The organization ID.
    pub id: String,
    /// The organization name.
    pub name: String,
    /// The organization slug.
    pub slug: String,
    /// The user's role in the organization.
    pub role: String,
    /// The organization settings.
    pub settings: serde_json::Value,
}

/// Creates the organization management router.
pub fn create_org_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_user_orgs))
        .route("/", post(create_org))
        .route("/{id}", get(get_org))
}

/// `GET /api/orgs` — List organizations the authenticated user belongs to.
async fn list_user_orgs(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<OrgListItem>>> {
    let user_id = extract_user_id_from_header(&headers, &state.config.app.jwt_secret)?;

    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
    let role_repo = PgRoleRepository::new((*state.db_pool).clone());

    let memberships = membership_repo.find_by_user_id(user_id).await?;

    let mut orgs = Vec::new();
    for membership in memberships {
        if let Some(org) = org_repo.find_by_id(membership.organization_id).await? {
            let role_name = role_repo
                .find_by_id(membership.role_id)
                .await?
                .map_or_else(|| "Member".to_string(), |r| r.name);

            orgs.push(OrgListItem {
                id: org.id.to_string(),
                name: org.name,
                slug: org.slug,
                role: role_name,
                settings: org.settings,
            });
        }
    }

    Ok(Json(orgs))
}

/// `POST /api/orgs` — Create a new organization.
///
/// The authenticated user is automatically added as the owner.
async fn create_org(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<CreateOrgRequest>,
) -> Result<(StatusCode, Json<OrgResponse>)> {
    let user_id = extract_user_id_from_header(&headers, &state.config.app.jwt_secret)?;

    // Create organization
    let org = Organization::new(&req.name, &req.slug);
    let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
    let created_org = org_repo.create(org).await?;

    // Create default Owner role
    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let owner_role = Role {
        id: Uuid::now_v7(),
        organization_id: created_org.id,
        name: "Owner".to_string(),
        permissions: serde_json::json!(["*"]),
        description: Some("Full access to all resources".to_string()),
        is_builtin: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _created_role = role_repo.create(owner_role.clone()).await?;

    // Create Member role (for future members)
    let member_role = Role {
        id: Uuid::now_v7(),
        organization_id: created_org.id,
        name: "Member".to_string(),
        permissions: serde_json::json!(["read", "write"]),
        description: Some("Read and write access to resources".to_string()),
        is_builtin: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    let _created_member_role = role_repo.create(member_role).await?;

    // Add creator as Owner
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = Membership {
        user_id,
        organization_id: created_org.id,
        role_id: owner_role.id,
        joined_at: chrono::Utc::now(),
        metadata: serde_json::json!({}),
    };
    let _created_membership = membership_repo.create(membership).await?;

    tracing::info!(org_id = %created_org.id, user_id = %user_id, "Organization created");
    Ok((StatusCode::CREATED, Json(OrgResponse::from(created_org))))
}

/// `GET /api/orgs/{id}` — Get a specific organization.
async fn get_org(
    State(state): State<AppState>,
    Path(org_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
) -> Result<Json<OrgResponse>> {
    let user_id = extract_user_id_from_header(&headers, &state.config.app.jwt_secret)?;

    // Verify user has membership in the org
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?;
    if membership.is_none() {
        return Err(Error::Forbidden(
            "You are not a member of this organization".to_string(),
        ));
    }

    let org_repo = PgOrganizationRepository::new((*state.db_pool).clone());
    let org = org_repo
        .find_by_id(org_id)
        .await?
        .ok_or_else(|| Error::OrganizationNotFound(org_id.to_string()))?;

    Ok(Json(OrgResponse::from(org)))
}

/// Extracts the user ID from the Authorization: Bearer <token> header.
fn extract_user_id_from_header(headers: &axum::http::HeaderMap, jwt_secret: &str) -> Result<Uuid> {
    let auth_header = headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .ok_or(Error::Unauthorized)?;

    let claims = flora_core::utils::jwt::decode_token(auth_header, jwt_secret)
        .map_err(|_| Error::Unauthorized)?;

    Ok(claims.user_id())
}
