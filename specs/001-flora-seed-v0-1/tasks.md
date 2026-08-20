# Tasks: Flora Seed v0.1 Core Features

**Input**: Design documents from `/specs/001-flora-seed-v0-1/`

**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: The examples below include test tasks. Tests are OPTIONAL - Flora uses a test-first approach per the Constitution.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US4)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Create project structure per implementation plan in `flora/`
- [X] T002 Initialize Rust workspace with Cargo.toml at `flora/Cargo.toml`
- [X] T003 [P] Configure linting (clippy, rustfmt) and deny unsafe code in workspace Cargo.toml
- [X] T004 [P] Add workspace dependencies: axum, thiserror, anyhow, tracing, sqlx, valkey-rs, meilisearch-sdk, openidconnect, serde, tokio, tower, uuid, chrono, mockall, backoff (for circuit breakers)
- [X] T005 [P] Setup gitignore for Rust, IDE, and environment files
- [X] T005.1 [P] Configure Meilisearch scoped indices template in `flora-search/src/config/meilisearch_template.json` (e.g., `flora_org_{org_id}`)
- [X] T005.2 [P] Configure Valkey pub/sub topic naming convention in `flora-messaging/src/config/valkey_topics.rs` (e.g., `org:{org_id}:channel:{channel_id}`)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 Setup database schema and migrations framework with `sqlx` in `flora-core/src/migrations/`
- [X] T006.1 [P] Add `organization_id` column (non-nullable, indexed) to all tenant-scoped tables (messages, tasks, files, channels, etc.) in `flora-core/src/migrations/`
- [X] T007 [P] Implement authentication/authorization framework using `openidconnect` in `flora-core/src/traits/auth_provider.rs`
- [X] T008 [P] Create base models for User, Organization, Membership, Role in `flora-core/src/models/`
- [X] T008.1 [P] Add `organization_id` field to all tenant-scoped models (e.g., `Message`, `Task`, `File`, `Channel`) in `flora-core/src/models/`
- [X] T009 [P] Implement multi-tenancy isolation middleware in `flora-api/src/extractors/auth.rs` (enforce `organization_id` filtering in all queries)
- [X] T010 [P] Setup error handling and logging infrastructure using `tracing` and `thiserror` in `flora-core/src/error.rs`
- [X] T011 Configure environment management with `.env` and `config` crate
- [X] T012 [P] Implement circuit breakers for external services (Zitadel, Meilisearch, Valkey) in `flora-core/src/utils/circuit_breaker.rs`
- [X] T012.1 [P] Configure fallback behavior for Zitadel outages (local email/password for existing users) in `flora-core/src/services/auth_service.rs`
- [X] T013 [P] Setup health check endpoints for all modules in `flora-api/src/routes/health.rs`
- [X] T014 [P] Implement connection pooling for PostgreSQL and Valkey
- [X] T015 [P] Create RustFS trait and local filesystem implementation in `flora-core/src/traits/storage_provider.rs` and `flora-core/src/storage/local.rs`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - User Registration and Organization Creation (Priority: P1) 🎯 MVP

**Goal**: Enable new users to create an account and set up their first organization.

**Independent Test**: Complete the registration flow with valid credentials and verify that a new organization is created and the user is authenticated and redirected to the organization dashboard.

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T016 [P] [US1] Contract test for `/auth/login` in `flora-tests/src/contract/auth.rs`
- [X] T017 [P] [US1] Integration test for registration flow in `flora-tests/src/integration/registration.rs`
- [X] T018 [P] [US1] Unit test for User model validation in `flora-core/tests/models/user.rs`

### Implementation for User Story 1

- [ ] T019 [P] [US1] Implement OIDC login flow with multi-org support in `flora-api/src/routes/auth.rs`
- [ ] T019.1 [P] [US1] Add session scoping to a single organization at a time in `flora-api/src/state.rs`
- [ ] T020 [P] [US1] Create Organization model and repository in `flora-core/src/models/organization.rs` and `flora-core/src/repositories/organization_repository.rs`
- [ ] T021 [P] [US1] Create User repository in `flora-core/src/repositories/user_repository.rs`
- [ ] T022 [P] [US1] Implement Membership model and repository in `flora-core/src/models/membership.rs` and `flora-core/src/repositories/membership_repository.rs`
- [ ] T023 [US1] Implement session management with configurable grace period in `flora-core/src/services/session_service.rs`
- [ ] T023.1 [US1] Add immediate session termination for security-sensitive actions (e.g., role revocation) in `flora-core/src/services/session_service.rs`
- [ ] T024 [US1] Add organization creation endpoint in `flora-api/src/routes/org.rs`
- [ ] T025 [US1] Implement role-based access control for organization owners in `flora-core/src/services/rbac_service.rs`
- [ ] T025.1 [US1] Add org/workspace-level scope to RBAC (FR-005) in `flora-core/src/services/rbac_service.rs`
- [ ] T026 [US1] Add validation for email format and password complexity
- [ ] T027 [US1] Implement error handling for duplicate emails and invalid inputs
- [ ] T028 [US1] Add logging for registration and login events

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 4 - Core Messaging - Channels and Messages (Priority: P1)

**Goal**: Enable real-time communication through channels and messages.

**Independent Test**: Create a channel, post messages, reply to messages, edit messages, and delete messages, with all operations persisting correctly and visible to other workspace members.

### Tests for User Story 4 ⚠️

- [X] T029 [P] [US4] Contract test for `/channels/{channel_id}/messages` in `flora-tests/src/contract/messaging.rs`
- [X] T030 [P] [US4] Integration test for messaging flow in `flora-tests/src/integration/messaging.rs`
- [X] T031 [P] [US4] Unit test for Message model validation in `flora-core/tests/models/message.rs`

### Implementation for User Story 4

- [ ] T032 [P] [US4] Create Channel model and repository in `flora-core/src/models/channel.rs` and `flora-core/src/repositories/channel_repository.rs`
- [ ] T033 [P] [US4] Create Message model and repository in `flora-core/src/models/message.rs` and `flora-core/src/repositories/message_repository.rs`
- [ ] T034 [P] [US4] Implement real-time pub/sub using Valkey with organization-scoped topics in `flora-messaging/src/lib.rs`
- [ ] T034.1 [P] [US4] Add `organization_id` validation on WebSocket connection and message publishing in `flora-api/src/websocket/mod.rs`
- [ ] T035 [US4] Implement channel creation endpoint in `flora-api/src/routes/messaging.rs`
- [ ] T036 [US4] Implement message posting, editing, and deletion endpoints in `flora-api/src/routes/messaging.rs`
- [ ] T036.1 [US4] Add performance targets for real-time messaging (WebSocket < 500ms, fan-out < 500ms) in `flora-messaging/src/lib.rs`
- [ ] T037 [US4] Add threading support for message replies
- [ ] T038 [US4] Implement soft-delete for messages with retention policy (30–365 days, default 90)
- [ ] T039 [US4] Add validation for message content and channel permissions
- [ ] T040 [US4] Add logging for messaging operations

**Checkpoint**: At this point, User Stories 1 AND 4 should both work independently

---

## Phase 5: User Story 2 - Organization and Workspace Management (Priority: P2)

**Goal**: Enable organization members to create and manage workspaces.

**Independent Test**: Create multiple workspaces, switch between them, and verify that each workspace maintains its own isolated data.

### Tests for User Story 2 ⚠️

- [ ] T041 [P] [US2] Contract test for `/workspaces` in `flora-tests/src/contract/workspace.rs`
- [ ] T042 [P] [US2] Integration test for workspace management in `flora-tests/src/integration/workspace.rs`
- [ ] T043 [P] [US2] Unit test for Workspace model validation in `flora-core/tests/models/workspace.rs`

### Implementation for User Story 2

- [ ] T044 [P] [US2] Create Workspace model and repository in `flora-core/src/models/workspace.rs` and `flora-core/src/repositories/workspace_repository.rs`
- [ ] T045 [US2] Implement workspace creation and management endpoints in `flora-api/src/routes/workspace.rs`
- [ ] T046 [US2] Add workspace switching logic in `flora-api/src/state.rs`
- [ ] T047 [US2] Implement data isolation between workspaces
- [ ] T048 [US2] Add validation for workspace names and descriptions
- [ ] T049 [US2] Add logging for workspace operations

**Checkpoint**: User Stories 1, 2, and 4 should now all work independently

---

## Phase 6: User Story 3 - Role and Permission Management (Priority: P2)

**Goal**: Enable organization owners and admins to assign roles and manage permissions.

**Independent Test**: Invite a user to an organization, assign them different roles, and verify that their access to organizational features corresponds to their role permissions.

### Tests for User Story 3 ⚠️

- [ ] T050 [P] [US3] Contract test for role management endpoints in `flora-tests/src/contract/rbac.rs`
- [ ] T051 [P] [US3] Integration test for role assignment in `flora-tests/src/integration/rbac.rs`
- [ ] T052 [P] [US3] Unit test for Role model validation in `flora-core/tests/models/role.rs`

### Implementation for User Story 3

- [ ] T053 [P] [US3] Implement Role model and repository in `flora-core/src/models/role.rs` and `flora-core/src/repositories/role_repository.rs`
- [ ] T054 [US3] Implement permission management service in `flora-core/src/services/permission_service.rs`
- [ ] T054.1 [US3] Add permission propagation within 1s for 99% of active sessions (FR-037) in `flora-core/src/services/permission_service.rs`
- [ ] T055 [US3] Add role assignment endpoints in `flora-api/src/routes/rbac.rs`
- [ ] T056 [US3] Implement immediate session termination on permission changes (with configurable grace period for unsaved work)
- [ ] T056.1 [US3] Add auto-save for drafts (messages, tasks) before session termination in `flora-core/src/services/draft_service.rs`
- [ ] T057 [US3] Add validation for role names and permission sets
- [ ] T058 [US3] Add logging for permission changes
- [ ] T058.1 [US3] Add alerting for permission propagation delays > 5s in `flora-core/src/services/permission_service.rs`

**Checkpoint**: User Stories 1, 2, 3, and 4 should now all work independently

---

## Phase 7: User Story 5 - Task Management (Priority: P2)

**Goal**: Enable team members to create, assign, and track tasks.

**Independent Test**: Create tasks, assign them to team members, update their status, and verify that all operations persist correctly and are visible to relevant team members.

### Tests for User Story 5 ⚠️

- [ ] T059 [P] [US5] Contract test for `/tasks` in `flora-tests/src/contract/tasks.rs`
- [ ] T060 [P] [US5] Integration test for task management in `flora-tests/src/integration/tasks.rs`
- [ ] T061 [P] [US5] Unit test for Task model validation in `flora-core/tests/models/task.rs`

### Implementation for User Story 5

- [ ] T062 [P] [US5] Create Task model and repository in `flora-core/src/models/task.rs` and `flora-core/src/repositories/task_repository.rs`
- [ ] T063 [US5] Implement task creation and management endpoints in `flora-api/src/routes/tasks.rs`
- [ ] T064 [US5] Add task assignment and status update logic
- [ ] T065 [US5] Implement soft-delete for tasks with retention policy
- [ ] T066 [US5] Add validation for task fields and permissions
- [ ] T067 [US5] Add logging for task operations

---

## Phase 8: User Story 6 - File Management (Priority: P2)

**Goal**: Enable team members to upload, organize, and share files.

**Independent Test**: Upload files to a workspace or channel, organize them in folders, share them with team members, and verify that files are accessible with correct permissions.

### Tests for User Story 6 ⚠️

- [ ] T068 [P] [US6] Contract test for `/files` in `flora-tests/src/contract/files.rs`
- [ ] T069 [P] [US6] Integration test for file upload and sharing in `flora-tests/src/integration/files.rs`
- [ ] T070 [P] [US6] Unit test for File model validation in `flora-core/tests/models/file.rs`

### Implementation for User Story 6

- [ ] T071 [P] [US6] Create File model and repository in `flora-core/src/models/file.rs` and `flora-core/src/repositories/file_repository.rs`
- [ ] T072 [US6] Implement file upload endpoint with chunking and performance targets (p95 < 30s for 100MB) in `flora-api/src/routes/files.rs`
- [ ] T072.1 [US6] Add quota enforcement (10GB/org, 2GB/workspace) with 413 response in `flora-core/src/services/quota_service.rs`
- [ ] T073 [US6] Add file sharing and permission logic (workspace/channel-level)
- [ ] T074 [US6] Implement soft-delete for files with retention policy (30–365 days, default 90)
- [ ] T075 [US6] Add validation for file types and size limits (block executables by default)
- [ ] T076 [US6] Add logging for file operations

---

## Phase 9: User Story 7 - Global Search (Priority: P2)

**Goal**: Enable workspace members to search across messages, tasks, and files.

**Independent Test**: Create sample data (messages, tasks, files) and verify that search returns relevant results based on keywords, with proper filtering by content type.

### Tests for User Story 7 ⚠️

- [ ] T077 [P] [US7] Contract test for `/search` in `flora-tests/src/contract/search.rs`
- [ ] T078 [P] [US7] Integration test for search functionality in `flora-tests/src/integration/search.rs`

### Implementation for User Story 7

- [ ] T079 [US7] Implement Meilisearch indexing for messages, tasks, and files with organization-scoped indices (`flora_org_{org_id}`) in `flora-search/src/lib.rs`
- [ ] T079.1 [US7] Add `organization_id` filtering to all search queries (prohibit query-time filtering) in `flora-search/src/lib.rs`
- [ ] T080 [US7] Add search endpoint in `flora-api/src/routes/search.rs`
- [ ] T080.1 [US7] Add performance targets for search (past week < 5s, past month < 10s, past year < 30s) in `flora-search/src/lib.rs`
- [ ] T081 [US7] Implement filtering by content type and date range
- [ ] T082 [US7] Add validation for search queries
- [ ] T083 [US7] Add logging for search operations

---

## Phase 10: User Story 8 - Notifications (Priority: P2)

**Goal**: Enable team members to receive in-app notifications for relevant activities.

**Independent Test**: Trigger notification events (mentions, task assignments, etc.) and verify that appropriate notifications appear in the notification center and are cleared when viewed.

### Tests for User Story 8 ⚠️

- [ ] T084 [P] [US8] Contract test for `/notifications` in `flora-tests/src/contract/notifications.rs`
- [ ] T085 [P] [US8] Integration test for notification delivery in `flora-tests/src/integration/notifications.rs`
- [ ] T086 [P] [US8] Unit test for Notification model validation in `flora-core/tests/models/notification.rs`

### Implementation for User Story 8

- [ ] T087 [P] [US8] Create Notification model and repository in `flora-core/src/models/notification.rs` and `flora-core/src/repositories/notification_repository.rs`
- [ ] T088 [US8] Implement notification dispatch using Valkey streams with delivery guarantees (p95 < 3s/7s/15s) in `flora-notifications/src/lib.rs`
- [ ] T088.1 [US8] Add queue and retry for failed deliveries (max 3 attempts) in `flora-notifications/src/lib.rs`
- [ ] T089 [US8] Add notification endpoints in `flora-api/src/routes/notifications.rs`
- [ ] T090 [US8] Implement user-configurable notification preferences (urgent vs. non-urgent)
- [ ] T090.1 [US8] Add explicit notification types (mentions, task assignments, replies, file shares, content updates) in `flora-core/src/models/notification.rs`
- [ ] T091 [US8] Add validation for notification types
- [ ] T092 [US8] Add logging for notification events
- [ ] T092.1 [US8] Add alerting for undelivered notifications in `flora-notifications/src/lib.rs`

---

## Phase 11: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T093 [P] Documentation updates in `docs/` (include auth/tenancy/performance decisions)
- [ ] T094 [P] Code cleanup and refactoring across all crates
- [ ] T095 [P] Performance optimization (load testing with k6, memory profiling)
- [ ] T095.1 [P] Implement degradation rules (disable real-time features, throttle search at 80% memory/70% CPU) in `flora-core/src/services/degradation_service.rs`
- [ ] T096 [P] Security hardening (dependency audits, penetration testing)
- [ ] T097 [P] Run quickstart.md validation scenarios
- [ ] T098 [P] Update research.md with any new decisions made during implementation
- [ ] T099 [P] Generate API documentation using `cargo doc`
- [ ] T100 Final integration testing across all user stories
- [ ] T100.1 [P] Validate permission propagation (1s for 99% of sessions) and retention bounds (30–365 days) in integration tests

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 4 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Integrates with US1
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - Depends on US1 (users/orgs)
- **User Story 5 (P2)**: Can start after Foundational (Phase 2) - Depends on US2 (workspaces)
- **User Story 6 (P2)**: Can start after Foundational (Phase 2) - Depends on US2 (workspaces)
- **User Story 7 (P2)**: Can start after Foundational (Phase 2) - Depends on US4, US5, US6 (data to index)
- **User Story 8 (P2)**: Can start after Foundational (Phase 2) - Depends on US4, US5 (events to notify)

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Models before services
- Services before endpoints
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together:
Task: "Contract test for /auth/login in flora-tests/src/contract/auth.rs"
Task: "Integration test for registration flow in flora-tests/src/integration/registration.rs"
Task: "Unit test for User model validation in flora-core/tests/models/user.rs"

# Launch all models for User Story 1 together:
Task: "Create Organization model and repository in flora-core/src/models/organization.rs and flora-core/src/repositories/organization_repository.rs"
Task: "Create User repository in flora-core/src/repositories/user_repository.rs"
Task: "Implement Membership model and repository in flora-core/src/models/membership.rs and flora-core/src/repositories/membership_repository.rs"
```

---

## Implementation Strategy

### MVP First (User Stories 1 + 4 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1 (Registration & Org Creation)
4. Complete Phase 4: User Story 4 (Messaging)
5. **STOP and VALIDATE**: Test User Stories 1 + 4 independently
6. Deploy/demo if ready (MVP!)

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Stories 1 + 4 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 (Workspaces) → Test independently → Deploy/Demo
4. Add User Story 3 (RBAC) → Test independently → Deploy/Demo
5. Add User Story 5 (Tasks) → Test independently → Deploy/Demo
6. Add User Story 6 (Files) → Test independently → Deploy/Demo
7. Add User Story 7 (Search) → Test independently → Deploy/Demo
8. Add User Story 8 (Notifications) → Test independently → Deploy/Demo
9. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 (Registration)
   - Developer B: User Story 4 (Messaging)
   - Developer C: User Story 2 (Workspaces)
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
