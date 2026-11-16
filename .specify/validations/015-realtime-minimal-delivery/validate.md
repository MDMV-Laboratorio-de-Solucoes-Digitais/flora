# Validation: 015 — Minimal Realtime Delivery & Catch-up

**Date:** 2026-11-16  
**Branch:** `feat/015-realtime-minimal-delivery`

## Deterministic checks (local)
- `cargo fmt --all -- --check` — PASS
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings` — PASS
- `cargo test -p flora-core --lib` — PASS
- `cargo test -p flora-api --lib` — PASS
- `cargo test -p flora-tests --lib` — PASS (DB-gated ignored by default)
- Frontend (`apps/frontend`): `bun run check` — PASS, `bun run lint` — PASS, `bun run test:unit -- --run --project server` — PASS (22 tests), `bun run build` — PASS

## DB-gated (CI)
- `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1`
- `cargo test -p flora-tests contract::messaging::db_ -- --ignored --test-threads=1` (includes `db_message_catch_up_after_cursor`)
- `cargo test -p flora-tests contract::websocket::db_ -- --ignored --test-threads=1`

## Implemented (015-A)
- Cursor catch-up: `GET /channels/{id}/messages?after=<uuid>` returns newer messages in ascending `(created_at,id)` order; repo trait/impl `find_by_channel_id_after`.
- In-process realtime hub (`flora-api/src/realtime.rs`) in `AppState`; canonical `NewMessage|MessageUpdated|MessageDeleted` events emitted after committed REST post/edit/delete and WS send.
- WS subscriptions + authorized same-org/channel fanout delivery, lag-tolerant receive loop; handshake auth/tenant guards preserved.
- Frontend: aligned envelopes, idempotent store upsert/remove, channel subscribe/catch-up/reconnect flow, chat load uses chronological order and syncs last-known cursor.

## NV / not exercised
- Local DB-gated execution may be blocked by host port/env conflicts (CI provides Postgres 18 service).
- Browser/multi-client realtime e2e, load/concurrency on 2vCPU/4GB, default-port smoke, post-handshake WS revocation window.
- Valkey/LISTEN/NOTIFY/multi-node delivery, binary files (015-B), Seed/VPS operability refactors (015-C).
