//! Integration tests for the registration flow.
//!
//! These tests verify the full OIDC-based registration and organization creation flow.
//!
//! TODO: Implement once the `TestApp` harness and OIDC mocking are available.

/// Placeholder for the registration flow test.
///
/// Verifies the full OIDC-based registration and organization creation flow once
/// the `TestApp` harness is in place.
#[tokio::test]
async fn test_registration_flow() -> anyhow::Result<()> {
    // TODO: Spawn a `TestApp`, hit `/auth/login`, assert the authorization URL shape,
    // simulate the OIDC callback, and assert the resulting session.
    Ok(())
}

/// Placeholder for the registration flow with a mocked OIDC provider.
#[tokio::test]
async fn test_registration_flow_with_mock_oidc() -> anyhow::Result<()> {
    // TODO: Insert a test user/session directly, encode a JWT, and verify round-trip
    // decoding once the `TestApp` harness is in place.
    Ok(())
}
