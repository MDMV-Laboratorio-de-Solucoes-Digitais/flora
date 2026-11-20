use flora_core::models::{CreateFileInput, File};
use uuid::Uuid;
use validator::Validate;

#[test]
fn test_file_new_sets_fields_correctly() {
    let org_id = Uuid::now_v7();
    let owner_id = Uuid::now_v7();
    let file_type = "image/png".to_string();
    let name = "test.png".to_string();
    let size_bytes = 1024;
    let storage_path = "orgs/abc/files/123".to_string();

    let file = File::new(
        org_id,
        owner_id,
        file_type.clone(),
        name.clone(),
        size_bytes,
        storage_path.clone(),
    );

    assert_eq!(file.organization_id, org_id);
    assert_eq!(file.owner_id, owner_id);
    assert_eq!(file.file_type, file_type);
    assert_eq!(file.name, name);
    assert_eq!(file.size_bytes, size_bytes);
    assert_eq!(file.storage_path, storage_path);
    assert!(!file.is_deleted);
    assert_eq!(file.checksum, None);
}

#[test]
fn test_create_file_input_validation() {
    let input = CreateFileInput {
        name: "a.txt".to_string(),
        file_type: "text/plain".to_string(),
        workspace_id: Uuid::now_v7(),
        metadata: None,
    };
    assert!(input.validate().is_ok());

    let invalid_input = CreateFileInput {
        name: String::new(), // Too short
        file_type: "text/plain".to_string(),
        workspace_id: Uuid::now_v7(),
        metadata: None,
    };
    assert!(invalid_input.validate().is_err());
}
