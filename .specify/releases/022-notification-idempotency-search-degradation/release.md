# Release: 022 — Notification Idempotency, Search Degradation, CI DB-Gated Enforcement

**Date:** 2026-11-16  
**Branch/PR:** `feat/022-notification-idempotency-search-degradation` -> `dev` (pending)  
**Baseline:** `dev@b5be7d16f`

## Delivered (slice S1)

- **DB-enforced unread notification dedup:** additive migration
  `flora-core/src/migrations/sql/20261116000003_notifications_unread_dedup.sql` creates a partial unique
  index on `notifications (user_id, event_type, target_id) WHERE is_read = false`.
  `NotificationRepository::create` is now conflict-safe (`ON CONFLICT DO NOTHING`, returns
  `Option<Notification>`; `None` = suppressed). 021's race-prone TOCTOU precheck was removed; assignment
  and reply producers treat suppression as success. Unread-only scope and best-effort post-persist delivery
  are preserved.
- **Search fail-open degradation:** Meilisearch query errors now fall back to Postgres FTS for the same
  request (with a warning log); precedence and the PG fallback contract are otherwise unchanged. Freshness
  parity of a healthy-but-unindexed Meilisearch is explicitly deferred.
- **CI DB-gated enforcement:** `.github/workflows/rust.yml` `db-tests` now runs `contract::search::db_`,
  `contract::notifications::db_`, and `contract::files::db_` in addition to the existing suites.
- **Incidental fixes required for CI green:** tri-state `assignee_id` PATCH (explicit `null` now clears the
  assignee), files contract status expectations (400 -> 422), and search contract fixtures/URI. See
  `.specify/validations/022-notification-idempotency-search-degradation/validate.md`.
- **Artifacts:** R1 research (6 docs), assessment, spec/plan/tasks, validation, this release note, and
  `OBJETIVO.md`/`ROADMAP.md` updated for 022 -> 023. No frontend changes (canonical `apps/frontend` has no
  search/task-edit/notification consumers).

## Evidence

- Local: `cargo fmt --all -- --check`, strict clippy, unit tests, and all six DB-gated contract suites green
  on a disposable PostgreSQL 18 database (details in `validate.md`).
- CI: result recorded in the PR (expanded `db-tests` job).

## NV / follow-ups (023 candidates)

- Meilisearch mutation-indexing hooks / parity (task/file/message), delete-on-soft-delete propagation,
  async task completion handling, rebuild/backfill tooling, Meilisearch-gated deterministic tests/CI.
- Cross-engine ranking/snippet parity; `pg_trgm`/i18n/CJK relevance; hyphenated-name sub-token relevance;
  channel/workspace-level search ACL.
- Outbox/tx-coupling, coalescing/epoch idempotency, unassign-specific events/recipients,
  mentions/completion/realtime/push delivery, notification `target_url` exposure.
- 2vCPU/4GB perf/load/OOM/restore validation, frontend adoption/e2e/multi-client.
