//! Workspace service.

use crate::traits::WorkspaceRepository;

/// Workspace management service.
#[derive(Debug)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct WorkspaceService {
    workspace_repo: Box<dyn WorkspaceRepository>,
}

impl WorkspaceService {
    /// Creates a new `WorkspaceService`.
    #[must_use]
    pub fn new(workspace_repo: Box<dyn WorkspaceRepository>) -> Self {
        Self { workspace_repo }
    }
}
