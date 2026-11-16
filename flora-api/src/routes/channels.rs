//! Channel and message routes for real-time messaging.
//!
//! Per T035, T036.

use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use flora_core::error::{Error, Result};
use flora_core::models::{Channel, Message, Notification, NotificationType};
use flora_core::repositories::{
    PgChannelRepository, PgMembershipRepository, PgMessageRepository, PgNotificationRepository,
    PgWorkspaceRepository,
};
use flora_core::traits::{
    ChannelRepository, MembershipRepository, MessageRepository, NotificationRepository,
    WorkspaceRepository,
};

use crate::AppState;
use crate::extractors::AuthExtractor;
use crate::realtime::{self, HubEnvelope, MessageEventPayload, RealtimeEvent};

/// Query params for listing channels in a workspace.
#[derive(Debug, Deserialize)]
pub struct ListChannelsQuery {
    /// Workspace to list channels for.
    pub workspace_id: Uuid,
}

/// Query params for listing messages.
#[derive(Debug, Deserialize)]
pub struct ListMessagesQuery {
    /// Maximum number of messages to return.
    pub limit: Option<i64>,
    /// Return messages after this ID (cursor catch-up, ascending).
    pub after: Option<Uuid>,
    /// Accepted for compatibility (currently not used for keyset paging).
    pub before: Option<Uuid>,
}

/// Request body for creating a channel.
#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    /// The workspace ID where the channel will be created.
    pub workspace_id: Uuid,
    /// The name of the channel.
    pub name: String,
    /// The type of channel ("public" or "private").
    #[serde(rename = "type")]
    pub channel_type: Option<String>,
}

/// Request body for creating a message.
#[derive(Debug, Deserialize)]
pub struct CreateMessageRequest {
    /// The message content.
    pub content: String,
    /// Optional thread ID for replies.
    pub thread_id: Option<Uuid>,
}

/// Request body for editing a message.
#[derive(Debug, Deserialize)]
pub struct EditMessageRequest {
    /// The new message content.
    pub content: String,
}

/// Response for a channel.
#[derive(Debug, Serialize)]
pub struct ChannelResponse {
    /// The channel ID.
    pub id: String,
    /// The workspace ID.
    pub workspace_id: String,
    /// The channel name.
    pub name: String,
    /// The channel type.
    #[serde(rename = "type")]
    pub channel_type: String,
    /// The creation timestamp.
    pub created_at: String,
}

/// Response for a message.
#[derive(Debug, Serialize, Clone)]
pub struct MessageResponse {
    /// The message ID.
    pub id: String,
    /// The channel ID.
    pub channel_id: String,
    /// The sender's user ID.
    pub sender_id: String,
    /// The message content.
    pub content: String,
    /// The thread ID if this is a reply.
    pub thread_id: Option<String>,
    /// Whether the message has been edited.
    pub is_edited: bool,
    /// The creation timestamp.
    pub created_at: String,
    /// The last update timestamp.
    pub updated_at: String,
}

impl From<Message> for MessageResponse {
    fn from(msg: Message) -> Self {
        Self {
            id: msg.id.to_string(),
            channel_id: msg.channel_id.to_string(),
            sender_id: msg.sender_id.to_string(),
            content: msg.content,
            thread_id: msg.thread_id.map(|id| id.to_string()),
            is_edited: msg.is_edited,
            created_at: msg.created_at.to_rfc3339(),
            updated_at: msg.updated_at.to_rfc3339(),
        }
    }
}

impl From<&MessageResponse> for MessageEventPayload {
    fn from(msg: &MessageResponse) -> Self {
        Self {
            id: msg.id.clone(),
            channel_id: msg.channel_id.clone(),
            sender_id: msg.sender_id.clone(),
            content: msg.content.clone(),
            thread_id: msg.thread_id.clone(),
            is_edited: msg.is_edited,
            created_at: msg.created_at.clone(),
            updated_at: msg.updated_at.clone(),
        }
    }
}

impl From<Channel> for ChannelResponse {
    fn from(ch: Channel) -> Self {
        Self {
            id: ch.id.to_string(),
            workspace_id: ch.workspace_id.to_string(),
            name: ch.name,
            channel_type: ch.channel_type.to_string(),
            created_at: ch.created_at.to_rfc3339(),
        }
    }
}

fn emit_event(state: &AppState, org_id: Uuid, channel_id: Uuid, event: RealtimeEvent) {
    realtime::publish(
        &state.realtime_hub,
        HubEnvelope::new(org_id, channel_id, event),
    );
}

/// Creates the channels router.
pub fn create_channels_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_channels))
        .route("/", post(create_channel))
        .route("/{channel_id}/messages", get(list_messages))
        .route("/{channel_id}/messages", post(post_message))
        .route("/{channel_id}/messages/{msg_id}", patch(edit_message))
        .route("/{channel_id}/messages/{msg_id}", delete(delete_message))
}

/// `GET /api/v1/channels?workspace_id=...` — List channels in a workspace.
async fn list_channels(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Query(params): Query<ListChannelsQuery>,
) -> Result<Json<Vec<ChannelResponse>>> {
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

    let workspace_repo = PgWorkspaceRepository::new((*state.db_pool).clone());
    let workspace = workspace_repo
        .find_by_id(params.workspace_id)
        .await?
        .ok_or_else(|| Error::Validation {
            field: "workspace_id".to_string(),
            message: "workspace not found".to_string(),
        })?;
    if workspace.organization_id != org_id {
        return Err(Error::Forbidden(
            "Workspace belongs to a different organization".to_string(),
        ));
    }

    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    let channels = channel_repo
        .find_by_workspace_id(params.workspace_id)
        .await?;
    Ok(Json(
        channels.into_iter().map(ChannelResponse::from).collect(),
    ))
}

/// `POST /api/v1/channels` — Create a channel in a workspace.
async fn create_channel(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Json(req): Json<CreateChannelRequest>,
) -> Result<(StatusCode, Json<ChannelResponse>)> {
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

    let workspace_repo = PgWorkspaceRepository::new((*state.db_pool).clone());
    let workspace = workspace_repo
        .find_by_id(req.workspace_id)
        .await?
        .ok_or_else(|| Error::Validation {
            field: "workspace_id".to_string(),
            message: "workspace not found".to_string(),
        })?;
    if workspace.organization_id != org_id {
        return Err(Error::Forbidden(
            "Workspace belongs to a different organization".to_string(),
        ));
    }

    let channel_type =
        req.channel_type
            .as_deref()
            .map_or(flora_core::models::ChannelType::Public, |s| match s {
                "Private" | "private" => flora_core::models::ChannelType::Private,
                _ => flora_core::models::ChannelType::Public,
            });
    let channel = Channel {
        id: Uuid::now_v7(),
        workspace_id: req.workspace_id,
        organization_id: org_id,
        name: req.name,
        channel_type,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    let created = channel_repo.create(channel).await?;

    tracing::info!(channel_id = %created.id, workspace_id = %req.workspace_id, user_id = %user_id, "Channel created");
    Ok((StatusCode::CREATED, Json(ChannelResponse::from(created))))
}

/// `GET /api/v1/channels/{channel_id}/messages` — List messages in a channel.
///
/// - With `after=<message_id>`: returns catch-up messages after cursor in ascending timeline order.
/// - Without `after`: returns recent messages (current behavior, newest-first).
async fn list_messages(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path(channel_id): Path<Uuid>,
    Query(params): Query<ListMessagesQuery>,
) -> Result<Json<Vec<MessageResponse>>> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;

    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    let channel = channel_repo
        .find_by_id(channel_id)
        .await?
        .ok_or_else(|| Error::ChannelNotFound(channel_id.to_string()))?;

    if channel.organization_id != org_id {
        return Err(Error::ChannelNotAccessible(channel_id.to_string()));
    }

    let limit = usize::try_from(params.limit.unwrap_or(50).clamp(1, 100)).unwrap_or(50);
    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());

    if let Some(after_id) = params.after {
        let msgs = msg_repo
            .find_by_channel_id_after(channel_id, after_id, limit)
            .await?;
        let resp: Vec<MessageResponse> = msgs
            .into_iter()
            .filter(|m| !m.is_deleted)
            .map(MessageResponse::from)
            .collect();
        return Ok(Json(resp));
    }

    let page = msg_repo
        .find_by_channel_id(
            channel_id,
            flora_core::models::Pagination { limit, offset: 0 },
        )
        .await?;

    let msgs: Vec<MessageResponse> = page
        .items
        .into_iter()
        .filter(|m| !m.is_deleted)
        .map(MessageResponse::from)
        .collect();

    Ok(Json(msgs))
}

/// `POST /api/v1/channels/{channel_id}/messages` — Post a message.
async fn post_message(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path(channel_id): Path<Uuid>,
    Json(req): Json<CreateMessageRequest>,
) -> Result<(StatusCode, Json<MessageResponse>)> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;
    let user_id = auth.user_id;

    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    let channel = channel_repo
        .find_by_id(channel_id)
        .await?
        .ok_or_else(|| Error::ChannelNotFound(channel_id.to_string()))?;

    if channel.organization_id != org_id {
        return Err(Error::ChannelNotAccessible(channel_id.to_string()));
    }

    if req.content.trim().is_empty() {
        return Err(Error::Validation {
            field: "content".to_string(),
            message: "message content cannot be empty".to_string(),
        });
    }

    let mut message = Message::new(channel_id, org_id, user_id, &req.content);
    message.thread_id = req.thread_id;
    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());
    let created = msg_repo.create(message).await?;

    // 019: best-effort reply notification (must not fail the message path).
    if let Some(parent_id) = created.thread_id
        && let Err(e) = maybe_notify_reply(
            &state,
            org_id,
            channel_id,
            parent_id,
            created.sender_id,
            created.id,
        )
        .await
    {
        tracing::warn!(
            error = %e,
            message_id = %created.id,
            parent_id = %parent_id,
            "Failed to create reply notification"
        );
    }

    let resp = MessageResponse::from(created);

    emit_event(
        &state,
        org_id,
        channel_id,
        RealtimeEvent::NewMessage {
            channel_id,
            message: MessageEventPayload::from(&resp),
        },
    );

    tracing::debug!(message_id = %resp.id, channel_id = %channel_id, user_id = %user_id, "Message posted");
    Ok((StatusCode::CREATED, Json(resp)))
}

async fn maybe_notify_reply(
    state: &AppState,
    org_id: Uuid,
    channel_id: Uuid,
    parent_id: Uuid,
    sender_id: Uuid,
    reply_id: Uuid,
) -> Result<()> {
    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());
    let Some(parent) = msg_repo.find_by_id(parent_id).await? else {
        return Ok(());
    };

    if parent.is_deleted
        || parent.organization_id != org_id
        || parent.channel_id != channel_id
        || parent.sender_id == sender_id
    {
        return Ok(());
    }

    let notif_repo = PgNotificationRepository::new((*state.db_pool).clone());
    let notification = Notification::new(
        org_id,
        parent.sender_id,
        NotificationType::Reply,
        reply_id,
        None,
    );
    // 022: conflict-safe unread dedup; suppression is not an error.
    let _ = notif_repo.create(notification).await?;
    Ok(())
}

/// `PATCH /api/v1/channels/{channel_id}/messages/{msg_id}` — Edit a message.
async fn edit_message(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path((channel_id, msg_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<EditMessageRequest>,
) -> Result<Json<MessageResponse>> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;
    let user_id = auth.user_id;

    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    let channel = channel_repo
        .find_by_id(channel_id)
        .await?
        .ok_or_else(|| Error::ChannelNotFound(channel_id.to_string()))?;
    if channel.organization_id != org_id {
        return Err(Error::ChannelNotAccessible(channel_id.to_string()));
    }

    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());
    let existing = msg_repo
        .find_by_id(msg_id)
        .await?
        .ok_or_else(|| Error::MessageNotFound(msg_id.to_string()))?;

    if existing.channel_id != channel_id || existing.organization_id != org_id {
        return Err(Error::MessageNotFound(msg_id.to_string()));
    }
    if existing.sender_id != user_id {
        return Err(Error::Forbidden("Not the message author".to_string()));
    }
    if req.content.trim().is_empty() {
        return Err(Error::Validation {
            field: "content".to_string(),
            message: "message content cannot be empty".to_string(),
        });
    }

    let mut updated = existing.clone();
    updated.content = req.content;
    let saved = msg_repo.update(msg_id, updated).await?;
    let resp = MessageResponse::from(saved);

    emit_event(
        &state,
        org_id,
        channel_id,
        RealtimeEvent::MessageUpdated {
            channel_id,
            message: MessageEventPayload::from(&resp),
        },
    );

    Ok(Json(resp))
}

/// `DELETE /api/v1/channels/{channel_id}/messages/{msg_id}` — Delete a message.
async fn delete_message(
    State(state): State<AppState>,
    auth: AuthExtractor,
    Path((channel_id, msg_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode> {
    let org_id = auth.require_org().map_err(|(_s, m)| Error::Validation {
        field: "organization_id".to_string(),
        message: m,
    })?;
    let user_id = auth.user_id;

    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    let channel = channel_repo
        .find_by_id(channel_id)
        .await?
        .ok_or_else(|| Error::ChannelNotFound(channel_id.to_string()))?;
    if channel.organization_id != org_id {
        return Err(Error::ChannelNotAccessible(channel_id.to_string()));
    }

    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());
    let existing = msg_repo
        .find_by_id(msg_id)
        .await?
        .ok_or_else(|| Error::MessageNotFound(msg_id.to_string()))?;

    if existing.channel_id != channel_id || existing.organization_id != org_id {
        return Err(Error::MessageNotFound(msg_id.to_string()));
    }
    if existing.sender_id != user_id {
        return Err(Error::Forbidden("Not the message author".to_string()));
    }

    msg_repo.soft_delete(msg_id).await?;
    let deleted = msg_repo.find_by_id(msg_id).await?.unwrap_or(existing);
    let resp = MessageResponse::from(deleted);

    emit_event(
        &state,
        org_id,
        channel_id,
        RealtimeEvent::MessageDeleted {
            channel_id,
            message: MessageEventPayload::from(&resp),
        },
    );

    Ok(StatusCode::NO_CONTENT)
}
