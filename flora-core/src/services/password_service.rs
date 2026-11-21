//! Argon2id hashing for the local password fallback.
//!
//! Uses the OWASP-recommended parameters (m=19456 KiB, t=2, p=1) and stores
//! the result as a PHC string. Verification uses the constant-time comparison
//! provided by `password-hash`; unknown users are verified against a dummy
//! hash so the response time does not reveal whether an e-mail exists.

use std::sync::LazyLock;

use argon2::{
    Algorithm, Argon2, Params, Version,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::error::{Error, Result};

/// Minimum accepted password length, in characters.
pub const MIN_PASSWORD_LENGTH: usize = 15;
/// Maximum accepted password length, in characters.
pub const MAX_PASSWORD_LENGTH: usize = 1024;

const DUMMY_PASSWORD: &str = "flora-dummy-password-not-a-real-secret";

/// Password hasher/verifier shared by bootstrap, login and startup recovery.
#[derive(Debug)]
pub struct PasswordService {
    argon: Argon2<'static>,
    dummy_hash: String,
}

impl Default for PasswordService {
    fn default() -> Self {
        Self::new()
    }
}

impl PasswordService {
    /// Creates a hasher with Argon2id `m=19456,t=2,p=1` and a dummy hash.
    #[must_use]
    pub fn new() -> Self {
        let argon = Argon2::new(Algorithm::Argon2id, Version::V0x13, Params::DEFAULT);
        let salt = SaltString::generate(&mut OsRng);
        let dummy_hash = argon
            .hash_password(DUMMY_PASSWORD.as_bytes(), &salt)
            .map_or_else(|_| String::new(), |hash| hash.to_string());
        Self { argon, dummy_hash }
    }

    /// Hashes a password with a fresh random salt and returns its PHC string.
    ///
    /// # Errors
    ///
    /// Returns an internal error if the hasher cannot encode the hash.
    pub fn hash(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        self.argon
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|err| Error::Internal(format!("password hash failed: {err}")))
    }

    /// Verifies a password against a stored PHC string.
    #[must_use]
    pub fn verify(&self, password: &str, stored_hash: &str) -> bool {
        let Ok(parsed) = PasswordHash::new(stored_hash) else {
            return false;
        };
        self.argon
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }

    /// Verifies a password, spending the same work on unknown users.
    #[must_use]
    pub fn verify_or_dummy(&self, password: &str, stored_hash: Option<&str>) -> bool {
        if let Some(stored_hash) = stored_hash
            && let Ok(parsed) = PasswordHash::new(stored_hash)
        {
            return self
                .argon
                .verify_password(password.as_bytes(), &parsed)
                .is_ok();
        }
        let _ = std::hint::black_box(self.verify_dummy(password));
        false
    }

    /// Checks password length and complexity through the shared validator.
    ///
    /// # Errors
    ///
    /// Returns a validation error when the password is outside 15..1024 chars
    /// or fails the shared complexity policy.
    pub fn validate_password(password: &str, field: &str) -> Result<()> {
        let length = password.chars().count();
        if !(MIN_PASSWORD_LENGTH..=MAX_PASSWORD_LENGTH).contains(&length) {
            return Err(Error::Validation {
                field: field.to_owned(),
                message: format!("{field} must be 15-1024 characters"),
            });
        }
        crate::utils::validation::validate_password(password).map_err(|err| Error::Validation {
            field: field.to_owned(),
            message: err.to_string(),
        })
    }
    /// Process-wide hasher, initialized once.
    #[must_use]
    pub fn shared() -> &'static Self {
        static SHARED: LazyLock<PasswordService> = LazyLock::new(PasswordService::new);
        &SHARED
    }

    fn verify_dummy(&self, password: &str) -> bool {
        let Ok(parsed) = PasswordHash::new(&self.dummy_hash) else {
            return false;
        };
        self.argon
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::{MAX_PASSWORD_LENGTH, MIN_PASSWORD_LENGTH, PasswordService};

    #[test]
    fn hash_uses_argon2id_owasp_parameters() -> crate::Result<()> {
        let service = PasswordService::new();
        let hash = service.hash("SenhaForte@12345")?;
        assert!(hash.starts_with("$argon2id$v=19$m=19456,t=2,p=1$"));
        assert!(service.verify("SenhaForte@12345", &hash));
        Ok(())
    }

    #[test]
    fn each_hash_uses_a_fresh_random_salt() -> crate::Result<()> {
        let service = PasswordService::new();
        assert_ne!(
            service.hash("SenhaForte@12345")?,
            service.hash("SenhaForte@12345")?
        );
        Ok(())
    }

    #[test]
    fn wrong_password_and_unknown_user_are_rejected() -> crate::Result<()> {
        let service = PasswordService::new();
        let hash = service.hash("SenhaForte@12345")?;
        assert!(!service.verify("SenhaErrada@123", &hash));
        assert!(!service.verify_or_dummy("SenhaErrada@123", None));
        assert!(!service.verify_or_dummy("SenhaErrada@123", Some("phc-invalido")));
        assert!(service.verify_or_dummy("SenhaForte@12345", Some(&hash)));
        Ok(())
    }

    #[test]
    fn password_length_bounds_are_enforced() {
        let short = "a".repeat(MIN_PASSWORD_LENGTH - 1);
        let minimum = "SenhaForte@1234";
        assert_eq!(minimum.chars().count(), MIN_PASSWORD_LENGTH);
        let maximum = format!("{}a1!", "A".repeat(MAX_PASSWORD_LENGTH - 3));
        let too_long = format!("{}a1!", "A".repeat(MAX_PASSWORD_LENGTH - 2));
        assert!(PasswordService::validate_password(&short, "admin_password").is_err());
        assert!(PasswordService::validate_password(minimum, "admin_password").is_ok());
        assert!(PasswordService::validate_password(&maximum, "admin_password").is_ok());
        assert!(PasswordService::validate_password(&too_long, "admin_password").is_err());
    }
}
