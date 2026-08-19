//! File service.

use crate::traits::FileRepository;

/// File management service.
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
