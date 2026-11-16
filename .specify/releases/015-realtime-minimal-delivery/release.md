# Release: 015 — Minimal Realtime Delivery & Catch-up

**Target:** `dev` (PR to merge)

## Summary
Implements cycle 015 slice A: Postgres-authoritative minimal realtime delivery and deterministic catch-up for persisted channel messages on single-node Seed profile.

- Cursor-correct catch-up (`after` keyset, ascending) and canonical mutation events after committed REST/WS mutations.
- In-process WS hub/subscription registry with bounded best-effort fanout and org/channel isolation (no new mandatory infra).
- Frontend idempotent upsert/subscribe/reconnect/catch-up alignment for coherent REST+realtime flow.

## Key changes
- `flora-core`: message repository trait/impl `find_by_channel_id_after` for `(created_at,id)` catch-up.
- `flora-api`: `realtime` hub module, `AppState.realtime_hub`, channels routes emit events + `after` query, websocket subscription/fanout and canonical envelopes.
- `apps/frontend`: websocket client/store/mappers/chat updated for envelope alignment, idempotent upsert/remove, channel subscription and catch-up sync.
- `flora-tests`: DB-gated `db_message_catch_up_after_cursor`; AppState test initializers updated for new hub field.
- SDD/spec artifacts for 015.

## Verification
- See `.specify/validations/015-realtime-minimal-delivery/validate.md`.
- CI must be green (Rust build/test, db-tests incl. auth/messaging/websocket, frontend CI as applicable).

## Risks / NV
- Single-node best-effort push; lag/drop recovery depends on REST catch-up. No post-handshake WS revalidation.
- No browser/multi-client e2e or 2vCPU/4GB load evidence. Valkey/LISTEN/NOTIFY deferred.
