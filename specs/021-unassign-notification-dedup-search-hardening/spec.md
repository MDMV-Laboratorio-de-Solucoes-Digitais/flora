# Feature Specification: 021 — Task Unassign Contract, Assignment Dedup, PG Search Hardening

**Feature Branch**: `feat/021-unassign-notification-dedup-search-hardening`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/021-next-vertical-slice/decision.md` (verdict: go, slice S1)

## Goal
Harden task assignment semantics and Postgres-first search fallback for Seed/VPS by making task `assignee_id` PATCH updates explicit (tri-state, including unassign), emitting `Assignment` notifications only on effective assign/reassign transitions with a simple best-effort unread dedup guard, and locking deterministic PG fallback search edge behavior with DB-gated contracts. Record an explicit Meilisearch task/file parity/precedence decision (deferred indexing wiring).

## Scope

### Task assignment contract (API)
- `PATCH /api/v1/tasks/{id}` (`UpdateTaskRequest.assignee_id`) supports tri-state merge-patch semantics:
  - field omitted => no change to `assignee_id`
  - `assignee_id: null` => explicit clear/unassign (`assignee_id` becomes `None`)
  - `assignee_id: "<uuid>"` => set/assign to that user
- `title`, `description`, `status` preserve existing update behavior (description still uses existing merge semantics unless separately addressed; out of scope except preserving behavior).
- Create task contract unchanged (absent/value as today).

### Assignment notification semantics
- Compute effective transition `old_assignee_id -> new_assignee_id` from persisted state and request.
- Best-effort create `NotificationType::Assignment` for the new assignee only when:
  - request results in `new_assignee_id = Some(new)` (effective assignment present after update/create as applicable),
  - `Some(new) != old_assignee_id` (effective transition), and
  - `new != actor_id` (not self).
- No `Assignment` notification on:
  - unassign/clear (`Some(old) -> None`),
  - no-op (effective assignee unchanged),
  - self-assignment/self-transition target is actor.
- Simple dedup guard (best-effort, no migration): before inserting, skip creation if an unread `Assignment` notification already exists for the same `(user_id, target_id)` (query-before-insert). Race-prone under concurrency remains NV.

### Search hardening / precedence decision
- Preserve `/api/v1/search` contract and current backend precedence:
  - if `state.search_service` is configured, use Meilisearch path (unchanged);
  - if unavailable, use Postgres FTS fallback (`message|task|file`) as authoritative multi-entity parity path for Seed.
- Harden/lock PG fallback behavior with deterministic DB-gated tests (type token handling, limits/bounds as applicable, tenant and `is_deleted=false` invariants, empty/whitespace query). Keep implementation changes minimal and contract-compatible.
- Explicit decision (documented, NV): Meilisearch task/file mutation-indexing parity and Meilisearch-gated deterministic tests are deferred; Meilisearch-enabled mode may not reflect task/file freshness/completeness in 021.

### Verification
- DB-gated contracts on disposable PostgreSQL 18 (env-gated):
  - task assign/reassign/unassign via explicit `null`, self/no-op guards, notification visibility and unread dedup behavior.
  - search PG fallback edge/normalization and isolation invariants (extend existing coverage as needed).
- Local: `cargo fmt --all -- --check`, `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`, `cargo test -p flora-core --lib`, `cargo test -p flora-api`, `cargo test -p flora-tests --lib`.
- No frontend changes required (canonical frontend does not consume unassign/search/notification flows).

## Non-goals
- Meilisearch indexing hooks, backfill/reindex tooling, Meilisearch-gated tests, cross-engine ranking/snippet parity.
- `pg_trgm`, i18n/CJK relevance tuning, channel/workspace-level ACL in search.
- Uniqueness constraints/migrations for notifications, transactional outbox/tx-coupling, race-proof idempotency, unassign-specific notification types, assignee membership policy changes, mentions/completion/realtime/push delivery.
- Frontend task-edit/unassign/search/notification UI, 2vCPU/4GB load/OOM/restore benchmarks, browser e2e/multi-client.
- New mandatory services or crate convergence.

## NV
- Dedup guard is unread-only and race-prone (query-before-insert); duplicate notifications may still occur under concurrency/retries.
- Meilisearch mode task/file freshness/completeness and ranking/snippet parity unverified; 2vCPU/4GB perf and multilingual relevance NV.
- Description clear semantics and broader PATCH field tri-state behavior unchanged/out of scope except as required for compatibility.
