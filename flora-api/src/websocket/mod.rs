//! WebSocket module for real-time messaging with organization-scoped connections.
//!
//! Per T034.1: Add `organization_id` validation on WebSocket connection and message publishing.

use axum::{
    extract::{
        State,
        ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use flora_core::{
    error::{Error, Result},
    repositories::PgMembershipRepository,
    traits::MembershipRepository,
};
use futures::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::{AppState, extractors::AuthExtractor};

/// WebSocket message types for client-server communication.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsClientMessage {
    /// Subscribe to a channel's real-time updates.
    Subscribe {
        /// The channel ID to subscribe to.
        channel_id: Uuid,
    },
    /// Unsubscribe from a channel.
    Unsubscribe {
        /// The channel ID to unsubscribe from.
        channel_id: Uuid,
    },
    /// Send a new message to a channel.
    SendMessage {
        /// The channel ID to send the message to.
        channel_id: Uuid,
        /// The message content.
        content: String,
        /// Optional thread ID for replies.
        thread_id: Option<Uuid>,
    },
    /// Ping to keep connection alive.
    Ping,
}

/// Server-to-client WebSocket messages.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum WsServerMessage {
    /// Confirmation of successful subscription.
    Subscribed { channel_id: Uuid },
    /// Confirmation of successful unsubscription.
    Unsubscribed { channel_id: Uuid },
    /// A new message received in a subscribed channel.
    NewMessage {
        channel_id: Uuid,
        message_id: uuid::Uuid,
        sender_id: Uuid,
        content: String,
        thread_id: Option<Uuid>,
        created_at: String,
    },
    /// Message sent confirmation.
    MessageSent { message_id: Uuid, channel_id: Uuid },
    /// Pong response to ping.
    Pong,
    /// Error message.
    Error { code: String, message: String },
}

/// Handles WebSocket upgrade and connection.
///
/// Per T034.1: Validates organization_id on connection and enforces
/// multi-tenancy isolation for all WebSocket operations.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<crate::AppState>,
    headers: axum::http::HeaderMap,
) -> Response {
    // Extract and validate user and organization from headers
    let (user_id, org_id) = match extract_org_context(&headers) {
        Ok(ctx) => ctx,
        Err(e) => {
            tracing::warn!("WebSocket connection rejected: {}", e);
            return Response::builder()
                .status(axum::http::StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::empty())
                .unwrap();
        }
    };

    tracing::info!(user_id = %user_id, org_id = %org_id, "WebSocket connection established");

    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id, org_id))
}

/// Extracts user and organization context from headers.
fn extract_org_context(headers: &axum::http::HeaderMap) -> Result<(Uuid, Uuid)> {
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

/// Handles an established WebSocket connection.
async fn handle_socket(mut socket: WebSocket, state: crate::AppState, user_id: Uuid, org_id: Uuid) {
    let mut redis = state.redis.clone();
    let mut subscribed_channels = std::collections::HashSet::<Uuid>::new();

    // Spawn a task to handle incoming WebSocket messages
    let mut sender = socket.sender();
    let mut receiver = socket.receiver();

    // Channel for sending messages to the client
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<WsServerMessage>();

    // Spawn task to send messages to the client
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender
                .send(WsMessage::Text(serde_json::to_string(&msg).unwrap()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    // Handle incoming messages
    while let Some(Ok(msg)) = receiver.next().await {
        if let WsMessage::Text(text) = msg {
            if let Ok(client_msg) = serde_json::from_str::<WsClientMessage>(&text) {
                match client_msg {
                    WsClientMessage::Subscribe { channel_id } => {
                        // Validate user has access to this channel in the organization
                        if let Err(e) =
                            validate_channel_access(&state, user_id, org_id, channel_id).await
                        {
                            let _ = tx.send(WsServerMessage::Error {
                                code: "FORBIDDEN".to_string(),
                                message: e.to_string(),
                            });
                            continue;
                        }

                        // Subscribe to Valkey channel
                        let valkey_channel = format!("org:{}:channel:{}", org_id, channel_id);
                        if let Err(e) = redis::cmd("SUBSCRIBE")
                            .arg(&valkey_channel)
                            .query_async(&mut redis.clone())
                            .await
                        {
                            tracing::error!("Failed to subscribe to Valkey channel: {}", e);
                            let _ = tx.send(WsServerMessage::Error {
                                code: "SUBSCRIPTION_FAILED".to_string(),
                                message: "Failed to subscribe to channel".to_string(),
                            });
                            continue;
                        }

                        subscribed_channels.insert(channel_id);
                        let _ = tx.send(WsServerMessage::Subscribed { channel_id });
                        tracing::debug!(user_id = %user_id, channel_id = %channel_id, "Subscribed to channel");
                    }
                    WsClientMessage::Unsubscribe { channel_id } => {
                        let valkey_channel = format!("org:{}:channel:{}", org_id, channel_id);
                        let _ = redis::cmd("UNSUBSCRIBE")
                            .arg(&valkey_channel)
                            .query_async(&mut redis.clone())
                            .await;
                        subscribed_channels.remove(&channel_id);
                        let _ = tx.send(WsServerMessage::Unsubscribed { channel_id });
                        tracing::debug!(user_id = %user_id, channel_id = %channel_id, "Unsubscribed from channel");
                    }
                    WsClientMessage::SendMessage {
                        channel_id,
                        content,
                        thread_id,
                    } => {
                        // Validate user has access to this channel
                        if let Err(e) =
                            validate_channel_access(&state, user_id, org_id, channel_id).await
                        {
                            let _ = tx.send(WsServerMessage::Error {
                                code: "FORBIDDEN".to_string(),
                                message: e.to_string(),
                            });
                            continue;
                        }

                        // Create message in database
                        let msg =
                            flora_core::models::Message::new(channel_id, org_id, user_id, &content);
                        let msg_repo = flora_core::repositories::PgMessageRepository::new(
                            (*state.db_pool).clone(),
                        );
                        let created = match msg_repo.create(message).await {
                            Ok(m) => m,
                            Err(e) => {
                                tracing::error!("Failed to create message: {}", e);
                                let _ = tx.send(WsServerMessage::Error {
                                    code: "MESSAGE_CREATE_FAILED".to_string(),
                                    message: "Failed to create message".to_string(),
                                });
                                continue;
                            }
                        };

                        // Publish to Valkey for real-time delivery
                        let valkey_msg = flora_messaging::Message {
                            channel_id,
                            organization_id: org_id,
                            sender_id: user_id,
                            content: content.clone(),
                            thread_id: thread_id,
                        };
                        let messaging = flora_messaging::MessagingService::new(state.redis.clone());
                        if let Err(e) = messaging.publish(valkey_msg).await {
                            tracing::error!("Failed to publish message to Valkey: {}", e);
                        }

                        let _ = tx.send(WsServerMessage::MessageSent {
                            message_id: created.id,
                            channel_id,
                        });
                    }
                    WsClientMessage::Ping => {
                        let _ = tx.send(WsServerMessage::Pong);
                    }
                }
            }
        }
    }

    // Cleanup subscriptions on disconnect
    for channel_id in subscribed_channels {
        let valkey_channel = format!("org:{}:channel:{}", org_id, channel_id);
        let _ = redis::cmd("UNSUBSCRIBE")
            .arg(&valkey_channel)
            .query_async(&mut redis)
            .await;
    }

    tracing::info!(user_id = %user_id, org_id = %org_id, "WebSocket connection closed");
}

/// Validates that a user has access to a channel in the given organization.
async fn validate_channel_access(
    state: &crate::AppState,
    user_id: Uuid,
    org_id: Uuid,
    channel_id: Uuid,
) -> Result<()> {
    let channel_repo = flora_core::repositories::PgChannelRepository::new((*state.db_pool).clone());
    let channel = channel_repo
        .find_by_id(channel_id)
        .await?
        .ok_or_else(|| flora_core::Error::ChannelNotFound(channel_id.to_string()))?;

    if channel.organization_id != org_id {
        return Err(flora_core::Error::ChannelNotAccessible(
            channel_id.to_string(),
        ));
    }

    // Check if user is a member of the organization
    let membership_repo =
        flora_core::repositories::PgMembershipRepository::new((*state.db_pool).clone());
    let membership = membership_repo
        .find_by_user_and_organization(user_id, org_id)
        .await?
        .ok_or_else(|| {
            flora_core::Error::Forbidden("Not a member of this organization".to_string())
        })?;

    // For private channels, we might need additional checks here
    // For now, any organization member can access any channel
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_message_serialization() {
        let msg = WsClientMessage::Subscribe {
            channel_id: Uuid::now_v7(),
        };
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Subscribe"));
    }
}
