//! File service.

use crate::traits::FileRepository;

/// File management service.
#[derive(Debug)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct FileService {
    file_repo: Box<dyn FileRepository>,
}

impl FileService {
    /// Creates a new `FileService`.
    #[must_use]
    pub fn new(file_repo: Box<dyn FileRepository>) -> Self {
        Self { file_repo }
    }
}
