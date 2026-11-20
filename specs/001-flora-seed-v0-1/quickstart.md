# Flora Seed v0.1 - Quickstart Validation Guide

This guide provides runnable validation scenarios to prove Flora Seed v0.1 works end-to-end. It is designed for developers and testers to quickly verify core functionality.

## Prerequisites

- Docker & Docker Compose
- `curl` or Postman (for API testing)
- A configured OIDC provider (e.g., Zitadel instance)

## Setup

1. Clone the repository:
   ```bash
   git clone https://github.com/mdmv/flora.git
   cd flora
   ```

2. Set up environment variables:
   ```bash
   cp .env.example .env
   # Edit .env with your OIDC provider details
   ```

3. Start the Seed topology:
   ```bash
   docker compose up
   ```

4. Verify services are running:
   ```bash
   curl http://localhost:3000/api/v1/health
   # Expected: HTTP 200 with { "status": "ok" }
   ```

---

## Validation Scenarios

### 1. User Registration & Organization Creation

**Objective**: Verify that a new user can register and create an organization.

**Steps**:
1. Initiate OIDC login:
   ```bash
   curl -v "http://localhost:3000/api/v1/auth/login?redirect_uri=http://localhost:3000/auth/callback"
   ```
2. Follow the redirect to your OIDC provider and authenticate.
3. After callback, verify the response includes a `session_token` and `user_id`.
4. Use the session token to create an organization:
   ```bash
   curl -X POST http://localhost:3000/api/v1/organizations \
     -H "Authorization: Bearer {session_token}" \
     -H "Content-Type: application/json" \
     -d '{"name": "Acme Inc", "slug": "acme"}'
   ```

**Expected Outcome**: HTTP 201 Created with the organization details.

---

### 2. Workspace & Channel Creation

**Objective**: Verify that a user can create a workspace and a channel within it.

**Steps**:
1. Create a workspace:
   ```bash
   curl -X POST http://localhost:3000/api/v1/workspaces \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}" \
     -H "Content-Type: application/json" \
     -d '{"name": "Engineering", "description": "The Engineering team workspace"}'
   ```
2. Create a channel:
   ```bash
   curl -X POST http://localhost:3000/api/v1/channels \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}" \
     -H "Content-Type: application/json" \
     -d '{"workspace_id": "{workspace_id}", "name": "general", "type": "Public"}'
   ```

**Expected Outcome**: HTTP 201 Created for both requests.

---

### 3. Messaging

**Objective**: Verify that a user can post and retrieve messages in a channel.

**Steps**:
1. Post a message:
   ```bash
   curl -X POST http://localhost:3000/api/v1/channels/{channel_id}/messages \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}" \
     -H "Content-Type: application/json" \
     -d '{"content": "Hello, world!"}'
   ```
2. Retrieve messages:
   ```bash
   curl "http://localhost:3000/api/v1/channels/{channel_id}/messages?limit=10" \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}"
   ```

**Expected Outcome**: The posted message appears in the response.

---

### 4. Task Management

**Objective**: Verify that a user can create and update a task.

**Steps**:
1. Create a task:
   ```bash
   curl -X POST http://localhost:3000/api/v1/tasks \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}" \
     -H "Content-Type: application/json" \
     -d '{"title": "Implement login flow", "description": "Add OIDC login with Zitadel", "workspace_id": "{workspace_id}"}'
   ```
2. Update the task status:
   ```bash
   curl -X PATCH http://localhost:3000/api/v1/tasks/{task_id} \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}" \
     -H "Content-Type: application/json" \
     -d '{"status": "InProgress"}'
   ```

**Expected Outcome**: The task is created and updated successfully.

---

### 5. File Upload

**Objective**: Verify that a user can upload a file.

**Steps**:
1. Upload a file:
   ```bash
   curl -X POST http://localhost:3000/api/v1/files \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}" \
     -F "file=@/path/to/test.pdf" \
     -F "workspace_id={workspace_id}"
   ```

**Expected Outcome**: HTTP 201 Created with the file metadata.

---

### 6. Global Search

**Objective**: Verify that a user can search across messages, tasks, and files.

**Steps**:
1. Search for a term:
   ```bash
   curl "http://localhost:3000/api/v1/search?q=hello&type=messages,tasks,files" \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}"
   ```

**Expected Outcome**: Search results include relevant messages, tasks, or files.

---

### 7. Notifications

**Objective**: Verify that a user receives and can mark notifications as read.

**Steps**:
1. List unread notifications:
   ```bash
   curl "http://localhost:3000/api/v1/notifications" \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}"
   ```
2. Mark a notification as read:
   ```bash
   curl -X POST http://localhost:3000/api/v1/notifications/{notification_id}/read \
     -H "Authorization: Bearer {session_token}" \
     -H "X-Organization-ID: {organization_id}"
   ```

**Expected Outcome**: The notification is marked as read.

---

## Cleanup

To stop the services:
```bash
 docker compose down
```