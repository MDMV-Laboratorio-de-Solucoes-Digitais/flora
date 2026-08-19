//! Notification service.

use crate::traits::NotificationRepository;

/// Notification management service.
#[derive(Debug)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct NotificationService {
    notification_repo: Box<dyn NotificationRepository>,
}

impl NotificationService {
    /// Creates a new `NotificationService`.
    #[must_use]
    pub fn new(notification_repo: Box<dyn NotificationRepository>) -> Self {
        Self { notification_repo }
    }
}
