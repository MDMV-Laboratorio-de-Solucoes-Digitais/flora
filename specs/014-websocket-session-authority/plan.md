# Plan: 014 — WebSocket Session Authority & Protocol Alignment

## Backend
1. `flora-api/src/extractors/auth.rs`: extract `verify_bearer_token` (or equivalent) returning verified session context; reuse in `authenticate_bearer`/HTTP extractors without behavior change.
2. `flora-api/src/routes/websocket.rs`: token resolution (header/query), DB-backed verification pre-upgrade, generic failures, nil-org rejection, preserve org checks on actions; minimal protocol helpers as needed.
3. `flora-api/src/routes/mod.rs`: expose `/api/v1/ws` (and keep `/ws`).

## Frontend
1. `apps/frontend/src/lib/api/websocket.ts`: align URL/path/transport usage, parse tagged envelopes safely, remove `/messages/sync` dependency, classify close/auth failures (no infinite retry on auth), bounded backoff+jitter, reconnect hooks.
2. `apps/frontend/src/routes/(app)/+layout.svelte` (and auth integration): connect/disconnect on auth/org/token lifecycle appropriately.

## Tests/CI
1. Unit tests: extractor/ws helpers and frontend WS client behavior.
2. DB-gated contract tests in `flora-tests` for WS handshake/session matrix and CI `db-tests` filter extension.
3. Full deterministic gates; NV documented.
