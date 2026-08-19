//! Organization service.

use crate::traits::{MembershipRepository, OrganizationRepository, RoleRepository};

/// Organization management service.
pub struct OrganizationService {
    orgs: Box<dyn OrganizationRepository>,
    memberships: Box<dyn MembershipRepository>,
    roles: Box<dyn RoleRepository>,
}

impl OrganizationService {
    /// Creates a new `OrganizationService`.
    #[must_use]
    pub fn new(
        orgs: Box<dyn OrganizationRepository>,
        memberships: Box<dyn MembershipRepository>,
        roles: Box<dyn RoleRepository>,
    ) -> Self {
        Self { orgs, memberships, roles }
    }
}
