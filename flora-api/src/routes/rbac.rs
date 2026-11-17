//! Role and permission management endpoints.
//!
//! Per T055: RBAC role assignment and permission management.
//! Cycle 042: `GET /roles` is backed by the real role repository; the remaining
//! handlers stay placeholders until their semantics are decided by the owner.

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{get, post},
};
use serde::Serialize;
use uuid::Uuid;

use crate::extractors::auth::AuthExtractor;
use crate::state::AppState;
use flora_core::error::{Error, Result};
use flora_core::models::Permission;
use flora_core::repositories::{PgMembershipRepository, PgRoleRepository};
use flora_core::traits::{MembershipRepository, RoleRepository};

/// Creates the RBAC (role-based access control) router.
pub fn create_rbac_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_roles).post(create_role))
        .route("/{id}", get(get_role))
        .route("/{id}/assign", post(assign_role))
        .route("/{id}/revoke", post(revoke_role))
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

/// `GET /api/v1/roles` — List roles in the caller's organization.
///
/// Tenancy comes exclusively from the verified session (`auth.require_org()`),
/// never from a header or path. The caller must be an org member with
/// `OrgRead` or `OrgAdmin`; organizations with no roles return `[]`.
async fn list_roles(
    State(state): State<AppState>,
    auth: AuthExtractor,
) -> Result<Json<Vec<RoleResponse>>> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;
    let user_id = auth.user_id;

    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let membership_role = role_repo
        .find_by_id(membership.role_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?;
    if !can_list_roles(&membership_role) {
        return Err(Error::Forbidden("Insufficient permissions".to_string()));
    }

    let roles = role_repo.find_by_organization_id(org_id).await?;
    Ok(Json(roles.into_iter().map(RoleResponse::from).collect()))
}

/// `POST /api/v1/roles` — Create a custom role. (placeholder — out of scope for 042)
async fn create_role(State(_state): State<AppState>) -> Result<Json<String>> {
    Ok(Json("created".to_string()))
}

/// `GET /api/v1/roles/{id}` — Get a role by ID. (placeholder — out of scope for 042)
async fn get_role(Path(id): Path<Uuid>, State(_state): State<AppState>) -> Result<Json<String>> {
    Ok(Json(id.to_string()))
}

/// `POST /api/v1/roles/{id}/assign` — Assign a role to a user. (placeholder — out of scope for 042)
async fn assign_role(Path(id): Path<Uuid>, State(_state): State<AppState>) -> Result<Json<String>> {
    Ok(Json(id.to_string()))
}

/// `POST /api/v1/roles/{id}/revoke` — Revoke a role from a user. (placeholder — out of scope for 042)
async fn revoke_role(Path(id): Path<Uuid>, State(_state): State<AppState>) -> Result<Json<String>> {
    Ok(Json(id.to_string()))
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
