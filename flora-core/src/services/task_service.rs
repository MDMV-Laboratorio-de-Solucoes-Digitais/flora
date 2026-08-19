//! Task service.

use crate::traits::TaskRepository;

/// Task management service.
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
