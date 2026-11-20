# 014 — WebSocket/realtime codebase audit (read-only)

**Date:** 2026-11-16  
**Scope:** implementation-readiness for cycle 014 (A) DB-backed WS session auth at handshake and (B) minimal reliable delivery of persisted channel messages.  
**Source:** background audit subagent `ses_e1dfec4efffehUXMHP3CfVgNFP` (read-only; file persisted by orchestrator).

## Summary

- WS handshake authenticates via JWT decode only (no DB session authority), unlike HTTP extractors.
- `/ws` is registered but protocol/path/auth expectations diverge from frontend client.
- Message send/persist/publish path exists partially; there is no server-side subscription registry/fanout loop and no Valkey subscriber/WS bridge in API state.
- REST channel/message DTOs and paths differ from WS event shapes/names; frontend realtime handling is incomplete and chat currently relies on REST.
- No `/api/v1/messages/sync` endpoint exists despite client usage.

## Evidence and gaps

### Backend WS route/auth/registration
- `flora-api/src/routes/websocket.rs:100-141` — upgrades via `extract_org_context` (Bearer JWT decode only, `jwt_secret`, no `session_service.validate_session` / active / expiry / user-org checks unlike `flora-api/src/extractors/auth.rs:33-115`).
- Route registration: `GET /ws` (`flora-api/src/routes/mod.rs:47`).

### Protocol, broadcast, persistence
- Handles `Subscribe` / `Unsubscribe` / `SendMessage` / `Ping` (`flora-api/src/routes/websocket.rs:23-94,160-327`) but no subscription registry and no fanout loop.
- `SendMessage` validates channel org (`174-184,205-219`), persists via `PgMessageRepository` (`224-236`), publishes `flora_messaging::Message` to Valkey if configured (`239-250`), and only acks sender with `MessageSent` (`252-256`).
- `flora-messaging/src/lib.rs:44-123` is publish-only; no subscriber/WS bridge surfaced in API state (`flora-api/src/state/app_state.rs:49,144-166`).

### REST/DTO mismatch
- HTTP messages `POST/GET/PATCH/DELETE /api/v1/channels/{channel_id}/messages` return `MessageResponse {id,channel_id,sender_id,content,thread_id,is_edited,created_at,updated_at}` (`flora-api/src/routes/channels.rs:89-121,140-143,243-519`) and do **not** publish realtime events.
- WS `NewMessage` uses a different shape/names and is never emitted.
- Frontend expects `event.id`/`type` (`apps/frontend/src/lib/api/websocket.ts:89-97`), maps REST via `apps/frontend/src/lib/api/mappers.ts:31-44` (`authorId`, no `isEdited`), and `MessageStore.addMessage` dedupes by `id` (`apps/frontend/src/lib/state/MessageStore.svelte.ts:14-17`).

### Frontend consumers
- Root layout connects/disconnects globally (`apps/frontend/src/routes/+layout.svelte:9-14`); app layout does not use WS (`apps/frontend/src/routes/(app)/+layout.svelte`).
- Client sends token as `?token=` to `/api/v1/ws` (`apps/frontend/src/lib/api/websocket.ts:23-30`), has no subscribe/send handling and no channel awareness.
- Chat uses REST only (`apps/frontend/src/lib/components/chat/ChatLayout.svelte:33-76,165,191-205`).
- Sync calls missing `GET /api/v1/messages/sync` (`apps/frontend/src/lib/api/websocket.ts:78-83`).

## Smallest change sets (014 candidates)

### (A) DB-backed handshake auth
- Refactor `flora-api/src/routes/websocket.rs` to accept token from header and/or `?token=`.
- Reuse `AppState.session_service` + JWT decode path equivalent to `extractors/auth.rs` (or shared helper) requiring active/non-expired/user-org-matching session row.
- Reject nil-org if policy requires concrete org context; return appropriate close/error semantics.

### (B) Minimal reliable persisted-channel delivery
- On REST create/edit/delete (`flora-api/src/routes/channels.rs` message handlers), publish canonical events via `state.messaging_service` (include org/channel/message id + DTO fields/timestamps).
- Add Valkey subscribe/dispatch task (per-process or shared) that tracks per-socket subscribed channel IDs and forwards only matching org/channel events as WS `NewMessage` frames.
- Update frontend `WsClient` to subscribe/unsubscribe per channel, handle `NewMessage`/`MessageSent` and map to store, and align connect auth with server.

## Risks / NV
- No WS fanout implementation and no `/messages/sync` route.
- Query-param token auth mismatch and event schema drift.
- `ChatLayout` appends on REST success, so WS echo/dedupe semantics must be defined.
- Multi-instance fanout/guaranteed delivery semantics out of scope for Seed single-node 80/20 unless explicitly chosen.
