# Concept: 013 — REST-first workspace→channel→message vertical

## Chosen approach (80/20)
Deliver a **persisted, readable, tenant-safe messaging slice over REST** aligned to the existing backend models, deferring realtime fanout/attachments.

### 1) Backend (minimal API/contract fixes)
- Add `GET /api/v1/channels?workspace_id=<uuid>`: auth+org+membership required; verify workspace belongs to org; return channels for workspace ordered by `created_at` (or stable order). Keep response `ChannelResponse` snake_case.
- `POST /api/v1/channels/{id}/messages`: accept and persist `thread_id` from request (validate UUID/thread belongs to same channel if cheap; else persist as provided with FK/null semantics per schema — verify schema before implementing). Reject empty content (existing).
- `PATCH/DELETE /api/v1/channels/{channel_id}/messages/{msg_id}`: enforce `msg.channel_id == channel_id` (fix C19 path bypass) in addition to sender/org checks.
- `GET /api/v1/channels/{channel_id}/messages`: keep stable shape; optionally honor `before` later — NV/defer if non-trivial.
- Do **not** add `deny_unknown_fields` globally in 013 (risk to other routes) except consider for new/changed DTOs if isolated.

### 2) Frontend (minimal usable flow)
- Introduce explicit mappers (e.g. `lib/api/mappers.ts` or colocated): `toChannel(ChannelResponse)`, `toMessage(MessageResponse)` mapping `sender_id→authorId`, `thread_id→threadId`, `channel_id→channelId`, timestamps, default `fileAttachments: []`.
- Replace hardcoded chat/channel wiring: in app context, resolve workspace → fetch channels for workspace → allow create/select channel → load messages via `GET /channels/{id}/messages` and post via REST using backend field names (`content`, `thread_id` when replying) while UI uses mapped model.
- Keep `ThreadReply`/message rendering working off mapped fields (no `undefined.at(0)`); hide/disable attachment upload affordance or gate it (no binary path) to avoid broken promises.
- Do not depend on WS for correctness; WS connect/fanout remains NV.

### 3) Tenancy & safety (in-scope only)
- Preserve membership+org checks on new/changed handlers; fix C19. Cross-tenant `workspace_id` on task/file (C18) and task listing semantics (C11) are related but **defer** unless touched incidentally — record as follow-up.

## Alternatives considered
- **Fix WS first (C):** does not unblock persistence/read UX; higher protocol complexity.
- **Files first (B):** storage/binary work exceeds 80/20 for communication MVP.
- **Full realtime chat:** requires subscription registry/fanout/sync; too large for one safe slice.

## Debloat / VPS notes
- Postgres-only, no Valkey/Meilisearch dependency added; works with existing stack. Poll/refresh-on-action acceptable for Seed instead of realtime fanout.
