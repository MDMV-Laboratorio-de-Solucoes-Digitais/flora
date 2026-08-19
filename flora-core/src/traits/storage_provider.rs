//! Storage provider trait for abstract file storage.
//!
//! This allows swapping between `LocalFileSystem`, S3-compatible, etc.
#![allow(
    async_fn_in_trait,
    reason = "Async trait methods are needed for I/O operations across different storage backends"
)]

use crate::error::Result;

/// Trait for abstract file storage operations.
pub trait StorageProvider: Send + Sync {
    /// Store data at the given path.
    async fn put(&self, path: &str, data: &[u8]) -> Result<()>;

    /// Retrieve data from the given path.
    async fn get(&self, path: &str) -> Result<Vec<u8>>;

    /// Delete data at the given path.
    async fn delete(&self, path: &str) -> Result<()>;

    /// Check if a path exists.
    async fn exists(&self, path: &str) -> Result<bool>;

    /// Generate a presigned URL for direct upload/download (optional).
    ///
    /// Returns `Ok(None)` if not supported by the provider.
    async fn presigned_url(&self, _path: &str, _expiry_secs: u64) -> Result<Option<String>> {
        Ok(None)
    }
}
