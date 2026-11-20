//! Message service.

use crate::traits::MessageRepository;
use std::sync::Arc;

/// Message service for channel messages.
#[derive(Debug, Clone)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct MessageService {
    message_repo: Arc<dyn MessageRepository + Send + Sync>,
}

impl MessageService {
    /// Creates a new `MessageService`.
    #[must_use]
    pub fn new(message_repo: Arc<dyn MessageRepository + Send + Sync>) -> Self {
        Self { message_repo }
    }
}
