//! Shared tenant-scope helpers.
//!
//! OWASP A01:2021 / CWE-639 / BOLA: a client-supplied `workspace_id` is a
//! *selector*, never proof of authorization. Every handler that accepts one must
//! verify that the workspace belongs to the organization derived from the verified
//! session (`AuthExtractor::require_org()`). RLS is defense in depth, not a
//! replacement for this check (ADR-0005, OWASP Multi-Tenant Cheat Sheet).

use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::repositories::{PgMembershipRepository, PgWorkspaceRepository};
use flora_core::traits::{MembershipRepository, WorkspaceRepository};

use crate::state::AppState;

/// Ensure `workspace_id` belongs to `org_id` (the session-verified organization).
///
/// Returns a validation error on a missing or cross-org workspace. Lookup is scoped
/// by id and then the owner org is compared — fail closed, never an unscoped write.
///
/// # Errors
///
/// Returns an error when the workspace cannot be found, the database query fails,
/// or the workspace belongs to an organization other than `org_id`.
pub async fn ensure_workspace_in_org(
    state: &AppState,
    org_id: Uuid,
    workspace_id: Uuid,
) -> Result<()> {
    let workspace_repo = PgWorkspaceRepository::new((*state.db_pool).clone());
    let workspace = workspace_repo
        .find_by_id(workspace_id)
        .await?
        .ok_or_else(|| Error::Validation {
            field: "workspace_id".to_string(),
            message: "workspace not found".to_string(),
        })?;
    if workspace.organization_id != org_id {
        return Err(Error::Validation {
            field: "workspace_id".to_string(),
            message: "workspace not found".to_string(),
        });
    }
    Ok(())
}

/// Ensure `user_id` is a member of `org_id` (the session-verified organization).
///
/// Guards `assignee_id` supplied in task create/update bodies: assigning a task to
/// a user from another organization is BOLA (OWASP A01:2021 / CWE-639). The error
/// deliberately does not disclose whether the user exists.
///
/// # Errors
///
/// Returns an error when the database query fails or the user is not a member of
/// `org_id` (including when the user does not exist).
pub async fn ensure_user_in_org(state: &AppState, org_id: Uuid, user_id: Uuid) -> Result<()> {
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?;
    if membership.is_none() {
        return Err(Error::Validation {
            field: "assignee_id".to_string(),
            message: "assignee not found".to_string(),
        });
    }
    Ok(())
}
