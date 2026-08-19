//! Notification service.

use crate::traits::NotificationRepository;

/// Notification management service.
#[expect(dead_code)]
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
