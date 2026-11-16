# Concept: 021

Deliver a minimal, verifiable correctness/consolidation slice for Seed/Postgres-first:

1. **Task assignment contract:** make `PATCH /api/v1/tasks/{id}` `assignee_id` tri-state (omit=no change, `null`=clear, UUID=set), resolve effective assignee transitions deterministically, and emit `Assignment` only for effective assign/reassign to a non-actor user (no notification on unassign/no-op/self), with a simple best-effort dedup guard for duplicate unread assignment signals.
2. **Search contract hardening/decision:** harden and DB-gated-test PG fallback behavior (type tokens, limits/bounds, tenant/soft-delete invariants) while explicitly documenting search backend precedence/contract: PG fallback is authoritative for multi-entity parity when Meilisearch is unavailable; Meilisearch-enabled mode remains unchanged but task/file indexing parity is deferred pending mutation hooks and deterministic tests.
3. **Evidence:** DB-gated contracts on disposable PostgreSQL 18 cover unassign/assignment transitions and search edge cases without adding mandatory services or frontend changes.
