//! WebSocket module for real-time messaging with organization-scoped connections.
//!
//! Per T034.1 & FR-035: Validates `organization_id` on connection and message publishing,
//! enforcing multi-tenancy isolation for all WebSocket operations.

use std::collections::HashSet;

use axum::{
    extract::{
        Query, State,
        ws::{WebSocket, WebSocketUpgrade, rejection::WebSocketUpgradeRejection},
    },
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use flora_core::models::Message;
use flora_core::repositories::{PgChannelRepository, PgMessageRepository};
use flora_core::traits::{ChannelRepository, MessageRepository};
use tokio::sync::broadcast::error::RecvError;
use uuid::Uuid;

use crate::AppState;
use crate::realtime::{self, HubEnvelope, MessageEventPayload, RealtimeEvent};

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
        /// Canonical message payload.
        message: MessageEventPayload,
    },
    /// A message was edited in a subscribed channel.
    MessageUpdated {
        /// The channel ID where the message was edited.
        channel_id: Uuid,
        /// Canonical message payload.
        message: MessageEventPayload,
    },
    /// A message was deleted in a subscribed channel.
    MessageDeleted {
        /// The channel ID where the message was deleted.
        channel_id: Uuid,
        /// Canonical message payload.
        message: MessageEventPayload,
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

/// Query parameters for WebSocket authentication fallback.
#[derive(Debug, serde::Deserialize)]
pub struct WsTokenQuery {
    /// Bearer token provided as query parameter for browser clients.
    pub token: Option<String>,
}

/// Handles WebSocket upgrade and connection.
///
/// Validates the bearer token against DB-backed session authority before upgrade
/// and enforces organization-scoped isolation for WebSocket operations.
pub async fn websocket_handler(
    ws: Result<WebSocketUpgrade, WebSocketUpgradeRejection>,
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(params): Query<WsTokenQuery>,
) -> Response {
    // Validate session authority first so invalid auth returns 401 (not 426).
    let (user_id, org_id) = match extract_org_context(&headers, &params, &state).await {
        Ok(ctx) => ctx,
        Err(status) => {
            tracing::warn!("WebSocket connection rejected");
            return Response::builder()
                .status(status)
                .body(axum::body::Body::empty())
                .unwrap_or_else(|_| Response::new(axum::body::Body::empty()));
        }
    };

    let ws = match ws {
        Ok(ws) => ws,
        Err(rejection) => {
            tracing::warn!("WebSocket upgrade rejected after authentication");
            return rejection.into_response();
        }
    };

    tracing::info!(user_id = %user_id, org_id = %org_id, "WebSocket connection established");

    ws.on_upgrade(move |socket| handle_socket(socket, state, user_id, org_id))
}

/// Resolves a bearer token from supported transports.
///
/// Prefers the `Authorization: Bearer <token>` header when present.
fn resolve_ws_token<'a>(headers: &'a HeaderMap, params: &'a WsTokenQuery) -> Option<&'a str> {
    headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .or(params.token.as_deref())
}

/// Extracts user and organization context from a verified, DB-backed session.
///
/// Rejects nil organization for WebSocket connections.
async fn extract_org_context(
    headers: &HeaderMap,
    params: &WsTokenQuery,
    state: &AppState,
) -> Result<(Uuid, Uuid), StatusCode> {
    let Some(token) = resolve_ws_token(headers, params) else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let verified = crate::extractors::auth::verify_bearer_token(token, state)
        .await
        .map_err(|(status, _)| status)?;

    if verified.organization_id.is_nil() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok((verified.user_id, verified.organization_id))
}

/// Helper function to send a WebSocket message to the client.
async fn send_ws_response(
    socket: &mut WebSocket,
    response: WsServerMessage,
) -> Result<(), flora_core::error::Error> {
    let resp_text = serde_json::to_string(&response).map_err(|e| {
        tracing::error!(error = %e, "Failed to serialize WebSocket response");
        flora_core::error::Error::IoError(std::io::Error::other(e))
    })?;
    socket
        .send(axum::extract::ws::Message::Text(resp_text.into()))
        .await
        .map_err(|e| {
            tracing::warn!(error = %e, "Failed to send WebSocket response");
            flora_core::error::Error::IoError(std::io::Error::other(e))
        })?;
    Ok(())
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

/// Handle subscription to a channel.
async fn handle_subscribe(
    socket: &mut WebSocket,
    state: &AppState,
    user_id: Uuid,
    org_id: Uuid,
    channel_id: Uuid,
    subscriptions: &mut HashSet<Uuid>,
) {
    if !validate_channel_ownership(state, channel_id, org_id).await {
        tracing::warn!(%user_id, %org_id, %channel_id, "Subscription rejected: channel not accessible");
        let response = WsServerMessage::Error {
            code: "FORBIDDEN".to_string(),
            message: "Channel does not belong to your organization".to_string(),
        };
        let _ = send_ws_response(socket, response).await;
        return;
    }
    let _ = subscriptions.insert(channel_id);
    tracing::info!(%user_id, %org_id, %channel_id, "Subscribed to channel");
    let response = WsServerMessage::Subscribed { channel_id };
    let _ = send_ws_response(socket, response).await;
}

/// Handle unsubscription from a channel.
async fn handle_unsubscribe(
    socket: &mut WebSocket,
    user_id: Uuid,
    org_id: Uuid,
    channel_id: Uuid,
    subscriptions: &mut HashSet<Uuid>,
) {
    let _ = subscriptions.remove(&channel_id);
    tracing::info!(%user_id, %org_id, %channel_id, "Unsubscribed from channel");
    let response = WsServerMessage::Unsubscribed { channel_id };
    let _ = send_ws_response(socket, response).await;
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

    let created = match message_repo.create(new_msg).await {
        Ok(saved) => saved,
        Err(e) => {
            tracing::error!(error = %e, "Failed to persist WebSocket message");
            let response = WsServerMessage::Error {
                code: "INTERNAL".to_string(),
                message: "Failed to persist message".to_string(),
            };
            let _ = send_ws_response(socket, response).await;
            return;
        }
    };

    // Best-effort Valkey publish (optional, non-authoritative)
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

    // Emit canonical realtime event for subscribed peers
    let payload = MessageEventPayload {
        id: created.id.to_string(),
        channel_id: created.channel_id.to_string(),
        sender_id: created.sender_id.to_string(),
        content: created.content.clone(),
        thread_id: created.thread_id.map(|id| id.to_string()),
        is_edited: created.is_edited,
        created_at: created.created_at.to_rfc3339(),
        updated_at: created.updated_at.to_rfc3339(),
    };
    realtime::publish(
        &state.realtime_hub,
        HubEnvelope::new(
            org_id,
            channel_id,
            RealtimeEvent::NewMessage {
                channel_id,
                message: payload,
            },
        ),
    );

    let response = WsServerMessage::MessageSent {
        message_id: created.id,
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

async fn handle_client_text(
    socket: &mut WebSocket,
    state: &AppState,
    user_id: Uuid,
    org_id: Uuid,
    subscriptions: &mut HashSet<Uuid>,
    text: &str,
) {
    match serde_json::from_str::<WsClientMessage>(text) {
        Ok(client_msg) => match client_msg {
            WsClientMessage::Subscribe { channel_id } => {
                handle_subscribe(socket, state, user_id, org_id, channel_id, subscriptions).await;
            }
            WsClientMessage::Unsubscribe { channel_id } => {
                handle_unsubscribe(socket, user_id, org_id, channel_id, subscriptions).await;
            }
            WsClientMessage::SendMessage {
                channel_id,
                content,
                thread_id,
            } => {
                handle_send_message(socket, state, user_id, org_id, channel_id, content, thread_id)
                    .await;
            }
            WsClientMessage::Ping => {
                handle_ping(socket).await;
            }
        },
        Err(e) => {
            handle_parse_error(socket, user_id, org_id, e).await;
        }
    }
}

async fn forward_hub_event(
    socket: &mut WebSocket,
    org_id: Uuid,
    subscriptions: &HashSet<Uuid>,
    envelope: HubEnvelope,
) -> bool {
    if envelope.organization_id != org_id || !subscriptions.contains(&envelope.channel_id) {
        return true;
    }
    let response = match envelope.event {
        RealtimeEvent::NewMessage { channel_id, message } => {
            WsServerMessage::NewMessage { channel_id, message }
        }
        RealtimeEvent::MessageUpdated { channel_id, message } => {
            WsServerMessage::MessageUpdated { channel_id, message }
        }
        RealtimeEvent::MessageDeleted { channel_id, message } => {
            WsServerMessage::MessageDeleted { channel_id, message }
        }
    };
    send_ws_response(socket, response).await.is_ok()
}

/// Handles an established WebSocket connection.
async fn handle_socket(mut socket: WebSocket, state: AppState, user_id: Uuid, org_id: Uuid) {
    tracing::info!(%user_id, %org_id, "WebSocket connection handled");

    let mut subscriptions: HashSet<Uuid> = HashSet::new();
    let mut hub_rx = state.realtime_hub.subscribe();

    loop {
        tokio::select! {
            biased;
            recv = socket.recv() => {
                let Some(msg) = recv else { break; };
                let msg = match msg {
                    Ok(m) => m,
                    Err(e) => {
                        tracing::error!(%user_id, %org_id, error = %e, "WebSocket error");
                        break;
                    }
                };

                if let axum::extract::ws::Message::Text(text) = msg {
                    handle_client_text(&mut socket, &state, user_id, org_id, &mut subscriptions, &text).await;
                }
            }
            hub = hub_rx.recv() => {
                match hub {
                    Ok(envelope) => {
                        if !forward_hub_event(&mut socket, org_id, &subscriptions, envelope).await {
                            break;
                        }
                    }
                    Err(RecvError::Lagged(skipped)) => {
                        tracing::warn!(%user_id, %org_id, skipped, "WebSocket hub lagged; client should catch up");
                    }
                    Err(RecvError::Closed) => break,
                }
            }
        }
    }

    tracing::info!(%user_id, %org_id, "WebSocket connection closed");
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

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

    #[test]
    fn test_resolve_ws_token_prefers_header() {
        let mut headers = HeaderMap::new();
        let _ = headers.insert(
            axum::http::header::AUTHORIZATION,
            axum::http::HeaderValue::from_static("Bearer header-token"),
        );
        let params = WsTokenQuery {
            token: Some("query-token".to_string()),
        };

        assert_eq!(resolve_ws_token(&headers, &params), Some("header-token"));
    }

    #[test]
    fn test_resolve_ws_token_falls_back_to_query() {
        let headers = HeaderMap::new();
        let params = WsTokenQuery {
            token: Some("query-token".to_string()),
        };

        assert_eq!(resolve_ws_token(&headers, &params), Some("query-token"));
    }

    #[test]
    fn test_resolve_ws_token_missing() {
        let headers = HeaderMap::new();
        let params = WsTokenQuery { token: None };

        assert_eq!(resolve_ws_token(&headers, &params), None);
    }
}
