# Concept: 020

Deliver a minimal, Postgres-first vertical extension of 019:

1. **Search:** additive migrations add generated `search_tsv` + partial GIN for `tasks` and `files` metadata; PG fallback search becomes multi-entity (`message|task|file`) with the same tenant-safe contract, safe tsquery parsing, and bounded results.
2. **Notification:** task create/update emits a best-effort `Assignment` notification only on effective assignee transition to a different, non-actor user, persisted via existing `PgNotificationRepository` and visible via existing notifications API.
3. **Evidence:** DB-gated contract tests on disposable PostgreSQL 18 validate matching, type filters, tenant/soft-delete isolation, fallback behavior, and assignment creation/guards without introducing mandatory services or frontend changes.
