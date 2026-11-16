# Decision: 020 — Next vertical slice

**Verdict:** **go** (scoped)

## Chosen slice

**S1 — Postgres FTS fallback for tasks/files + task-assignment in-app notification (disposable PostgreSQL 18, Seed/VPS).**  
Primary 80/20 for cycle 020: extend the 019 PG search fallback to include tasks (`title`,`description`) and file metadata (`name`,`file_type`) via additive `search_tsv`+GIN and unified typed results, and emit one deterministic in-app `Assignment` notification on effective task assignee transition (create/update), verified by DB-gated tests.

## Justification (evidence)

- **Roadmap fit:** `OBJETIVO.md`/`ROADMAP.md` define 020 as extending PG FTS to tarefas/metadados de arquivos and adding deterministic task-assignment notification, Postgres-first and verifiable on cheap VPS.
- **Convergent R1:** `docs/research/2026-11-16-020-pg-fts-tasks-files-vps.md`, `docs/research/2026-11-16-020-task-assignment-notification-vps.md`, `docs/research/2026-11-16-020-macro-tasks-files-assignment-flows.md`, `docs/research/2026-11-16-020-db-gated-tests-search-notifications-assignment.md`, `docs/research/2026-11-16-020-pg18-multi-entity-fts-vps-ops.md`, and audits (`ses_e1d674de4ffevX4wonv9WQ0JAS`, `ses_e1d670690ffeDzKAP9DWM7qZlR`) converge on additive schema, route-level transition hook (mirroring 019), no frontend change, and DB-gated determinism.
- **Value/80/20:** makes core work items discoverable without Meilisearch and closes the assignment feedback loop using existing models/repos/APIs, with minimal surface and no Kafka/SQS/OpenSearch/outbox/workers/push.

## Scope summary (in)

- Additive migration: generated stored `search_tsv` (`simple`, `title||description` / `name||file_type`) and partial GIN indexes (`WHERE is_deleted = false`) for `tasks` and `files`.
- Extend PG search fallback in `flora-api/src/routes/search.rs` to return typed `message|task|file` hits when Meilisearch is unavailable (and preserve Meilisearch path), keeping contract/tenant/membership checks, safe tsquery parsing, soft-delete exclusion, type filtering, and bounded limit/snippets.
- Task assignment notification hook in `flora-api/src/routes/tasks.rs` (create/update): on effective transition to `Some(new)` where `old != Some(new)` and `new != actor`, best-effort insert `NotificationType::Assignment` (`target_id=task.id`, `target_url=None`) for assignee; do not fail task mutation.
- DB-gated contract tests on disposable PG18 for multi-entity/type-filtered/tenant/soft-delete search fallback and assignment notification create/self/no-op/transition visibility.
- SDD artifacts, local verification evidence, PR/CI green, merge/sync, planning updates.

## Key risks

1. **Search contract/type semantics drift or perf on small VPS:** mitigate with additive migration/indexes, fixed `'simple'`, bounded per-request limits, deterministic merge/order, and DB-gated assertions; record NVs (relevance/i18n/CJK, trgm/ACL).
2. **Task assignment semantics limited by current PATCH contract:** `assignee_id` cannot be explicitly cleared (`or(existing)` merge) and routes bypass `TaskService`; mitigate by emitting only on explicit provided `Some(new)` transitions, guarding self/no-op, best-effort warn-on-failure, and documenting NVs (unassign, dedup/idempotency, tx-coupling, membership/race).
3. **Meilisearch vs PG parity divergence:** keep Meilisearch behavior unchanged and validate PG fallback path; document NV for tasks/files parity in Meilisearch mode.

## NVs / non-goals

- NV: relevance/i18n/CJK quality and `pg_trgm`, channel/workspace-level ACL in search, Meilisearch indexing/query parity for tasks/files, outbox/workers/push/realtime delivery, mentions/completion notifications, dedup/idempotency uniqueness/tx-coupling/race hardening, explicit unassign API contract, assignee membership enforcement changes, 2vCPU/4GB perf/load/OOM/restore benchmarks, browser e2e/multi-client.
- Non-goals: frontend UI, Meilisearch removal, notification `target_url` exposure/API shape changes, crate convergence, multi-node infra, new mandatory services.
