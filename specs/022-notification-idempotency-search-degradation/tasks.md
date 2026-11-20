# Tasks: 022 — Notification Idempotency, Search Degradation, CI DB-Gated Enforcement

## Implementation
- [x] T001 Add migration `flora-core/src/migrations/sql/20261116000003_notifications_unread_dedup.sql` (partial unique index for unread notifications).
- [x] T002 Make `NotificationRepository::create` conflict-safe/idempotent (`Option<Notification>`, `ON CONFLICT DO NOTHING`), update trait/impl/callers, and preserve transition semantics.
- [x] T003 Harden search route to fall back to PG FTS on Meilisearch query errors (fail-open) with warning logs; keep precedence/contract otherwise unchanged.
- [x] T004 Extend `.github/workflows/rust.yml` `db-tests` to run `contract::search::db_`, `contract::notifications::db_`, and `contract::files::db_`.
- [x] T005 Add/update DB-gated contracts for unread dedup suppression; fix incidental CI-enabling defects (tri-state PATCH null, files status expectations, search fixtures/URI); run local verification (`fmt`, `clippy`, all DB suites).

## PR/Release
- [x] T006 Record validation/release artifacts, update `OBJETIVO.md`/`ROADMAP.md` for 022→023, commit/push, open PR to `dev`, ensure CI green, merge, sync, and clean branch.
