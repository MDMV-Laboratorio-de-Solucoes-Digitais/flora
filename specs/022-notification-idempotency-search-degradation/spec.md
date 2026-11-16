# Feature Specification: 022 — Notification Idempotency, Search Degradation, CI DB-Gated Enforcement

**Feature Branch**: `feat/022-notification-idempotency-search-degradation`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/022-next-vertical-slice/decision.md` (verdict: go, slice S1)

## Goal
Harden notification correctness and search resilience for Seed/Postgres-first by enforcing deterministic unread-notification dedup at the database level, making Meilisearch query failures fail-open to the Postgres FTS correctness baseline, and extending CI `db-tests` to cover search/notifications/files DB-gated contracts.

## Scope

### Notification idempotency/dedup (DB-enforced)
- Additive migration: create partial unique index on `notifications (user_id, event_type, target_id) WHERE is_read = false`.
- `NotificationRepository::create` becomes conflict-safe/idempotent for that arbiter:
  - use `INSERT ... ON CONFLICT (user_id, event_type, target_id) WHERE is_read = false DO NOTHING RETURNING ...`
  - return `Result<Option<Notification>>` (`Some` inserted, `None` suppressed as duplicate unread signal; not an error)
- Update trait/impl and call sites to treat suppression as success (best-effort producers continue to warn only on real errors).
- Preserve existing transition semantics and delivery posture:
  - `Assignment` emitted only on effective assign/reassign to non-actor (create/update tri-state PATCH)
  - no notification on unassign/self/no-op (U0 unchanged)
  - best-effort, post-persist (no tx-coupling/outbox)
- Unread-only dedup scope: a new notification is allowed after the prior one is read or when target/type/user differs.

### Search degradation/contract
- Preserve precedence: when `state.search_service` is configured, attempt Meilisearch search first.
- On Meilisearch query error, log warning and fall back to `search_pg` for the same request (fail-open to Postgres FTS correctness baseline).
- Keep PG fallback deterministic and tenant/soft-delete safe with contract-compatible behavior.
- Explicit deferred NV/contract: mutation-indexing hooks, async task completion guarantees, freshness/parity, and rebuild/backfill tooling for Meilisearch are not implemented in 022.

### CI / deterministic verification
- Extend `.github/workflows/rust.yml` `db-tests` job to run:
  - `cargo test -p flora-tests contract::search::db_ -- --ignored --test-threads=1`
  - `cargo test -p flora-tests contract::notifications::db_ -- --ignored --test-threads=1`
  - `cargo test -p flora-tests contract::files::db_ -- --ignored --test-threads=1`
  (in addition to existing suites, on disposable PostgreSQL 18).
- Add/update DB-gated contracts to verify conflict-safe unread dedup suppression and preserve existing invariants (env-gated).

### Non-goals
- Meilisearch mutation-indexing hooks, delete-on-soft-delete propagation, async task waiting, rebuild/backfill tooling, Meilisearch-gated deterministic CI, cross-engine ranking/snippet parity.
- Outbox/tx-coupling, coalescing upsert/read-policy changes, unassign-specific notification events/recipients, mentions/completion/realtime/push delivery.
- `pg_trgm`/i18n/CJK relevance, channel/workspace-level ACL in search.
- Frontend changes, 2vCPU/4GB load/OOM/restore benchmarks, browser e2e/multi-client, new mandatory services.

## NV
- Meilisearch freshness/staleness when queries succeed but index is incomplete (hooks/parity deferred).
- Delivery is still best-effort/post-persist; suppression is unread-only and does not model transition epochs beyond same unread target/type.
- Broader ops/perf/ACL/relevance and frontend adoption remain out of scope.
