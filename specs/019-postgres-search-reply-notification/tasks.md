# Tasks: 019 — Postgres FTS Search Fallback + Reply Notification

- [x] T019.1 Add additive migration for `messages.search_tsv` (generated, `simple`) and partial GIN index.
- [x] T019.2 Implement PG-backed message search and wire fallback in `/api/v1/search` preserving contract/tenant checks (no 503 when Meilisearch absent).
- [x] T019.3 Emit best-effort `Reply` notification on threaded message creation (valid parent, same org/channel, not deleted, not self).
- [x] T019.4 Add DB-gated contract tests for PG FTS search fallback and reply notification creation/visibility/guards.
- [x] T019.5 Run verification, record SDD evidence, open PR to `dev`, ensure CI green, merge/sync, update `OBJETIVO.md`/`ROADMAP.md`, and define 020.
