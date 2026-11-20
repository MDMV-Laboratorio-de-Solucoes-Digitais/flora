# Plan: 015 — Minimal Realtime Delivery & Catch-up

## Inputs
- Decision: `.specify/assessments/015-next-vertical-slice/decision.md` (go, slice A)
- Research: `docs/research/2026-11-16-015-realtime-minimal-delivery-design.md`, `docs/research/2026-11-16-014-ws-codebase-audit.md`

## Design choices (minimal/safe)
- **Source of truth:** Postgres commits; push is best-effort acceleration.
- **Catch-up cursor:** `after=<message_uuid>` query param on `GET /channels/{id}/messages` returning messages newer than the reference message in deterministic ascending timeline order for sync (`(created_at,id)` keyset against reference row). Default/no-after keeps current list semantics as compatible as practical.
- **Canonical event:** `NewMessage | MessageUpdated | MessageDeleted` with payload `{ channel_id, message: MessageResponse }` (and ids where useful) emitted after successful DB mutation.
- **Hub:** `tokio::sync::broadcast`-based in-process channel fanout (or per-channel registry) stored in `AppState`; bounded buffers, drop/lag tolerant; no Valkey dependency required.
- **WS:** on subscribe register sender for channel; on mutation events deliver to authorized subscribers; on disconnect cleanup. Keep handshake auth/tenant checks unchanged.
- **Frontend:** idempotent upsert by id (merge/replace, handle delete), subscribe active channel, catch-up via REST `after=lastKnownId`, reconnect triggers resubscribe+catch-up, align envelope parsing with backend.

## Files (expected)
- `flora-core/src/traits/repository.rs`, `flora-core/src/repositories/message_repository.rs` (cursor query support)
- `flora-api/src/routes/channels.rs` (after cursor, emit events)
- `flora-api/src/routes/websocket.rs` (subscriptions, fanout delivery, canonical envelopes)
- `flora-api/src/state/app_state.rs` (hub)
- `apps/frontend/src/lib/api/{websocket.ts,mappers.ts}`, `apps/frontend/src/lib/state/MessageStore.svelte.ts`, `apps/frontend/src/lib/components/chat/ChatLayout.svelte`
- `flora-tests/src/contract/{messaging.rs,websocket.rs}` (DB-gated 015 behaviors/regressions)
- `.specify/{validations,releases}/015-realtime-minimal-delivery/*`, `OBJETIVO.md`, `ROADMAP.md`

## Verification
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p flora-core --lib`, `cargo test -p flora-api`, `cargo test -p flora-tests --lib`
- DB-gated: `cargo test -p flora-tests contract::messaging::db_ -- --ignored --test-threads=1`, `contract::websocket::db_`
- Frontend (apps/frontend): `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build`
