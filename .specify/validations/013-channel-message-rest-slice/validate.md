# Validation: 013 — Channel & Message REST Slice (80/20)

**Branch:** `feat/013-channel-message-rest-slice`  
**Base:** `dev@8028aa28e` (post #215)  
**Date:** 2026-11-16

## Scope (delivered)
- **Backend (`flora-api/src/routes/channels.rs`)**: `GET /api/v1/channels?workspace_id=...` with org/membership/workspace-ownership checks; `POST` message persists `thread_id`; `PATCH/DELETE` reject `(channel_id,msg_id)` path mismatch (C19 hardening). Router exposes `GET /` and `POST /`.
- **Core model fix (`flora-core/src/models/channel.rs`)**: `ChannelType` sqlx mapping aligned to VARCHAR storage (`PascalCase`, `type_name="varchar"`) to match schema/check constraint.
- **Frontend**: `apps/frontend/src/lib/api/mappers.ts` (snake_case channel/message DTO mapping), `MessageStore.setMessages/clear`, REST chat in `ChatLayout`/`ThreadReply` (real `channelId`, fetch/post, `thread_id` replies), channels page with workspace→channel list/create/select, layout recents neutralized for slice safety.
- **Tests/CI**: DB-gated messaging contract tests in `flora-tests/src/contract/messaging.rs` (`db_channel_and_message_rest_slice`, `db_message_path_mismatch_rejected`, `db_cross_org_channel_access_forbidden`); `.github/workflows/rust.yml` runs `contract::messaging::db_` in `db-tests`.
- **Research/SDD**: 013 R1 assessment + WS authority and Seed/VPS audits; spec/plan/tasks/checklist scaffolded.

## Deterministic verification (local)
- `cargo fmt --all -- --check` (via fmt applied) — pass
- `cargo check -p flora-api -p flora-core -p flora-tests` — pass
- `cargo clippy -p flora-api -p flora-core -p flora-tests --all-targets -- -D warnings` — pass
- `cargo test -p flora-core --lib` — **8 passed**
- `cargo test -p flora-tests --lib` — **51 passed, 15 ignored**
- `FLORA_DATABASE__POSTGRES_URL=postgresql://postgres:postgres@127.0.0.1:15434/flora cargo test -p flora-tests contract::messaging::db_ -- --ignored --test-threads=1` — **3 passed** (Postgres 18 disposable, alt port; host 5432 occupied externally)
- Frontend: `bun run check` — 0 errors/warnings; `bun run lint` — pass; `bun run test:unit -- --run --project server` — **8 files, 19 tests passed**; `bun run build` — pass

## NV / limitations
- Remote CI/merge pending PR.
- DB-gated evidence executed on disposable PG18 (alt port 15434) due to external occupancy of 5432 in this environment.
- Realtime fanout/WS DB-backed session authority, `/messages/sync`, attachments/binaries, DMs UX, search/notifications wiring, OIDC/JWKS e2e, browser e2e, and `apps/web` out of scope (per decision).
- Channels created in DB tests via raw SQL insert for schema-compat determinism; API create path relies on repository/model mapping fixed in this cycle (validate in CI/integration env).
