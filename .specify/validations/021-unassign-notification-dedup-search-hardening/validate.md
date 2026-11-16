# Validation: 021 — Task Unassign Contract, Assignment Dedup, PG Search Hardening

**Date:** 2026-11-16  
**Branch:** `feat/021-unassign-notification-dedup-search-hardening` (from `dev@0dcde353a`)  
**Scope:** S1 per `.specify/assessments/021-next-vertical-slice/decision.md` / `specs/021-unassign-notification-dedup-search-hardening/spec.md`

## Evidence (local)

### Automated checks (pass)
- `cargo fmt --all -- --check` ✅
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings` ✅
- `cargo test -p flora-core --lib` ✅ (8 passed)
- `cargo test -p flora-api` ✅ (11 passed, 1 ignored)
- `cargo test -p flora-tests --lib` ✅ (44 passed, 40 ignored; includes new 021 DB-gated contracts, env-gated)

### DB-gated contract tests (env-gated; CI `db-tests` expected)
- `flora-tests/src/contract/notifications.rs`
  - `db_task_unassign_no_notification_and_unread_dedup` (021): tri-state PATCH semantics (`null` clears/unassigns, omitted `assignee_id` preserves), no `Assignment` notification on unassign, unread dedup guard suppresses duplicate `Assignment` for same `(user_id,target_id)` when re-assigned while unread, and no-op/omit-preserve does not duplicate.
  - Existing 019/020 assignment/reply cases retained.
- `flora-tests/src/contract/search.rs`
  - `db_search_pg_fallback_type_normalization_and_limit_bounds` (021): mixed-case/plural/CSV type tokens normalized, unknown-only tokens return empty, explicit high `limit` remains server-capped/bounded on PG fallback path.
  - Existing 019/020 PG fallback cases retained (tenant/soft-delete isolation, multi-entity behavior).

DB-gated tests require `FLORA_DATABASE__POSTGRES_URL` or `DATABASE_URL` and run migrations on disposable PostgreSQL; not executed locally in this environment (env not set).

## Implementation notes (validated by code/tests wiring)
- `flora-api/src/routes/tasks.rs`: `UpdateTaskRequest.assignee_id` is tri-state (`Option<Option<Uuid>>`, serde default) — omit => preserve, `null` => clear, value => set. Effective-transition notification fires only for `Some(new)` where `new != old` and `new != actor`; unassign/no-op/self do not notify. Assignment creation uses best-effort unread dedup guard `(user_id, event_type=Assignment, target_id)`.
- `flora-core/src/traits/repository.rs`, `flora-core/src/repositories/notification_repository.rs`: added `has_unread_for_target` (query-before-insert, unread-only, race-prone by design/NV).
- `flora-api/src/routes/search.rs`: PG fallback type-token parsing ignores empty CSV tokens (contract-compatible hardening); Meilisearch precedence/path unchanged. Explicit decision recorded: Meilisearch task/file mutation-indexing parity and Meilisearch-gated deterministic tests deferred in 021 (PG fallback authoritative for guaranteed Seed multi-entity parity when Meilisearch unavailable).
- `apps/frontend`: no changes (canonical frontend has no task unassign/search/notification consumers; backend contract evolution is FE-adoptable later).

## NV / residual
- Dedup guard is query-before-insert, unread-only, and race-prone (no uniqueness constraint/tx-coupling/outbox/idempotency key).
- Meilisearch task/file indexing hooks/parity, reindex/backfill, and Meilisearch-gated deterministic tests deferred; cross-engine ranking/snippet parity and 2vCPU/4GB perf/multilingual relevance NV.
- No unassign-specific notification type; assignee membership/policy semantics unchanged; description clear semantics unchanged.
- Search ACL/relevance/i18n/`pg_trgm`, realtime/push/mentions/completion delivery, frontend adoption/e2e, and broader ops/load/restore validation deferred.
