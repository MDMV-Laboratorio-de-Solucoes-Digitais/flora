# Concept: 022

Deliver a minimal, verifiable hardening slice for Seed/Postgres-first:

1. **Notification idempotency/dedup (DB-enforced):** add a partial unique index for unread notifications on `(user_id, event_type, target_id)` and make notification creation conflict-safe (`ON CONFLICT DO NOTHING`) so concurrent/retried inserts cannot produce duplicate unread signals; keep transition/U0 unassign semantics and best-effort post-persist delivery.
2. **Search degradation contract:** preserve Meilisearch precedence when configured and healthy, but fall back to Postgres FTS on Meilisearch query errors (fail-open to correctness baseline). Keep PG fallback deterministic/tenant+soft-delete safe; explicitly defer mutation-indexing parity/hooks, async task waiting, and rebuild tooling (freshness NV).
3. **Deterministic CI enforcement:** extend `db-tests` to run DB-gated `search`, `notifications`, and `files` contract suites on disposable PostgreSQL 18.
4. **Evidence:** DB-gated contracts (and CI) cover dedup/idempotent suppression behavior and existing PG fallback invariants without adding mandatory services or frontend changes.
