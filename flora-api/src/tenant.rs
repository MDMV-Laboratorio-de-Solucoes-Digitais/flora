//! Shared tenant-scope helpers.
//!
//! OWASP A01:2021 / CWE-639 / BOLA: a client-supplied `workspace_id` is a
//! *selector*, never proof of authorization. Every handler that accepts one must
//! verify that the workspace belongs to the organization derived from the verified
//! session (`AuthExtractor::require_org()`). RLS is defense in depth, not a
//! replacement for this check (ADR-0005, OWASP Multi-Tenant Cheat Sheet).

use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::repositories::PgWorkspaceRepository;
use flora_core::traits::WorkspaceRepository;

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
