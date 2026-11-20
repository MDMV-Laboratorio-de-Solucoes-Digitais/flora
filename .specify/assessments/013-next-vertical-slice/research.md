# Research: 013 — workspace→channel→message (REST-first)

## Evidence sources
- `OBJETIVO.md` (P0: canais/mensagens persistidas e realtime mínimo confiável), `ROADMAP.md` (P0.5 fatia vertical e2e mínima).
- `docs/research/2026-11-15-012-frontend-api-contract-audit.md` (C4–C6,C11–C13,C18–C20,C22,C25; §5.3–5.4) and `docs/research/2026-11-16-012-session-authority-international.md` (session authority context).
- `.specify/assessments/012-session-authority-revalidation/decision.md` (HTTP done; WS/follow-ups out of scope).
- Code: `flora-api/src/routes/channels.rs`, `workspace.rs`, `tasks.rs`, `websocket.rs`; `apps/frontend/src/lib/components/chat/ChatLayout.svelte`, `.../thread/ThreadReply.svelte`, `lib/api/websocket.ts`, `lib/types/models.ts`, `lib/state/MessageStore.svelte`, `routes/(app)/channels/+page.svelte`, `routes/(app)/dms/+page.svelte`, `(app)/+layout.svelte`.

## Current state (verified)
- **Backend channels/messages:** `POST /api/v1/channels` (workspace-scoped, membership+workspace-org checked), `GET/POST /api/v1/channels/{id}/messages` (org-checked, persists via `Message::new`), `PATCH/DELETE /api/v1/channels/{id}/messages/{msg_id}` (sender+org checked). **No** `GET /api/v1/channels` list endpoint. `POST /messages` ignores `thread_id` from request (`channels.rs` drops it; `CreateMessageRequest` lacks thread handling). Edit/delete ignore `channel_id` path segment (C19). `before` pagination accepted but unused (C22).
- **DTO drift:** backend returns `sender_id, channel_id, thread_id, created_at, updated_at, is_edited` (snake_case); FE `Message` expects `authorId, channelId, threadId, createdAt, updatedAt, fileAttachments` (`models.ts`), and UI reads `message.authorId.at(0)` (`ChatLayout.svelte`). `Message` model has no attachments relation. Reply sends `threadId` (camelCase) ignored by serde.
- **Frontend wiring:** Chat mounted with hardcoded non-UUID ids (`default-channel`, `dm-channel-1`) causing `Path<Uuid>` 400; channels/DMs pages and sidebar are static placeholders; no channel selection by workspace.
- **Websocket:** FE connects `/api/v1/ws?token=`, BE serves `/ws` with `Authorization` header only and JWT-only decode (no DB `jti` check). No broadcast registry; sync endpoint missing. Out of scope for this slice except noting NV.
- **Tenancy gaps (relevant):** `create_task`/`upload_file` don't verify `workspace_id` belongs to org (C18); channel edit/delete ignore path channel (C19). `list_tasks` is assignee-scoped and pre-filters before org (C11) — adjacent but not this slice.

## Feasibility notes
- Channel/message repositories and DB tables already exist; adding list-by-workspace and threading persistence is small, localized to `channels.rs` + repo/trait if needed.
- FE can introduce a minimal `toMessage`/`toChannel` mapper + workspace-scoped channel view without redesigning the whole app.
- Deterministic verification: Rust unit/serialization tests for DTOs + DB-gated contract/integration tests (extend `db-tests`) for create/list/post/read and tenancy; FE Vitest for mappers/client behavior. Avoids OIDC e2e.
