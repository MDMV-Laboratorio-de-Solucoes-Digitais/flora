//! Validation utilities for email, password, and other inputs.

use validator::ValidationError;

/// Validates an email format.
///
/// Returns `Ok(())` if the email is valid, otherwise returns a `ValidationError`.
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    let email = email.trim();
    if email.is_empty() {
        return Err(ValidationError::new("email cannot be empty"));
    }

    if !email.contains('@') || !email.contains('.') {
        return Err(ValidationError::new("invalid email format"));
    }

    // Basic length check
    if email.len() > 254 {
        return Err(ValidationError::new("email too long"));
    }

    // Simple regex-like check for valid email format
    let at_pos = email.find('@').unwrap();
    let local = &email[..at_pos];
    let domain = &email[at_pos + 1..];

    if local.is_empty() || local.len() > 64 {
        return Err(ValidationError::new("invalid email local part"));
    }

    if domain.is_empty() || domain.len() > 255 {
        return Err(ValidationError::new("invalid email domain"));
    }

    // Check for invalid characters in local part
    if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
        return Err(ValidationError::new("invalid email local part format"));
    }

    // Domain must have at least one dot
    if !domain.contains('.') {
        return Err(ValidationError::new("invalid email domain format"));
    }

    Ok(())
}

/// Validates password complexity.
///
/// Returns `Ok(())` if the password meets complexity requirements, otherwise returns a `ValidationError`.
///
/// Requirements:
/// - At least 8 characters
/// - At least one uppercase letter
/// - At least one lowercase letter
/// - At least one digit
/// - At least one special character (!@#$%^&*()_+-=[]{}|;:,.<>?)
pub fn validate_password(password: &str) -> Result<(), ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::new(
            "password must be at least 8 characters",
        ));
    }

    let mut has_upper = false;
    let mut has_lower = false;
    let mut has_digit = false;
    let mut has_special = false;

    let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";

    for c in password.chars() {
        if c.is_uppercase() {
            has_upper = true;
        } else if c.is_lowercase() {
            has_lower = true;
        } else if c.is_ascii_digit() {
            has_digit = true;
        } else if special_chars.contains(c) {
            has_special = true;
        }
    }

    if !has_upper {
        return Err(ValidationError::new(
            "password must contain at least one uppercase letter",
        ));
    }
    if !has_lower {
        return Err(ValidationError::new(
            "password must contain at least one lowercase letter",
        ));
    }
    if !has_digit {
        return Err(ValidationError::new(
            "password must contain at least one digit",
        ));
    }
    if !has_special {
        return Err(ValidationError::new(
            "password must contain at least one special character (!@#$%^&*()_+-=[]{}|;:,.<>?)",
        ));
    }

    Ok(())
}

/// Validates that a string is not empty and within a length range.
pub fn validate_length(
    value: &str,
    field_name: &str,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let len = value.chars().count();
    if len < min {
        return Err(ValidationError::new(&format!(
            "{} must be at least {} characters",
            field_name, min
        )));
    }
    if len > max {
        return Err(ValidationError::new(&format!(
            "{} must be at most {} characters",
            field_name, max
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("test@example.com").is_ok());
        assert!(validate_email("user.name@domain.org").is_ok());
        assert!(validate_email("user+tag@example.co.uk").is_ok());

        assert!(validate_email("").is_err());
        assert!(validate_email("invalid").is_err());
        assert!(validate_email("missing@domain").is_err());
        assert!(validate_email("@missinglocal.com").is_err());
        assert!(validate_email("local@").is_err());
        assert!(validate_email("local..part@domain.com").is_err());
        assert!(validate_email(".local@domain.com").is_err());
        assert!(validate_email("local.@domain.com").is_err());
        assert!(validate_email("a@b").is_err());
    }

    #[test]
    fn test_validate_password() {
        assert!(validate_password("Passw0rd!").is_ok());
        assert!(validate_password("Str0ng!Pass").is_ok());
        assert!(validate_password("Aa1!aaaa").is_ok());

        assert!(validate_password("short").is_err());
        assert!(validate_password("nouppercase1!").is_err());
        assert!(validate_password("NOLOWERCASE1!").is_err());
        assert!(validate_password("NoDigits!").is_err());
        assert!(validate_password("NoSpecial123").is_err());
        assert!(validate_password("alllowercase1!").is_err());
        assert!(validate_password("ALLUPPERCASE1!").is_err());
    }
}
