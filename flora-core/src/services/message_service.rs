//! Message service.

use crate::traits::MessageRepository;

/// Message service for channel messages.
#[derive(Debug)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct MessageService {
    message_repo: Box<dyn MessageRepository>,
}

impl MessageService {
    /// Creates a new `MessageService`.
    #[must_use]
    pub fn new(message_repo: Box<dyn MessageRepository>) -> Self {
        Self { message_repo }
    }
}
