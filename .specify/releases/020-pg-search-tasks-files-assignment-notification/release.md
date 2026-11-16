# Release: 020 — PG FTS Tasks/Files + Task Assignment Notification

**Date:** 2026-11-16  
**Branch/PR:** `feat/020-pg-search-tasks-files-assignment-notification` -> `dev` (PR pending)  
**Baseline:** `dev@dde602b3b`

## Delivered (slice S1)
- Postgres-first multi-entity search fallback: additive `search_tsv`+GIN for tasks/files metadata and unified PG fallback in `/api/v1/search` (`message|task|file`, tenant/soft-delete safe, contract preserved, Meilisearch path unchanged).
- Deterministic in-app task assignment notification (`NotificationType::Assignment`) on effective assignee transition (create/update, best-effort, non-self/no-op guards) via existing notification primitives/API.
- DB-gated contract tests for PG fallback task/file search and assignment notification behavior (env-gated disposable PostgreSQL 18; CI db-tests expected).

## Evidence
- Local: `cargo fmt --all -- --check`, `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`, `cargo test -p flora-core --lib`, `cargo test -p flora-api`, `cargo test -p flora-tests --lib` (44 passed, 38 ignored).
- CI: PR checks required green (including `db-tests`) before merge (recorded in PR).

## NV / follow-ups (021 candidates)
- Meilisearch parity for tasks/files, relevance/i18n/CJK/`pg_trgm`, and channel/workspace ACL in search.
- Task unassign contract semantics, notification idempotency/tx-coupling/race/membership hardening, mentions/completion and delivery (realtime/push/outbox/workers).
- 2vCPU/4GB perf/load/index-maintenance and restore validation; browser e2e/multi-client; broader DB-gated edge coverage (quota/cleanup/replay).
