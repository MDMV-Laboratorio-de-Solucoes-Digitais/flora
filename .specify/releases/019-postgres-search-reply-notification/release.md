# Release: 019 — Postgres FTS Search Fallback + Reply Notification

## Summary
- Implements 019 slice S1 per `.specify/assessments/019-next-vertical-slice/decision.md`.
- **Search:** additive migration adds `messages.search_tsv` (generated, `simple`) + partial GIN index; `/api/v1/search` now falls back to Postgres FTS for messages when Meilisearch is unavailable/disabled, preserving contract, tenant/membership checks, and empty-query behavior (no 503 solely due to missing Meilisearch).
- **Notifications:** threaded message replies emit a best-effort `Reply` notification to the parent message author when the parent is valid (same org/channel, not deleted, not self), persisted via existing core repository and visible through existing notifications API.
- **Tests:** DB-gated contract tests added for PG search fallback (match/isolation/deleted/type/empty) and reply notification creation/visibility/guards.

## Verification
- `cargo fmt --all -- --check` ✅
- `cargo clippy -p flora-api -p flora-tests --all-targets -- -D warnings` ✅
- `cargo test -p flora-core --lib` ✅ (8)
- `cargo test -p flora-api` ✅ (11 passed, 1 ignored)
- `cargo test -p flora-tests --lib` ✅ (44 passed, 35 ignored)
- DB-gated 019 contracts: added/compile; execution in CI db-tests/disposable PG (NV locally without DB URL)
- CI to confirm full gates before merge.

## NV / Follow-ups
- Tasks/files search, `pg_trgm`, relevance/i18n/CJK tuning, channel-level ACL in search, backfill/outbox/workers/push/realtime delivery.
- Notification dedup/idempotency/tx-coupling hardening and additional events (mentions/assignment/completion).
- 2vCPU/4GB perf/load/proxy/restore validation, browser e2e/multi-client.
- 020 candidate: extend search to tasks/files metadata or add next high-value notification/event wiring with deterministic DB-gated coverage, per updated planning.
