//! File model — a digital asset stored via `RustFS`.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

/// A file uploaded to Flora.
///
/// Files are stored via the `RustFS` abstraction (local or S3--compatible).
/// Soft-deletion is supported via `is_deleted` flag.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct File {
    /// Unique identifier.
    pub id: Uuid,
    /// The organization this file belongs to.
    pub organization_id: Uuid,
    /// The user who uploaded the file.
    pub owner_id: Uuid,
    /// MIME type of the file.
    pub file_type: String,
    /// Original filename.
    pub name: String,
    /// Size in bytes.
    pub size_bytes: i64,
    /// Path within the storage backend.
    pub storage_path: String,
    /// Optional checksum for integrity verification.
    pub checksum: Option<String>,
    /// Optional metadata (EXIF, ID3, etc.).
    pub metadata: serde_json::Value,
    /// Soft-delete flag.
    pub is_deleted: bool,
    /// When the file record was created.
    pub created_at: DateTime<Utc>,
    /// When the file record was last updated.
    pub updated_at: DateTime<Utc>,
}

impl File {
    /// Creates a new file record.
    #[must_use]
    pub fn new(
        organization_id: Uuid,
        owner_id: Uuid,
        file_type: String,
        name: String,
        size_bytes: i64,
        storage_path: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::now_v7(),
            organization_id,
            owner_id,
            file_type,
            name,
            size_bytes,
            storage_path,
            checksum: None,
            metadata: serde_json::json!({}),
            is_deleted: false,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Input for uploading a file (metadata only; file bytes handled separately).
#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateFileInput {
    #[validate(length(min = 1, max = 255, message = "name must be 1-255 characters"))]
    /// File name.
    pub name: String,
    /// MIME type.
    pub file_type: String,
    /// The workspace to upload to.
    pub workspace_id: Uuid,
    /// Optional metadata.
    pub metadata: Option<serde_json::Value>,
}
