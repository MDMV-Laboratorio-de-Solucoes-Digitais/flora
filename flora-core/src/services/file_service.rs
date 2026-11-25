//! File service.

use crate::config::storage::StorageConfig;
use crate::error::{Error, Result};
use crate::models::{CreateFileInput, File, Page, Pagination};
use crate::storage::LocalFileSystem;
use crate::traits::{FileRepository, storage_provider::StorageProvider};
use chrono::{Duration, Utc};
use std::sync::Arc;
use tokio::io::AsyncRead;
use uuid::Uuid;
use validator::Validate;

/// File management service.
#[derive(Debug, Clone)]
pub struct FileService {
    file_repo: Arc<dyn FileRepository + Send + Sync>,
    config: Arc<StorageConfig>,
    storage: LocalFileSystem,
}

/// Result of a binary upload.
#[derive(Debug, Clone)]
pub struct UploadedFile {
    /// Persisted file metadata.
    pub file: File,
}

impl FileService {
    /// Creates a new `FileService`.
    #[must_use]
    pub fn new(
        file_repo: Arc<dyn FileRepository + Send + Sync>,
        config: Arc<StorageConfig>,
    ) -> Self {
        let storage = LocalFileSystem::new(config.local_path.clone());
        Self {
            file_repo,
            config,
            storage,
        }
    }

    fn sanitize_display_name(name: &str) -> Result<String> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(Error::Validation {
                field: "name".to_string(),
                message: "file name must not be empty".to_string(),
            });
        }
        if trimmed.contains('/') || trimmed.contains('\\') || trimmed.contains('\0') {
            return Err(Error::Validation {
                field: "name".to_string(),
                message: "invalid file name".to_string(),
            });
        }
        let base = std::path::Path::new(trimmed)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(trimmed)
            .trim_matches('.');
        let safe = if base.is_empty() { "file" } else { base };
        let limited: String = safe.chars().take(255).collect();
        Ok(limited)
    }

    fn normalize_mime(input: &str) -> String {
        input
            .split(';')
            .next()
            .unwrap_or("application/octet-stream")
            .trim()
            .to_ascii_lowercase()
    }

    fn is_allowed_mime(mime: &str) -> bool {
        matches!(
            mime,
            "application/pdf"
                | "image/png"
                | "image/jpeg"
                | "image/webp"
                | "text/plain"
                | "text/markdown"
                | "application/json"
                | "application/octet-stream"
        )
    }

    fn sniff_mime(header: &[u8], fallback: &str) -> String {
        if header.starts_with(b"%PDF-") {
            return "application/pdf".to_string();
        }
        if header.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]) {
            return "image/png".to_string();
        }
        if header.starts_with(&[0xFF, 0xD8, 0xFF]) {
            return "image/jpeg".to_string();
        }
        if header.len() >= 12 && &header[0..4] == b"RIFF" && &header[8..12] == b"WEBP" {
            return "image/webp".to_string();
        }
        if !header.is_empty()
            && header
                .iter()
                .all(|b| b.is_ascii_graphic() || b.is_ascii_whitespace())
        {
            if fallback.starts_with("text/") || fallback == "application/json" {
                return fallback.to_string();
            }
            return "text/plain".to_string();
        }
        fallback.to_string()
    }

    /// Uploads a binary file to local storage and persists metadata.
    ///
    /// # Errors
    /// Returns an error on validation, IO, quota, or database failure.
    pub async fn upload_file<R>(
        &self,
        org_id: Uuid,
        workspace_id: Uuid,
        uploader_id: Uuid,
        original_name: &str,
        declared_mime: &str,
        reader: R,
    ) -> Result<UploadedFile>
    where
        R: AsyncRead + Unpin + Send,
    {
        let name = Self::sanitize_display_name(original_name)?;
        let declared = Self::normalize_mime(declared_mime);
        if !Self::is_allowed_mime(&declared) {
            return Err(Error::FileTypeNotAllowed(declared));
        }

        let max = self
            .config
            .max_upload_bytes
            .min(self.config.workspace_quota_bytes)
            .min(self.config.org_quota_bytes);
        if max == 0 {
            return Err(Error::Validation {
                field: "max_upload_bytes".to_string(),
                message: "upload limit must be > 0".to_string(),
            });
        }

        let file_id = Uuid::now_v7();
        let storage_path = format!("orgs/{org_id}/workspaces/{workspace_id}/files/{file_id}");

        // Stream to temp+final via storage
        let (size_bytes, checksum) = self.storage.put_stream(&storage_path, reader, max).await?;
        if size_bytes == 0 {
            // best-effort cleanup
            let _ = self.storage.delete(&storage_path).await;
            return Err(Error::Validation {
                field: "file".to_string(),
                message: "empty files are not allowed".to_string(),
            });
        }

        // Best-effort sniffing from written file header
        let detected = match self.storage.open(&storage_path).await {
            Ok(mut f) => {
                use tokio::io::AsyncReadExt as _;
                let mut head = [0u8; 512];
                let n = f.read(&mut head).await.unwrap_or(0);
                Self::sniff_mime(&head[..n], &declared)
            }
            Err(_) => declared.clone(),
        };
        let final_mime = if Self::is_allowed_mime(&detected) {
            detected
        } else {
            declared.clone()
        };
        if !Self::is_allowed_mime(&final_mime) {
            let _ = self.storage.delete(&storage_path).await;
            return Err(Error::FileTypeNotAllowed(final_mime));
        }

        let mut file = File::new(
            org_id,
            uploader_id,
            final_mime,
            name,
            i64::try_from(size_bytes).unwrap_or(i64::MAX),
            storage_path.clone(),
        );
        file.id = file_id;
        file.workspace_id = Some(workspace_id);
        file.checksum = Some(checksum);

        match self
            .file_repo
            .create_with_quota(
                file,
                self.config.org_quota_bytes,
                self.config.workspace_quota_bytes,
            )
            .await
        {
            Ok(created) => {
                tracing::info!(file_id = %created.id, org_id = %org_id, workspace_id = %workspace_id, "File uploaded");
                Ok(UploadedFile { file: created })
            }
            Err(e) => {
                let _ = self.storage.delete(&storage_path).await;
                Err(e)
            }
        }
    }

    /// Create a new file record (metadata-only, legacy/compat path).
    ///
    /// # Errors
    ///
    /// Returns an error if validation fails, file is too large, or database creation fails.
    pub async fn create_file(
        &self,
        org_id: Uuid,
        input: CreateFileInput,
        uploader_id: Uuid,
        size_bytes: u64,
    ) -> Result<File> {
        input.validate()?;
        let name = Self::sanitize_display_name(&input.name)?;
        let file_type = Self::normalize_mime(&input.file_type);
        if !Self::is_allowed_mime(&file_type) {
            return Err(Error::FileTypeNotAllowed(file_type));
        }

        if size_bytes > self.config.org_quota_bytes {
            return Err(Error::FileTooLarge {
                size: size_bytes,
                max: self.config.org_quota_bytes,
            });
        }
        if size_bytes > self.config.workspace_quota_bytes {
            return Err(Error::FileTooLarge {
                size: size_bytes,
                max: self.config.workspace_quota_bytes,
            });
        }

        let file_id = Uuid::now_v7();
        let storage_path = format!(
            "orgs/{org_id}/workspaces/{}/files/{file_id}",
            input.workspace_id
        );

        let mut file = File::new(
            org_id,
            uploader_id,
            file_type,
            name,
            size_bytes.cast_signed(),
            storage_path,
        );
        file.id = file_id;
        file.workspace_id = Some(input.workspace_id);
        if let Some(meta) = input.metadata {
            file.metadata = meta;
        }

        let created = self
            .file_repo
            .create_with_quota(
                file,
                self.config.org_quota_bytes,
                self.config.workspace_quota_bytes,
            )
            .await?;
        tracing::info!(file_id = %created.id, org_id = %org_id, uploader_id = %uploader_id, "File record created");
        Ok(created)
    }

    /// Get a file by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the file is not found or database query fails.
    pub async fn get_file(&self, file_id: Uuid) -> Result<File> {
        self.file_repo
            .find_by_id(file_id)
            .await?
            .ok_or_else(|| Error::FileNotFound(file_id.to_string()))
    }

    /// Opens a stored file for streaming download.
    ///
    /// # Errors
    /// Returns an error if file metadata is missing/deleted or storage cannot be opened.
    pub async fn open_file_for_download(&self, file: &File) -> Result<tokio::fs::File> {
        if file.is_deleted {
            return Err(Error::FileNotFound(file.id.to_string()));
        }
        self.storage.open(&file.storage_path).await
    }

    /// Returns absolute storage path for a file (contained).
    ///
    /// # Errors
    /// Returns an error if path is invalid.
    pub async fn absolute_path(&self, file: &File) -> Result<std::path::PathBuf> {
        self.storage.absolute_path(&file.storage_path).await
    }

    /// List files in a workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn list_files(
        &self,
        organization_id: Uuid,
        workspace_id: Uuid,
        pagination: Pagination,
    ) -> Result<Page<File>> {
        self.file_repo
            .find_by_workspace_id(organization_id, workspace_id, pagination)
            .await
    }

    /// Soft-delete a file and best-effort remove stored bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails.
    pub async fn delete_file(&self, file_id: Uuid) -> Result<()> {
        if let Ok(file) = self.get_file(file_id).await {
            let _ = self.storage.delete(&file.storage_path).await;
        }
        self.file_repo.soft_delete(file_id).await?;
        tracing::info!(file_id = %file_id, "File soft-deleted");
        Ok(())
    }

    /// Restore a soft-deleted file.
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails.
    pub async fn restore_file(&self, file_id: Uuid) -> Result<()> {
        self.file_repo.restore(file_id).await?;
        tracing::info!(file_id = %file_id, "File restored");
        Ok(())
    }

    /// Purge old files permanently.
    ///
    /// # Errors
    ///
    /// Returns an error if the database deletion fails.
    pub async fn purge_old_files(&self, retention_days: i64) -> Result<usize> {
        let retention = if (30..=365).contains(&retention_days) {
            retention_days
        } else {
            90
        };
        let older_than = Utc::now() - Duration::days(retention);
        let count = self.file_repo.purge_old(older_than).await?;
        if count > 0 {
            tracing::info!(
                purged_count = count,
                retention_days = retention,
                "Purged old soft-deleted files"
            );
        }
        Ok(count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize_display_name() {
        assert!(FileService::sanitize_display_name("valid_file.txt").is_ok());
        assert!(FileService::sanitize_display_name("a/b.txt").is_err());
        assert!(FileService::sanitize_display_name("..\\evil.txt").is_err());
    }

    #[test]
    fn test_allowed_mime() {
        assert!(FileService::is_allowed_mime("image/png"));
        assert!(FileService::is_allowed_mime("application/pdf"));
        assert!(!FileService::is_allowed_mime("application/x-executable"));
    }
}
