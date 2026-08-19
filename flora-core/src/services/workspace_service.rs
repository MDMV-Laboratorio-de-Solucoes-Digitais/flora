//! Workspace service.

use crate::traits::WorkspaceRepository;

/// Workspace management service.
#[expect(dead_code)]
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
