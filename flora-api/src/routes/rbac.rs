//! Role and permission management endpoints.
//!
//! Per T055: RBAC role assignment and permission management.
//! Cycle 042: `GET /roles` is backed by the real role repository.
//! Cycle 066: the remaining handlers are real — authority is `RoleWrite || OrgAdmin`
//! and built-in roles are immutable (see `.specify/memory/STATE.md`).

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use crate::extractors::auth::AuthExtractor;
use crate::state::AppState;
use flora_core::error::{Error, Result};
use flora_core::models::{Membership, Permission};
use flora_core::repositories::{PgMembershipRepository, PgRoleRepository};
use flora_core::traits::{MembershipRepository, RoleRepository};

/// Creates the RBAC (role-based access control) router.
pub fn create_rbac_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_roles).post(create_role))
        .route(
            "/{id}",
            get(get_role).patch(update_role).delete(delete_role),
        )
        .route("/{id}/assign", post(assign_role))
        .route("/{id}/revoke", post(revoke_role))
}

/// Request body for creating a custom role.
#[derive(Debug, Deserialize)]
pub struct CreateRoleRequest {
    /// Role name.
    pub name: String,
    /// Optional human-readable description.
    pub description: Option<String>,
    /// Permission strings in canonical `role:write` form.
    pub permissions: Vec<String>,
}

/// Request body for updating a custom role. All fields are optional.
#[derive(Debug, Deserialize)]
pub struct UpdateRoleRequest {
    /// New role name, when provided.
    pub name: Option<String>,
    /// New description, when provided.
    pub description: Option<String>,
    /// Replacement permission list, when provided.
    pub permissions: Option<Vec<String>>,
}

/// Request body for assigning or revoking a role on a user.
#[derive(Debug, Deserialize)]
pub struct UserRoleRequest {
    /// Target user identifier.
    pub user_id: Uuid,
}

/// Validates and converts permission strings from a request body.
fn parse_permissions(raw: &[String]) -> Result<Vec<Permission>> {
    raw.iter()
        .map(|value| {
            Permission::from_str(value)
                .map_err(|_| Error::InvalidInput(format!("Unknown permission: {value}")))
        })
        .collect()
}

/// Public view of a role — omits `organization_id` and timestamps.
#[derive(Debug, Serialize)]
pub struct RoleResponse {
    /// Role identifier.
    pub id: Uuid,
    /// Role name (e.g. "Owner", "Admin", "Member").
    pub name: String,
    /// Human-readable description, when present.
    pub description: Option<String>,
    /// JSONB array of permission strings.
    pub permissions: serde_json::Value,
    /// Whether this is a built-in role that cannot be deleted.
    pub is_builtin: bool,
}

impl From<flora_core::models::Role> for RoleResponse {
    fn from(role: flora_core::models::Role) -> Self {
        Self {
            id: role.id,
            name: role.name,
            description: role.description,
            permissions: role.permissions,
            is_builtin: role.is_builtin,
        }
    }
}

/// Pure authorization check for listing roles. Extracted so the decision is
/// verifiable without a database (the `cargo test --workspace` gate).
fn can_list_roles(role: &flora_core::models::Role) -> bool {
    role.has_permission(Permission::OrgRead) || role.has_permission(Permission::OrgAdmin)
}

/// Pure authorization check for mutating roles (create/update/delete/assign/revoke).
/// `RoleWrite` is the dedicated permission; `OrgAdmin` keeps it enabled by default
/// for the built-in Admin/Owner roles. `OrgRead` alone is not enough.
fn can_manage_roles(role: &flora_core::models::Role) -> bool {
    role.has_permission(Permission::RoleWrite) || role.has_permission(Permission::OrgAdmin)
}

/// Pure guard for modifying/deleting a role: built-ins are immutable by SE decision.
const fn can_modify_role(role: &flora_core::models::Role) -> bool {
    !role.is_builtin
}

/// Resolve the verified session organization and require an admin membership role.
///
/// C046/R1: these mutating endpoints were public no-ops echoing the id. Even while
/// their semantics are unimplemented, the surface must be authenticated and
/// tenant-scoped so a future implementation cannot inherit an unauthenticated
/// endpoint that accepts arbitrary `role_id`/`user_id` (OWASP A01:2021 / BOLA).
async fn require_org_admin(state: &AppState, auth: &AuthExtractor) -> Result<Uuid> {
    let org_id = auth
        .require_org()
        .map_err(|_| Error::OrganizationContextRequired)?;
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(auth.user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;
    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let role = role_repo
        .find_by_id_and_organization(membership.role_id, org_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?;
    if !can_manage_roles(&role) {
        return Err(Error::Forbidden("Insufficient permissions".to_string()));
    }
    Ok(org_id)
}

/// `GET /api/v1/roles` — List roles in the caller's organization.
///
/// Tenancy comes exclusively from the verified session (`auth.require_org()`),
/// never from a header or path. The caller must be an org member with
/// `OrgRead` or `OrgAdmin`; organizations with no roles return `[]`.
async fn list_roles(
    State(state): State<AppState>,
    auth: AuthExtractor,
) -> Result<Json<Vec<RoleResponse>>> {
    let org_id = auth
        .require_org()
        .map_err(|_| Error::OrganizationContextRequired)?;
    let user_id = auth.user_id;

    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let membership_role = role_repo
        .find_by_id_and_organization(membership.role_id, org_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?;
    if !can_list_roles(&membership_role) {
        return Err(Error::Forbidden("Insufficient permissions".to_string()));
    }

    let roles = role_repo.find_by_organization_id(org_id).await?;
    Ok(Json(roles.into_iter().map(RoleResponse::from).collect()))
}

/// `POST /api/v1/roles` — Create a custom role in the caller's organization.
async fn create_role(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<RoleResponse>)> {
    let org_id = require_org_admin(&state, &auth).await?;
    if req.name.trim().is_empty() {
        return Err(Error::InvalidInput("Role name is required".to_string()));
    }
    let permissions = parse_permissions(&req.permissions)?;
    let mut role = flora_core::models::Role::new(org_id, req.name.trim());
    for permission in permissions {
        role.add_permission(permission);
    }
    role.description = req.description;
    let repo = PgRoleRepository::new((*state.db_pool).clone());
    let created = repo.create(role).await?;
    Ok((StatusCode::CREATED, Json(RoleResponse::from(created))))
}

/// `GET /api/v1/roles/{id}` — Get a role in the caller's organization.
async fn get_role(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    auth: AuthExtractor,
) -> Result<Json<RoleResponse>> {
    let org_id = auth
        .require_org()
        .map_err(|_| Error::OrganizationContextRequired)?;
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(auth.user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;
    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let membership_role = role_repo
        .find_by_id_and_organization(membership.role_id, org_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?;
    if !can_list_roles(&membership_role) {
        return Err(Error::Forbidden("Insufficient permissions".to_string()));
    }
    let role = role_repo
        .find_by_id_and_organization(id, org_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(id.to_string()))?;
    Ok(Json(RoleResponse::from(role)))
}

/// `PATCH /api/v1/roles/{id}` — Update a custom role. Built-in roles are immutable.
async fn update_role(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<UpdateRoleRequest>,
) -> Result<Json<RoleResponse>> {
    let org_id = require_org_admin(&state, &auth).await?;
    let repo = PgRoleRepository::new((*state.db_pool).clone());
    let mut role = repo
        .find_by_id_and_organization(id, org_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(id.to_string()))?;
    if !can_modify_role(&role) {
        return Err(Error::Forbidden("Cannot modify built-in role".to_string()));
    }
    if let Some(name) = req.name {
        if name.trim().is_empty() {
            return Err(Error::InvalidInput("Role name is required".to_string()));
        }
        role.name = String::from(name.trim());
    }
    if let Some(description) = req.description {
        role.description = Some(description);
    }
    if let Some(raw) = req.permissions {
        role.permissions = serde_json::Value::Array(
            parse_permissions(&raw)?
                .into_iter()
                .map(|p| serde_json::Value::String(p.to_string()))
                .collect(),
        );
    }
    let updated = repo.update(id, role).await?;
    Ok(Json(RoleResponse::from(updated)))
}

/// `DELETE /api/v1/roles/{id}` — Delete a custom role. Built-in roles are immutable.
async fn delete_role(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    auth: AuthExtractor,
) -> Result<StatusCode> {
    let org_id = require_org_admin(&state, &auth).await?;
    let repo = PgRoleRepository::new((*state.db_pool).clone());
    let role = repo
        .find_by_id_and_organization(id, org_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(id.to_string()))?;
    if !can_modify_role(&role) {
        return Err(Error::Forbidden("Cannot delete built-in role".to_string()));
    }
    repo.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// `POST /api/v1/roles/{id}/assign` — Assign a role to a user in the caller's org.
async fn assign_role(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<UserRoleRequest>,
) -> Result<StatusCode> {
    let org_id = require_org_admin(&state, &auth).await?;
    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let _target_role = role_repo
        .find_by_id_and_organization(id, org_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(id.to_string()))?;

    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    if membership_repo
        .find_by_user_and_organization(req.user_id, org_id)
        .await?
        .is_some()
    {
        membership_repo.delete(req.user_id, org_id).await?;
    }
    let _membership = membership_repo
        .create(Membership::new(req.user_id, org_id, id))
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

/// `POST /api/v1/roles/{id}/revoke` — Remove the user's membership (and role) from the org.
async fn revoke_role(
    Path(_id): Path<Uuid>,
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<UserRoleRequest>,
) -> Result<StatusCode> {
    let org_id = require_org_admin(&state, &auth).await?;
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    membership_repo.delete(req.user_id, org_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flora_core::models::Role;

    fn role_with(perms: &[&str]) -> Role {
        let mut role = Role::new(Uuid::now_v7(), "Test");
        role.permissions = serde_json::json!(perms);
        role
    }

    #[test]
    fn can_list_roles_accepts_org_read_and_org_admin() {
        assert!(can_list_roles(&role_with(&["org:read"])));
        assert!(can_list_roles(&role_with(&["org:admin"])));
        assert!(can_list_roles(&role_with(&["*"])));
    }

    #[test]
    fn can_list_roles_rejects_unrelated_and_empty_permissions() {
        assert!(!can_list_roles(&role_with(&[])));
        assert!(!can_list_roles(&role_with(&["task:write"])));
        assert!(!can_list_roles(&role_with(&["workspace:read"])));
    }

    #[test]
    fn can_manage_roles_accepts_role_write_or_org_admin() {
        assert!(can_manage_roles(&role_with(&["role:write"])));
        assert!(can_manage_roles(&role_with(&["org:admin"])));
        assert!(can_manage_roles(&role_with(&["*"])));
        assert!(!can_manage_roles(&role_with(&["org:read"])));
        assert!(!can_manage_roles(&role_with(&["workspace:admin"])));
        assert!(!can_manage_roles(&role_with(&[])));
    }

    #[test]
    fn can_modify_role_rejects_builtins() {
        let mut builtin = role_with(&["role:write"]);
        builtin.is_builtin = true;
        assert!(!can_modify_role(&builtin));
        assert!(can_modify_role(&role_with(&["role:write"])));
    }

    #[test]
    fn role_response_omits_organization_id_and_timestamps() {
        let role = role_with(&["org:read"]);
        let resp = RoleResponse::from(role);
        let serialized = serde_json::to_value(&resp);
        let leaked = serialized.ok().and_then(|v| {
            v.as_object().map(|obj| {
                (
                    obj.contains_key("organization_id"),
                    obj.contains_key("created_at"),
                    obj.contains_key("updated_at"),
                    obj.contains_key("id"),
                    obj.contains_key("permissions"),
                )
            })
        });
        assert_eq!(
            leaked,
            Some((false, false, false, true, true)),
            "RoleResponse must omit tenant/timestamps and keep id/permissions"
        );
    }
}
