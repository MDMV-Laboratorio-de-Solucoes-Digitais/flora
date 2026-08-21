//! Role-Based Access Control (RBAC) service.
//!
//! Per T025 and T025.1, this service enforces organization and workspace-level
//! permissions for users based on their assigned roles.

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{Permission, Role, User};
use crate::traits::{MembershipRepository, RoleRepository};
use std::str::FromStr;

/// Default role name for organization owners.
const DEFAULT_OWNER_ROLE: &str = "Owner";

/// RBAC service for Flora.
///
/// This service enforces permissions at the organization and workspace level.
#[derive(Debug, Clone)]
pub struct RbacService {
    role_repo: Arc<dyn RoleRepository + Send + Sync>,
    membership_repo: Arc<dyn MembershipRepository + Send + Sync>,
    // In-memory cache for roles (optional, can be disabled)
    role_cache: Arc<RwLock<HashMap<Uuid, Role>>>,
}

impl RbacService {
    /// Creates a new `RbacService`.
    pub fn new(
        role_repo: Arc<dyn RoleRepository + Send + Sync>,
        membership_repo: Arc<dyn MembershipRepository + Send + Sync>,
    ) -> Self {
        Self {
            role_repo,
            membership_repo,
            role_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Initializes the RBAC service with default roles for a new organization.
    ///
    /// # Errors
    ///
    /// Returns an error if the role creation fails.
    pub async fn initialize_organization(&self, organization_id: Uuid) -> Result<()> {
        // Create default roles for the organization
        let mut owner_role = Role::new(organization_id, DEFAULT_OWNER_ROLE);
        owner_role.is_builtin = true;
        owner_role.description = Some("Full access to all resources".to_string());

        // Add all permissions to owner
        for perm in &[
            Permission::OrgRead,
            Permission::OrgWrite,
            Permission::OrgAdmin,
            Permission::WorkspaceRead,
            Permission::WorkspaceWrite,
            Permission::WorkspaceAdmin,
            Permission::ChannelRead,
            Permission::ChannelWrite,
            Permission::ChannelDelete,
            Permission::MessageRead,
            Permission::MessageWrite,
            Permission::MessageEdit,
            Permission::MessageDelete,
            Permission::TaskRead,
            Permission::TaskWrite,
            Permission::TaskAssign,
            Permission::TaskDelete,
            Permission::FileRead,
            Permission::FileWrite,
            Permission::FileDelete,
            Permission::SearchGlobal,
            Permission::NotificationRead,
            Permission::RoleRead,
            Permission::RoleWrite,
            Permission::MemberRead,
            Permission::MemberInvite,
            Permission::MemberRemove,
        ] {
            owner_role.add_permission(*perm);
        }
        let _ = self.role_repo.create(owner_role).await?;
        Ok(())
    }

    /// Checks if a user has a specific permission in an organization.
    ///
    /// # Errors
    ///
    /// Returns an error if the user is not a member of the organization,
    /// if the role cannot be found, or if the permission string is invalid.
    pub async fn has_permission(
        &self,
        user: &User,
        organization_id: Uuid,
        permission: &str,
    ) -> Result<bool> {
        // Admins and owners bypass permission checks
        if self.is_admin(user, organization_id).await? {
            return Ok(true);
        }

        let membership = self
            .membership_repo
            .find_by_user_and_organization(user.id, organization_id)
            .await?
            .ok_or_else(|| {
                Error::Forbidden("User is not a member of this organization".to_string())
            })?;

        let role = self.get_role(membership.role_id).await?;
        // Convert string permission to Permission enum
        let perm = Permission::from_str(permission)
            .map_err(|_| Error::InvalidInput(format!("Unknown permission: {permission}")))?;
        Ok(role.has_permission(perm))
    }

    /// Checks if a user has a specific permission in a workspace.
    ///
    /// This method checks the user's organization-level role permissions for the given permission.
    /// It assumes the workspace belongs to the specified organization.
    ///
    /// # Errors
    ///
    /// Returns an error if the user is not a member of the organization,
    /// if the role cannot be found, or if the permission string is invalid.
    pub async fn has_permission_in_workspace(
        &self,
        user: &User,
        organization_id: Uuid,
        permission: &str,
    ) -> Result<bool> {
        // Delegate to the organization-level permission check since workspace permissions
        // are derived from the user's role in the organization.
        self.has_permission(user, organization_id, permission).await
    }

    /// Checks if a user is an admin or owner of an organization.
    ///
    /// # Errors
    ///
    /// Returns an error if the user is not a member of the organization
    /// or if the role cannot be found.
    pub async fn is_admin(&self, user: &User, organization_id: Uuid) -> Result<bool> {
        let membership = self
            .membership_repo
            .find_by_user_and_organization(user.id, organization_id)
            .await?
            .ok_or_else(|| {
                Error::Forbidden("User is not a member of this organization".to_string())
            })?;

        let role = self.get_role(membership.role_id).await?;
        Ok(role.name == DEFAULT_OWNER_ROLE || role.name == "Admin")
    }

    /// Gets a role by ID, using the cache if available.
    async fn get_role(&self, role_id: Uuid) -> Result<Role> {
        {
            // Check cache first
            let cache = self.role_cache.read().await;
            if let Some(role) = cache.get(&role_id) {
                return Ok(role.clone());
            }
        }

        // Fetch from database
        let role = self
            .role_repo
            .find_by_id(role_id)
            .await?
            .ok_or_else(|| Error::RoleNotFound(role_id.to_string()))?;

        // Update cache
        {
            let mut cache = self.role_cache.write().await;
            let _ = cache.insert(role_id, role.clone());
        }

        Ok(role)
    }

    /// Invalidates the cached role.
    pub async fn invalidate_role_cache(&self, role_id: Uuid) {
        let mut cache = self.role_cache.write().await;
        let _ = cache.remove(&role_id);
    }
}
