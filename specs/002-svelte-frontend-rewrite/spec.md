# Feature Specification: Flora Seed v0.1 (Svelte 5 Rewrite)

**Feature Branch**: `[###-feature-name]`

**Created**: 2026-08-21

**Status**: Draft

**Input**: User description: "Define the core requirements and user stories for "Flora Workspace" Milestone 1 (Flora Seed v0.1), focusing on the Svelte 5 frontend rewrite..."


## Clarifications

### Session 2026-08-21
- Q: What should happen if Zitadel experiences an outage during an active session's token refresh? → A: Hybrid Security Model: lock write operations and enter a secure "Pending Re-authentication / Read-Only Cache" mode for a 5-minute grace period before forcing a clean redirect, while persisting local drafts.
- Q: How should the frontend handle Valkey WebSocket reconnects to prevent message duplication or loss? → A: Pause rendering new events, fetch missed messages via a REST fallback sync using a `last_known_message_id`, merge and deduplicate using Svelte 5 Runes, then resume the Valkey stream.
- Q: How should the system handle file uploads that exceed the allowed size? → A: Enforce strict limits at the API gateway level with a client-side pre-flight size check to save bandwidth. Handle rejections via explicit Promise catch blocks.
- Q: How should the mandated structured logging be implemented? → A: A strictly typed Svelte 5 logger utility replaces `console.log`, buffering JSON logs locally and batch-sending them to a dedicated Rust (Axum) endpoint, which forwards to Otel.
- Q: How should the Svelte 5 frontend mock backend services during local development? → A: Run against the actual local Rust backend configured with "dev/offline" trait implementations (SQLite, LocalFileSystem) instead of JS mock contracts.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Secure Identity & Access Management (Priority: P1)

As an organization administrator, I need to create and manage workspaces, assign roles to users, and ensure secure authentication via OIDC (Zitadel) so that only authorized personnel can access the company's Flora ecosystem.

**Why this priority**: Without secure access and identity management, the workspace cannot function safely. This is the foundational layer upon which all other features rely.

**Independent Test**: Can be fully tested by simulating an admin creating an organization, inviting a user, assigning them a role, and having that user successfully authenticate via Zitadel and land on their workspace dashboard.

**Acceptance Scenarios**:

1. **Given** an unauthenticated user, **When** they attempt to access the workspace, **Then** they are redirected to the OIDC login flow.
2. **Given** an admin user in the workspace settings, **When** they invite a new user and assign a "Member" role, **Then** the invitation is sent and the system registers the role assignment.
3. **Given** the frontend Svelte 5 codebase, **When** the code is compiled, **Then** `svelte-check` must pass with zero warnings, enforcing `strict: true` and no `any` types.

---

### User Story 2 - Real-Time Collaboration & Messaging (Priority: P1)

As a team member, I need to communicate via channels, direct messages, and threaded replies in real time so that I can collaborate effectively without delays.

**Why this priority**: Real-time communication is the primary value driver of a modern workspace application.

**Independent Test**: Can be fully tested by having two users log in on separate sessions and exchange messages in a channel and a thread, verifying that updates appear instantly without page reloads (powered by PGMQ + Valkey).

**Acceptance Scenarios**:

1. **Given** a user viewing a channel, **When** another user posts a message, **Then** the message appears immediately in the channel view.
2. **Given** a message in a channel, **When** a user replies to it, **Then** a thread is created and linked correctly.
3. **Given** a user is mentioned in a thread, **When** the mention occurs, **Then** a real-time notification is dispatched and displayed.

---

### User Story 3 - Task Tracking & Productivity (Priority: P2)

As a workspace user, I need to create, assign, and track tasks so that our team can manage project workflows and deliverables effectively.

**Why this priority**: Tasks provide structure to the collaboration enabled by messaging.

**Independent Test**: Can be fully tested by creating a task, assigning it to a user, changing its status to "Done", and verifying the state persists to the database.

**Acceptance Scenarios**:

1. **Given** an authenticated user on the tasks page, **When** they submit a new task with a title and assignee, **Then** the task appears in the designated column/list.
2. **Given** a task list, **When** a user filters by assignee, **Then** only tasks matching that assignee are displayed.

---

### User Story 4 - File Management via RustFS (Priority: P2)

As a workspace user, I need to upload, organize, and retrieve files within channels and tasks so that relevant documents are always accessible where discussions happen.

**Why this priority**: File sharing is a critical component of collaboration, but secondary to the ability to communicate and create tasks.

**Independent Test**: Can be fully tested by uploading a document to a channel and verifying it can be downloaded and previewed (if applicable) by another user in that channel.

**Acceptance Scenarios**:

1. **Given** a user composing a message, **When** they attach a valid file, **Then** the file is uploaded to RustFS and linked to the message.
2. **Given** a fallible upload operation, **When** an upload fails (e.g., network error), **Then** the Promise is handled explicitly and gracefully, displaying a user-friendly error without utilizing `console.log`.

---

### Edge Cases

- **OIDC Token Refresh Failure**: The system MUST enforce a Hybrid Security Model: lock write operations and place the active session into a secure 'Pending Re-authentication / Read-Only Cache' mode for a 5-minute grace period. If Zitadel does not recover, force a clean redirect to login while persisting local draft states.
- **WebSocket Reconnection**: Upon WebSocket reconnect, the frontend MUST pause new events, fetch missed messages via a REST fallback sync endpoint using a `last_known_message_id` (cursor-based pagination querying PostgreSQL), merge and deduplicate by ID into local state using Svelte 5 Runes, and then resume the Valkey stream.
- **Large File Uploads**: The frontend MUST check file size (max 50MB) before initiating the request to save bandwidth. The API gateway enforces strict upload limits (50MB). Gateway rejections MUST be handled via explicit Promise catch blocks and display a user-friendly error component without `console.log`.
- **Active Session Invalidation**: If a user is removed from a workspace while holding an active session, a real-time Valkey Pub/Sub event (`workspace.user.removed`) is emitted to the user's channel. The frontend listener MUST immediately invalidate the local session, overlay a full-screen UI lock, and force a redirect to the login flow.

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: System MUST provide OIDC-based authentication and token management via Zitadel.
- **FR-002**: System MUST allow creation and management of Organizations, Workspaces, Users, and Roles.
- **FR-003**: System MUST support real-time text channels, direct messages, and threaded conversations.
- **FR-004**: System MUST deliver real-time notifications via Valkey pub/sub and PGMQ (Frontend connects exclusively to Valkey WS; backend bridges PGMQ events).
- **FR-005**: System MUST allow users to create, assign, and track the status of Tasks.
- **FR-006**: System MUST support file uploads and downloads integrated with RustFS.
- **FR-007**: Frontend codebase MUST be written in Svelte 5 (Runes Mode) and TypeScript.
- **FR-008**: Frontend compilation MUST pass `svelte-check` with zero warnings.
- **FR-009**: Frontend TypeScript configuration MUST enforce `strict: true`, absolutely no `any` types, and no unsafe assignments.
- **FR-010**: Frontend ESLint configuration MUST strictly match the rigor of Rust's `clippy::pedantic` level (utilizing strict TypeScript rules and `eslint-plugin-unicorn`).
- **FR-011**: Frontend MUST explicitly and gracefully handle all Promises and fallible operations.
- **FR-012**: Frontend MUST NOT use `console.log` in production code. A strictly typed Svelte 5 logger utility MUST format logs as JSON, buffer them locally, and batch-send them to a dedicated Rust (Axum) backend endpoint, which forwards to the Otel Collector.
- **FR-013**: Code changes MUST follow Test-Driven Development (TDD); tests MUST be written to satisfy the "Definition of Done" before implementation.
- **FR-014**: System MUST enforce strict file upload limits (50MB) at the API gateway, and the frontend MUST perform client-side pre-flight size checks.

### Key Entities *(include if feature involves data)*

- **Organization**: Top-level entity representing a company.
- **Workspace**: A dedicated environment within an Organization.
- **User**: An authenticated individual mapped via OIDC.
- **Role**: Permissions assigned to a User within a Workspace.
- **Channel**: A space for message exchange.
- **Message**: A text entry (potentially with file attachments) within a Channel or Thread.
- **Thread**: A nested conversation linked to a parent Message.
- **Notification**: An alert dispatched to a User.
- **Task**: An actionable item with status and assignee.
- **File**: Metadata and blob reference stored via RustFS.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: Users can successfully authenticate, create a workspace, and send a real-time message without page refreshes.
- **SC-002**: The application boots successfully on a fresh VPS environment utilizing only PostgreSQL, Valkey, RustFS, and Meilisearch (no proprietary SaaS).
- **SC-003**: Code Quality Gates: 100% `svelte-check` pass rate, zero `console.log` instances, and 100% explicit Promise handling in production builds.

## Assumptions

- Users have modern browsers capable of running Svelte 5 and WebSocket connections.
- Local frontend development MUST run against the actual local Rust backend configured with 'dev/offline' trait implementations (e.g., SQLite, LocalFileSystem) to guarantee 100% API contract fidelity. JS mock contracts like MSW will NOT be used.
- The Rust backend API contract is strictly defined and developed concurrently using Trait-Driven Architecture.
- Meilisearch is available for any required search indexing, though specific search features are secondary to core CRUD operations in Milestone 1.
- OpenTelemetry collectors are properly configured in the deployment environment to receive structured logs from the frontend.
