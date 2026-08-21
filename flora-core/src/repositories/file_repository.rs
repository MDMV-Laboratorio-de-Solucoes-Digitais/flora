//! File repository implementation using `PostgreSQL` + sqlx.

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{File, Pagination};
use crate::traits::FileRepository;

/// `PostgreSQL` implementation of the `FileRepository` trait.
#[derive(Debug)]
pub struct PgFileRepository {
    pool: PgPool,
}

impl PgFileRepository {
    /// Creates a new `PgFileRepository`.
    #[must_use]
    pub const fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl FileRepository for PgFileRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<File>> {
        let file = sqlx::query_as::<_, File>(
            "SELECT id, organization_id, owner_id, file_type, name, size_bytes,
                    storage_path, checksum, metadata, is_deleted, created_at, updated_at
             FROM files
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(file)
    }

    async fn find_by_workspace_id(
        &self,
        _workspace_id: Uuid,
        pagination: Pagination,
    ) -> Result<crate::models::Page<File>> {
        // Files don't have direct workspace_id, need to go through tasks or messages
        // For now, implement a basic pagination - would need joins in real implementation
        let limit = pagination.resolved_limit();
        let files = sqlx::query_as::<_, File>(
            "SELECT id, organization_id, owner_id, file_type, name, size_bytes,
                    storage_path, checksum, metadata, is_deleted, created_at, updated_at
             FROM files
             WHERE is_deleted = false
             ORDER BY created_at DESC
             LIMIT $1",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(crate::models::Page::new(files, None))
    }

    async fn find_by_owner_id(
        &self,
        owner_id: Uuid,
        pagination: Pagination,
    ) -> Result<crate::models::Page<File>> {
        let limit = pagination.resolved_limit();
        let files = sqlx::query_as::<_, File>(
            "SELECT id, organization_id, owner_id, file_type, name, size_bytes,
                    storage_path, checksum, metadata, is_deleted, created_at, updated_at
             FROM files
             WHERE owner_id = $1 AND is_deleted = false
             ORDER BY created_at DESC
             LIMIT $2",
        )
        .bind(owner_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(crate::models::Page::new(files, None))
    }

    async fn create(&self, file: File) -> Result<File> {
        let created = sqlx::query_as::<_, File>(
            "INSERT INTO files (id, organization_id, owner_id, file_type, name, size_bytes,
                                 storage_path, checksum, metadata)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             RETURNING id, organization_id, owner_id, file_type, name, size_bytes,
                       storage_path, checksum, metadata, is_deleted, created_at, updated_at",
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
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }

    async fn update(&self, id: Uuid, file: File) -> Result<File> {
        let updated = sqlx::query_as::<_, File>(
            "UPDATE files
             SET name = $2, file_type = $3, size_bytes = $4, storage_path = $5,
                 checksum = $6, metadata = $7
             WHERE id = $1
             RETURNING id, organization_id, owner_id, file_type, name, size_bytes,
                       storage_path, checksum, metadata, is_deleted, created_at, updated_at",
        )
        .bind(id)
        .bind(&file.name)
        .bind(&file.file_type)
        .bind(file.size_bytes)
        .bind(&file.storage_path)
        .bind(&file.checksum)
        .bind(&file.metadata)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?
        .ok_or_else(|| Error::FileNotFound(id.to_string()))?;
        Ok(updated)
    }

    async fn soft_delete(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE files SET is_deleted = true WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::FileNotFound(id.to_string()));
        }
        Ok(())
    }

    async fn restore(&self, id: Uuid) -> Result<()> {
        let result = sqlx::query("UPDATE files SET is_deleted = false WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        if result.rows_affected() == 0 {
            return Err(Error::FileNotFound(id.to_string()));
        }
        Ok(())
    }

    async fn purge_old(&self, older_than: chrono::DateTime<chrono::Utc>) -> Result<usize> {
        let result = sqlx::query("DELETE FROM files WHERE is_deleted = true AND updated_at < $1")
            .bind(older_than)
            .execute(&self.pool)
            .await
            .map_err(Error::from_sqlx)?;
        Ok(usize::try_from(result.rows_affected()).unwrap_or(0))
    }
}
