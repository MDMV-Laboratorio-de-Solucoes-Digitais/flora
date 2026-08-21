//! File service.

use crate::config::storage::StorageConfig;
use crate::error::{Error, Result};
use crate::models::{CreateFileInput, File, Page, Pagination};
use crate::traits::FileRepository;
use chrono::{Duration, Utc};
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

/// File management service.
#[derive(Debug, Clone)]
pub struct FileService {
    file_repo: Arc<dyn FileRepository + Send + Sync>,
    config: Arc<StorageConfig>,
}

impl FileService {
    /// Creates a new `FileService`.
    #[must_use]
    pub fn new(file_repo: Arc<dyn FileRepository + Send + Sync>, config: Arc<StorageConfig>) -> Self {
        Self { file_repo, config }
    }

    /// Validates file name.
    fn validate_filename(name: &str) -> Result<()> {
        if name.contains('/') || name.contains('\\') || name.contains("..") {
            return Err(Error::InvalidInput("invalid filename: path traversal characters are not allowed".to_string()));
        }
        Ok(())
    }

    /// Validates MIME type.
    fn validate_mime_type(file_type: &str) -> Result<()> {
        let blocked_types = [
            "application/x-executable",
            "application/x-sh",
            "application/x-bat",
            "application/x-msdownload",
        ];
        
        if blocked_types.contains(&file_type) {
            return Err(Error::FileTypeNotAllowed(file_type.to_string()));
        }
        Ok(())
    }

    /// Create a new file record.
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
        Self::validate_filename(&input.name)?;
        Self::validate_mime_type(&input.file_type)?;

        if size_bytes > self.config.workspace_quota_bytes {
            return Err(Error::FileTooLarge {
                size: size_bytes,
                max: self.config.workspace_quota_bytes,
            });
        }

        let file_id = Uuid::now_v7();
        let storage_path = format!("orgs/{org_id}/files/{file_id}");

        let mut file = File::new(
            org_id,
            uploader_id,
            input.file_type.clone(),
            input.name.clone(),
            size_bytes.cast_signed(),
            storage_path,
        );
        file.id = file_id;
        if let Some(meta) = input.metadata {
            file.metadata = meta;
        }

        let created = self.file_repo.create(file).await?;
        tracing::info!(
            file_id = %created.id,
            org_id = %org_id,
            uploader_id = %uploader_id,
            "File record created"
        );
        
        Ok(created)
    }

    /// Get a file by ID.
    ///
    /// # Errors
    ///
    /// Returns an error if the file is not found or database query fails.
    pub async fn get_file(&self, file_id: Uuid) -> Result<File> {
        let file = self.file_repo
            .find_by_id(file_id)
            .await?
            .ok_or_else(|| Error::FileNotFound(file_id.to_string()))?;
        Ok(file)
    }

    /// List files in a workspace.
    ///
    /// # Errors
    ///
    /// Returns an error if the database query fails.
    pub async fn list_files(
        &self,
        workspace_id: Uuid,
        pagination: Pagination,
    ) -> Result<Page<File>> {
        self.file_repo.find_by_workspace_id(workspace_id, pagination).await
    }

    /// Soft-delete a file.
    ///
    /// # Errors
    ///
    /// Returns an error if the database update fails.
    pub async fn delete_file(&self, file_id: Uuid) -> Result<()> {
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
            tracing::info!(purged_count = count, retention_days = retention, "Purged old soft-deleted files");
        }
        
        Ok(count)
    }
}
