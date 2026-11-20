# Research: 020 — Tasks/files PG FTS + assignment notification

**Sources:** R1 research docs, codebase/frontend audits, 019 artifacts, `OBJETIVO.md`/`ROADMAP.md`.

## Convergence (80/20)
- **PG FTS multi-entity:** extend 019 messages-only fallback by adding additive stored/generated `search_tsv` (`'simple'`) + partial GIN (`WHERE is_deleted = false`) for `tasks(title,description)` and `files(name,file_type)`. Use same safe parsing (`COALESCE(websearch_to_tsquery('simple',$q), plainto_tsquery('simple',$q))`), strict `organization_id` and `is_deleted=false` filters per entity, typed results (`message|task|file`) merged with global `LIMIT`, and preserve `/api/v1/search` contract. Do not regress Meilisearch path.
- **Data/tenant facts:** `tasks` has `organization_id`, `is_deleted`, `workspace_id`, `assignee_id`; `files` has `organization_id`, `is_deleted`, nullable `workspace_id` (legacy) — org-scoped filter is sufficient for tenant isolation in global search. Generated columns avoid backfill and keep repos unchanged.
- **Assignment notification:** `NotificationType::Assignment` and schema/API/repo already exist. Emit on effective transition to new assignee (`old != Some(new)`) where `new != actor` (covers create with assignee and update/reassign via provided `assignee_id`); best-effort post-persist in `flora-api/src/routes/tasks.rs` (routes bypass `TaskService`) with warn-on-failure like 019 reply hook. No notify for self-assign, no-op, or cases where effective assignee unchanged. `PATCH` currently merges `assignee_id.or(existing.assignee_id)` so explicit unassign is not representable (NV/contract gap) — 020 must not fabricate unassign behavior.
- **Tests:** extend DB-gated env-gated contract tests in `flora-tests/src/contract/search.rs` and `flora-tests/src/contract/notifications.rs` (and task cases as needed) on disposable PG18 with `search_service: None` fallback path, unique fixtures, tenant/soft-delete assertions, and deterministic assignment guards.
- **Frontend:** `apps/frontend` has no search/notification integration; additive backend changes are wire-compatible (no FE change required in 020).

## Key risks/NVs
- Relevance/tokenization/i18n (pt-BR/es/de/fr, zh-CN/ja) limited with `'simple'`; `pg_trgm` optional/deferred. Channel/workspace ACL in search beyond org boundary is NV.
- Task PATCH cannot express explicit clear/unassign; idempotency/tx-coupling/race safety and assignee membership validation are NV/hardening.
- 2vCPU/4GB perf, index bloat/autovacuum tuning, Meilisearch parity for tasks/files, mentions/completion/outbox/push/realtime delivery, browser e2e are deferred.
