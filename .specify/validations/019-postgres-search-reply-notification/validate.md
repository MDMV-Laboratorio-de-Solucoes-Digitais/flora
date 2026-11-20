# Validation: 019 — Postgres FTS Search Fallback + Reply Notification

## Scope
- Slice S1 (80/20): PG FTS fallback for message search (no Meilisearch required) + best-effort in-app `Reply` notification on threaded replies, with DB-gated contracts.

## Local verification (2026-11-16)
- `cargo fmt --all -- --check` — **pass**
- `cargo clippy -p flora-api -p flora-tests --all-targets -- -D warnings` — **pass**
- `cargo test -p flora-core --lib` — **pass** (8 tests)
- `cargo test -p flora-api` — **pass** (11 passed, 1 ignored)
- `cargo test -p flora-tests --lib` — **pass** (44 passed, 35 ignored)

## DB-gated contracts (added, env-gated)
- `flora-tests/src/contract/search.rs::db_search`: PG fallback returns matching message, respects empty-query and type-filter behavior, enforces tenant isolation and excludes deleted messages (`#[ignore]`, requires `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`).
- `flora-tests/src/contract/notifications.rs::db_notifications`: threaded reply creates `Reply` notification for parent author (visible via notifications API), actor not notified, and self-reply creates no `Reply` notification (`#[ignore]`, requires DB).

**Local DB execution:** NV in this session (no disposable PG URL configured). Expected to run in CI db-tests / environments with disposable PostgreSQL 18.

## NV / not exercised
- Relevance ranking quality, CJK/multilingual tokenization (`simple` config), channel-level ACL filtering in search, and 2vCPU/4GB perf/load behavior.
- Meilisearch-backed path behavior unchanged but not revalidated end-to-end here.
- Notification idempotency/dedup under retries, tx-coupling guarantees, and realtime/push delivery (REST/API visibility only).
- Tasks/files search, `pg_trgm`, outbox/workers, browser e2e/multi-client.
