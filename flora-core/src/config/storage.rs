//! Storage configuration (`RustFS` abstraction — Local or S3-compatible).

use serde::{Deserialize, Serialize};

/// Configuration for file storage (local or S3-compatible).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    /// Storage backend: "local" or "s3".
    pub backend: String,
    /// Local storage root directory.
    pub local_path: String,
    /// S3 endpoint URL (optional, for S3-compatible services like `MinIO`).
    pub s3_endpoint: Option<String>,
    /// S3 bucket name.
    pub s3_bucket: Option<String>,
    /// S3 access key.
    pub s3_access_key: Option<String>,
    /// S3 secret key.
    pub s3_secret_key: Option<String>,
    /// S3 region.
    pub s3_region: Option<String>,
    /// Maximum file size in bytes per organization (default: 10 GB).
    pub org_quota_bytes: u64,
    /// Maximum file size in bytes per workspace (default: 2 GB).
    pub workspace_quota_bytes: u64,
    /// Whether to enable multipart uploads (default: true).
    pub multipart_enabled: bool,
    /// Chunk size for multipart uploads in bytes (default: 5 MB).
    pub chunk_size_bytes: u64,
}

impl Default for StorageConfig {
    fn default() -> Self {
        Self {
            backend: std::env::var("STORAGE_BACKEND").unwrap_or_else(|_| "local".to_owned()),
            local_path: std::env::var("STORAGE_LOCAL_PATH")
                .unwrap_or_else(|_| "/var/flora/storage".to_owned()),
            s3_endpoint: std::env::var("S3_ENDPOINT").ok(),
            s3_bucket: std::env::var("S3_BUCKET").ok(),
            s3_access_key: std::env::var("S3_ACCESS_KEY").ok(),
            s3_secret_key: std::env::var("S3_SECRET_KEY").ok(),
            s3_region: std::env::var("S3_REGION").ok(),
            org_quota_bytes: 10 * 1024 * 1024 * 1024, // 10 GB
            workspace_quota_bytes: 2 * 1024 * 1024 * 1024, // 2 GB
            multipart_enabled: true,
            chunk_size_bytes: 5 * 1024 * 1024, // 5 MB
        }
    }
}

impl StorageConfig {
    /// Validates the storage configuration.
    ///
    /// # Errors
    ///
    /// Returns an error if the configuration is invalid.
    pub fn validate(&self) -> Result<(), crate::Error> {
        if self.backend == "local" && self.local_path.is_empty() {
            return Err(crate::Error::Configuration(
                "storage.local_path is required when backend is 'local'".to_owned(),
            ));
        }
        if self.backend == "s3" && self.s3_bucket.is_none() {
            return Err(crate::Error::Configuration(
                "storage.s3_bucket is required when backend is 's3'".to_owned(),
            ));
        }
        Ok(())
    }

    /// Returns `true` if the storage backend is local.
    #[must_use]
    pub fn is_local(&self) -> bool {
        self.backend == "local"
    }
}
