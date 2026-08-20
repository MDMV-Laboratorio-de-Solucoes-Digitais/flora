//! Unit tests for the Message model and validation.

use flora_core::models::{CreateMessageInput, Message, UpdateMessageInput};
use validator::Validate;

#[test]
fn test_message_creation() {
    let channel_id = uuid::Uuid::now_v7();
    let organization_id = uuid::Uuid::now_v7();
    let sender_id = uuid::Uuid::now_v7();

    let message = Message::new(channel_id, organization_id, sender_id, "Hello, world!");

    assert_eq!(message.content, "Hello, world!");
    assert_eq!(message.channel_id, channel_id);
    assert_eq!(message.organization_id, organization_id);
    assert_eq!(message.sender_id, sender_id);
    assert!(!message.is_edited);
    assert!(!message.is_deleted);
    assert!(message.thread_id.is_none());
}

#[test]
fn test_create_message_input_validation() {
    // Valid input
    let input = CreateMessageInput {
        content: "Hello, world!".to_string(),
        thread_id: None,
    };
    assert!(input.validate().is_ok());

    // Empty content should fail
    let input = CreateMessageInput {
        content: "".to_string(),
        thread_id: None,
    };
    assert!(input.validate().is_err());

    // Too long content (over 10000 chars) should fail
    let long_content = "a".repeat(10001);
    let input = CreateMessageInput {
        content: long_content,
        thread_id: None,
    };
    assert!(input.validate().is_err());

    // Content with thread_id should be valid
    let thread_id = uuid::Uuid::now_v7();
    let input = CreateMessageInput {
        content: "Reply message".to_string(),
        thread_id: Some(thread_id),
    };
    assert!(input.validate().is_ok());
}

#[test]
fn test_update_message_input_validation() {
    // Valid input
    let input = UpdateMessageInput {
        content: "Updated message".to_string(),
    };
    assert!(input.validate().is_ok());

    // Empty content should fail
    let input = UpdateMessageInput {
        content: "".to_string(),
    };
    assert!(input.validate().is_err());

    // Too long content should fail
    let long_content = "a".repeat(10001);
    let input = UpdateMessageInput {
        content: long_content,
    };
    assert!(input.validate().is_err());
}

#[test]
fn test_message_threading() {
    let channel_id = uuid::Uuid::now_v7();
    let organization_id = uuid::Uuid::now_v7();
    let sender_id = uuid::Uuid::now_v7();
    let parent_message_id = uuid::Uuid::now_v7();

    // Create a reply message (thread)
    let reply = CreateMessageInput {
        content: "This is a reply".to_string(),
        thread_id: Some(parent_message_id),
    };

    assert_eq!(reply.thread_id, Some(parent_message_id));
    assert!(reply.validate().is_ok());
}
