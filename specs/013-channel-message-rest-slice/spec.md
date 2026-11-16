# Feature Specification: 013 — Channel & Message REST Slice (80/20)

**Feature Branch**: `feat/013-channel-message-rest-slice`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/013-next-vertical-slice/{intake,research,problem,concept,decision}.md` (verdict: go)

## Goal
Deliver a tenant-safe, Postgres-backed workspace→channel→message slice usable from the canonical frontend via REST (no realtime dependency): list/create channels by workspace, post/list messages with thread persistence, and align frontend DTOs/IDs with backend contracts.

## User Scenarios & Testing
- **US1 (P0):** member selects a workspace, sees/creates channels, posts a message, and reads persisted messages in that channel.
- **US2 (P0):** reply posted with `thread_id` is persisted and rendered as a thread reply.
- **US3 (P1):** cross-org/mismatched workspace or channel/message path access is rejected.
- **US4 (P2):** deterministic checks (Rust fmt/check/clippy/tests, DB-gated messaging contract tests, frontend check/lint/test:unit/build) pass.

## Requirements
- **FR-001:** `GET /api/v1/channels?workspace_id=` lists workspace channels with org/membership/workspace ownership checks. **To do**
- **FR-002:** `POST /api/v1/channels/{id}/messages` persists `thread_id`; edit/delete reject mismatched `(channel_id,msg_id)` path. **To do**
- **FR-003:** Frontend maps snake_case message/channel DTOs to UI model and uses real channel/workspace IDs (no hardcoded demo UUIDs in the slice). **To do**
- **FR-004:** Channels page provides minimal workspace→channel selection/creation feeding chat; reply uses `thread_id`. **To do**
- **FR-005:** DB-gated messaging contract tests cover list/create/post/list/thread/tenancy/path mismatch. **To do**
- **FR-006:** SDD artifacts + verification evidence (NV where applicable). **To do**

## Non-goals
- WebSocket fanout/subscription/DB auth, `/messages/sync`, attachments/binary files, DMs UX, channel roles UI, search/notifications wiring, task listing semantics (C11), OIDC e2e, `apps/web`.
