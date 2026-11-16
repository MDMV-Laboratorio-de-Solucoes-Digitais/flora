# Plan: 020 — PG FTS Tasks/Files + Task Assignment Notification

## Inputs
- Assessment: `.specify/assessments/020-next-vertical-slice/{intake,research,problem,concept,decision}.md`
- Research: `docs/research/2026-11-16-020-{pg-fts-tasks-files-vps,task-assignment-notification-vps,macro-tasks-files-assignment-flows,db-gated-tests-search-notifications-assignment,pg18-multi-entity-fts-vps-ops}.md`
- 019 baseline: `docs/research/2026-11-16-019-{postgres-fts-search-vps,inapp-notification-wiring-vps,db-gated-tests-search-notifications}.md`, `specs/019-postgres-search-reply-notification/`
- Audits: `ses_e1d674de4ffevX4wonv9WQ0JAS` (codebase), `ses_e1d670690ffeDzKAP9DWM7qZlR` (frontend)

## Design
1. **Migration:** `20261118000001_add_tasks_files_search_fts.sql` adds generated stored `search_tsv` (`simple`) and partial GIN indexes (`WHERE is_deleted = false`) for `tasks` (`title||description`) and `files` (`name||file_type`).
2. **Search PG fallback:** refactor/extend `flora-api/src/routes/search.rs` to query requested entities in PG when `search_service` absent:
   - messages (existing behavior) plus tasks and files with `organization_id` + `is_deleted=false` + `search_tsv @@ tsq`.
   - parse types CSV (`message(s)`, `task(s)`, `file(s)`), support empty/unknown-only, merge results and apply global `limit` with deterministic ordering.
   - keep Meilisearch path unchanged and preserve API contract/snippet shape.
3. **Assignment notification:** in `flora-api/src/routes/tasks.rs`, mirror 019 reply pattern:
   - add `maybe_notify_task_assignment(...)` helper using `PgNotificationRepository`/`Notification::new(..., NotificationType::Assignment, task_id, None)` with best-effort warn.
   - create path: notify on initial `assignee_id` when not actor.
   - update path: detect explicit provided `assignee_id` transition (`Some(new)` differs from existing and not actor) post-persist.
4. **Tests:** extend `flora-tests/src/contract/search.rs` (multi-entity PG fallback cases, keep 019 regressions valid) and `flora-tests/src/contract/notifications.rs` (assignment cases using tasks+notifications routers and seeded multi-member org), env-gated for disposable PG18.

## Verification
- `cargo fmt --all -- --check`
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`
- `cargo test -p flora-core --lib`, `cargo test -p flora-api`, `cargo test -p flora-tests --lib`
- DB-gated search/notification/assignment tests when `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` available (CI db-tests)
- Frontend checks not required (no FE changes)

## Risks/NVs
- Relevance/i18n/CJK and search ACL/perf on 2vCPU/4GB remain NV.
- Task unassign not expressible in current PATCH contract; no idempotency/tx-coupling/race hardening.
- Meilisearch tasks/files parity deferred (PG fallback path validated).
