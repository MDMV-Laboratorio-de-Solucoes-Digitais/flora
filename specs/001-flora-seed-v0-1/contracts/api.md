# API Contract: Flora Seed v0.1 (REST/JSON)

**Version**: 1.0.0-seed
**Base URL**: `https://api.flora.local/v1`
**Authentication**: Bearer JWT (via Zitadel OIDC)

## Security & Tenancy Headers

All requests must include:
- `Authorization: Bearer <token>`
- The backend will extract `user_id` and `organization_id` from the JWT claims.

## Endpoints

### 1. Authentication & Identity

| Method | Endpoint | Description | Scope |
|--------|----------|-------------|-------|
| `POST` | `/auth/register` | Register new account and create initial org | Public |
| `POST` | `/auth/token` | Exchange OIDC code for local session JWT | Public |
| `POST` | `/auth/logout` | Invalidate local session | Authenticated |

### 2. Organization & Workspace Management

| Method | Endpoint | Description | Scope |
|--------|----------|-------------|-------|
| `GET` | `/orgs` | List all organizations the user belongs to | `org:read` |
| `POST` | `/orgs` | Create a new organization | `org:create` |
| `GET` | `/orgs/{org_id}/workspaces` | List workspaces in an organization | `org:read` |
| `POST` | `/orgs/{org_id}/workspaces` | Create a new workspace | `org:write` |

### 3. Messaging (Channels & Messages)

| Method | Endpoint | Description | Scope |
|--------|----------|-------------|-------|
| `GET` | `/workspaces/{ws_id}/channels` | List channels in a workspace | `msg:read` |
| `POST` | `/channels/{ch_id}/messages` | Post a new message | `msg:write` |
| `GET` | `/channels/{ch_id}/messages` | List messages in a channel (with pagination) | `msg:read` |
| `PATCH` | `/messages/{msg_id}` | Edit an existing message | `msg:write` |
| `DELETE`| `/messages/{msg_id}` | Soft-delete a message | `msg:write` |

### 4. Task Management

| Method | Endpoint | Description | Scope |
|--------|----------|-------------|-------|
| `GET` | `/workspaces/{ws_id}/tasks` | List tasks in a workspace | `task:read` |
| `POST` | `/workspaces/{ws_id}/tasks` | Create a new task | `task:write` |
| `PATCH` | `/tasks/{task_id}` | Update task status or assignment | `task:write` |
| `DELETE`| `/tasks/{task_id}` | Soft-delete a task | `task:write` |

### 5. File Management

| Method | Endpoint | Description | Scope |
|--------|----------|-------------|-------|
| `POST` | `/channels/{ch_id}/files` | Upload a file to a channel | `file:write` |
| `GET` | `/files/{file_id}` | Get file metadata & download URL | `file:read` |

## Error Responses

All errors follow this format:
```json
{
  "error": "ERROR_CODE",
  "message": "Human readable explanation",
  "details": {} 
}
```

**Common Codes**:
- `UNAUTHORIZED`: Missing or invalid token.
- `FORBIDDEN`: Insufficient permissions for the resource or organization mismatch.
- `NOT_FOUND`: Resource does not exist.
- `VALIDATION_FAILED`: Input data does not meet constraints.
