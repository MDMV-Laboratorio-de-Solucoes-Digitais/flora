//! File upload and management routes.

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::repositories::{PgFileRepository, PgMembershipRepository};
use flora_core::traits::FileRepository;

use crate::AppState;

/// Request body for uploading a file.
#[derive(Debug, Deserialize)]
pub struct UploadFileRequest {
    /// The workspace ID.
    pub workspace_id: Uuid,
    /// The file name.
    pub name: String,
    /// The file MIME type.
    pub file_type: String,
    /// The file size in bytes.
    pub size_bytes: u64,
}

/// Minimal file metadata response.
#[derive(Debug, Serialize)]
pub struct FileResponse {
    /// The file ID.
    pub id: String,
    /// The file name.
    pub name: String,
    /// The file MIME type.
    pub file_type: String,
    /// The file size in bytes.
    pub size_bytes: u64,
    /// The owner's user ID.
    pub owner_id: String,
    /// The creation timestamp.
    pub created_at: String,
}

/// Creates the files router.
pub fn create_files_router() -> Router<AppState> {
    Router::new()
        .route("/", post(upload_file))
        .route("/{id}", get(get_file))
        .route("/{id}", delete(delete_file))
}

fn require_org_context(headers: &axum::http::HeaderMap) -> Result<(Uuid, Uuid)> {
    let user_id = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or(Error::Unauthorized)?;

    let org_id = headers
        .get("x-organization-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or(Error::OrganizationContextRequired)?;

    Ok((user_id, org_id))
}

/// `POST /api/v1/files` — Record a file upload.
/// Full multipart upload with chunking is implemented via T072 (TODO).
async fn upload_file(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<UploadFileRequest>,
) -> Result<(StatusCode, Json<FileResponse>)> {
    let (user_id, org_id) = require_org_context(&headers)?;

    let _membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let file_repo = PgFileRepository::new((*state.db_pool).clone());

    // TODO: T072.1 — Check quota (10GB/org, 2GB/workspace)
    let max_bytes = state.config.app.max_upload_bytes;
    if req.size_bytes > max_bytes {
        return Err(Error::FileTooLarge {
            size: req.size_bytes,
            max: max_bytes,
        });
    }

    let file_id = Uuid::now_v7();
    let storage_path = format!("orgs/{org_id}/files/{file_id}");

    let file = flora_core::models::File {
        id: file_id,
        organization_id: org_id,
        owner_id: user_id,
        name: req.name,
        file_type: req.file_type,
        size_bytes: req.size_bytes.cast_signed(),
        storage_path,
        checksum: None,
        metadata: serde_json::json!({}),
        is_deleted: false,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let created = file_repo.create(file).await?;

    tracing::info!(file_id = %created.id, user_id = %user_id, size_bytes = %req.size_bytes, "File upload recorded");
    Ok((
        StatusCode::CREATED,
        Json(FileResponse {
            id: created.id.to_string(),
            name: created.name,
            file_type: created.file_type,
            size_bytes: u64::try_from(created.size_bytes).unwrap_or(0),
            owner_id: created.owner_id.to_string(),
            created_at: created.created_at.to_rfc3339(),
        }),
    ))
}

/// `GET /api/v1/files/{id}` — Get file metadata.
async fn get_file(
    State(state): State<AppState>,
    Path(file_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
) -> Result<Json<FileResponse>> {
    let (_user_id, org_id) = require_org_context(&headers)?;

    let file_repo = PgFileRepository::new((*state.db_pool).clone());
    let file = file_repo
        .find_by_id(file_id)
        .await?
        .ok_or_else(|| Error::FileNotFound(file_id.to_string()))?;

    if file.organization_id != org_id {
        return Err(Error::Forbidden(
            "cross-organization access forbidden".to_string(),
        ));
    }

    Ok(Json(FileResponse {
        id: file.id.to_string(),
        name: file.name,
        file_type: file.file_type,
        size_bytes: u64::try_from(file.size_bytes).unwrap_or(0),
        owner_id: file.owner_id.to_string(),
        created_at: file.created_at.to_rfc3339(),
    }))
}

/// `DELETE /api/v1/files/{id}` — Soft-delete a file.
async fn delete_file(
    State(state): State<AppState>,
    Path(file_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
) -> Result<StatusCode> {
    let (user_id, org_id) = require_org_context(&headers)?;

    let file_repo = PgFileRepository::new((*state.db_pool).clone());
    let file = file_repo
        .find_by_id(file_id)
        .await?
        .ok_or_else(|| Error::FileNotFound(file_id.to_string()))?;

    if file.organization_id != org_id {
        return Err(Error::Forbidden(
            "cross-organization access forbidden".to_string(),
        ));
    }
    if file.owner_id != user_id {
        return Err(Error::Forbidden(
            "only the file owner can delete it".to_string(),
        ));
    }

    file_repo.soft_delete(file_id).await?;
    tracing::info!(file_id = %file_id, user_id = %user_id, "File soft-deleted");
    Ok(StatusCode::NO_CONTENT)
}
