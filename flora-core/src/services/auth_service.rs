//! Authentication service.

use crate::models::Session;
use crate::traits::{AuthProvider, SessionRepository, UserRepository};
use crate::Result;

/// Authentication service for user login and session management.
#[expect(dead_code)]
pub struct AuthService {
    auth_provider: Box<dyn AuthProvider>,
    session_repo: Box<dyn SessionRepository>,
    user_repo: Box<dyn UserRepository>,
}

impl AuthService {
    /// Creates a new `AuthService`.
    #[must_use]
    pub fn new(
        auth_provider: Box<dyn AuthProvider>,
        session_repo: Box<dyn SessionRepository>,
        user_repo: Box<dyn UserRepository>,
    ) -> Self {
        Self {
            auth_provider,
            session_repo,
            user_repo,
        }
    }

    /// Authenticates a user with the given credentials.
    ///
    /// # Errors
    ///
    /// Returns an error if authentication fails.
    pub async fn login(&self, _email: &str, _password: &str) -> Result<Session> {
        // TODO: Implementation per T019
        unimplemented!()
    }
}
