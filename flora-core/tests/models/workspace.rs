//! Unit tests for the Workspace model and validation.

use flora_core::models::{CreateWorkspaceInput, UpdateWorkspaceInput, Workspace};
use validator::Validate;

#[test]
fn test_workspace_creation() {
    let org_id = uuid::Uuid::now_v7();
    let workspace = Workspace::new(org_id, "Test Workspace", Some("Description".to_owned()));

    assert_eq!(workspace.name, "Test Workspace");
    assert_eq!(workspace.description, Some("Description".to_owned()));
    assert_eq!(workspace.organization_id, org_id);
    assert!(!workspace.id.to_string().is_empty());
    assert_eq!(workspace.created_at, workspace.updated_at);
}

#[test]
fn test_workspace_creation_without_description() {
    let org_id = uuid::Uuid::now_v7();
    let workspace = Workspace::new(org_id, "Test Workspace", None);

    assert_eq!(workspace.name, "Test Workspace");
    assert_eq!(workspace.description, None);
    assert_eq!(workspace.organization_id, org_id);
    assert!(!workspace.id.to_string().is_empty());
}

#[test]
fn test_create_workspace_input_validation() {
    // Valid input
    let input = CreateWorkspaceInput {
        name: "Valid Workspace".to_string(),
        description: Some("A valid description".to_string()),
    };
    assert!(input.validate().is_ok());

    // Empty name should fail
    let input = CreateWorkspaceInput {
        name: String::new(),
        description: None,
    };
    assert!(input.validate().is_err());

    // Too long name (over 255 chars) should fail
    let long_name = "a".repeat(256);
    let input = CreateWorkspaceInput {
        name: long_name,
        description: None,
    };
    assert!(input.validate().is_err());

    // Valid name at boundary (255 chars)
    let boundary_name = "a".repeat(255);
    let input = CreateWorkspaceInput {
        name: boundary_name,
        description: None,
    };
    assert!(input.validate().is_ok());

    // Too long description (over 1000 chars) should fail
    let long_desc = "a".repeat(1001);
    let input = CreateWorkspaceInput {
        name: "Valid Name".to_string(),
        description: Some(long_desc),
    };
    assert!(input.validate().is_err());

    // Valid description at boundary (1000 chars)
    let boundary_desc = "a".repeat(1000);
    let input = CreateWorkspaceInput {
        name: "Valid Name".to_string(),
        description: Some(boundary_desc),
    };
    assert!(input.validate().is_ok());
}

#[test]
fn test_update_workspace_input_validation() {
    // Valid input with both fields
    let input = UpdateWorkspaceInput {
        name: Some("Updated Workspace".to_string()),
        description: Some("Updated description".to_string()),
    };
    assert!(input.validate().is_ok());

    // Valid input with only name
    let input = UpdateWorkspaceInput {
        name: Some("Updated Workspace".to_string()),
        description: None,
    };
    assert!(input.validate().is_ok());

    // Valid input with only description
    let input = UpdateWorkspaceInput {
        name: None,
        description: Some("Updated description".to_string()),
    };
    assert!(input.validate().is_ok());

    // Valid input with empty fields (both optional)
    let input = UpdateWorkspaceInput {
        name: None,
        description: None,
    };
    assert!(input.validate().is_ok());

    // Empty name should fail
    let input = UpdateWorkspaceInput {
        name: Some(String::new()),
        description: None,
    };
    assert!(input.validate().is_err());

    // Too long name should fail
    let long_name = "a".repeat(256);
    let input = UpdateWorkspaceInput {
        name: Some(long_name),
        description: None,
    };
    assert!(input.validate().is_err());

    // Name at boundary (255 chars) should pass
    let boundary_name = "a".repeat(255);
    let input = UpdateWorkspaceInput {
        name: Some(boundary_name),
        description: None,
    };
    assert!(input.validate().is_ok());
}
