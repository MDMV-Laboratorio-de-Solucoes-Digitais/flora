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
use flora_core::repositories::PgMembershipRepository;
use flora_core::traits::MembershipRepository;

use crate::AppState;
use crate::extractors::AuthExtractor;

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

/// `POST /api/v1/files` — Record a file upload.
/// Delegates to `FileService` for quota checking, MIME validation, and storage.
async fn upload_file(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<UploadFileRequest>,
) -> Result<(StatusCode, Json<FileResponse>)> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;
    let user_id = auth.user_id;

    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let _ = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

    let input = flora_core::models::CreateFileInput {
        name: req.name,
        file_type: req.file_type,
        workspace_id: req.workspace_id,
        metadata: None,
    };

    let created = state
        .file_service
        .create_file(org_id, input, user_id, req.size_bytes)
        .await?;

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
    auth: AuthExtractor,
    Path(file_id): Path<Uuid>,
) -> Result<Json<FileResponse>> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;

    let file = state.file_service.get_file(file_id).await?;

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
    auth: AuthExtractor,
    Path(file_id): Path<Uuid>,
) -> Result<StatusCode> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;
    let user_id = auth.user_id;

    let file = state.file_service.get_file(file_id).await?;

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

    state.file_service.delete_file(file_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
