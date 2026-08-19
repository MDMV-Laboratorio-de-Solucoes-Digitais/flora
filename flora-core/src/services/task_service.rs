//! Task service.

use crate::traits::TaskRepository;

/// Task management service.
#[derive(Debug)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct TaskService {
    task_repo: Box<dyn TaskRepository>,
}

impl TaskService {
    /// Creates a new `TaskService`.
    #[must_use]
    pub fn new(task_repo: Box<dyn TaskRepository>) -> Self {
        Self { task_repo }
    }
}
