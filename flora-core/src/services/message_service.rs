//! Message service.

use crate::traits::MessageRepository;

/// Message service for channel messages.
#[expect(dead_code)]
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
