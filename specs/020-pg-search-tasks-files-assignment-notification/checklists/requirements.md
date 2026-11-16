# Requirements Checklist: 020

- [x] Scope limited to 80/20: PG FTS fallback for tasks/files metadata + deterministic task-assignment notification.
- [x] No new mandatory services; Postgres-first; additive migration/indexes.
- [x] Existing API contracts preserved (search response shape/params, notifications list/read shape, Meilisearch path intact).
- [x] Tenant isolation and soft-delete exclusion enforced in search queries.
- [x] Assignment notification best-effort/non-fatal and guarded (effective transition, no self, no no-op).
- [x] DB-gated deterministic tests defined for disposable PostgreSQL 18.
- [x] NVs/non-goals documented (unassign contract, idempotency/tx-coupling, ACL, CJK/relevance/perf, Meilisearch parity, frontend UI).
