//! Flora Tests - Integration and Contract Tests
#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive dependency overrides"
)]
/// Contract tests module - tests the API contract.
pub mod contract {
    /// Authentication contract tests.
    pub mod auth;
}
pub mod integration;

#[cfg(test)]
mod tests {
    use flora_core::models::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("test@example.com", "Test User");
        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.display_name, "Test User");
        assert!(user.is_active);
    }

    #[test]
    fn test_organization_creation() {
        let org = Organization::new("Test Org", "test-org");
        assert_eq!(org.name, "Test Org");
        assert_eq!(org.slug, "test-org");
    }

    #[test]
    fn test_workspace_creation() {
        let org_id = uuid::Uuid::now_v7();
        let ws = Workspace::new(org_id, "Test Workspace", Some("Description".to_owned()));
        assert_eq!(ws.name, "Test Workspace");
        assert_eq!(ws.organization_id, org_id);
    }

    #[test]
    fn test_message_creation() {
        let channel_id = uuid::Uuid::now_v7();
        let org_id = uuid::Uuid::now_v7();
        let sender_id = uuid::Uuid::now_v7();
        let msg = Message::new(channel_id, org_id, sender_id, "Hello, world!");
        assert_eq!(msg.content, "Hello, world!");
        assert!(!msg.is_deleted);
    }

    #[test]
    fn test_task_creation() {
        let ws_id = uuid::Uuid::now_v7();
        let org_id = uuid::Uuid::now_v7();
        let creator_id = uuid::Uuid::now_v7();
        let task = Task::new(ws_id, org_id, creator_id, "Test Task");
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.status, TaskStatus::Todo);
    }
}
