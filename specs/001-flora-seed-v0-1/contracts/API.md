# Flora Seed v0.1 - API Contracts

## Overview

Flora Seed v0.1 exposes a RESTful JSON API over HTTPS. All endpoints require authentication via OIDC Bearer Token. Every request (except `/auth/*`) must include the `X-Organization-ID` header to enforce multi-tenancy isolation.

**Base URL**: `https://{domain}/api/v1`

## Authentication

### POST `/auth/login`
**Purpose**: Authenticate a user via OIDC and return a session token.
**Request**:
```json
{
  "email": "user@example.com",
  "redirect_uri": "https://flora.example.com/auth/callback"
}
```
**Response (Success)**:
```json
{
  "session_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 3600,
  "user_id": "550e8400-e29b-41d4-a716-446655440000"
}
```
**Response (Failure)**:
```json
{
  "error": "invalid_credentials",
  "message": "Authentication failed"
}
```

### POST `/auth/refresh`
**Purpose**: Refresh an expired session token.
**Request**:
```json
{
  "session_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

---

## Organizations

### GET `/organizations/{organization_id}`
**Purpose**: Retrieve organization details.
**Headers**: `Authorization: Bearer {session_token}`
**Response (Success)**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Acme Inc",
  "slug": "acme",
  "settings": {
    "file_upload_limit_mb": 100,
    "retention_days": 30
  }
}
```

---

## Workspaces

### GET `/workspaces`
**Purpose**: List all workspaces in the organization.
**Headers**: `Authorization: Bearer {session_token}`, `X-Organization-ID: {organization_id}`
**Response (Success)**:
```json
[
  {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Engineering",
    "description": "The Engineering team workspace"
  }
]
```

### POST `/workspaces`
**Purpose**: Create a new workspace.
**Request**:
```json
{
  "name": "Engineering",
  "description": "The Engineering team workspace"
}
```

---

## Channels & Messaging

### GET `/channels/{channel_id}/messages`
**Purpose**: Retrieve messages in a channel.
**Query Params**: `?limit=50&before={message_id}`

### POST `/channels/{channel_id}/messages`
**Purpose**: Post a new message.
**Request**:
```json
{
  "content": "Hello, world!",
  "thread_id": null
}
```

---

## Tasks

### POST `/tasks`
**Purpose**: Create a new task.
**Request**:
```json
{
  "title": "Implement login flow",
  "description": "Add OIDC login with Zitadel",
  "assignee_id": "550e8400-e29b-41d4-a716-446655440002",
  "workspace_id": "550e8400-e29b-41d4-a716-446655440001"
}
```

---

## Files

### POST `/files`
**Purpose**: Upload a file (multipart).
**Headers**: `Content-Type: multipart/form-data`
**Form Fields**: `file`, `workspace_id`

---

## Search

### GET `/search`
**Purpose**: Global search.
**Query Params**: `?q=hello&type=messages,tasks&limit=20`
**Response (Success)**:
```json
{
  "results": [
    {
      "type": "message",
      "id": "550e8400-e29b-41d4-a716-446655440003",
      "snippet": "Hello, world!"
    }
  ]
}
```

---

## Notifications

### GET `/notifications`
**Purpose**: List unread notifications.
**Query Params**: `?limit=10`

### POST `/notifications/{notification_id}/read`
**Purpose**: Mark a notification as read.

---

## Error Contract

**Status Codes**:
- `200 OK`: Success
- `201 Created`: Resource created
- `400 Bad Request`: Validation error
- `401 Unauthorized`: Missing/invalid auth
- `403 Forbidden`: Permission denied
- `404 Not Found`: Resource not found
- `500 Internal Server Error`: Server error

**Error Response**:
```json
{
  "error": "invalid_request",
  "message": "The 'email' field is required",
  "details": {
    "field": "email"
  }
}
```