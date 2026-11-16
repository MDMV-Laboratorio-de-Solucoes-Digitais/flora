//! Local filesystem implementation of the `StorageProvider` trait.
//!
//! Stores files on disk at a configurable root directory with basic containment.

use crate::error::{Error, Result};
use crate::traits::storage_provider::StorageProvider;
use sha2::Digest as _;
use std::fmt::Write as _;
use std::path::{Component, Path, PathBuf};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWriteExt};

/// Local filesystem storage provider.
#[derive(Debug, Clone)]
pub struct LocalFileSystem {
    root: PathBuf,
}

impl LocalFileSystem {
    /// Creates a new `LocalFileSystem` with the given root directory.
    #[must_use]
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    fn sanitize_relative_path(path: &str) -> Result<PathBuf> {
        let p = Path::new(path);
        if p.is_absolute() {
            return Err(Error::Validation {
                field: "storage_path".to_string(),
                message: "absolute paths are not allowed".to_string(),
            });
        }
        let mut clean = PathBuf::new();
        for comp in p.components() {
            match comp {
                Component::Normal(s) => clean.push(s),
                Component::CurDir => {}
                Component::ParentDir | Component::RootDir | Component::Prefix(_) => {
                    return Err(Error::Validation {
                        field: "storage_path".to_string(),
                        message: "path traversal is not allowed".to_string(),
                    });
                }
            }
        }
        if clean.as_os_str().is_empty() {
            return Err(Error::Validation {
                field: "storage_path".to_string(),
                message: "storage path must not be empty".to_string(),
            });
        }
        Ok(clean)
    }

    fn full_path(&self, path: &str) -> Result<PathBuf> {
        let rel = Self::sanitize_relative_path(path)?;
        Ok(self.root.join(rel))
    }

    /// Returns canonical storage root, creating it if needed.
    ///
    /// # Errors
    /// Returns an error if the root cannot be created or canonicalized.
    pub async fn ensure_root(&self) -> Result<PathBuf> {
        tokio::fs::create_dir_all(&self.root)
            .await
            .map_err(Error::from)?;
        tokio::fs::canonicalize(&self.root)
            .await
            .map_err(Error::from)
    }

    /// Streams data from reader into a temp file under root, then atomically renames to destination.
    /// Returns (`bytes_written`, `sha256_hex`).
    ///
    /// # Errors
    /// Returns an error on IO failure, size limit exceeded, or invalid path.
    pub async fn put_stream<R>(
        &self,
        rel_path: &str,
        mut reader: R,
        max_bytes: u64,
    ) -> Result<(u64, String)>
    where
        R: AsyncRead + Unpin + Send,
    {
        let root = self.ensure_root().await?;
        let dest = root.join(Self::sanitize_relative_path(rel_path)?);
        let parent = dest.parent().ok_or_else(|| Error::Validation {
            field: "storage_path".to_string(),
            message: "invalid destination path".to_string(),
        })?;
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(Error::from)?;

        let tmp_dir = root.join(".tmp");
        tokio::fs::create_dir_all(&tmp_dir)
            .await
            .map_err(Error::from)?;
        let tmp_path = tmp_dir.join(format!("{}.part", uuid::Uuid::now_v7()));

        let mut file = tokio::fs::File::create(&tmp_path)
            .await
            .map_err(Error::from)?;
        let mut hasher = sha2::Sha256::new();
        let mut written: u64 = 0;
        let mut buf = vec![0u8; 64 * 1024];
        loop {
            let n = reader.read(&mut buf).await.map_err(Error::from)?;
            if n == 0 {
                break;
            }
            written = written.saturating_add(u64::try_from(n).unwrap_or(u64::MAX));
            if written > max_bytes {
                let _ = tokio::fs::remove_file(&tmp_path).await;
                return Err(Error::FileTooLarge {
                    size: written,
                    max: max_bytes,
                });
            }
            hasher.update(&buf[..n]);
            file.write_all(&buf[..n]).await.map_err(Error::from)?;
        }
        file.flush().await.map_err(Error::from)?;
        drop(file);

        // Best-effort containment: destination must remain under canonical root.
        if !dest.starts_with(&root) {
            let _ = tokio::fs::remove_file(&tmp_path).await;
            return Err(Error::Validation {
                field: "storage_path".to_string(),
                message: "destination escapes storage root".to_string(),
            });
        }
        tokio::fs::rename(&tmp_path, &dest)
            .await
            .map_err(Error::from)?;
        let digest = hasher.finalize();
        let mut hash = String::with_capacity(digest.len() * 2);
        for b in digest {
            let _ = write!(hash, "{b:02x}");
        }
        Ok((written, hash))
    }

    /// Opens a stored file for streaming reads.
    ///
    /// # Errors
    /// Returns an error if the path is invalid or file cannot be opened.
    pub async fn open(&self, rel_path: &str) -> Result<tokio::fs::File> {
        let root = self.ensure_root().await?;
        let path = root.join(Self::sanitize_relative_path(rel_path)?);
        if !path.starts_with(&root) {
            return Err(Error::Validation {
                field: "storage_path".to_string(),
                message: "path escapes storage root".to_string(),
            });
        }
        tokio::fs::File::open(&path).await.map_err(Error::from)
    }

    /// Returns absolute path for an existing stored file (after containment check).
    ///
    /// # Errors
    /// Returns an error if path is invalid.
    pub async fn absolute_path(&self, rel_path: &str) -> Result<PathBuf> {
        let root = self.ensure_root().await?;
        let path = root.join(Self::sanitize_relative_path(rel_path)?);
        if !path.starts_with(&root) {
            return Err(Error::Validation {
                field: "storage_path".to_string(),
                message: "path escapes storage root".to_string(),
            });
        }
        Ok(path)
    }
}

impl StorageProvider for LocalFileSystem {
    async fn put(&self, path: &str, data: &[u8]) -> Result<()> {
        let full = self.full_path(path)?;
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
        let full = self.full_path(path)?;
        tokio::fs::read(&full).await.map_err(Error::from)
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let full = self.full_path(path)?;
        tokio::fs::remove_file(&full).await.map_err(Error::from)?;
        Ok(())
    }

    async fn exists(&self, path: &str) -> Result<bool> {
        let full = self.full_path(path)?;
        Ok(tokio::fs::metadata(&full).await.is_ok())
    }

    #[expect(
        clippy::unused_async_trait_impl,
        reason = "StorageProvider trait requires async fn; local backend does not support presigned URLs."
    )]
    async fn presigned_url(&self, _path: &str, _expiry_secs: u64) -> Result<Option<String>> {
        Ok(None)
    }
}
