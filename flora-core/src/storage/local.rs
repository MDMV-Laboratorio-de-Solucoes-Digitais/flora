//! Local filesystem implementation of the `StorageProvider` trait.
//!
//! Stores files on disk at a configurable root directory.

use crate::error::{Error, Result};
use crate::traits::storage_provider::StorageProvider;
use std::path::Path;

/// Local filesystem storage provider.
#[derive(Debug)]
pub struct LocalFileSystem {
    root: std::path::PathBuf,
}

impl LocalFileSystem {
    /// Creates a new `LocalFileSystem` with the given root directory.
    #[must_use]
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }
}

impl StorageProvider for LocalFileSystem {
    async fn put(&self, path: &str, data: &[u8]) -> Result<()> {
        let full = self.root.join(path);
        tokio::fs::create_dir_all(full.parent().ok_or_else(|| {
            Error::IoError(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "parent path missing",
            ))
        })?)
        .await
        .map_err(Error::from)?;
        tokio::fs::write(&full, data).await.map_err(Error::from)?;
        Ok(())
    }

    async fn get(&self, path: &str) -> Result<Vec<u8>> {
        let full = self.root.join(path);
        tokio::fs::read(&full).await.map_err(Error::from)
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let full = self.root.join(path);
        tokio::fs::remove_file(&full).await.map_err(Error::from)?;
        Ok(())
    }

    async fn exists(&self, path: &str) -> Result<bool> {
        let full = self.root.join(path);
        Ok(tokio::fs::metadata(&full).await.is_ok())
    }

    async fn presigned_url(&self, _path: &str, _expiry_secs: u64) -> Result<Option<String>> {
        Ok(None)
    }
}
