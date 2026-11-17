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
use flora_core::models::{Notification, NotificationType, Task, TaskStatus};
use flora_core::repositories::{
    PgMembershipRepository, PgNotificationRepository, PgTaskRepository,
};
use flora_core::traits::{MembershipRepository, NotificationRepository, TaskRepository};
use std::str::FromStr;

use crate::AppState;
use crate::extractors::AuthExtractor;
use crate::tenant::{ensure_user_in_org, ensure_workspace_in_org};

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

/// Deserialize the tri-state `assignee_id` of a PATCH request.
///
/// Serde maps an explicit JSON `null` on a plain `Option<Option<Uuid>>` to the
/// outer `None`, making it indistinguishable from an omitted field. With
/// `#[serde(default)]`, missing fields use the default (`None`) without calling
/// this function, so here `null` can safely mean "explicitly clear".
#[expect(
    clippy::option_option,
    reason = "tri-state PATCH assignee: omit vs explicit null vs UUID value must stay distinguishable"
)]
fn deserialize_tri_state_assignee<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<Option<Uuid>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(Some(None)),
        serde_json::Value::String(s) => Uuid::parse_str(&s)
            .map(|uuid| Some(Some(uuid)))
            .map_err(serde::de::Error::custom),
        other => Err(serde::de::Error::custom(format!(
            "assignee_id must be null or a UUID string, got {other}"
        ))),
    }
}

/// Request body for updating a task.
///
/// `assignee_id` is tri-state (021, merge-patch semantics):
/// - omitted => no change
/// - `null` => explicit unassign/clear
/// - `"<uuid>"` => assign/set
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    /// Optional new title.
    pub title: Option<String>,
    /// Optional new description.
    pub description: Option<String>,
    /// Optional new status.
    pub status: Option<String>,
    /// Optional new assignee (tri-state: omit=no change, null=clear).
    #[serde(default, deserialize_with = "deserialize_tri_state_assignee")]
    pub assignee_id: Option<Option<Uuid>>,
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

/// `GET /api/v1/tasks` — List tasks.
async fn list_tasks(
    State(state): State<AppState>,
    auth: AuthExtractor,
) -> Result<Json<Vec<TaskResponse>>> {
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
        .filter(|t| !t.is_deleted && t.organization_id == org_id)
        .map(TaskResponse::from)
        .collect();

    Ok(Json(tasks))
}

/// `POST /api/v1/tasks` — Create a task.
async fn create_task(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<TaskResponse>)> {
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

    // C045 / OWASP A01:2021 (CWE-639/BOLA): workspace_id from the request body is a
    // selector, not proof of authorization. Reject a workspace outside the session's
    // verified organization before creating the task (defense in depth alongside RLS).
    ensure_workspace_in_org(&state, org_id, req.workspace_id).await?;
    // Assignee (when provided) must belong to the session's verified org (BOLA guard).
    if let Some(assignee_id) = req.assignee_id {
        ensure_user_in_org(&state, org_id, assignee_id).await?;
    }

    let task_repo = PgTaskRepository::new((*state.db_pool).clone());
    let mut task = Task::new(req.workspace_id, org_id, user_id, &req.title);
    task.description = req.description;
    task.assignee_id = req.assignee_id;

    let created = task_repo.create(task).await?;

    // 020/021: best-effort assignment notification on effective initial assignment.
    if let Some(assignee_id) = created.assignee_id
        && let Err(e) = maybe_notify_task_assignment(
            &state,
            org_id,
            None,
            Some(assignee_id),
            user_id,
            created.id,
        )
        .await
    {
        tracing::warn!(
            error = %e,
            task_id = %created.id,
            assignee_id = %assignee_id,
            "Failed to create task assignment notification"
        );
    }

    tracing::info!(task_id = %created.id, user_id = %user_id, "Task created");
    Ok((StatusCode::CREATED, Json(TaskResponse::from(created))))
}

/// `GET /api/v1/tasks/{id}` — Get a task.
async fn get_task(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path(task_id): Path<Uuid>,
) -> Result<Json<TaskResponse>> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;

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
    auth: AuthExtractor,
    Path(task_id): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponse>> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;

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

    // 021 tri-state: omit => keep, null => clear, value => set (validate only a set).
    if let Some(Some(assignee_id)) = req.assignee_id {
        ensure_user_in_org(&state, org_id, assignee_id).await?;
    }
    let old_assignee_id = existing.assignee_id;
    let new_assignee_id = req.assignee_id.unwrap_or(old_assignee_id);

    let status = req
        .status
        .as_ref()
        .and_then(|s| TaskStatus::from_str(s).ok())
        .unwrap_or(existing.status);

    let updated = Task {
        title: req.title.unwrap_or(existing.title),
        description: req.description.or(existing.description),
        assignee_id: new_assignee_id,
        status,
        updated_at: chrono::Utc::now(),
        ..existing
    };
    let result = task_repo.update(task_id, updated).await?;

    // 021: notify only on effective transition to an assigned, non-actor user.
    if let Err(e) = maybe_notify_task_assignment(
        &state,
        org_id,
        old_assignee_id,
        new_assignee_id,
        auth.user_id,
        result.id,
    )
    .await
    {
        tracing::warn!(
            error = %e,
            task_id = %result.id,
            assignee_id = ?new_assignee_id,
            "Failed to create task assignment notification"
        );
    }

    Ok(Json(TaskResponse::from(result)))
}

async fn maybe_notify_task_assignment(
    state: &AppState,
    org_id: Uuid,
    old_assignee_id: Option<Uuid>,
    new_assignee_id: Option<Uuid>,
    actor_id: Uuid,
    task_id: Uuid,
) -> Result<()> {
    let Some(assignee_id) = new_assignee_id else {
        return Ok(());
    };
    if Some(assignee_id) == old_assignee_id || assignee_id == actor_id {
        return Ok(());
    }

    let notif_repo = PgNotificationRepository::new((*state.db_pool).clone());
    let notification = Notification::new(
        org_id,
        assignee_id,
        NotificationType::Assignment,
        task_id,
        None,
    );
    // 022: DB-enforced unread dedup (conflict-safe/idempotent); suppression is not an error.
    let _ = notif_repo.create(notification).await?;
    Ok(())
}

/// `DELETE /api/v1/tasks/{id}` — Soft-delete a task.
async fn delete_task(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path(task_id): Path<Uuid>,
) -> Result<StatusCode> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;

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
