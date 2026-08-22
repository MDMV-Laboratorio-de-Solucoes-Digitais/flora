# Implementation Tasks: Flora Seed v0.1 (Svelte 5 Rewrite)

**Feature**: Flora Seed v0.1 (Svelte 5 Rewrite)
**Branch**: `002-svelte-frontend-rewrite`

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [ ] T001 Initialize Bun project in `apps/frontend`
- [ ] T002 Initialize SvelteKit project with Svelte 5 (Runes Mode) in `apps/frontend`
- [ ] T003 [P] Configure `tsconfig.json` with strict TypeScript requirements (`strict: true`, `noImplicitAny: true`) in `apps/frontend/tsconfig.json`
- [ ] T004 [P] Configure ESLint with `eslint-plugin-unicorn`, `@typescript-eslint/strict`, and `@typescript-eslint/stylistic-type-checked` to ban `console.log`, `any` types, unsafe assignments (`@typescript-eslint/no-unsafe-assignment`), and mandate Promise handling (`@typescript-eslint/no-floating-promises`, `@typescript-eslint/no-misused-promises`) matching `clippy::pedantic` rigor in `apps/frontend/eslint.config.js`
- [ ] T005 [P] Configure Tailwind CSS and shadcn-svelte in `apps/frontend/tailwind.config.js` and `apps/frontend/components.json`
- [ ] T006 [P] Add Vitest and Playwright for unit and E2E testing in `apps/frontend/vitest.config.ts` and `apps/frontend/playwright.config.ts`
- [ ] T006b [P] Configure Vite proxy in `apps/frontend/vite.config.ts` and `.env` to route `/api` to the local offline Rust backend

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core utilities and API clients required before any UI work.

- [ ] T007 Implement structured OpenTelemetry-compatible `Logger` class replacing console.log, including a 5-second interval and `beforeunload` batch POST logic to the Axum backend in `apps/frontend/src/lib/utils/logger.ts`
- [ ] T008 Implement base global API client using native fetch in `apps/frontend/src/lib/api/client.ts`
- [ ] T009 Implement Global 401 Interceptor and 5-Minute Read-Only Cache UI lock in `apps/frontend/src/lib/api/interceptor.ts` and `apps/frontend/src/lib/components/SessionLock.svelte`
- [ ] T010 Define base TypeScript entity models (UUID, ISO8601Date) in `apps/frontend/src/lib/models/types.ts`

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Secure Identity & Access Management (Priority: P1) 🎯 MVP

**Goal**: As an organization administrator, create/manage workspaces, assign roles, and authenticate via OIDC (Zitadel).
**Independent Test**: Simulate an admin creating an organization, inviting a user, assigning them a role, and authenticating via Zitadel.

### Tests for User Story 1
> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [ ] T011 [P] [US1] Unit test for AuthState runes in `apps/frontend/src/lib/state/AuthState.test.ts`
- [ ] T012 [P] [US1] E2E test for OIDC login flow in `apps/frontend/tests/auth.e2e.ts`
- [ ] T012b [P] [US1] E2E test for Organization creation, Workspace setup, and User invitation flow in `apps/frontend/tests/org.e2e.ts`
- [ ] T013 [P] [US1] Unit test for Workspace and Role management UI components in `apps/frontend/src/routes/(app)/settings/workspace/Workspace.test.ts`
- [ ] T013b [P] [US1] Unit test for Organization creation and User invitation UI components in `apps/frontend/src/routes/(app)/settings/organization/Organization.test.ts`

- [ ] **GATE**: Stop and wait for User Approval of the above tests before beginning implementation.

### Implementation for User Story 1

- [ ] T014 [P] [US1] Define `User`, `Organization`, `Workspace`, and `WorkspaceRole` TypeScript interfaces in `apps/frontend/src/lib/models/auth.ts`
- [ ] T015 [US1] Implement `AuthState.svelte.ts` Runes for global session management in `apps/frontend/src/lib/state/AuthState.svelte.ts`
- [ ] T016 [US1] Implement Zitadel OIDC login route and callback handler in `apps/frontend/src/routes/(auth)/login/+page.svelte` and `apps/frontend/src/routes/(auth)/callback/+server.ts`
- [ ] T017 [US1] Implement Hybrid Security Model (5-minute Read-Only Cache mode on refresh failure) in `apps/frontend/src/lib/state/AuthState.svelte.ts`
- [ ] T018 [US1] Create Workspace and Role management UI components in `apps/frontend/src/routes/(app)/settings/workspace/+page.svelte`
- [ ] T018b [US1] Create Organization creation and User invitation flow UI components in `apps/frontend/src/routes/(app)/settings/organization/+page.svelte`

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Real-Time Collaboration & Messaging (Priority: P1)

**Goal**: As a team member, communicate via channels, DMs, and threads in real time.
**Independent Test**: Two users log in on separate sessions and exchange messages instantly without page reloads.

### Tests for User Story 2

- [ ] T019 [P] [US2] Unit test for Message deduplication logic in `apps/frontend/src/lib/state/MessageStore.test.ts`
- [ ] T020 [P] [US2] Unit test for WebSocket reconnect fallback sync in `apps/frontend/src/lib/api/socket.test.ts`
- [ ] T021 [P] [US2] Unit test for Channel layout and message list UI in `apps/frontend/src/routes/(app)/channels/[id]/Channel.test.ts`
- [ ] T022 [P] [US2] Unit test for Thread overlay UI in `apps/frontend/src/lib/components/messaging/ThreadView.test.ts`
- [ ] T023 [P] [US2] Unit test for Notification bell/toast UI in `apps/frontend/src/lib/components/messaging/NotificationUI.test.ts`
- [ ] T023b [P] [US2] E2E test for real-time WebSocket messaging flow in `apps/frontend/tests/messaging.e2e.ts`

- [ ] **GATE**: Stop and wait for User Approval of the above tests before beginning implementation.

### Implementation for User Story 2

- [ ] T024 [P] [US2] Define `Channel`, `Message`, `Thread`, and `Notification` TypeScript interfaces in `apps/frontend/src/lib/models/messaging.ts`
- [ ] T025 [US2] Implement `MessageStore.svelte.ts` Runes for message state in `apps/frontend/src/lib/state/MessageStore.svelte.ts`
- [ ] T026 [US2] Implement Valkey WebSocket client with connection lifecycle management in `apps/frontend/src/lib/api/socket.ts`
- [ ] T027 [US2] Implement REST Fallback Sync (`last_known_message_id`) and Rune deduplication on WS reconnect in `apps/frontend/src/lib/state/MessageStore.svelte.ts`
- [ ] T027b [US2] Implement Valkey Pub/Sub listener for `workspace.user.removed` to trigger an immediate Full-Screen Redirect Lock in `apps/frontend/src/lib/api/socket.ts`
- [ ] T028 [US2] Build Channel layout and real-time message list UI in `apps/frontend/src/routes/(app)/channels/[id]/+page.svelte`
- [ ] T028b [US2] Build Direct Message (DM) list view and chat layout in `apps/frontend/src/routes/(app)/dms/[id]/+page.svelte`
- [ ] T029 [US2] Build Thread overlay/sidebar UI in `apps/frontend/src/lib/components/messaging/ThreadView.svelte`
- [ ] T030 [US2] Build Notification bell/toast UI component in `apps/frontend/src/lib/components/messaging/NotificationUI.svelte`

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Task Tracking & Productivity (Priority: P2)

**Goal**: As a workspace user, create, assign, and track tasks.
**Independent Test**: Create a task, assign to user, change status to "Done", verify state persists.

### Tests for User Story 3

- [ ] T031 [P] [US3] Unit test for Task state management in `apps/frontend/src/lib/state/TaskStore.test.ts`
- [ ] T032 [P] [US3] Unit test for Task Board/List UI in `apps/frontend/src/routes/(app)/tasks/Tasks.test.ts`
- [ ] T033 [P] [US3] Unit test for Task Modal UI in `apps/frontend/src/lib/components/tasks/TaskModal.test.ts`
- [ ] T033b [P] [US3] E2E test for Task creation and board filtering in `apps/frontend/tests/tasks.e2e.ts`

- [ ] **GATE**: Stop and wait for User Approval of the above tests before beginning implementation.

### Implementation for User Story 3

- [ ] T034 [P] [US3] Define `Task` TypeScript interface in `apps/frontend/src/lib/models/productivity.ts`
- [ ] T035 [US3] Implement `TaskStore.svelte.ts` Runes for task state in `apps/frontend/src/lib/state/TaskStore.svelte.ts`
- [ ] T036 [US3] Implement Task Board/List UI with filtering in `apps/frontend/src/routes/(app)/tasks/+page.svelte`
- [ ] T037 [US3] Create Task form modal for creation and editing in `apps/frontend/src/lib/components/tasks/TaskModal.svelte`

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: User Story 4 - File Management via RustFS (Priority: P2)

**Goal**: As a workspace user, upload, organize, and retrieve files within channels and tasks.
**Independent Test**: Upload a document to a channel and verify it can be downloaded and previewed by another user.

### Tests for User Story 4

- [ ] T038 [P] [US4] Unit test for 50MB file size pre-flight check in `apps/frontend/src/lib/api/upload.test.ts`
- [ ] T039 [P] [US4] Unit test for File Uploader UI in `apps/frontend/src/lib/components/shared/FileUploader.test.ts`
- [ ] T040 [P] [US4] Unit test for File Attachment rendering in `apps/frontend/src/lib/components/messaging/MessageItem.test.ts`
- [ ] T040b [P] [US4] E2E test for File upload and attachment rendering in `apps/frontend/tests/files.e2e.ts`

- [ ] **GATE**: Stop and wait for User Approval of the above tests before beginning implementation.

### Implementation for User Story 4

- [ ] T041 [P] [US4] Define `FileAttachment` TypeScript interface in `apps/frontend/src/lib/models/productivity.ts`
- [ ] T042 [US4] Implement upload API client with client-side 50MB pre-flight size checks in `apps/frontend/src/lib/api/upload.ts`
- [ ] T043 [US4] Implement file picker and drag-drop UI component in `apps/frontend/src/lib/components/shared/FileUploader.svelte`
- [ ] T044 [US4] Add file attachment rendering to Message components in `apps/frontend/src/lib/components/messaging/MessageItem.svelte`
- [ ] T045 [US4] Implement explicit Promise catch blocks to route upload rejections to the Logger and display UI toasts in `apps/frontend/src/lib/components/shared/FileUploader.svelte` and `apps/frontend/src/lib/api/upload.ts`

---

## Phase N: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T046 [P] Verify `svelte-check --fail-on-warnings` passes locally across the entire `apps/frontend` workspace
- [ ] T047 Verify zero `console.log` instances exist via strict ESLint run
- [ ] T048 Set up CI pipeline configuration for Lefthook / GitHub Actions to run `svelte-check`, `vitest`, and `eslint` in `.github/workflows/frontend.yml` and `lefthook.yml`
- [ ] T048b Configure SvelteKit `@sveltejs/adapter-static` to output static assets to be served by the Rust Axum backend for production VPS deployment
- [ ] T048c Integrate the frontend container into the project's VPS deployment configuration (e.g., `docker-compose.yml`)
- [ ] T049 Run `quickstart.md` validation scenarios end-to-end

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### Parallel Opportunities

- All tasks marked [P] can run in parallel
- Once Foundational phase completes, User Stories 1, 2, 3, and 4 can start in parallel (if team capacity allows)
- Unit tests and TypeScript models for each story can be written in parallel before UI implementation
