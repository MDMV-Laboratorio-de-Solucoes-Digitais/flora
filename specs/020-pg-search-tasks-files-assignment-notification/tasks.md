# Tasks: 020 — PG FTS Tasks/Files + Task Assignment Notification

## Implementation
- [x] T001 Add migration `flora-core/src/migrations/sql/20261118000001_add_tasks_files_search_fts.sql` (`tasks`/`files` generated `search_tsv` + partial GIN).
- [x] T002 Extend PG search fallback in `flora-api/src/routes/search.rs` for `task`/`file` entities with type filtering, tenant/soft-delete guards, safe tsquery, and preserved contract/Meilisearch path.
- [x] T003 Add best-effort task assignment notification hook/helper in `flora-api/src/routes/tasks.rs` (create + effective update transition, no self/no-op).
- [x] T004 Extend DB-gated contracts in `flora-tests/src/contract/search.rs` for multi-entity PG fallback (tasks/files, filters, isolation, soft-delete, empty/type behavior).
- [x] T005 Extend DB-gated contracts in `flora-tests/src/contract/notifications.rs` for assignment create/self/no-op/reassign visibility.
- [x] T006 Run verification (fmt/clippy/tests, DB-gated when available), record validation/release artifacts, and update planning docs.

## PR/Release
- [ ] T007 Commit scoped changes, push branch, open PR to `dev`, ensure CI green, merge, sync `dev`/`origin/dev`, clean branch, update `OBJETIVO.md`/`ROADMAP.md` for 021.
