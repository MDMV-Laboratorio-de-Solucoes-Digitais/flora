//! File service.

use crate::traits::FileRepository;
use std::sync::Arc;

/// File management service.
#[derive(Debug, Clone)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct FileService {
    file_repo: Arc<dyn FileRepository + Send + Sync>,
}

impl FileService {
    /// Creates a new `FileService`.
    #[must_use]
    pub fn new(file_repo: Arc<dyn FileRepository + Send + Sync>) -> Self {
        Self { file_repo }
    }
}
