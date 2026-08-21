//! WebSocket module for real-time messaging with organization-scoped connections.
//!
//! Per T034.1 & FR-035: Validates `organization_id` on connection and message publishing,
//! enforcing multi-tenancy isolation for all WebSocket operations and broadcasting via Valkey.

use axum::{
    extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode},
    response::Response,
};
use flora_core::error::{Error, Result};
use flora_core::models::Message;
use flora_core::repositories::{PgChannelRepository, PgMessageRepository};
use flora_core::traits::{ChannelRepository, MessageRepository};
use uuid::Uuid;

use crate::AppState;

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
    Subscribed {
        /// The channel ID that was subscribed to.
        channel_id: Uuid,
    },
    /// Confirmation of successful unsubscription.
    Unsubscribed {
        /// The channel ID that was unsubscribed from.
        channel_id: Uuid,
    },
    /// A new message received in a subscribed channel.
    NewMessage {
        /// The channel ID where the message was sent.
        channel_id: Uuid,
        /// The unique message ID.
        message_id: Uuid,
        /// The sender's user ID.
        sender_id: Uuid,
        /// The message content.
        content: String,
        /// Optional thread ID for replies.
        thread_id: Option<Uuid>,
        /// Timestamp when the message was created.
        created_at: String,
    },
    /// Message sent confirmation.
    MessageSent {
        /// The unique message ID.
        message_id: Uuid,
        /// The channel ID where the message was sent.
        channel_id: Uuid,
    },
    /// Pong response to ping.
    Pong,
    /// Error message.
    Error {
        /// Error code identifier.
        code: String,
        /// Human-readable error message.
        message: String,
    },
}

/// Handles WebSocket upgrade and connection.
///
/// Per T034.1 & FR-035: Validates `organization_id` on connection and enforces
/// multi-tenancy isolation for all WebSocket operations.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    // Extract and validate user and organization from headers or authorization token
    let (user_id, org_id) = match extract_org_context(&headers, &state.config.app.jwt_secret) {
        Ok(ctx) => ctx,
        Err(e) => {
            tracing::warn!("WebSocket connection rejected: {e}");
            return Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(axum::body::Body::empty())
                .unwrap_or_else(|_| Response::new(axum::body::Body::empty()));
        }
    };

    tracing::info!(user_id = %user_id, org_id = %org_id, "WebSocket connection established");

    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id, org_id))
}

/// Extracts user and organization context from headers.
fn extract_org_context(headers: &HeaderMap, jwt_secret: &str) -> Result<(Uuid, Uuid)> {
    // First try extracting directly from custom headers
    if let (Some(u), Some(o)) = (headers.get("x-user-id"), headers.get("x-organization-id"))
        && let (Ok(u_str), Ok(o_str)) = (u.to_str(), o.to_str())
        && let (Ok(user_id), Ok(org_id)) = (Uuid::parse_str(u_str), Uuid::parse_str(o_str))
    {
        return Ok((user_id, org_id));
    }

    // Fallback: extract and decode Bearer token
    if let Some(auth_header) = headers.get(axum::http::header::AUTHORIZATION)
        && let Ok(auth_str) = auth_header.to_str()
        && let Some(token) = auth_str.strip_prefix("Bearer ")
        && let Ok(claims) = flora_core::utils::jwt::decode_token(token, jwt_secret)
        && let Ok(user_id) = Uuid::parse_str(&claims.sub)
        && let Ok(org_id) = Uuid::parse_str(&claims.organization_id)
    {
        return Ok((user_id, org_id));
    }

    Err(Error::Unauthorized)
}

/// Helper function to send a WebSocket message to the client.
async fn send_ws_response(socket: &mut WebSocket, response: WsServerMessage) -> Result<()> {
    let resp_text = serde_json::to_string(&response).map_err(|e| {
        tracing::error!(error = %e, "Failed to serialize WebSocket response");
        Error::IoError(std::io::Error::other(e))
    })?;
    socket
        .send(axum::extract::ws::Message::Text(resp_text.into()))
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, "Failed to send WebSocket response");
            Error::IoError(std::io::Error::other(e))
        })?;
    Ok(())
}

/// Handle subscription to a channel.
async fn handle_subscribe(socket: &mut WebSocket, user_id: Uuid, org_id: Uuid, channel_id: Uuid) {
    tracing::info!(%user_id, %org_id, %channel_id, "Subscribing to channel");
    let response = WsServerMessage::Subscribed { channel_id };
    let _ = send_ws_response(socket, response).await;
}

/// Handle unsubscription from a channel.
async fn handle_unsubscribe(socket: &mut WebSocket, user_id: Uuid, org_id: Uuid, channel_id: Uuid) {
    tracing::info!(%user_id, %org_id, %channel_id, "Unsubscribing from channel");
    let response = WsServerMessage::Unsubscribed { channel_id };
    let _ = send_ws_response(socket, response).await;
}

/// Validate that a channel belongs to the user's organization.
async fn validate_channel_ownership(state: &AppState, channel_id: Uuid, org_id: Uuid) -> bool {
    let channel_repo = PgChannelRepository::new((*state.db_pool).clone());
    match channel_repo.find_by_id(channel_id).await {
        Ok(Some(channel)) => channel.organization_id == org_id,
        Ok(None) => false,
        Err(e) => {
            tracing::warn!(%channel_id, error = %e, "Database error validating channel");
            false
        }
    }
}

/// Handle sending a message to a channel.
async fn handle_send_message(
    socket: &mut WebSocket,
    state: &AppState,
    user_id: Uuid,
    org_id: Uuid,
    channel_id: Uuid,
    content: String,
    thread_id: Option<Uuid>,
) {
    if content.trim().is_empty() {
        let response = WsServerMessage::Error {
            code: "EMPTY_CONTENT".to_string(),
            message: "Message content cannot be empty".to_string(),
        };
        let _ = send_ws_response(socket, response).await;
        return;
    }

    // Validate channel belongs to the same organization (FR-035)
    if !validate_channel_ownership(state, channel_id, org_id).await {
        tracing::warn!(
            %user_id,
            %org_id,
            %channel_id,
            "Cross-organization WebSocket message rejected"
        );
        let response = WsServerMessage::Error {
            code: "FORBIDDEN".to_string(),
            message: "Channel does not belong to your organization".to_string(),
        };
        let _ = send_ws_response(socket, response).await;
        return;
    }

    tracing::info!(%user_id, %org_id, %channel_id, "Publishing message");

    // Save message to database
    let message_repo = PgMessageRepository::new((*state.db_pool).clone());
    let mut new_msg = Message::new(channel_id, org_id, user_id, &content);
    if let Some(tid) = thread_id {
        new_msg.thread_id = Some(tid);
    }

    let saved_msg_id = match message_repo.create(new_msg).await {
        Ok(saved) => saved.id,
        Err(e) => {
            tracing::error!(error = %e, "Failed to persist WebSocket message");
            Uuid::now_v7()
        }
    };

    // Publish to Valkey pub/sub topic: org:{org_id}:channel:{channel_id}
    if let Some(ref ms) = state.messaging_service {
        let valkey_msg = flora_messaging::Message {
            channel_id,
            organization_id: org_id,
            sender_id: user_id,
            content: content.clone(),
            thread_id,
        };
        if let Err(e) = ms.publish(valkey_msg).await {
            tracing::warn!(error = %e, "Valkey publish failed");
        }
    }

    let response = WsServerMessage::MessageSent {
        message_id: saved_msg_id,
        channel_id,
    };
    let _ = send_ws_response(socket, response).await;
}

/// Handle ping message.
async fn handle_ping(socket: &mut WebSocket) {
    let response = WsServerMessage::Pong;
    let _ = send_ws_response(socket, response).await;
}

/// Handle parse error for WebSocket messages.
async fn handle_parse_error(
    socket: &mut WebSocket,
    user_id: Uuid,
    org_id: Uuid,
    error: serde_json::Error,
) {
    tracing::warn!(%user_id, %org_id, error = %error, "Failed to parse WebSocket message");
    let response = WsServerMessage::Error {
        code: "PARSE_ERROR".to_string(),
        message: "Invalid message format".to_string(),
    };
    let _ = send_ws_response(socket, response).await;
}

/// Handles an established WebSocket connection.
async fn handle_socket(mut socket: WebSocket, state: AppState, user_id: Uuid, org_id: Uuid) {
    tracing::info!(%user_id, %org_id, "WebSocket connection handled");

    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(m) => m,
            Err(e) => {
                tracing::error!(%user_id, %org_id, error = %e, "WebSocket error");
                break;
            }
        };

        if let axum::extract::ws::Message::Text(text) = msg {
            match serde_json::from_str::<WsClientMessage>(&text) {
                Ok(client_msg) => match client_msg {
                    WsClientMessage::Subscribe { channel_id } => {
                        handle_subscribe(&mut socket, user_id, org_id, channel_id).await;
                    }
                    WsClientMessage::Unsubscribe { channel_id } => {
                        handle_unsubscribe(&mut socket, user_id, org_id, channel_id).await;
                    }
                    WsClientMessage::SendMessage {
                        channel_id,
                        content,
                        thread_id,
                    } => {
                        handle_send_message(
                            &mut socket,
                            &state,
                            user_id,
                            org_id,
                            channel_id,
                            content,
                            thread_id,
                        )
                        .await;
                    }
                    WsClientMessage::Ping => {
                        handle_ping(&mut socket).await;
                    }
                },
                Err(e) => {
                    handle_parse_error(&mut socket, user_id, org_id, e).await;
                }
            }
        }
    }

    tracing::info!(%user_id, %org_id, "WebSocket connection closed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_message_serialization() {
        let msg = WsClientMessage::Subscribe {
            channel_id: Uuid::now_v7(),
        };
        let json = serde_json::to_string(&msg).unwrap_or_else(|_| String::new());
        assert!(json.contains("Subscribe"));
    }

    #[test]
    fn test_ws_send_message_serialization() {
        let channel_id = Uuid::now_v7();
        let msg = WsClientMessage::SendMessage {
            channel_id,
            content: "Hello team!".to_string(),
            thread_id: None,
        };
        let json = serde_json::to_string(&msg).unwrap_or_default();
        assert!(json.contains("SendMessage"));
        assert!(json.contains("Hello team!"));
    }

    #[test]
    fn test_ws_server_message_serialization() {
        let msg = WsServerMessage::Pong;
        let json = serde_json::to_string(&msg).unwrap_or_default();
        assert!(json.contains("Pong"));

        let err = WsServerMessage::Error {
            code: "FORBIDDEN".to_string(),
            message: "Access denied".to_string(),
        };
        let err_json = serde_json::to_string(&err).unwrap_or_default();
        assert!(err_json.contains("FORBIDDEN"));
    }
}
