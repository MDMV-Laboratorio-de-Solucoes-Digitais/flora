//! Role-Based Access Control (RBAC) service.
//!
//! Per T025 and T025.1, this service enforces organization and workspace-level
//! permissions for users based on their assigned roles.

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::error::{Error, Result};
use crate::models::{Membership, Organization, Role, User};
use crate::repositories::{PgMembershipRepository, PgRoleRepository};
use crate::traits::{MembershipRepository, RoleRepository};

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
    pub async fn initialize_organization(&self, organization_id: Uuid) -> Result<()> {
        // Create default roles for the organization
        let owner_role = Role::new(
            organization_id,
            DEFAULT_OWNER_ROLE.to_string(),
            vec![
                "org:read".to_string(),
                "org:write".to_string(),
                "org:delete".to_string(),
                "org:manage_members".to_string(),
                "org:manage_workspaces".to_string(),
                "org:manage_roles".to_string(),
            ],
        );
        self.role_repo.create(owner_role).await?;
        Ok(())
    }

    /// Checks if a user has a specific permission in an organization.
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
                Error::PermissionDenied("User is not a member of this organization".to_string())
            })?;

        let role = self.get_role(membership.role_id).await?;
        Ok(role.permissions.contains(&permission.to_string()))
    }

    /// Checks if a user has a specific permission in a workspace.
    ///
    /// This method checks the user's organization-level role permissions for the given permission.
    /// It assumes the workspace belongs to the specified organization.
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
    pub async fn is_admin(&self, user: &User, organization_id: Uuid) -> Result<bool> {
        let membership = self
            .membership_repo
            .find_by_user_and_organization(user.id, organization_id)
            .await?
            .ok_or_else(|| {
                Error::PermissionDenied("User is not a member of this organization".to_string())
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
            cache.insert(role_id, role.clone());
        }

        Ok(role)
    }
}

/// Role model — customizable RBAC within an organization.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Role {
    /// Unique identifier.
    pub id: Uuid,
    /// The organization this role belongs to.
    pub organization_id: Uuid,
    /// Role name (e.g., "Admin", "Member").
    pub name: String,
    /// List of permissions (e.g., ["org:read", "org:write"]).
    pub permissions: Vec<String>,
}

impl Role {
    /// Creates a new role with a generated UUID.
    pub fn new(organization_id: Uuid, name: String, permissions: Vec<String>) -> Self {
        Self {
            id: Uuid::now_v7(),
            organization_id,
            name,
            permissions,
        }
    }
}

#[async_trait]
impl RoleRepository for PgRoleRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Role>> {
        let role = sqlx::query_as::<_, Role>(
            "SELECT id, organization_id, name, permissions
             FROM roles
             WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(role)
    }

    async fn find_by_organization_id(&self, organization_id: Uuid) -> Result<Vec<Role>> {
        let roles = sqlx::query_as::<_, Role>(
            "SELECT id, organization_id, name, permissions
             FROM roles
             WHERE organization_id = $1",
        )
        .bind(organization_id)
        .fetch_all(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(roles)
    }

    async fn create(&self, role: Role) -> Result<Role> {
        let created = sqlx::query_as::<_, Role>(
            "INSERT INTO roles (id, organization_id, name, permissions)
             VALUES ($1, $2, $3, $4)
             RETURNING id, organization_id, name, permissions",
        )
        .bind(role.id)
        .bind(role.organization_id)
        .bind(&role.name)
        .bind(&role.permissions)
        .fetch_one(&self.pool)
        .await
        .map_err(Error::from_sqlx)?;
        Ok(created)
    }
}
