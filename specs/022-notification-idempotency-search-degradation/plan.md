# Plan: 022 — Notification Idempotency, Search Degradation, CI DB-Gated Enforcement

## Inputs
- Assessment: `.specify/assessments/022-next-vertical-slice/{intake,research,problem,concept,decision}.md`
- Research: `docs/research/2026-11-16-022-{meilisearch-indexing-hooks-reindex-parity-vps,meilisearch-pg-coherence-degradation-contract-vps,notification-idempotency-unassign-semantics,db-meilisearch-gated-test-harness,macro-search-notification-reference-flows,meilisearch-vps-resource-ops}.md`
- Audits: codebase (`ses_e1c0bc4e5`), frontend (`ses_e1c0bc4c8`)

## Design
1. **DB migration:** add `flora-core/src/migrations/sql/20261116000003_notifications_unread_dedup.sql` creating partial unique index `idx_notifications_unread_user_event_target_unique` on `notifications(user_id, event_type, target_id) WHERE is_read = false`.
2. **Idempotent notification create:**
   - Change `NotificationRepository::create` to `async fn create(&self, notification: Notification) -> crate::Result<Option<Notification>>` in `flora-core/src/traits/repository.rs`.
   - Implement in `PgNotificationRepository` with `INSERT ... ON CONFLICT (user_id, event_type, target_id) WHERE is_read = false DO NOTHING RETURNING ...` via `fetch_optional`.
   - Update callers (`flora-api/src/routes/tasks.rs`, `flora-api/src/routes/channels.rs` reply path; websocket has no notification create) and any in-tree usages to accept `Option` suppression as success.
   - Simplify assignment dedup path to rely on conflict-safe insert (remove redundant TOCTOU precheck or keep non-authoritative; prefer conflict-safe path as source of truth).
3. **Search fail-open degradation:** in `flora-api/src/routes/search.rs`, when `search_service` exists, attempt Meilisearch search and on `Err` log `tracing::warn!` with error/context then execute PG fallback for same request; on success return Meilisearch results as today.
4. **CI enforcement:** extend `.github/workflows/rust.yml` `db-tests` with `contract::search::db_`, `contract::notifications::db_`, `contract::files::db_` ignored test runs.
5. **Tests:** extend/update `flora-tests/src/contract/notifications.rs` for DB-enforced unread dedup/idempotent suppression (and keep transition semantics); ensure existing search/files DB-gated suites remain valid under CI.
6. **SDD/planning:** spec/plan/tasks, validation/release, update `OBJETIVO.md`/`ROADMAP.md` for 022 delivery and 023 candidate.

## Verification
- `cargo fmt --all -- --check`
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`
- `cargo test -p flora-core --lib`
- `cargo test -p flora-api`
- `cargo test -p flora-tests --lib` (DB-gated tests env-gated locally)
- CI `db-tests` green including search/notifications/files suites (disposable PostgreSQL 18)

## Risks/NVs
- Partial index/`ON CONFLICT` predicate must match exactly; verified by migration + DB-gated tests.
- Unread dedup applies across notification types for same user/type/target (intentional hardening).
- Meilisearch staleness on successful responses remains NV (hooks/rebuild deferred); only query errors fall back.
