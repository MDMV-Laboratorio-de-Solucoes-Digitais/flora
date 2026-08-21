//! Workspace management routes.
//!
//! Per T045: Workspace CRUD endpoints.

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::models::{CreateWorkspaceInput, UpdateWorkspaceInput, Workspace};
use flora_core::repositories::{PgMembershipRepository, PgRoleRepository};
use flora_core::traits::{MembershipRepository, RoleRepository};

use super::AppState;

/// Query parameters for listing workspaces.
#[derive(Debug, Deserialize)]
pub struct ListWorkspacesQuery {
    /// Optional organization ID filter.
    pub organization_id: Option<Uuid>,
}

/// Request body for creating a workspace.
#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    /// Workspace name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
}

/// Response body for a workspace.
#[derive(Debug, Serialize)]
pub struct WorkspaceResponse {
    /// Workspace ID.
    pub id: String,
    /// Organization ID.
    pub organization_id: String,
    /// Workspace name.
    pub name: String,
    /// Optional description.
    pub description: Option<String>,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
}

impl From<Workspace> for WorkspaceResponse {
    fn from(workspace: Workspace) -> Self {
        Self {
            id: workspace.id.to_string(),
            organization_id: workspace.organization_id.to_string(),
            name: workspace.name,
            description: workspace.description,
            created_at: workspace.created_at.to_rfc3339(),
            updated_at: workspace.updated_at.to_rfc3339(),
        }
    }
}

/// Creates the workspace router.
pub fn create_workspace_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_workspaces))
        .route("/", post(create_workspace))
        .route("/:workspace_id", get(get_workspace))
        .route("/:workspace_id", patch(update_workspace))
        .route("/:workspace_id", delete(delete_workspace))
}

/// Extracts the organization ID from the X-Organization-ID header.
fn extract_org_id_from_header(headers: &axum::http::HeaderMap) -> Result<Uuid> {
    headers
        .get("x-organization-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or(Error::OrganizationContextRequired)
}

/// Extracts the user ID from the X-User-ID header.
fn extract_user_id_from_header(headers: &axum::http::HeaderMap) -> Result<Uuid> {
    headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or(Error::Unauthorized)
}

/// `GET /api/v1/workspaces` — List workspaces in an organization.
async fn list_workspaces(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<WorkspaceResponse>>> {
    let org_id = extract_org_id_from_header(&headers)?;

    let workspaces = state.workspace_service.list_workspaces(org_id).await?;

    let response: Vec<WorkspaceResponse> = workspaces
        .into_iter()
        .map(WorkspaceResponse::from)
        .collect();

    Ok(Json(response))
}

/// `POST /api/v1/workspaces` — Create a new workspace.
async fn create_workspace(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<CreateWorkspaceRequest>,
) -> Result<(StatusCode, Json<WorkspaceResponse>)> {
    let org_id = extract_org_id_from_header(&headers)?;
    let user_id = extract_user_id_from_header(&headers)?;

    // Verify user has permission to create workspaces in this org
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let _membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

    let input = CreateWorkspaceInput {
        name: req.name,
        description: req.description,
    };

    let workspace = state
        .workspace_service
        .create_workspace(org_id, input)
        .await?;

    tracing::info!(workspace_id = %workspace.id, org_id = %org_id, user_id = %user_id, "Workspace created");
    Ok((
        StatusCode::CREATED,
        Json(WorkspaceResponse::from(workspace)),
    ))
}

/// `GET /api/v1/workspaces/{workspace_id}` — Get a workspace by ID.
async fn get_workspace(
    State(state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
) -> Result<Json<WorkspaceResponse>> {
    let org_id = extract_org_id_from_header(&headers)?;
    let user_id = extract_user_id_from_header(&headers)?;

    // Verify user has access to this workspace
    let workspace = state.workspace_service.get_workspace(workspace_id).await?;

    if workspace.organization_id != org_id {
        return Err(Error::Forbidden(
            "Workspace belongs to a different organization".to_string(),
        ));
    }

    // Verify user is a member of the organization
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let _ = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

    Ok(Json(WorkspaceResponse::from(workspace)))
}

/// `PATCH /api/v1/workspaces/{workspace_id}` — Update a workspace.
async fn update_workspace(
    State(state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
    Json(req): Json<UpdateWorkspaceInput>,
) -> Result<Json<WorkspaceResponse>> {
    let org_id = extract_org_id_from_header(&headers)?;
    let user_id = extract_user_id_from_header(&headers)?;

    // Verify user has permission to update workspaces in this org
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

    // Check if user has workspace:write or workspace:admin permission
    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let role = role_repo
        .find_by_id(membership.role_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?;
    if !role.has_permission(flora_core::models::Permission::WorkspaceWrite)
        && !role.has_permission(flora_core::models::Permission::WorkspaceAdmin)
    {
        return Err(Error::Forbidden("Insufficient permissions".to_string()));
    }

    let workspace = state
        .workspace_service
        .update_workspace(workspace_id, req)
        .await?;

    tracing::info!(workspace_id = %workspace.id, org_id = %org_id, user_id = %user_id, "Workspace updated");
    Ok(Json(WorkspaceResponse::from(workspace)))
}

/// `DELETE /api/v1/workspaces/{workspace_id}` — Delete a workspace.
async fn delete_workspace(
    State(state): State<AppState>,
    Path(workspace_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
) -> Result<StatusCode> {
    let org_id = extract_org_id_from_header(&headers)?;
    let user_id = extract_user_id_from_header(&headers)?;

    // Verify user has permission to delete workspaces in this org
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

    // Check if user has workspace:admin permission
    let role_repo = PgRoleRepository::new((*state.db_pool).clone());
    let role = role_repo
        .find_by_id(membership.role_id)
        .await?
        .ok_or_else(|| Error::RoleNotFound(membership.role_id.to_string()))?;
    if !role.has_permission(flora_core::models::Permission::WorkspaceAdmin) {
        return Err(Error::Forbidden("Insufficient permissions".to_string()));
    }

    state
        .workspace_service
        .delete_workspace(workspace_id)
        .await?;

    tracing::info!(workspace_id = %workspace_id, org_id = %org_id, user_id = %user_id, "Workspace deleted");
    Ok(StatusCode::NO_CONTENT)
}
