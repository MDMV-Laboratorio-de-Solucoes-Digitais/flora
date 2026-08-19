//! Task management API routes.

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::models::{Task, TaskStatus};
use flora_core::repositories::{PgMembershipRepository, PgTaskRepository};
use flora_core::traits::TaskRepository;
use std::str::FromStr;

use crate::AppState;

/// Request body for creating a task.
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    /// The task title.
    pub title: String,
    /// Optional task description.
    pub description: Option<String>,
    /// The workspace ID.
    pub workspace_id: Uuid,
    /// Optional assignee user ID.
    pub assignee_id: Option<Uuid>,
}

/// Request body for updating a task.
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    /// Optional new title.
    pub title: Option<String>,
    /// Optional new description.
    pub description: Option<String>,
    /// Optional new status.
    pub status: Option<String>,
    /// Optional new assignee.
    pub assignee_id: Option<Uuid>,
}

/// Task response.
#[derive(Debug, Serialize)]
pub struct TaskResponse {
    /// The task ID.
    pub id: String,
    /// The workspace ID.
    pub workspace_id: String,
    /// The creator's user ID.
    pub creator_id: String,
    /// Optional assignee's user ID.
    pub assignee_id: Option<String>,
    /// The task title.
    pub title: String,
    /// Optional task description.
    pub description: Option<String>,
    /// The task status.
    pub status: String,
    /// The creation timestamp.
    pub created_at: String,
}

/// Creates the tasks router.
pub fn create_tasks_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_tasks))
        .route("/", post(create_task))
        .route("/{id}", get(get_task))
        .route("/{id}", patch(update_task))
        .route("/{id}", delete(delete_task))
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        Self {
            id: task.id.to_string(),
            workspace_id: task.workspace_id.to_string(),
            creator_id: task.creator_id.to_string(),
            assignee_id: task.assignee_id.map(|id| id.to_string()),
            title: task.title,
            description: task.description,
            status: task.status.to_string(),
            created_at: task.created_at.to_rfc3339(),
        }
    }
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

/// `GET /api/v1/tasks` — List tasks.
async fn list_tasks(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
) -> Result<Json<Vec<TaskResponse>>> {
    let (user_id, _org_id) = require_org_context(&headers)?;

    let _membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let task_repo = PgTaskRepository::new((*state.db_pool).clone());

    let page = task_repo
        .find_by_assignee_id(
            user_id,
            flora_core::models::Pagination {
                limit: 50,
                offset: 0,
            },
        )
        .await?;

    let tasks: Vec<TaskResponse> = page
        .items
        .into_iter()
        .filter(|t| !t.is_deleted)
        .map(TaskResponse::from)
        .collect();

    Ok(Json(tasks))
}

/// `POST /api/v1/tasks` — Create a task.
async fn create_task(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<TaskResponse>)> {
    let (user_id, org_id) = require_org_context(&headers)?;

    let _membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let task_repo = PgTaskRepository::new((*state.db_pool).clone());

    let task = Task::new(req.workspace_id, org_id, user_id, &req.title);

    let created = task_repo.create(task).await?;

    tracing::info!(task_id = %created.id, user_id = %user_id, "Task created");
    Ok((StatusCode::CREATED, Json(TaskResponse::from(created))))
}

/// `GET /api/v1/tasks/{id}` — Get a task.
async fn get_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
) -> Result<Json<TaskResponse>> {
    let (_user_id, org_id) = require_org_context(&headers)?;

    let task_repo = PgTaskRepository::new((*state.db_pool).clone());
    let task = task_repo
        .find_by_id(task_id)
        .await?
        .ok_or_else(|| Error::TaskNotFound(task_id.to_string()))?;

    if task.organization_id != org_id {
        return Err(Error::Forbidden(
            "cross-organization access forbidden".to_string(),
        ));
    }

    Ok(Json(TaskResponse::from(task)))
}

/// `PATCH /api/v1/tasks/{id}` — Update a task.
async fn update_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>> {
    let (_user_id, org_id) = require_org_context(&headers)?;

    let task_repo = PgTaskRepository::new((*state.db_pool).clone());
    let existing = task_repo
        .find_by_id(task_id)
        .await?
        .ok_or_else(|| Error::TaskNotFound(task_id.to_string()))?;

    if existing.organization_id != org_id {
        return Err(Error::Forbidden(
            "cross-organization access forbidden".to_string(),
        ));
    }

    let status = req
        .status
        .as_ref()
        .and_then(|s| TaskStatus::from_str(s).ok())
        .unwrap_or(existing.status);

    let updated = Task {
        title: req.title.unwrap_or(existing.title),
        description: req.description.or(existing.description),
        assignee_id: req.assignee_id.or(existing.assignee_id),
        status,
        updated_at: chrono::Utc::now(),
        ..existing
    };
    let result = task_repo.update(task_id, updated.clone()).await?;
    Ok(Json(TaskResponse::from(result)))
}

/// `DELETE /api/v1/tasks/{id}` — Soft-delete a task.
async fn delete_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
) -> Result<StatusCode> {
    let (_user_id, org_id) = require_org_context(&headers)?;

    let task_repo = PgTaskRepository::new((*state.db_pool).clone());
    let existing = task_repo
        .find_by_id(task_id)
        .await?
        .ok_or_else(|| Error::TaskNotFound(task_id.to_string()))?;

    if existing.organization_id != org_id {
        return Err(Error::Forbidden(
            "cross-organization access forbidden".to_string(),
        ));
    }

    task_repo.soft_delete(task_id).await?;
    tracing::info!(task_id = %task_id, "Task soft-deleted");
    Ok(StatusCode::NO_CONTENT)
}
