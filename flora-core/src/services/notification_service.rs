//! Notification service.

use crate::traits::NotificationRepository;
use std::sync::Arc;

/// Notification management service.
#[derive(Debug, Clone)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct NotificationService {
    notification_repo: Arc<dyn NotificationRepository + Send + Sync>,
}

impl NotificationService {
    /// Creates a new `NotificationService`.
    #[must_use]
    pub fn new(notification_repo: Arc<dyn NotificationRepository + Send + Sync>) -> Self {
        Self { notification_repo }
    }
}
