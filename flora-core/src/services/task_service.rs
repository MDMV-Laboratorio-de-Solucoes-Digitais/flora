//! Task service.

use crate::traits::TaskRepository;
use std::sync::Arc;

/// Task management service.
#[derive(Debug, Clone)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct TaskService {
    task_repo: Arc<dyn TaskRepository + Send + Sync>,
}

impl TaskService {
    /// Creates a new `TaskService`.
    #[must_use]
    pub fn new(task_repo: Arc<dyn TaskRepository + Send + Sync>) -> Self {
        Self { task_repo }
    }
}
