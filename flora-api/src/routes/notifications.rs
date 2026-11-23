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
use flora_core::models::Notification;
use flora_core::repositories::{PgMembershipRepository, PgNotificationRepository};
use flora_core::traits::{MembershipRepository, NotificationRepository};

use crate::AppState;
use crate::extractors::AuthExtractor;

/// Query parameters for listing notifications.
#[derive(Debug, Deserialize)]
pub struct ListNotificationsQuery {
    /// Maximum number of notifications to return.
    pub limit: Option<i64>,
    /// Whether to return only unread notifications.
    pub unread_only: Option<bool>,
}

/// Response for a notification.
#[derive(Debug, Serialize, PartialEq, Eq)]
pub struct NotificationResponse {
    /// The notification ID.
    pub id: String,
    /// The event type.
    pub event_type: String,
    /// The target resource ID.
    pub target_id: String,
    /// Optional deep-link URL for the notification.
    pub target_url: Option<String>,
    /// Whether the notification has been read.
    pub is_read: bool,
    /// The creation timestamp.
    pub created_at: String,
}

/// Creates the notifications router.
pub fn create_notifications_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_notifications))
        .route("/read-all", post(mark_all_as_read))
        .route("/{id}/read", post(mark_as_read))
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

fn resolve_list_limit(limit: Option<i64>) -> usize {
    limit.map_or(20, |value| {
        usize::try_from(value.clamp(1, 100)).unwrap_or(20)
    })
}

fn to_notification_response(notification: Notification) -> NotificationResponse {
    NotificationResponse {
        id: notification.id.to_string(),
        event_type: notification.event_type.to_string(),
        target_id: notification.target_id.to_string(),
        target_url: notification.target_url,
        is_read: notification.is_read,
        created_at: notification.created_at.to_rfc3339(),
    }
}

/// `GET /api/v1/notifications` — List notifications for the authenticated user in the active organization.
async fn list_notifications(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Query(params): Query<ListNotificationsQuery>,
) -> Result<Json<Vec<NotificationResponse>>> {
    let org_id = ensure_member(&auth, &state).await?;
    let user_id = auth.user_id;
    let limit = resolve_list_limit(params.limit);

    let notif_repo = PgNotificationRepository::new((*state.db_pool).clone());
    let page = if params.unread_only.unwrap_or(false) {
        notif_repo
            .find_unread_by_user_and_organization(
                user_id,
                org_id,
                flora_core::models::Pagination { limit, offset: 0 },
            )
            .await?
    } else {
        notif_repo
            .find_by_user_and_organization(
                user_id,
                org_id,
                flora_core::models::Pagination { limit, offset: 0 },
            )
            .await?
    };

    let results: Vec<NotificationResponse> = page
        .items
        .into_iter()
        .map(to_notification_response)
        .collect();

    Ok(Json(results))
}

/// `POST /api/v1/notifications/{id}/read` — Mark a notification as read in the active organization.
async fn mark_as_read(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path(notif_id): Path<Uuid>,
) -> Result<StatusCode> {
    let org_id = ensure_member(&auth, &state).await?;
    let user_id = auth.user_id;

    let notif_repo = PgNotificationRepository::new((*state.db_pool).clone());
    let notif = notif_repo
        .find_by_id(notif_id)
        .await?
        .ok_or_else(|| Error::NotificationNotFound(notif_id.to_string()))?;

    if notif.organization_id != org_id {
        return Err(Error::NotificationNotFound(notif_id.to_string()));
    }

    if notif.user_id != user_id {
        return Err(Error::Forbidden(
            "cannot mark another user's notification as read".to_string(),
        ));
    }

    notif_repo.mark_as_read(notif_id).await?;
    Ok(StatusCode::NO_CONTENT)
}

/// `POST /api/v1/notifications/read-all` — Mark all notifications for the user in the active organization as read.
async fn mark_all_as_read(
    State(state): State<AppState>,
    auth: AuthExtractor,
) -> Result<StatusCode> {
    let org_id = ensure_member(&auth, &state).await?;
    let notif_repo = PgNotificationRepository::new((*state.db_pool).clone());
    notif_repo
        .mark_all_as_read_in_organization(auth.user_id, org_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use flora_core::models::{Notification, NotificationType};
    use uuid::Uuid;

    use super::{resolve_list_limit, to_notification_response};

    #[test]
    fn test_resolve_list_limit_defaults_and_clamps() {
        assert_eq!(resolve_list_limit(None), 20);
        assert_eq!(resolve_list_limit(Some(-5)), 1);
        assert_eq!(resolve_list_limit(Some(0)), 1);
        assert_eq!(resolve_list_limit(Some(35)), 35);
        assert_eq!(resolve_list_limit(Some(100)), 100);
        assert_eq!(resolve_list_limit(Some(500)), 100);
    }

    #[test]
    fn test_to_notification_response_includes_target_url() {
        let org_id = Uuid::now_v7();
        let user_id = Uuid::now_v7();
        let target_id = Uuid::now_v7();
        let with_url = Notification::new(
            org_id,
            user_id,
            NotificationType::Assignment,
            target_id,
            Some("/tasks".to_string()),
        );
        let mapped = to_notification_response(with_url.clone());
        assert_eq!(mapped.id, with_url.id.to_string());
        assert_eq!(mapped.event_type, "Assignment");
        assert_eq!(mapped.target_id, target_id.to_string());
        assert_eq!(mapped.target_url.as_deref(), Some("/tasks"));
        assert!(!mapped.is_read);

        let without_url =
            Notification::new(org_id, user_id, NotificationType::Reply, target_id, None);
        let mapped_none = to_notification_response(without_url);
        assert_eq!(mapped_none.event_type, "Reply");
        assert_eq!(mapped_none.target_url, None);
    }
}
