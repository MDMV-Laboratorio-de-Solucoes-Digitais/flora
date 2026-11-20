# Tasks: 023 — Postgres-Authoritative Search Contract

## Implementation
- [x] T001 Write DB-gated contracts first (authoritative `source` with unreachable Meili configured; hyphenated-name sub-token match).
- [x] T002 Make `search_pg` the sole request path and add `source: "postgres"` to `SearchResponse`.
- [x] T003 Add additive migration normalizing `-`/`.`/`_` in generated `search_tsv` for messages/tasks/files (+ partial GIN indexes).
- [x] T004 Align `SearchConfig::default().enabled = false`; expand `.env.example` search keys; document capability/degradation in `docs/dev-smoke.md`.
- [x] T005 Run local verification (fmt, clippy, unit + all DB-gated suites) and fix fallout.

## PR/Release
- [x] T006 Record validation/release artifacts, update `OBJETIVO.md`/`ROADMAP.md` for 023→024, commit/push, PR to `dev`, CI green, merge, sync, clean branches.
