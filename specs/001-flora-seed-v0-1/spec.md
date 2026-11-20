# Feature Specification: Flora Seed v0.1 Core Features

**Feature Branch**: `[001-flora-seed-v0-1]`

**Created**: 2026-08-17

**Status**: Draft

**Input**: User description: "Your task is to generate the comprehensive Feature Specification document (Requirements, Epics, and User Stories) for the First Milestone: Flora Seed v0.1 of the Flora Workspace project.

Context & Philosophy:
Flora Workspace is a complete, lightweight, and free (AGPL) ecosystem of corporate apps. Our guiding philosophy is: 'Every installation is a seed. Each module is a tree. A collaborative forest - the more seeds and trees planted here, the more fertile the soil becomes.'
For this milestone, we are defining the absolute minimal, yet highly robust foundation necessary for a team to collaborate. It must deliver immediate value while strictly adhering to our 'Debloat' principle (the system must eventually be lightweight enough to run on a cheap VPS).

Task:
Focus strictly on the 'What' and the 'Why' (business logic, user experience, and functional requirements). Please define detailed Epics, User Stories (using the As a... I want to... So that... format), and Acceptance Criteria for the following core modules:

Workspace & Identity Ecosystem:

Organizations and Workspaces (creation, management, switching).

Users, Roles, and Permissions.

Authentication (OIDC integration).

Core Messaging:

Channels (public/private).

Messages and Threads (replies, edits, deletions).

Task Management:

Basic task creation, assignment, status tracking, and lists.

File Management:

Uploading, organizing, and sharing files within workspaces/channels.

Platform Features:

Global Search (finding messages, tasks, and files).

Notifications (in-app alerts for mentions, task assignments, etc.).

Explicit Exclusions (Out of Scope for v0.1):
Do not write specifications or user stories for the following features: AI integrations, CRM, Calendar, Audio/Video Calls, MCP, Agents, Email Client, or GitHub clone functionalities.

Please structure the output clearly with headings, bullet points, and distinct sections for each Epic to ensure it can perfectly drive the next steps of our Spec-Driven Development workflow."

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - User Registration and Organization Creation (Priority: P1)

As a new user, I want to create an account and set up my first organization so that I can start collaborating with my team immediately.

**Why this priority**: This is the entry point to the system - without account creation and organization setup, users cannot access any other functionality. It delivers immediate value by allowing users to establish their workspace.

**Independent Test**: Can be fully tested by completing the registration flow with valid credentials and verifying that a new organization is created and the user is authenticated and redirected to the organization dashboard.

**Acceptance Scenarios**:
1. **Given** I am on the registration page, **When** I enter valid email, password, and organization name, **Then** my account is created, an organization is created with me as owner, I am logged in, and I am redirected to the organization dashboard.
2. **Given** I am on the registration page, **When** I enter an email that is already registered, **Then** I see an error message indicating the email is already in use.
3. **Given** I am on the registration page, **When** I enter a password that doesn't meet complexity requirements, **Then** I see an error message indicating the password requirements.

---

### User Story 2 - Organization and Workspace Management (Priority: P2)

As an organization member, I want to create and manage workspaces within my organization so that I can organize my team's collaboration around different projects or topics.

**Why this priority**: Workspaces provide the primary organizational structure for collaboration. Once users have an account, organizing their work into meaningful containers is essential for productive collaboration.

**Independent Test**: Can be fully tested by creating multiple workspaces, switching between them, and verifying that each workspace maintains its own isolated data (channels, messages, tasks, files).

**Acceptance Scenarios**:
1. **Given** I am logged into an organization, **When** I create a new workspace with a name and description, **Then** the workspace is created and appears in my workspace list.
2. **Given** I am in a workspace, **When** I switch to another workspace, **Then** the UI updates to show the new workspace's channels, messages, tasks, and files.
3. **Given** I am in a workspace, **When** I update the workspace name or description, **Then** the changes are persisted and reflected in the workspace list.

---

### User Story 3 - Role and Permission Management (Priority: P2)

As an organization owner or admin, I want to assign roles and manage permissions for team members so that I can control access to organizational resources and maintain security.

**Why this priority**: Proper access control is essential for organizational security and effective collaboration. This enables the principle of least privilege while ensuring team members have appropriate access.

**Independent Test**: Can be fully tested by inviting a user to an organization, assigning them different roles, and verifying that their access to organizational features corresponds to their role permissions.

**Acceptance Scenarios**:
1. **Given** I am an organization owner, **When** I invite a user by email and assign them the "member" role, **Then** the user receives an invitation and upon accepting, has member-level access to the organization.
2. **Given** I am an organization admin, **When** I change a user's role from "member" to "admin", **Then** the user gains administrative privileges in the organization.
3. **Given** I am a user with "member" role, **When** I attempt to access admin-only settings, **Then** I am denied access and shown an appropriate error message.

---

### User Story 4 - Core Messaging - Channels and Messages (Priority: P1)

As a workspace member, I want to communicate with my team through channels and messages so that I can collaborate effectively in real-time.

**Why this priority**: Communication is the core function of a collaboration platform. Without messaging, users cannot collaborate, making this essential for delivering the primary value proposition.

**Independent Test**: Can be fully tested by creating a channel, posting messages, replying to messages, editing messages, and deleting messages, with all operations persisting correctly and visible to other workspace members.

**Acceptance Scenarios**:
1. **Given** I am in a workspace, **When** I create a new channel (public or private), **Then** the channel is created and visible to appropriate workspace members based on its privacy setting.
2. **Given** I am in a channel, **When** I post a new message, **Then** the message appears in the channel timeline for all channel members.
3. **Given** I am in a channel, **When** I reply to a message, **Then** my reply is threaded under the original message and visible in the conversation view.
4. **Given** I am in a channel, **When** I edit my own message, **Then** the message content is updated and an edit indicator is shown.
5. **Given** I am in a channel, **When** I delete my own message, **Then** the message is removed from the channel and is no longer visible to channel members.

---

### User Story 5 - Task Management (Priority: P2)

As a team member, I want to create, assign, and track tasks so that I can manage my work and collaborate with teammates on actionable items.

**Why this priority**: Task management transforms communication into actionable work, enabling teams to track progress and accountability. This extends the platform from pure communication to productive collaboration.

**Independent Test**: Can be fully tested by creating tasks, assigning them to team members, updating their status, and verifying that all operations persist correctly and are visible to relevant team members.

**Acceptance Scenarios**:
1. **Given** I am in a workspace, **When** I create a new task with title, description, assignee, and status, **Then** the task is created and visible in the task list with the correct attributes.
2. **Given** I am assigned to a task, **When** I update the task status, **Then** the status change is persisted and visible to all task collaborators.
3. **Given** I am a task assignee, **When** I add a comment to a task, **Then** the comment is added to the task's activity log and notifications are sent to relevant parties.

---

### User Story 6 - File Management (Priority: P2)

As a team member, I want to upload, organize, and share files within workspaces and channels so that I can collaborate on documents and share resources with my team.

**Why this priority**: File sharing is essential for collaborative work on documents, designs, code, and other assets. This enables the platform to support complete workflows beyond just text-based communication.

**Independent Test**: Can be fully tested by uploading files to a workspace or channel, organizing them in folders, sharing them with team members, and verifying that files are accessible with correct permissions.

**Acceptance Scenarios**:
1. **Given** I am in a workspace or channel, **When** I upload a file, **Then** the file is stored and appears in the file browser with correct metadata (name, size, type, uploader).
2. **Given** I am in a workspace, **When** I create a folder and move files into it, **Then** the file organization is persisted and visible in the file browser.
3. **Given** I have uploaded a file, **When** I share the file with a team member or make it public in a channel, **Then** the specified users can access and download the file.

---

### User Story 7 - Global Search (Priority: P2)

As a workspace member, I want to search across messages, tasks, and files so that I can quickly find information I need from past collaboration.

**Why this priority**: As collaboration history grows, the ability to find past information becomes increasingly important for productivity and avoiding duplicated work.

**Independent Test**: Can be fully tested by creating sample data (messages, tasks, files) and verifying that search returns relevant results based on keywords, with proper filtering by content type.

**Acceptance Scenarios**:
1. **Given** I have messages, tasks, and files in my workspace, **When** I search for a term that appears in a message, **Then** that message appears in the search results.
2. **Given** I have messages, tasks, and files in my workspace, **When** I search for a term that appears in a file name or content, **Then** that file appears in the search results.
3. **Given** I have messages, tasks, and files in my workspace, **When** I search with filters (e.g., "only messages", "only from last week"), **Then** the search results are correctly filtered according to the criteria.

---

### User Story 8 - Notifications (Priority: P2)

As a team member, I want to receive in-app notifications for relevant activities so that I can stay informed about important updates without constantly checking the platform.

**Why this priority**: Notifications reduce the cognitive load of monitoring for updates and help ensure timely responses to important events like mentions or task assignments.

**Independent Test**: Can be fully tested by triggering notification events (mentions, task assignments, etc.) and verifying that appropriate notifications appear in the notification center and are cleared when viewed.

**Acceptance Scenarios**:
1. **Given** I am a team member, **When** another member mentions me in a message (@username), **Then** I receive a notification of the mention in my notification center.
2. **Given** I am a team member, **When** I am assigned to a task, **Then** I receive a notification of the task assignment.
3. **Given** I have unread notifications, **When** I open the notification center and view the notifications, **Then** the notifications are marked as read and the notification badge is updated.

---

### Edge Cases

- What happens when a user tries to register with an email format that is invalid?
- How does the system handle network interruptions during file upload?
- What occurs when a user attempts to create a workspace with a name that already exists in their organization?
- How does the system behave when search returns no results?
- What happens when a notification is clicked - does it navigate to the relevant context?
- How are permissions handled when a user's role is changed while they have open resources? (Answer: Immediate revocation; active sessions terminated, open resources closed but saved work preserved)
- What happens when a user tries to upload a file exceeding the organization's size limit?
- How does the system handle file uploads of blocked file types (e.g., .exe)?
- What occurs when a soft-deleted item reaches the end of its retention period?
- How are permission changes applied to existing shared content (e.g., if a user loses access to a folder they previously shared)?
- What happens when the OIDC provider (Zitadel) becomes unavailable — can existing users with active sessions continue working, and for how long?
- What happens when a user attempts to log in while the OIDC provider is unreachable — is there a local fallback or is login denied?
- What happens when a query bug or misconfiguration causes data from one organization to be returned to a user in a different organization?
- What happens when a query or operation bypasses `organization_id` filtering — are exceptions documented, approved, and audited with compensating controls?
- What happens when a user's session expires and the silent refresh fails — is the user redirected to login automatically?
- What happens when OIDC Single Logout (SLO) to Zitadel fails — is the local logout still completed?
- What happens when a user exceeds the authentication rate limit — is the lockout temporary with exponential backoff?
- What happens when a user with 2FA enabled loses their authenticator device — how is recovery handled via backup codes?
- What happens when local email/password fallback is enabled and a user attempts to use it — is there a cooldown period or audit logging?
- What happens when OIDC claim mapping produces no matching Flora role — is the user assigned a default role or denied access?
- What happens when claim mapping configuration is invalid or missing — does authentication fail or use defaults?
- What happens when OIDC access token expires and silent refresh fails — is the user immediately redirected to login?
- What happens when admin revokes a user's tokens in Zitadel — how quickly does Flora detect and terminate the session?
- What happens when Valkey is unavailable — are messages queued locally and replayed on recovery?
- What happens when Meilisearch is unavailable — do search queries return empty results with a notice, or error?
- What happens when network connectivity is lost during real-time messaging — does the UI show "reconnecting..." with exponential backoff?
- What happens when authentication latency exceeds targets (login > 500ms, refresh > 100ms) — is it logged/monitored for alerting?
- What happens when session validation latency spikes — does it trigger circuit breaker or degrade gracefully?
- What happens when search latency exceeds tiered targets — is it logged and alerted for capacity planning?
- What happens when a search query returns extremely large result sets — is pagination enforced with limits?
- What happens when notification delivery latency exceeds tiered targets — is queue depth monitored and alerted?
- What happens when notification queue backs up under high load — are notifications dropped, batched, or delayed?
- What happens when WebSocket connection latency exceeds 500ms — is fallback to polling triggered?
- What happens when message fan-out latency spikes — are messages queued or dropped?
- What happens when a user attempts to access a workspace they're not a member of — is access denied at API layer with 403?
- What happens when workspace membership changes mid-session — is the user's workspace access re-validated on next request?
- What happens when organization storage quota is exceeded — is upload rejected with 413 and quota details?
- What happens when workspace quota is exceeded but org quota has space — is creation blocked?
- What happens when a user exceeds concurrent session limit — is oldest session evicted (LRU) with notification?
- What happens when "log out everywhere" is invoked — are all sessions terminated immediately across devices?
- What happens when soft-deleted data retention period expires — is hard delete executed by daily batch job?
- What happens when a user tries to recover soft-deleted item — is it restored from "deleted items" view?
- What happens when retention period configuration is changed — does it apply retroactively to already-deleted items?

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST allow users to register an account with email and password
- **FR-002**: System MUST enable users to create an organization during registration
- **FR-003**: System MUST authenticate users via OIDC integration with Zitadel
- **FR-004**: System MUST allow organization owners to create and manage workspaces
- **FR-005**: System MUST support customizable role-based access control with granular permissions:
  - **Org-level:** Roles (e.g., Owner, Admin, Member) and global permissions (e.g., "manage org settings," "invite users").
  - **Workspace-level:** Channel/file access (e.g., "read/write channel X," "upload files to workspace Y").
  - **Unsupported:** Per-message or per-task permissions (planned for v0.2+).
  - **UI:** Admin interface for org/workspace admins to configure permissions.
- **FR-006**: System MUST enable creation of public and private channels within workspaces
- **FR-007**: System MUST allow users to post, reply to, edit, and delete messages in channels
- **FR-008**: System MUST maintain message threading for replies to messages
- **FR-009**: System MUST enable users to create, assign, update status, and comment on tasks
- **FR-010**: System MUST allow users to upload, organize, and share files within workspaces and channels with a default limit of 100MB per file (configurable by organization) and block executable file types (.exe, .bat, .sh, .ps1, etc.) (configurable by organization)
- **FR-011**: System MUST provide global search across messages, tasks, and files with keyword search and basic filtering by content type and date range
  - **FR-011.1:** Search indices must be organization-scoped (e.g., `flora_org_{org_id}`). No global search index is allowed.
- **FR-012**: System MUST generate and display comprehensive in-app notifications for the following events:
  - **Types:** Mentions (@user, @channel), task assignments/replies, file shares/uploads, content updates (e.g., message edits), workspace/channel invites.
  - **Delivery Guarantees:** p95 latency targets: low load < 3s, medium load < 7s, high load < 15s (aligned with SC-007).
  - **User Controls:** Granular opt-in/opt-out per type, with "urgent" (e.g., mentions) and "non-urgent" (e.g., file shares) categories.
  - **Edge Cases:** Queue and retry failed deliveries (max 3 attempts), log undelivered notifications for admin review.
- **FR-013**: System MUST enforce access controls so users can only access resources they have permission for; all data queries MUST include **automatic `organization_id` filtering at the database level** to prevent cross-organization data leakage. Exceptions (e.g., admin views) must be explicitly documented and require manual overrides.
- **FR-014**: System MUST persist all data (accounts, organizations, workspaces, messages, tasks, files) reliably with soft-delete capability and organization-configurable retention periods
- **FR-015**: System MUST support real-time updates for messaging and presence indicators
- **FR-018**: System MUST manage user sessions with a default 24-hour lifetime and automatic silent refresh via rotating refresh tokens every 6 hours, configurable per organization
- **FR-019**: System MUST support configurable logout behavior per organization — local session termination with optional best-effort OIDC Single Logout (SLO) to Zitadel (non-blocking by default)
- **FR-020**: System MUST enforce rate limits on authentication endpoints — default 5 login attempts/minute per IP, 20 token refreshes/minute per user, 10 invite acceptances/hour per IP, configurable per organization
- **FR-021**: System MUST support optional TOTP-based 2FA per organization, configurable by organization administrators, with backup codes for recovery
- **FR-022**: System MUST support local email/password authentication as fallback when Zitadel is unavailable:
  - **Scope:** Existing users only (new registrations blocked unless admin-enabled local registration).
  - **Behavior:** Read-only mode for cached data (e.g., messages, tasks) if Zitadel is unreachable. Write operations (e.g., send messages, upload files) require Zitadel recovery or admin override.
  - **Admin Override:** Org admins can enable local registration for critical users during outages (audit-logged).
  - **Recovery:** Sync local changes with Zitadel once available (e.g., pending registrations, password changes).
  - **Default:** Disabled by default, enabled per organization by administrators.
- **FR-023**: System MUST support configurable OIDC claim-to-role mapping per organization — default mapping `groups` claim to Flora roles, with admin UI for custom mapping rules
- **FR-024**: System MUST handle expired or revoked OIDC tokens during active sessions — configurable per organization: strict mode (immediate session termination, default) or lenient mode (grace period)
- **FR-025**: System MUST implement circuit breaker pattern for external dependencies — Valkey failure: disable real-time features, fall back to polling with local queue for replay; Meilisearch failure: disable search, return empty results with "search unavailable" notice; both with automatic recovery
- **FR-026**: System MUST assume stable internet connection for real-time features in v0.1 — implement basic reconnection logic with exponential backoff and "reconnecting..." UI indicator; offline-first support deferred to v0.2+
- **FR-027**: System MUST meet authentication performance targets — OIDC login callback p95 < 500ms, token refresh p95 < 100ms, session validation p95 < 10ms
- **FR-028**: System MUST meet search performance targets — tiered by time range: past week p95 < 5s, past month p95 < 10s, past year p95 < 30s; concurrent searches (50 users) p95 < 15s
- **FR-029**: System MUST meet notification delivery latency targets — tiered by load: low < 3s, medium < 7s, high < 15s p95
- **FR-030**: System MUST meet real-time messaging performance targets — WebSocket connection p95 < 500ms, message fan-out p95 < 500ms per hop, presence updates p95 < 2s
- **FR-031**: System MUST enforce workspace isolation at the API layer — verify workspace membership before allowing access; **`workspace_id` filtering in queries is optional but recommended for defense-in-depth**.
- **FR-032**: System MUST enforce resource quotas — default org: 10GB file storage / 100 users; workspace: 2GB inherited; enforced on upload/creation with 413 response; 90% soft limit with warnings; configurable by admin
- **FR-033**: System MUST enforce concurrent session limits — default 5 sessions per user; "log out everywhere" endpoint terminates all except current; LRU eviction on limit; configurable per organization
- **FR-034**: System MUST manage soft-deleted data retention — excluded from search indexes by default; separate "deleted items" view for recovery; daily batch cleanup job; retention 30-365 days (default 90), configurable per organization
- **FR-035**: System MUST enforce `organization_id` scoping in shared infrastructure:
  - **Search indices (Meilisearch):** Search indices must be organization-scoped (e.g., `flora_org_{org_id}`). Query-time filtering is prohibited for security. Exceptions must be documented and approved.
  - **Pub/sub topics (Valkey):** All topics must be organization-scoped (e.g., `org:{org_id}:channel:{channel_id}`). Cross-organization topics are prohibited.
  - **Real-time streams:** WebSocket connections must validate `organization_id` on connection and message publishing.
- **FR-036**: System MUST implement explicit degradation rules when approaching resource limits (1GB RAM, 1 vCPU):
  - **Memory > 80% or CPU > 70%:** Disable real-time features (fall back to polling), throttle search queries (increase latency targets by 50%), and log warnings.
  - **Memory > 90% or CPU > 85%:** Reject new uploads (429), disable non-critical background jobs, and alert admins.
  - **Recovery:** Restore features automatically when resources return to < 70% for 5 minutes.
- **FR-037**: System MUST enforce permission change propagation:
  - **Active sessions:**
    - **Security-sensitive actions** (e.g., role revocation, org removal): Immediate termination (within 1s for 99% of sessions).
    - **Non-security actions** (e.g., channel access revocation): Configurable grace period (default 30s) to save unsaved work (e.g., drafts, uploads). Org admins can disable grace periods.
    - **Unsaved work:** Auto-save drafts (messages, tasks) before termination if grace period enabled.
  - **Offline users:** Sync on reconnect (force re-authentication if permissions changed).
  - **Edge cases:** Log and alert on propagation delays > 5s or grace period violations.

*Example of marking unclear requirements:*

- **FR-016**: System MUST retain uploaded files indefinitely until explicitly deleted by a user with appropriate permissions, with organization-level storage quotas applied
- **FR-017**: System MUST allow workspace members to invite new users via email invitations only in v0.1, with shareable link invitations planned for future versions

### Key Entities

- **User**: Represents an individual account holder with authentication credentials (OIDC), profile information, and membership in **one or more organizations**. **Each session is scoped to a single organization at a time**, with independent role assignments per org.
- **Organization**: Represents a company or group that contains workspaces, users, roles, and shared resources; includes configurable retention policies for deleted content. **All entities (messages, tasks, files, channels) must include an `organization_id` column for database-level isolation.**
- **Workspace**: Represents a collaborative area within an organization for a specific project, team, or topic, containing channels, messages, tasks, and files. **Workspace isolation is enforced at the API layer, but `workspace_id` filtering in queries is optional for defense-in-depth.**
- **Role**: Represents a customizable set of permissions that can be assigned to users within an organization or workspace; permissions are granular and definable by organization administrators
- **Channel**: Represents a communication space within a workspace for text-based messaging, which can be public (open to all workspace members) or private (restricted to specific members)
- **Message**: Represents a unit of communication within a channel, which can be a standalone message or a reply to another message (forming a thread); supports soft-delete with recovery based on organization retention settings
- **Task**: Represents an actionable item that can be created, assigned to users, tracked through statuses, and commented on; supports soft-delete with recovery based on organization retention settings
- **File**: Represents a digital asset that can be uploaded (max 100MB default, org-configurable), organized in folders, and shared within workspaces and channels; executable file types (.exe, .bat, .sh, .ps1, etc.) blocked by default (org-configurable); supports soft-delete with recovery based on organization retention settings
- **Notification**: Represents an alert about a relevant event that a user should be aware of (mentions, task assignments, replies, file shares, content updates); user-configurable for each notification type

## Assumptions

- **AS-008:** During Zitadel unavailability, the system falls back to local email/password for existing users, blocks new registrations (unless admin-enabled), and operates in read-only mode for cached data.

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: New users can complete account registration and organization creation in under 3 minutes
- **SC-002**: Users can send and receive a message in under 2 seconds under normal network conditions (end-to-end delivery time, measured from WebSocket `send` to `onMessage` in the client).
  - **SC-002.1:** API response latency (e.g., `POST /messages`) must meet **sub-200ms p95** targets (see FR-027).
- **SC-003**: System supports concurrent active usage by at least 50 users per organization without significant performance degradation — degradation defined as p95 latency increase ≤ 50% from baseline, error rate ≤ 1%, memory ≤ 80% container limit, CPU ≤ 70%
- **SC-004**: 90% of users can successfully create and assign a task on their first attempt without assistance
- **SC-005**: Users can find a specific message from the past week using search in under 5 seconds
- **SC-011**: Search performance scales with time range — past month p95 < 10s, past year p95 < 30s; concurrent searches (50 users) p95 < 15s
- **SC-006**: File uploads of up to 10MB complete successfully in under 10 seconds on a standard broadband connection
- **SC-006.1:** File uploads must meet p95 < 30s for 100MB files on 10Mbps connections.
- **SC-007**: Notification alerts appear in the user interface within 3 seconds of the triggering event (low load) — tiered: low < 3s, medium < 7s, high < 15s p95
- **SC-012**: Real-time messaging performance — WebSocket connection p95 < 500ms, message fan-out p95 < 500ms per hop, presence updates p95 < 2s
- **SC-008**: System maintains 99.9% uptime for core functionality (authentication, messaging, task operations) during normal operations
- **SC-009**: Users report a satisfaction score of at least 4/5 for core collaboration features in post-usage surveys
- **SC-010**: Organizations with up to 100 active members can operate without requiring dedicated IT administration for routine operations

## Assumptions

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right assumptions based on reasonable defaults
  chosen when the feature description did not specify certain details.
-->

- Users have access to a modern web browser (Chrome, Firefox, Safari, or Edge) with JavaScript enabled
- Users have a stable internet connection for real-time collaboration features; v0.1 assumes stable connectivity — offline support and conflict resolution deferred to v0.2+; basic reconnection logic with exponential backoff implemented
- Organizations will typically have between 2-100 members for the Flora Seed v0.1 release
- External OIDC provider (Zitadel) is the primary authentication mechanism; when unavailable, new logins are denied but existing sessions continue via local cache for an organization-configurable grace period (default 15 minutes)
- Search functionality will index text content of common file types (PDF, DOC, TXT, MD) but not binary formats
- Notifications will be delivered via in-app indicators only (no email or push notifications in v0.1)
- The system will assume responsibility for data persistence and backup - users are not expected to manage their own backups in v0.1
- Initial deployment will target a single VPS or cloud instance following the "Seed" deployment topology from the constitution; performance and resource usage will be optimized to follow the "Debloat" principle, prioritizing minimal memory footprint and resource efficiency.
- Legal and compliance requirements (GDPR, etc.) will be addressed through configuration options rather than custom development
- User training will be minimal - the interface is designed to be intuitive for users familiar with modern collaboration tools

## Clarifications

### Session 2026-08-17
- Q: What specific permissions are associated with each role (Owner, Admin, Member) in the organization and workspace contexts? → A: Implement customizable role permissions with granular controls (more flexible but complex)
- Q: How does the system handle deletion of content (messages, tasks, files) - is it soft delete with recovery or hard delete immediately? → A: Soft delete with org-level retention
- Q: What file size limits and file type restrictions apply to uploads in Flora Seed v0.1? → A: Maximum 100MB per file default, configurable by the organization; block .exe, .bat, .sh, .ps1, and other executable file types for security, configurable by the organization.
- Q: What search capabilities are provided for global search across messages, tasks, and files? → A: Keyword search with basic filters
- Q: What types of notifications does the system generate, and can users configure which notifications they receive? → A: Comprehensive notifications with granular controls

### Session 2026-08-17 (Auth/Tenancy/Performance)
- Q: Can a single OIDC user belong to multiple organizations simultaneously? → A: One user can belong to multiple organizations with independent role assignments per org
- Q: When the OIDC provider (Zitadel) is unavailable during login or token validation, should the system fail closed or fail open with a cached/local fallback mechanism? → A: Fail closed for new logins, but allow existing sessions to continue via local cache for a short grace period, configurable per organization (default 15 minutes)
- Q: Is the 2-second messaging success criterion an end-to-end delivery time, while the sub-200ms p95 target from the plan applies only to API response latency? → A: Yes — 2s is end-to-end delivery, 200ms is API response latency; they are complementary targets
- Q: Should multi-tenancy data isolation be enforced at the application layer or database-level RLS? → A: Application-level filtering with organization_id on every query as the primary mechanism, with RLS considered for a future enhancement
### Session 2026-08-17 (Auth/Tenancy/Performance)
- Q: Should data isolation between organizations be enforced at the database query level or only at the application layer? → A: Database-level isolation with automatic `organization_id` filtering in all queries. Exceptions (e.g., shared admin views) must be explicitly documented and require manual overrides.
- Q: Should tenant isolation be enforced consistently across all entity types, or are exceptions allowed? → A: Controlled exceptions allowed. Exceptions must be documented in the data model, approved during code review, and include compensating controls (e.g., manual audits).
- Q: Should cross-organization access prevention requirements be explicitly defined for shared infrastructure like search, notifications, and real-time streams? → A: Yes — explicit requirements for shared infrastructure (search indices, pub/sub topics) to include `organization_id` scoping. Document and enforce these rules for defense in depth.
- Q: How should the system prevent data leakage via global search across organization boundaries? → A: Organization-scoped search indices (e.g., `flora_org_{org_id}`). Query-time filtering is insufficient for security.
- Q: Should the system define explicit degradation requirements when approaching resource limits on a minimal VPS? → A: Yes — explicit degradation rules: if memory > 80% or CPU > 70%, disable real-time features, throttle search, and degrade gracefully. Document fallback behavior.
- Q: Do the performance requirements in the spec (sub-2s messaging) and plan (sub-200ms p95 API latency) conflict? → A: No — they are complementary. "Sub-2s messaging" refers to end-to-end delivery time (user sends → recipient receives), while "sub-200ms p95" refers to API response latency (e.g., `POST /messages` returns 200). Documented in SC-002 and FR-027/FR-030.
- Q: Should file upload performance requirements be explicitly defined for the 100MB maximum? → A: Yes — explicit target: p95 < 30s for 100MB files on 10Mbps connections. 10MB target (SC-006) remains as baseline.
- Q: Should permission change propagation requirements be explicitly defined? → A: Yes — revoked permissions must take effect within 1s for 99% of active sessions. Offline users must sync on reconnect. Documented in FR-037.
- Q: Should the system define explicit requirements for handling permission changes on active user sessions? → A: Yes — immediate revocation for security-sensitive actions (e.g., role changes), with configurable grace periods (default 30s) for unsaved work (e.g., file uploads). Org admins can enforce immediate termination on demand. Documented in FR-037.
- Q: Should the relationship between OIDC identity provider and organization membership be explicitly defined? → A: Yes — one OIDC user can belong to multiple orgs, with independent role assignments per org. Sessions are scoped to a single org at a time. Documented in Key Entities (User).
- Q: Is the assumption that "Zitadel is always available" validated with fallback requirements? → A: Yes — explicit fallback to local email/password for **existing users only**. New registrations blocked during outages (admins can register locally if needed). Read-only mode for cached data. Documented in FR-022 and Assumptions.
- Q: Does "customizable role-based access control with granular permissions" define the scope of configurability? → A: Yes — explicit scope: org-level for roles/global permissions, workspace-level for channel/file access. Documented in FR-005.
- Q: Is "comprehensive in-app notifications" quantified with specific types and delivery guarantees? → A: Yes — explicit list: mentions (@user, @channel), task assignments, replies, file shares, content updates. Delivery guarantees: p95 < 3s (low load), < 7s (medium), < 15s (high). Documented in FR-012.
- Q: When a user's role is changed or permissions are revoked, should the change take effect immediately or with a delay? → A: Immediate — all active sessions are terminated instantly, any open resources are closed, but saved work is preserved and retrievable by the organization
- Q: What should be the default session lifetime and renewal behavior for authenticated users? → A: 24-hour session with automatic silent refresh via rotating refresh tokens every 6 hours
- Q: What logout behavior should the system implement — local session termination only, or full OIDC single logout (SLO) with Zitadel? → A: Configurable per organization: local-only or SLO with best-effort non-blocking SLO as default
- Q: How should token refresh be handled — automatic silent refresh in the background, or manual user-initiated refresh? → A: Automatic silent refresh using rotating refresh tokens (secure httpOnly cookies). On rotation failure, force re-authentication.
- Q: What rate limits should be applied to authentication endpoints (login, token refresh, invite acceptance)? → A: IP-based rate limits: 5 login attempts/minute, 20 token refreshes/minute per user, 10 invite acceptances/hour per IP. Configurable per organization.
- Q: Should MFA/2FA be explicitly excluded from v0.1, or included as a configurable option? → A: Include TOTP-based 2FA as optional, configurable per organization
- Q: Should fallback authentication mechanisms be defined for when OIDC integration fails or is misconfigured? → A: Local email/password fallback, disabled by default, enabled per organization by admin
- Q: How should OIDC claims be mapped to Flora roles and permissions? → A: Configurable per organization with sensible defaults: map OIDC `groups` claim to Flora roles, with admin UI for custom mapping rules
- Q: How should the system handle expired or revoked OIDC tokens during active user sessions? → A: Configurable per organization: strict (immediate termination - default) vs. lenient (grace period)
- Q: What failure behavior should be defined for external dependencies (Valkey for pub/sub, Meilisearch for search)? → A: Circuit breaker with graceful degradation: Valkey failure → disable real-time features, fall back to polling + queue locally for replay; Meilisearch failure → disable search, return empty results with notice; both with automatic recovery
- Q: Should the stable internet assumption for real-time features be reconciled with offline/degraded-network requirements? → A: v0.1 assumes stable connection; document limitation; basic reconnection logic with exponential backoff only; offline support in v0.2+
- Q: What specific performance targets should be defined for authentication flows (login, token refresh, session validation)? → A: Login (OIDC callback) p95 < 500ms; Token refresh p95 < 100ms; Session validation p95 < 10ms
- Q: What search performance requirements should be specified under concurrent load or for large result sets beyond the "past week" scope? → A: Tiered: Past week < 5s, Past month < 10s, Past year < 30s, Concurrent (50 users) < 15s
- Q: How should "significant performance degradation" in SC-003 be quantified with measurable thresholds? → A: Degradation defined as: p95 latency increase ≤ 50% from baseline, error rate ≤ 1%, memory usage ≤ 80% of container limit, CPU ≤ 70%
- Q: What notification delivery latency requirements should be defined under high event volume (many concurrent mentions/assignments)? → A: Tiered: Low < 3s, Medium < 7s, High < 15s
- Q: What real-time messaging performance requirements should be specified for concurrent active users (50 per org)? → A: WebSocket connect < 500ms, message fan-out < 500ms, presence updates < 2s; no specific resource targets
- Q: Should workspace isolation within an organization be specified with the same rigor as organization-level isolation? → A: Workspace isolation only at API layer (check membership, then query without workspace_id filter)
- Q: Should resource quota requirements (per-organization and per-workspace) be specified for file storage and user counts? → A: Default org: 10GB/100 users; workspace: 2GB inherited; enforced on upload with 413; 90% warning
- Q: Should concurrent session limits per user be specified, and requirements for session invalidation across devices defined? → A: Default 5 concurrent sessions/user; "log out everywhere" endpoint; LRU eviction; configurable per org
- Q: Are data retention requirements for soft-deleted content consistent with performance implications? → A: Soft-deleted data excluded from search indexes; separate "deleted items" view for recovery; daily cleanup batch; retention 30-365 days (default 90)
