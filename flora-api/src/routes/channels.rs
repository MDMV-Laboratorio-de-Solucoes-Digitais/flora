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
use flora_core::models::{Channel, Message};
use flora_core::repositories::{PgChannelRepository, PgMembershipRepository, PgMessageRepository};
use flora_core::traits::{ChannelRepository, MembershipRepository, MessageRepository};

use crate::AppState;

/// Query params for listing messages.
#[derive(Debug, Deserialize)]
pub struct ListMessagesQuery {
    /// Maximum number of messages to return.
    pub limit: Option<i64>,
    /// Return messages before this ID (for pagination).
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
#[derive(Debug, Serialize)]
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

/// Creates the channels router.
pub fn create_channels_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_channel))
        .route("/{channel_id}/messages", get(list_messages))
        .route("/{channel_id}/messages", post(post_message))
        .route("/{channel_id}/messages/{msg_id}", patch(edit_message))
        .route("/{channel_id}/messages/{msg_id}", delete(delete_message))
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

/// `POST /api/v1/channels` — Create a channel in a workspace.
async fn create_channel(
    State(state): State<AppState>,
    headers: axum::http::HeaderMap,
    Json(req): Json<CreateChannelRequest>,
) -> Result<(StatusCode, Json<ChannelResponse>)> {
    let (user_id, org_id) = require_org_context(&headers)?;

    let membership_repo = PgMembershipRepository::new((*state.db_pool).clone());
    let _membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| Error::Forbidden("Not a member".to_string()))?;

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
async fn list_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
    Query(params): Query<ListMessagesQuery>,
) -> Result<Json<Vec<MessageResponse>>> {
    let (_user_id, org_id) = require_org_context(&headers)?;

    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    let channel = channel_repo
        .find_by_id(channel_id)
        .await?
        .ok_or_else(|| Error::ChannelNotFound(channel_id.to_string()))?;

    if channel.organization_id != org_id {
        return Err(Error::ChannelNotAccessible(channel_id.to_string()));
    }

    let limit = usize::try_from(params.limit.unwrap_or(50).min(100)).unwrap_or(50);
    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());
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
    Path(channel_id): Path<Uuid>,
    headers: axum::http::HeaderMap,
    Json(req): Json<CreateMessageRequest>,
) -> Result<(StatusCode, Json<MessageResponse>)> {
    let (user_id, org_id) = require_org_context(&headers)?;

    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    let channel = channel_repo
        .find_by_id(channel_id)
        .await?
        .ok_or_else(|| Error::ChannelNotFound(channel_id.to_string()))?;

    if channel.organization_id != org_id {
        return Err(Error::ChannelNotAccessible(channel_id.to_string()));
    }

    if req.content.is_empty() {
        return Err(Error::Validation {
            field: "content".to_string(),
            message: "message content cannot be empty".to_string(),
        });
    }

    let message = Message::new(channel_id, org_id, user_id, &req.content);
    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());
    let created = msg_repo.create(message).await?;

    tracing::debug!(message_id = %created.id, channel_id = %channel_id, user_id = %user_id, "Message posted");
    Ok((StatusCode::CREATED, Json(MessageResponse::from(created))))
}

/// `PATCH /api/v1/channels/{channel_id}/messages/{msg_id}` — Edit a message.
async fn edit_message(
    State(state): State<AppState>,
    Path((_channel_id, msg_id)): Path<(Uuid, Uuid)>,
    headers: axum::http::HeaderMap,
    Json(req): Json<EditMessageRequest>,
) -> Result<Json<MessageResponse>> {
    let (user_id, org_id) = require_org_context(&headers)?;

    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());
    let msg = msg_repo
        .find_by_id(msg_id)
        .await?
        .ok_or_else(|| Error::MessageNotFound(msg_id.to_string()))?;

    if msg.organization_id != org_id {
        return Err(Error::Forbidden(
            "cross-organization access forbidden".to_string(),
        ));
    }
    if msg.sender_id != user_id {
        return Err(Error::Forbidden(
            "you can only edit your own messages".to_string(),
        ));
    }

    let updated = Message {
        content: req.content,
        is_edited: true,
        updated_at: chrono::Utc::now(),
        ..msg
    };
    let result = msg_repo.update(msg_id, updated.clone()).await?;
    Ok(Json(MessageResponse::from(result)))
}

/// `DELETE /api/v1/channels/{channel_id}/messages/{msg_id}` — Soft-delete a message.
async fn delete_message(
    State(state): State<AppState>,
    Path((_channel_id, msg_id)): Path<(Uuid, Uuid)>,
    headers: axum::http::HeaderMap,
) -> Result<StatusCode> {
    let (user_id, org_id) = require_org_context(&headers)?;

    let msg_repo = PgMessageRepository::new((*state.db_pool).clone());
    let msg = msg_repo
        .find_by_id(msg_id)
        .await?
        .ok_or_else(|| Error::MessageNotFound(msg_id.to_string()))?;

    if msg.organization_id != org_id {
        return Err(Error::Forbidden(
            "cross-organization access forbidden".to_string(),
        ));
    }
    if msg.sender_id != user_id {
        return Err(Error::Forbidden(
            "you can only delete your own messages".to_string(),
        ));
    }

    msg_repo.soft_delete(msg_id).await?;
    tracing::info!(message_id = %msg_id, user_id = %user_id, "Message soft-deleted");
    Ok(StatusCode::NO_CONTENT)
}
