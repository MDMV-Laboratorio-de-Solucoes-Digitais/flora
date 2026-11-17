//! File upload and management routes.

use axum::{
    Router,
    body::Body,
    extract::{Multipart, Path, Query, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Json, Response},
    routing::{delete, get, post},
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::repositories::PgMembershipRepository;
use flora_core::traits::MembershipRepository;

use crate::AppState;
use crate::extractors::AuthExtractor;
use crate::tenant::ensure_workspace_in_org;

/// Request body for legacy metadata-only upload.
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

/// Minimal file metadata response (legacy/metadata endpoint).
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
    /// Optional workspace ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    /// The creation timestamp.
    pub created_at: String,
}

/// Frontend-compatible file attachment response.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileAttachmentResponse {
    /// The file ID.
    pub id: String,
    /// The file name.
    pub file_name: String,
    /// The file MIME type.
    pub mime_type: String,
    /// The file size in bytes.
    pub byte_size: u64,
    /// Download URL.
    pub rust_fs_url: String,
    /// Upload timestamp.
    pub uploaded_at: String,
}

#[derive(Debug, Deserialize)]
struct UploadQuery {
    workspace_id: Option<Uuid>,
}

/// Creates the files router.
pub fn create_files_router() -> Router<AppState> {
    Router::new()
        .route("/", post(upload_file_metadata))
        .route("/upload", post(upload_file_multipart))
        .route("/download/{id}", get(download_file))
        .route("/{id}/download", get(download_file))
        .route("/{id}", get(get_file))
        .route("/{id}", delete(delete_file))
}

async fn ensure_member(auth: &AuthExtractor, state: &AppState) -> Result<Uuid> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;
    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let _ = membership_repo
        .find_by_user_and_organization(auth.user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;
    Ok(org_id)
}

fn to_attachment(file: &flora_core::models::File) -> FileAttachmentResponse {
    FileAttachmentResponse {
        id: file.id.to_string(),
        file_name: file.name.clone(),
        mime_type: file.file_type.clone(),
        byte_size: u64::try_from(file.size_bytes).unwrap_or(0),
        rust_fs_url: format!("/api/v1/files/download/{}", file.id),
        uploaded_at: file.created_at.to_rfc3339(),
    }
}

fn to_file_response(file: &flora_core::models::File) -> FileResponse {
    FileResponse {
        id: file.id.to_string(),
        name: file.name.clone(),
        file_type: file.file_type.clone(),
        size_bytes: u64::try_from(file.size_bytes).unwrap_or(0),
        owner_id: file.owner_id.to_string(),
        workspace_id: file.workspace_id.map(|id| id.to_string()),
        created_at: file.created_at.to_rfc3339(),
    }
}

/// `POST /api/v1/files/upload` — Stream-upload a binary file (multipart/form-data).
async fn upload_file_multipart(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Query(query): Query<UploadQuery>,
    mut multipart: Multipart,
) -> Result<(StatusCode, Json<FileAttachmentResponse>)> {
    let org_id = ensure_member(&auth, &state).await?;
    let user_id = auth.user_id;

    let mut workspace_id = query.workspace_id;
    let mut uploaded: Option<flora_core::models::File> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| Error::Validation {
            field: "multipart".to_string(),
            message: format!("invalid multipart payload: {e}"),
        })?
    {
        let field_name = field.name().unwrap_or_default().to_string();
        match field_name.as_str() {
            "workspace_id" | "workspaceId" => {
                let text = field.text().await.map_err(|e| Error::Validation {
                    field: "workspace_id".to_string(),
                    message: format!("invalid workspace_id: {e}"),
                })?;
                let parsed = Uuid::parse_str(text.trim()).map_err(|_| Error::Validation {
                    field: "workspace_id".to_string(),
                    message: "workspace_id must be a valid UUID".to_string(),
                })?;
                workspace_id = Some(parsed);
            }
            "file" => {
                if uploaded.is_some() {
                    return Err(Error::Validation {
                        field: "file".to_string(),
                        message: "only one file upload is supported per request".to_string(),
                    });
                }
                let name = field
                    .file_name()
                    .map_or_else(|| "file".to_string(), ToString::to_string);
                let mime = field.content_type().map_or_else(
                    || "application/octet-stream".to_string(),
                    ToString::to_string,
                );
                let ws = workspace_id.ok_or_else(|| Error::Validation {
                    field: "workspace_id".to_string(),
                    message:
                        "workspace_id is required as query parameter or as a form field before file"
                            .to_string(),
                })?;
                ensure_workspace_in_org(&state, org_id, ws).await?;

                let reader = tokio_util::io::StreamReader::new(TryStreamExt::map_err(field, |e| {
                    std::io::Error::other(e.to_string())
                }));
                let result = state
                    .file_service
                    .upload_file(org_id, ws, user_id, &name, &mime, reader)
                    .await?;
                uploaded = Some(result.file);
            }
            _ => {
                // ignore unknown fields
            }
        }
    }

    let file = uploaded.ok_or_else(|| Error::Validation {
        field: "file".to_string(),
        message: "file field is required".to_string(),
    })?;

    Ok((StatusCode::CREATED, Json(to_attachment(&file))))
}

/// `POST /api/v1/files` — Legacy metadata-only file record creation.
async fn upload_file_metadata(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<UploadFileRequest>,
) -> Result<(StatusCode, Json<FileResponse>)> {
    let org_id = ensure_member(&auth, &state).await?;
    let user_id = auth.user_id;
    ensure_workspace_in_org(&state, org_id, req.workspace_id).await?;

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

    Ok((StatusCode::CREATED, Json(to_file_response(&created))))
}

/// `GET /api/v1/files/{id}` — Get file metadata.
async fn get_file(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path(file_id): Path<Uuid>,
) -> Result<Json<FileResponse>> {
    let org_id = ensure_member(&auth, &state).await?;
    let file = state.file_service.get_file(file_id).await?;

    if file.organization_id != org_id || file.is_deleted {
        return Err(Error::FileNotFound(file_id.to_string()));
    }

    Ok(Json(to_file_response(&file)))
}

/// `GET /api/v1/files/download/{id}` — Stream-download a file.
async fn download_file(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path(file_id): Path<Uuid>,
) -> Result<Response> {
    let org_id = ensure_member(&auth, &state).await?;
    let file = state.file_service.get_file(file_id).await?;

    if file.organization_id != org_id || file.is_deleted {
        return Err(Error::FileNotFound(file_id.to_string()));
    }

    let f = state.file_service.open_file_for_download(&file).await?;
    let stream = ReaderStream::new(f);
    let body = Body::from_stream(stream);

    let mut response = body.into_response();
    let headers = response.headers_mut();
    let _ = headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(&file.file_type)
            .unwrap_or_else(|_| HeaderValue::from_static("application/octet-stream")),
    );
    if let Ok(len) = HeaderValue::from_str(&file.size_bytes.max(0).to_string()) {
        let _ = headers.insert(header::CONTENT_LENGTH, len);
    }
    let _ = headers.insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    // Basic attachment disposition with sanitized filename
    let safe_name: String = file
        .name
        .chars()
        .filter(|c| c.is_ascii_graphic() && *c != '"' && *c != '\\')
        .collect();
    let disp = format!(
        "attachment; filename=\"{}\"",
        if safe_name.is_empty() {
            "file"
        } else {
            &safe_name
        }
    );
    if let Ok(v) = HeaderValue::from_str(&disp) {
        let _ = headers.insert(header::CONTENT_DISPOSITION, v);
    }
    Ok(response)
}

/// `DELETE /api/v1/files/{id}` — Soft-delete a file.
async fn delete_file(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path(file_id): Path<Uuid>,
) -> Result<StatusCode> {
    let org_id = ensure_member(&auth, &state).await?;
    let user_id = auth.user_id;

    let file = state.file_service.get_file(file_id).await?;
    if file.organization_id != org_id {
        return Err(Error::FileNotFound(file_id.to_string()));
    }
    if file.owner_id != user_id {
        return Err(Error::Forbidden(
            "only the file owner can delete it".to_string(),
        ));
    }

    state.file_service.delete_file(file_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
