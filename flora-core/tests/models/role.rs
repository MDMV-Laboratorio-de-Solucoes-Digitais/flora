//! Unit tests for the Role model and validation.

use flora_core::models::{Permission, Role};
use uuid::Uuid;

#[test]
fn test_role_creation() {
    let org_id = Uuid::now_v7();
    let role = Role::new(org_id, "Admin");
    assert_eq!(role.organization_id, org_id);
    assert_eq!(role.name, "Admin");
    assert!(!role.is_builtin);
}

#[test]
fn test_role_permissions() {
    let org_id = Uuid::now_v7();
    let mut role = Role::new(org_id, "CustomRole");

    assert!(!role.has_permission(Permission::MessageRead));

    role.add_permission(Permission::MessageRead);
    assert!(role.has_permission(Permission::MessageRead));
    assert!(!role.has_permission(Permission::MessageWrite));
}

#[test]
fn test_permission_parsing() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;

    let p = Permission::from_str("message:read")?;
    assert_eq!(p, Permission::MessageRead);

    assert!(Permission::from_str("invalid:permission").is_err());
    Ok(())
}
