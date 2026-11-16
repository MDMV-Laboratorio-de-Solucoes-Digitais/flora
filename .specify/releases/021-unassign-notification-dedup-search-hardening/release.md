# Release: 021 — Task Unassign Contract, Assignment Dedup, PG Search Hardening

**Date:** 2026-11-16  
**Branch/PR:** `feat/021-unassign-notification-dedup-search-hardening` -> `dev` (PR pending CI)  
**Baseline:** `dev@0dcde353a`

## Delivered (slice S1)
- Explicit tri-state task `assignee_id` PATCH contract: omit=no change, `null`=clear/unassign, value=assign/set, with deterministic effective-transition handling.
- `Assignment` notifications emitted only on effective assign/reassign to a non-actor assignee (create/update); no notification on unassign/no-op/self. Simple best-effort unread dedup guard `(user_id, Assignment, target_id)` added to avoid obvious duplicate unread signals.
- PG fallback search hardening (contract-compatible type-token normalization) and DB-gated contracts for unassign/dedup/search edge behavior on disposable PostgreSQL 18 (env-gated). Meilisearch precedence unchanged with explicit decision: task/file indexing parity deferred (PG fallback authoritative for guaranteed Seed multi-entity parity when Meilisearch unavailable).

## Evidence
- Local: `cargo fmt --all -- --check`, `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`, `cargo test -p flora-core --lib` (8), `cargo test -p flora-api` (11 passed, 1 ignored), `cargo test -p flora-tests --lib` (44 passed, 40 ignored).
- CI: PR checks required green (including `db-tests`) before merge (to be recorded in PR).

## NV / follow-ups (022 candidates)
- Meilisearch mutation-indexing hooks/parity, reindex/backfill, Meilisearch-gated deterministic tests, cross-engine ranking/snippet equivalence.
- Strong notification idempotency/uniqueness (constraint), tx-coupling/outbox, unassign-specific events, assignee membership policy, mentions/completion, realtime/push delivery.
- Search ACL/relevance/i18n/`pg_trgm`, 2vCPU/4GB perf/load/OOM/restore validation, frontend adoption (task edit/unassign + search/notification UI) and browser e2e/multi-client.
