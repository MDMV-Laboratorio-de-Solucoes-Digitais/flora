# Tasks: Flora Seed v0.1 (Svelte 5 Rewrite)

**Feature**: [plan.md](./plan.md) | [spec.md](./spec.md)
**Status**: Ready for Implementation

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [X] T001 Initialize SvelteKit project in `apps/frontend/` with Svelte 5 (Runes Mode)
- [X] T002 Configure Tailwind CSS and shadcn-svelte in `apps/frontend/`
- [X] T003 [P] Configure strict `tsconfig.json` (`strict: true`, no `any`) in `apps/frontend/tsconfig.json`
- [X] T004 [P] Configure strict `eslint.config.js` (no-console, pedantic rules, `eslint-plugin-unicorn`) in `apps/frontend/eslint.config.js`
- [X] T005 Setup testing framework (Vitest, Playwright) in `apps/frontend/vitest.config.ts` and `apps/frontend/playwright.config.ts`
- [X] T006 Configure Lefthook CI for `svelte-check --fail-on-warnings` in `apps/frontend/lefthook.yml`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T007 Implement strict Logger utility per contract in `apps/frontend/src/lib/utils/logger.ts`
- [X] T008 Implement background log batching to `/api/v1/telemetry/logs` in `apps/frontend/src/lib/utils/logger.ts`
- [X] T009 [P] Create base API Client pointing to local Rust backend in `apps/frontend/src/lib/api/client.ts`
- [X] T010 Integrate Logger to replace all native console usage across the app

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Secure Identity & Access Management (Priority: P1) 🎯 MVP

**Goal**: System MUST provide OIDC-based authentication, token management, and organization management.

**Independent Test**: Users can successfully authenticate via Zitadel and load their workspace session.

### Tests for User Story 1 (MANDATORY)

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T011 [P] [US1] Write unit test for AuthState logic in `apps/frontend/tests/unit/AuthState.test.ts`
- [X] T012 [P] [US1] Write integration test for session interceptor in `apps/frontend/tests/integration/api-interceptor.test.ts`

### Implementation for User Story 1

- [X] T013 [P] [US1] Implement 401 Session Interceptor and Hybrid Security Model in `apps/frontend/src/lib/api/client.ts`
- [X] T014 [US1] Create AuthState Svelte 5 Rune in `apps/frontend/src/lib/state/AuthState.svelte.ts`
- [X] T015 [US1] Implement Zitadel OIDC login routes in `apps/frontend/src/routes/(auth)/+page.svelte`
- [X] T016 [US1] Implement Workspace creation UI components in `apps/frontend/src/lib/components/workspace/`
- [X] T017 [US1] Implement Organization management UI components in `apps/frontend/src/lib/components/organization/`
- [X] T018 [US1] Implement User and Role management UI components in `apps/frontend/src/lib/components/users/`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Real-Time Collaboration & Messaging (Priority: P1)

**Goal**: System MUST support real-time text channels and Valkey pub/sub notifications.

**Independent Test**: Users can send a real-time message without page refreshes and receive notifications.

### Tests for User Story 2 (MANDATORY)

- [X] T019 [P] [US2] Write unit test for MessageStore and UUID deduplication in `apps/frontend/tests/unit/MessageStore.test.ts`
- [X] T020 [P] [US2] Write unit test for WebSocket reconnection fallback logic in `apps/frontend/tests/unit/websocket.test.ts`

### Implementation for User Story 2

- [X] T021 [US2] Create MessageStore Svelte 5 Rune in `apps/frontend/src/lib/state/MessageStore.svelte.ts`
- [X] T022 [US2] Implement Valkey WebSocket client with reconnect and REST fallback sync in `apps/frontend/src/lib/api/websocket.ts`
- [X] T023 [US2] Add real-time event listener for `workspace.user.removed` session invalidation in `apps/frontend/src/lib/state/AuthState.svelte.ts`
- [X] T024 [US2] Implement chat and message UI components in `apps/frontend/src/lib/components/chat/`
- [X] T025 [US2] Build Channel layout and routing in `apps/frontend/src/routes/(app)/channels/`
- [X] T026 [US2] Build Direct Message layout and routing in `apps/frontend/src/routes/(app)/dms/`
- [X] T027 [US2] Implement Thread reply UI components in `apps/frontend/src/lib/components/chat/thread/`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Task Tracking & Productivity (Priority: P2)

**Goal**: System MUST allow users to create, assign, and track the status of Tasks.

**Independent Test**: Users can view and manage a list of tasks independently.

### Tests for User Story 3 (MANDATORY)

- [X] T028 [P] [US3] Write unit test for TaskStore in `apps/frontend/tests/unit/TaskStore.test.ts`
- [X] T029 [P] [US3] Write E2E test for task creation flow in `apps/frontend/tests/e2e/tasks.spec.ts`

### Implementation for User Story 3

- [X] T030 [P] [US3] Create TaskStore Svelte 5 Rune in `apps/frontend/src/lib/state/TaskStore.svelte.ts`
- [X] T031 [US3] Implement Task management UI components (forms, filters) in `apps/frontend/src/lib/components/tasks/`
- [X] T032 [US3] Implement Task routes in `apps/frontend/src/routes/(app)/tasks/`

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: User Story 4 - File Management via RustFS (Priority: P2)

**Goal**: System MUST support file uploads and downloads with client-side limits.

**Independent Test**: Users can successfully upload a file with pre-flight checks and handle errors gracefully.

### Tests for User Story 4 (MANDATORY)

- [X] T033 [P] [US4] Write unit test for client-side file size checks in `apps/frontend/tests/unit/fileUpload.test.ts`
- [X] T034 [P] [US4] Write E2E test for file upload failures in `apps/frontend/tests/e2e/files.spec.ts`

### Implementation for User Story 4

- [X] T035 [P] [US4] Create FileStore Svelte 5 Rune in `apps/frontend/src/lib/state/FileStore.svelte.ts`
- [X] T036 [US4] Implement 50MB pre-flight size check logic in `apps/frontend/src/lib/utils/files.ts`
- [X] T037 [US4] Implement file upload UI component with explicit Promise catch blocks in `apps/frontend/src/lib/components/files/`
- [X] T037b [US4] Implement file download and preview UI components in `apps/frontend/src/lib/components/files/`

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [X] T038 [P] Documentation updates in `apps/frontend/README.md`
- [X] T039 Review all Promises to ensure explicit `.catch()` handlers
- [X] T040 Run `svelte-check` and ensure 0 instances of `#ignore` or warnings
- [X] T041 Run `quickstart.md` manual validation scenarios
- [X] T042 Configure SvelteKit adapter (e.g. static/node) for VPS deployment in `apps/frontend/svelte.config.js`
- [X] T043 Update deployment proxy/container configuration for frontend assets in `apps/frontend/Dockerfile` or Coolify config

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P1)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable
- **User Story 4 (P2)**: Can start after Foundational (Phase 2) - May integrate with US2/US3 but should be independently testable

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
Task: T011 Write unit test for AuthState logic
Task: T012 Write integration test for session interceptor

# Launch independent implementation steps:
Task: T013 Implement 401 Session Interceptor
Task: T016 Implement Workspace creation UI components
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 & 3
   - Developer B: User Story 2 & 4
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

## Phase 8: Convergence

- [X] T044 CRITICAL: Fix all ESLint errors (including `any` types) per Constitution IV (contradicts)
- [X] T045 CRITICAL: Resolve all `svelte-check` warnings per Constitution IV (contradicts)
- [X] T046 CRITICAL: Remove native console usage (`console.error` in FileUpload) per Constitution IV (contradicts)
- [X] T047 Implement Hybrid Security Model for 401 responses (UI lock, 5-min grace period) per US1 Edge Cases (partial)
- [X] T048 Implement WebSocket REST fallback sync and message deduplication per US2 Edge Cases (partial)
- [X] T049 Implement full-screen UI lock for active session invalidation per US2 Edge Cases (partial)
- [X] T050 Configure SvelteKit adapter by creating `svelte.config.js` per plan.md Phase 7 (missing)
- [X] T051 Replace file upload `alert` with user-friendly error component per US4 Edge Cases (partial)
- [X] T052 Add `beforeunload` listener to batch logger per plan.md Phase 2 (missing)

## Phase 9: Convergence

- [X] T053 Implement real Zitadel OIDC flow per FR-001 (missing)
- [X] T054 Integrate workspace creation with backend API per FR-002 (partial)
- [X] T055 Implement functional chat and messaging UI with API integration per FR-003 (partial)
- [X] T056 Implement functional task management UI with API integration per FR-005 (partial)
- [X] T057 Integrate file upload with RustFS backend API per FR-006 (partial)
- [X] T058 Wire up WebSocket connection in app lifecycle per FR-004 (missing)
- [X] T059 Install and configure base shadcn-svelte UI components per plan.md Phase 1 (missing)
- [X] T060 Apply data model interfaces to stores and implement CRUD logic per data-model.md (partial)

## Phase 10: Convergence

- [X] T061 Implement forced redirect and full-screen UI lock on active session invalidation per US2 Edge Cases (partial)
- [X] T062 Implement user invitation and role assignment UI with API integration per US1/AC2 (partial)
- [X] T063 Implement Organization management UI with API integration per FR-002 (partial)
- [X] T064 Add task assignment, filtering by assignee, and status tracking UI per FR-005 (partial)
- [X] T065 Implement Direct Messages and Thread reply UI functionality per FR-003 (partial)
- [X] T066 Implement file download UI, file preview, and message attachment linking per FR-006 (partial)

## Phase 11: Convergence

- [X] T067 Inject `Authorization: Bearer <token>` into API client headers using `authState` per FR-001 (partial)
- [X] T068 Implement actual `WebSocket` connection in `WsClient` utilizing the auth token per FR-004 (missing)
- [X] T069 Build a global navigation sidebar to access core routes (Channels, DMs, Tasks, Settings) per plan: Structure (missing)
- [X] T070 Mount Organization, Workspace, and User management components into accessible routes per FR-002 (partial)
- [X] T071 Add `onMount` logic to `ChatLayout.svelte` to fetch initial messages for the channel per FR-003 (partial)

## Phase 12: Convergence

- [X] T072 Fix ESLint errors to pass strict unicorn and typescript rules per Constitution IV (contradicts)
- [X] T073 Rewrite E2E and unit tests to assert actual UI flows instead of stubs per Constitution V (contradicts)

## Phase 13: Convergence

- [X] T074 CRITICAL: Fix svelte-check errors in apps/frontend/tests/unit/TaskStore.test.ts per Constitution IV (contradicts)

