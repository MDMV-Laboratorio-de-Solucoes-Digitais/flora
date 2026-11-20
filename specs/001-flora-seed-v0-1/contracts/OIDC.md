# Flora Seed v0.1 - OIDC Integration Contract

## Overview

Flora Seed v0.1 integrates with Zitadel (or any standard-compliant OIDC provider) using the `openidconnect` crate. This contract defines the expected behavior and configuration.

## Configuration

**Required Environment Variables**:
- `OIDC_ISSUER_URL`: The OIDC provider's issuer URL (e.g., `https://accounts.zitadel.com`)
- `OIDC_CLIENT_ID`: The client ID registered with the OIDC provider
- `OIDC_CLIENT_SECRET`: The client secret
- `OIDC_REDIRECT_URI`: The callback URI (e.g., `https://flora.example.com/auth/callback`)

## Flow

1. **Discovery**: Flora fetches the OIDC provider's configuration from `{OIDC_ISSUER_URL}/.well-known/openid-configuration`.
2. **Authorization Request**: Redirect the user to the OIDC provider's authorization endpoint.
3. **Token Exchange**: Exchange the authorization code for an ID token and access token.
4. **User Info**: Fetch user details from the OIDC provider's userinfo endpoint.
5. **Session Creation**: Flora creates a local session and returns a Flora-specific session token to the client.

## Claims Mapping

| OIDC Claim | Flora Field | Notes |
|------------|-------------|-------|
| `sub`      | `external_id` | Unique identifier from the OIDC provider |
| `email`    | `email`     | User's email address |
| `name`     | `profile.name` | Display name |
| `picture`  | `profile.avatar_url` | Avatar image URL |

## Error Handling

- If the OIDC provider is unavailable during login, Flora **fails closed** (denies access).
- If the OIDC provider is unavailable during token validation, Flora **allows existing sessions to continue for a configurable grace period** (default: 15 minutes).
- Flora implements a **circuit breaker** to prevent cascading failures.

## Rate Limiting

Flora enforces rate limits on all `/auth/*` endpoints:
- `POST /auth/login`: 5 requests per minute per IP
- `POST /auth/refresh`: 10 requests per minute per user

## Multi-Tenancy

Flora supports multiple organizations per user. The OIDC integration does not directly handle organization membership; this is managed separately via Flora's membership system.