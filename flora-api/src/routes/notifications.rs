//! Notification API routes.

use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::repositories::PgNotificationRepository;
use flora_core::traits::NotificationRepository;

use crate::AppState;

/// Query parameters for listing notifications.
#[derive(Debug, Deserialize)]
pub struct ListNotificationsQuery {
    /// Maximum number of notifications to return.
    pub limit: Option<i64>,
    /// Whether to return only unread notifications.
    pub unread_only: Option<bool>,
}

/// Response for a notification.
#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    /// The notification ID.
    pub id: String,
    /// The event type.
    pub event_type: String,
    /// The target resource ID.
    pub target_id: String,
    /// Whether the notification has been read.
    pub is_read: bool,
    /// The creation timestamp.
    pub created_at: String,
}

/// Creates the notifications router.
pub fn create_notifications_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_notifications))
        .route("/{id}/read", post(mark_as_read))
}

fn require_user_context(headers: &axum::http::HeaderMap) -> Result<Uuid> {
    let user_id = headers
        .get("x-user-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or(Error::Unauthorized)?;
    Ok(user_id)
}

/// `GET /api/v1/notifications` — List notifications for the authenticated user.
async fn list_notifications(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Query(params): Query<ListNotificationsQuery>,
) -> Result<Json<Vec<NotificationResponse>>> {
    let user_id = require_user_context(&headers)?;
    let limit = usize::try_from(params.limit.unwrap_or(20).min(100)).unwrap_or(20);

    let notif_repo = PgNotificationRepository::new((*state.db_pool).clone());
    let page = if params.unread_only.unwrap_or(false) {
        notif_repo
            .find_unread_by_user_id(user_id, flora_core::models::Pagination { limit, offset: 0 })
            .await?
    } else {
        notif_repo
            .find_by_user_id(user_id, flora_core::models::Pagination { limit, offset: 0 })
            .await?
    };

    let results: Vec<NotificationResponse> = page
        .items
        .into_iter()
        .map(|n| NotificationResponse {
            id: n.id.to_string(),
            event_type: n.event_type.to_string(),
            target_id: n.target_id.to_string(),
            is_read: n.is_read,
            created_at: n.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(results))
}

/// `POST /api/v1/notifications/{id}/read` — Mark a notification as read.
async fn mark_as_read(
    State(state): State<AppState>,
    Path(notif_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
) -> Result<StatusCode> {
    let user_id = require_user_context(&headers)?;

    let notif_repo = PgNotificationRepository::new((*state.db_pool).clone());
    let notif = notif_repo
        .find_by_id(notif_id)
        .await?
        .ok_or_else(|| Error::NotificationNotFound(notif_id.to_string()))?;

    if notif.user_id != user_id {
        return Err(Error::Forbidden(
            "cannot mark another user's notification as read".to_string(),
        ));
    }

    notif_repo.mark_as_read(notif_id).await?;
    Ok(StatusCode::NO_CONTENT)
}
