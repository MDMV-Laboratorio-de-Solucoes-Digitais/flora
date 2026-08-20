//! Unit tests for the User model and validation.

use flora_core::models::{CreateUserInput, UpdateUserInput, User};
use validator::Validate;

#[test]
fn test_user_creation() {
    let user = User::new("test@example.com", "Test User");
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.display_name, "Test User");
    assert!(!user.id.to_string().is_empty());
    assert!(user.is_active);
}

#[test]
fn test_user_from_oidc() {
    let user = User::from_oidc(
        "sub123",
        "test@example.com",
        "Test User",
        Some("https://example.com/avatar.png".to_string()),
    );
    assert_eq!(user.email, "test@example.com");
    assert_eq!(user.oidc_subject, Some("sub123".to_string()));
    assert_eq!(
        user.avatar_url,
        Some("https://example.com/avatar.png".to_string())
    );
}

#[test]
fn test_create_user_input_validation() {
    // Valid input
    let input = CreateUserInput {
        email: "test@example.com".to_string(),
        display_name: "Test User".to_string(),
    };
    assert!(input.validate().is_ok());

    // Invalid email
    let input = CreateUserInput {
        email: "invalid-email".to_string(),
        display_name: "Test User".to_string(),
    };
    assert!(input.validate().is_err());

    // Empty display name
    let input = CreateUserInput {
        email: "test@example.com".to_string(),
        display_name: String::new(),
    };
    assert!(input.validate().is_err());
}

#[test]
fn test_update_user_input_validation() {
    // Valid input
    let input = UpdateUserInput {
        display_name: Some("Updated User".to_string()),
        avatar_url: Some("https://example.com/avatar.png".to_string()),
    };
    assert!(input.validate().is_ok());

    // Empty display name
    let input = UpdateUserInput {
        display_name: Some(String::new()),
        avatar_url: None,
    };
    assert!(input.validate().is_err());
}
