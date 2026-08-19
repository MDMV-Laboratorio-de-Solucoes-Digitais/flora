//! Flora Files - File Storage with RustFS Abstraction
//!
//! Note: Using `sqlx::query` (runtime) instead of `query!` (compile-time)
//! because this crate is compiled without DATABASE_URL in CI.

use flora_core::{Error, Result};
use sqlx::PgPool;
use std::sync::Arc;

/// Storage provider trait for file operations.
#[async_trait::async_trait]
pub trait StorageProvider: Send + Sync {
    async fn put(&self, path: &str, data: &[u8]) -> Result<()>;
    async fn get(&self, path: &str) -> Result<Vec<u8>>;
    async fn delete(&self, path: &str) -> Result<()>;
    async fn exists(&self, path: &str) -> Result<bool>;
}

/// Local filesystem storage implementation.
pub struct LocalStorage {
    root: std::path::PathBuf,
}

impl LocalStorage {
    #[must_use]
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }
}

#[async_trait::async_trait]
impl StorageProvider for LocalStorage {
    async fn put(&self, path: &str, data: &[u8]) -> Result<()> {
        let full_path = self.root.join(path);
        if let Some(parent) = full_path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|e| Error::Storage(e.to_string()))?;
        }
        tokio::fs::write(full_path, data)
            .await
            .map_err(|e| Error::Storage(e.to_string()))?;
        Ok(())
    }

    async fn get(&self, path: &str) -> Result<Vec<u8>> {
        let full_path = self.root.join(path);
        tokio::fs::read(full_path)
            .await
            .map_err(|e| Error::Storage(e.to_string()))
    }

    async fn delete(&self, path: &str) -> Result<()> {
        let full_path = self.root.join(path);
        tokio::fs::remove_file(full_path)
            .await
            .map_err(|e| Error::Storage(e.to_string()))?;
        Ok(())
    }

    async fn exists(&self, path: &str) -> Result<bool> {
        let full_path = self.root.join(path);
        Ok(full_path.exists())
    }
}

/// File service for managing files with database and storage.
pub struct FileService {
    db: PgPool,
    storage: Arc<dyn StorageProvider>,
}

impl FileService {
    #[must_use]
    pub fn new(db: PgPool, storage: Arc<dyn StorageProvider>) -> Self {
        Self { db, storage }
    }

    /// Creates a new file record in the database.
    pub async fn create(&self, file: flora_core::models::File) -> Result<flora_core::models::File> {
        sqlx::query(
            r#"
            INSERT INTO files (id, organization_id, owner_id, file_type, name, size_bytes,
                              storage_path, checksum, metadata, is_deleted, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#,
        )
        .bind(file.id)
        .bind(file.organization_id)
        .bind(file.owner_id)
        .bind(&file.file_type)
        .bind(&file.name)
        .bind(file.size_bytes)
        .bind(&file.storage_path)
        .bind(&file.checksum)
        .bind(&file.metadata)
        .bind(file.is_deleted)
        .bind(file.created_at)
        .bind(file.updated_at)
        .execute(&self.db)
        .await
        .map_err(Error::from_sqlx)?;

        Ok(file)
    }
}
