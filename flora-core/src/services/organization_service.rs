//! Organization service.

use crate::traits::{MembershipRepository, OrganizationRepository, RoleRepository};
use std::sync::Arc;

/// Organization management service.
#[derive(Debug, Clone)]
#[expect(
    dead_code,
    reason = "Service is part of the service layer and will be used once the API routes are wired up."
)]
pub struct OrganizationService {
    orgs: Arc<dyn OrganizationRepository + Send + Sync>,
    memberships: Arc<dyn MembershipRepository + Send + Sync>,
    roles: Arc<dyn RoleRepository + Send + Sync>,
}

impl OrganizationService {
    /// Creates a new `OrganizationService`.
    #[must_use]
    pub fn new(
        orgs: Arc<dyn OrganizationRepository + Send + Sync>,
        memberships: Arc<dyn MembershipRepository + Send + Sync>,
        roles: Arc<dyn RoleRepository + Send + Sync>,
    ) -> Self {
        Self {
            orgs,
            memberships,
            roles,
        }
    }
}
