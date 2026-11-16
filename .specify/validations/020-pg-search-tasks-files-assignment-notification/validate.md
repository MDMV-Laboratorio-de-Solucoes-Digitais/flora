# Validation: 020 — PG FTS Tasks/Files + Task Assignment Notification

**Date:** 2026-11-16  
**Branch:** `feat/020-pg-search-tasks-files-assignment-notification` (from `dev@dde602b3b`)  
**Scope:** S1 per `.specify/assessments/020-next-vertical-slice/decision.md` / `specs/020-pg-search-tasks-files-assignment-notification/spec.md`

## Evidence (local)

### Automated checks (pass)
- `cargo fmt --all -- --check` ✅
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings` ✅
- `cargo test -p flora-core --lib` ✅ (8 passed)
- `cargo test -p flora-api` ✅ (11 passed, 1 ignored)
- `cargo test -p flora-tests --lib` ✅ (44 passed, 38 ignored; includes new 020 DB-gated contracts compiled/env-gated)

### DB-gated contract tests (env-gated; CI db-tests expected)
- `flora-tests/src/contract/search.rs`
  - `db_search_pg_fallback_returns_tasks_and_files` (020): task/file hits by `type`, and merged default query includes both entities (`search_service: None`, PG fallback path).
  - `db_search_pg_fallback_tasks_files_tenant_and_deleted_isolation` (020): org isolation and `is_deleted=false` enforcement for tasks/files.
  - Existing 019 message cases retained (match, empty/type behavior, tenant/deleted isolation).
- `flora-tests/src/contract/notifications.rs`
  - `db_task_assignment_creates_notification_and_guards` (020): create-with-assignee notifies assignee (not actor), no-op update does not duplicate, reassignment notifies new assignee, self-assignment does not notify; visibility via `GET /api/v1/notifications`.
  - Existing 019 reply-notification cases retained.

DB-gated tests require `FLORA_DATABASE__POSTGRES_URL` or `DATABASE_URL` and run migrations on disposable PostgreSQL; not executed locally in this environment (env not set).

## Implementation notes (validated by code/tests wiring)
- Additive migration `20261118000001_add_tasks_files_search_fts.sql`: generated stored `search_tsv` (`simple`) + partial GIN (`WHERE is_deleted = false`) for `tasks(title,description)` and `files(name,file_type)`.
- `flora-api/src/routes/search.rs`: PG fallback extended to multi-entity `message|task|file` with safe tsquery parsing, org + soft-delete filters, bounded merge/limit, preserved contract and Meilisearch path.
- `flora-api/src/routes/tasks.rs`: best-effort `Assignment` notification on effective assignee transition (`Some(new)` provided, differs from existing, not actor) for create/update; mutation does not fail on notification error.
- `apps/frontend`: no changes (contract-compatible; search/notification UI integration remains NV).

## NV / residual
- 2vCPU/4GB runtime perf/load/OOM/index maintenance and restore drills unvalidated.
- Relevance/i18n/CJK quality with `'simple'`; `pg_trgm` and channel/workspace-level ACL in search deferred.
- Meilisearch indexing/query parity for tasks/files not implemented/validated (PG fallback path covered).
- Task `PATCH` cannot explicitly clear assignee (`req.assignee_id.or(existing.assignee_id)`), so unassign-driven notification semantics not expressible; notification idempotency/tx-coupling/race hardening and assignee membership policy unchanged.
- Outbox/workers/push/realtime delivery, mentions/completion, browser e2e/multi-client deferred.
