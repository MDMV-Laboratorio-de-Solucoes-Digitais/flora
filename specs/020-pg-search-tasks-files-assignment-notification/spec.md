# Feature Specification: 020 — PG FTS Tasks/Files + Task Assignment Notification

**Feature Branch**: `feat/020-pg-search-tasks-files-assignment-notification`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/020-next-vertical-slice/decision.md` (verdict: go, slice S1)

## Goal
Extend Postgres-first search and in-app notifications for Seed/VPS by making tasks and file metadata discoverable via the existing search fallback (without Meilisearch) and emitting deterministic task-assignment notifications through existing primitives, verified with DB-gated tests.

## Scope

### Search (PG fallback, multi-entity)
- Add additive migration `flora-core/src/migrations/sql/20261118000001_add_tasks_files_search_fts.sql`:
  - `tasks.search_tsv` `tsvector` generated/stored as `to_tsvector('simple', coalesce(title,'') || ' ' || coalesce(description,''))`.
  - `files.search_tsv` generated/stored as `to_tsvector('simple', coalesce(name,'') || ' ' || coalesce(file_type,''))`.
  - Partial GIN indexes on `tasks(search_tsv)` and `files(search_tsv)` with `WHERE is_deleted = false`.
- Extend `flora-api/src/routes/search.rs` PG fallback (when `state.search_service` is `None`) to search `message`, `task`, and `file` entities per requested `type` filter, preserving:
  - Contract: `GET /api/v1/search?q=&type=&limit` -> `{query, results:[{type,id,snippet}]}` with `type` in `message|task|file`.
  - Auth/org membership checks, empty/blank query -> `200` empty results, `limit` default 20 capped 50.
  - Tenant isolation (`organization_id = org`) and soft-delete exclusion (`is_deleted = false`) on every entity.
  - Safe parsing: `COALESCE(websearch_to_tsquery('simple',$q), plainto_tsquery('simple',$q))`, ranked results, bounded snippets.
- `type` behavior: omitted/empty -> all supported PG-fallback entities; CSV tokens include `message|messages`, `task|tasks`, `file|files`; unknown-only -> empty results. Meilisearch-backed path remains unchanged.

### Notifications (task assignment)
- In `flora-api/src/routes/tasks.rs`:
  - **Create:** after successful task persist, if `created.assignee_id = Some(assignee)` and `assignee != auth.user_id`, best-effort create `NotificationType::Assignment` for `assignee` in task org with `target_id=task.id`, `target_url=None`.
  - **Update:** capture `old_assignee = existing.assignee_id`; after successful persist, if request provides `assignee_id = Some(new)` and `old_assignee != Some(new)` and `new != auth.user_id`, best-effort create `Assignment` for `new`.
  - Best-effort/non-fatal: notification failure must not fail task mutation (log warning). No notification for self-assign, no-op/unchanged effective assignee, or when no explicit new assignee provided by request.
- Existing notification list/read API/response unchanged.

### Verification
- DB-gated contract tests on disposable PostgreSQL 18:
  - Search fallback: task/file matches, message regression, type filters, tenant isolation, soft-delete exclusion, empty query behavior, no-Meilisearch path (`search_service: None`).
  - Assignment notifications: create-with-assignee notifies assignee, self-assignment/no-op do not notify, reassignment/change notifies new assignee, visibility via `GET /api/v1/notifications` scoped to recipient.

## Non-goals
- `pg_trgm` fuzzy matching, relevance/i18n/CJK tuning, channel/workspace-level ACL in search, search backfill/reindex tooling.
- Meilisearch indexing/query parity for tasks/files, frontend search/notification UI.
- Explicit unassign API contract, outbox/workers/Valkey/push/realtime notification delivery, mention/completion fanout, dedup/idempotency uniqueness schema, tx-coupling/race hardening, assignee membership policy changes.
- 2vCPU/4GB perf/load benchmarks, browser e2e/multi-client.

## NV
- Task `PATCH` cannot explicitly clear assignee with current merge semantics (`req.assignee_id.or(existing.assignee_id)`); unassign-driven notification cases deferred.
- Notification idempotency under retries/duplicates, best-effort post-persist semantics, and concurrent update races.
- Meilisearch vs PG result parity/ranking differences for tasks/files; `files.workspace_id` null legacy visibility in global org search (org-scoped, allowed).
- Relevance/tokenization quality for pt-BR/es/de/fr/zh-CN/ja with `'simple'`; VPS perf/index maintenance.
