#! Workspace service — business logic for workspace management.

use std::sync::Arc;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{CreateWorkspaceInput, UpdateWorkspaceInput, Workspace};
use crate::traits::WorkspaceRepository;
use validator::Validate;

/// Workspace management service.
#[derive(Debug, Clone)]
pub struct WorkspaceService {
    workspace_repo: Arc<dyn WorkspaceRepository + Send + Sync>,
}

impl WorkspaceService {
    /// Creates a new `WorkspaceService`.
    #[must_use]
    pub fn new(workspace_repo: Arc<dyn WorkspaceRepository + Send + Sync>) -> Self {
        Self { workspace_repo }
    }

    /// Creates a new workspace in an organization.
    ///
    /// # Errors
    ///
    /// Returns an error if the input validation fails or if the workspace creation fails.
    pub async fn create_workspace(
        &self,
        organization_id: Uuid,
        input: CreateWorkspaceInput,
    ) -> Result<Workspace> {
        // Validate input
        input.validate().map_err(Error::from)?;

        let workspace = Workspace::new(organization_id, &input.name, input.description);
        self.workspace_repo.create(workspace).await
    }

    /// Lists workspaces in an organization.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn list_workspaces(&self, organization_id: Uuid) -> Result<Vec<Workspace>> {
        self.workspace_repo
            .find_by_organization_id(organization_id)
            .await
    }

    /// Gets a workspace by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace is not found or if the database query fails.
    pub async fn get_workspace(&self, workspace_id: Uuid) -> Result<Workspace> {
        self.workspace_repo
            .find_by_id(workspace_id)
            .await?
            .ok_or_else(|| Error::WorkspaceNotFound(workspace_id.to_string()))
    }

    /// Updates a workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the input validation fails, if the workspace is not found,
    /// or if the update fails.
    pub async fn update_workspace(
        &self,
        workspace_id: Uuid,
        input: UpdateWorkspaceInput,
    ) -> Result<Workspace> {
        // Validate input
        input.validate().map_err(Error::from)?;

        let workspace = self.get_workspace(workspace_id).await?;
        let updated = Workspace {
            name: input.name.unwrap_or(workspace.name),
            description: input.description.or(workspace.description),
            updated_at: chrono::Utc::now(),
            ..workspace
        };
        self.workspace_repo.update(workspace_id, updated).await
    }

    /// Deletes a workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the workspace is not found or if the database query fails.
    pub async fn delete_workspace(&self, workspace_id: Uuid) -> Result<()> {
        self.workspace_repo.delete(workspace_id).await
    }
}
