# API Contracts

This directory contains interface contracts for the Flora Seed v0.1 service.

## REST API

The primary external interface is a JSON-over-HTTP REST API served by the `flora-api` Axum server.

### Endpoints (high-level grouping)

- **Auth**: `/api/v1/auth/*` (OIDC callback, session management)
- **Organizations**: `/api/v1/organizations`
- **Workspaces**: `/api/v1/workspaces`
- **Channels**: `/api/v1/channels`
- **Messages**: `/api/v1/messages`
- **Tasks**: `/api/v1/tasks`
- **Files**: `/api/v1/files`
- **Search**: `/api/v1/search`
- **Notifications**: `/api/v1/notifications`

### Contract Details

The full API contract (OpenAPI 3.0) is generated from the Axum route definitions and can be found in `flora-api/src/api_doc.rs` or similar after implementation.

### Versioning

API versions are prefixed with `/api/v{major}/`. Breaking changes increment the major version.

### Content-Type

- Request bodies: `application/json`
- Responses: `application/json`
- File uploads: `multipart/form-data`

### Authentication

- Uses Bearer JWT tokens issued via Zitadel OIDC flow.
- Token validated by middleware; claims attached to request state for authorization.

### Error Responses

- JSON object with `error` (string) and optionally `details` (object).
- HTTP status codes follow RFC 7231.

## Internal Contracts

Trait boundaries between vertical slices and the core are defined as Rust traits in `flora-core`. These are not exposed externally but constitute internal contracts for swapability and testing.