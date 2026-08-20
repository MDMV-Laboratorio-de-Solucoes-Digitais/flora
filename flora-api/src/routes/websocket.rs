//! WebSocket module for real-time messaging with organization-scoped connections.
//!
//! Per T034.1: Validates `organization_id` on connection and enforces
//! multi-tenancy isolation for all WebSocket operations.

use axum::{
    extract::{
        State,
        ws::{WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, StatusCode},
    response::Response,
};
use flora_core::error::{Error, Result};
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
/// Per T034.1: Validates `organization_id` on connection and enforces
/// multi-tenancy isolation for all WebSocket operations.
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response {
    // Extract and validate user and organization from headers
    let (user_id, org_id) = match extract_org_context(&headers) {
        Ok(ctx) => ctx,
        Err(e) => {
            tracing::warn!("WebSocket connection rejected: {}", e);
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
fn extract_org_context(headers: &HeaderMap) -> Result<(Uuid, Uuid)> {
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
#[expect(
    clippy::unused_async,
    reason = "Placeholder implementation - will be expanded with actual WebSocket logic"
)]
async fn handle_socket(mut _socket: WebSocket, _state: AppState, user_id: Uuid, org_id: Uuid) {
    // TODO: Implement proper Redis/WebSocket handling
    // This is a stub to fix compilation errors
    tracing::info!(user_id = %user_id, org_id = %org_id, "WebSocket connection handled");

    // Close the socket for now
    // Note: Actual implementation would handle WebSocket communication here
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
}
