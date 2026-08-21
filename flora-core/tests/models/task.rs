//! Unit tests for the Task model and validation.

use flora_core::models::{CreateTaskInput, Task, TaskStatus, UpdateTaskInput};
use uuid::Uuid;
use validator::Validate;

#[test]
fn test_task_creation() {
    let workspace_id = Uuid::now_v7();
    let org_id = Uuid::now_v7();
    let creator_id = Uuid::now_v7();
    
    let task = Task::new(workspace_id, org_id, creator_id, "Test Task");
    
    assert_eq!(task.title, "Test Task");
    assert_eq!(task.workspace_id, workspace_id);
    assert_eq!(task.organization_id, org_id);
    assert_eq!(task.creator_id, creator_id);
    assert_eq!(task.status, TaskStatus::Todo);
    assert!(!task.is_deleted);
    assert!(task.assignee_id.is_none());
}

#[test]
fn test_task_status_parsing() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;
    assert_eq!(TaskStatus::from_str("Todo")?, TaskStatus::Todo);
    assert_eq!(TaskStatus::from_str("InProgress")?, TaskStatus::InProgress);
    assert_eq!(TaskStatus::from_str("Done")?, TaskStatus::Done);
    assert_eq!(TaskStatus::from_str("Archived")?, TaskStatus::Archived);
    assert!(TaskStatus::from_str("Unknown").is_err());
    Ok(())
}

#[test]
fn test_create_task_input_validation() {
    let valid_input = CreateTaskInput {
        title: "Valid Title".to_string(),
        description: Some("Valid description".to_string()),
        workspace_id: Uuid::now_v7(),
        assignee_id: None,
    };
    assert!(valid_input.validate().is_ok());

    let empty_title = CreateTaskInput {
        title: String::new(),
        description: None,
        workspace_id: Uuid::now_v7(),
        assignee_id: None,
    };
    assert!(empty_title.validate().is_err());
    
    let long_title = CreateTaskInput {
        title: "a".repeat(256),
        description: None,
        workspace_id: Uuid::now_v7(),
        assignee_id: None,
    };
    assert!(long_title.validate().is_err());
}

#[test]
fn test_update_task_input_validation() {
    let valid_input = UpdateTaskInput {
        title: Some("New Title".to_string()),
        description: None,
        status: Some(TaskStatus::Done),
        assignee_id: None,
    };
    assert!(valid_input.validate().is_ok());

    let empty_title = UpdateTaskInput {
        title: Some(String::new()),
        description: None,
        status: None,
        assignee_id: None,
    };
    assert!(empty_title.validate().is_err());
}
