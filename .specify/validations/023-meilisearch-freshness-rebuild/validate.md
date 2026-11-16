# Validation: 023 — Postgres-Authoritative Search Contract

**Date:** 2026-11-16  
**Branch:** `feat/023-postgres-authoritative-search` (from `dev@023bd2467`)  
**Scope:** S1 per `.specify/assessments/023-next-vertical-slice/decision.md`

## Evidence executed (local, disposable PostgreSQL 18 on 127.0.0.1:5432/flora_023_test)

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | OK |
| `cargo clippy -p flora-core -p flora-tests --all-targets -- -D warnings` | No issues |
| `cargo clippy -p flora-api --lib -- -D warnings` | No issues |
| `cargo test -p flora-core --lib` | 8 passed |
| `cargo test -p flora-api --lib` | 13 passed |
| `cargo test -p flora-tests --lib` | 44 passed, 43 ignored |
| `cargo test -p flora-tests contract::search::db_ -- --ignored --test-threads=1` | **8 passed** (incl. 2 new 023 contracts) |
| `contract::auth::db_` / `messaging` / `websocket` / `notifications` / `files` | 12 / 4 / 8 / 5 / 6 passed |

Note: a parallel agent's uncommitted `/metrics` work in `flora-api/src/routes/{health,mod}.rs` is intentionally
excluded from this branch's commit. Its test code trips `clippy::expect_used` under `--all-targets`; the
authoritative check for this cycle's code is `cargo clippy -p flora-api --lib -- -D warnings` plus the green
CI run on the PR (which does not carry those files).

TDD evidence: both new 023 contracts were executed **before** implementation and failed as expected:
- `db_search_pg_authoritative_with_meilisearch_configured` → missing `source` field.
- `db_search_pg_matches_hyphenated_names` → hyphenated file name returned no match.

## Contract coverage

- **PG authority:** `/api/v1/search` always executes `search_pg`; the Meilisearch request branch was removed.
  `SearchResponse` now contains `source: "postgres"` for both empty and normal responses. The new DB-gated
  contract injects a `SearchService` pointing at a closed port (`Client::new` is lazy) and asserts PG results
  plus `source == "postgres"` — deterministic, with no Meilisearch process required.
- **PG FTS hardening:** additive migration `20261116000004_search_fts_separator_normalization.sql` recreates
  generated `tsvector` columns for `messages`/`tasks`/`files`, replacing `-`, `.`, `_` with spaces, and restores
  the partial GIN indexes. New DB-gated contract verifies hyphenated file/task names match sub-token queries.
- **Config/docs:** `SearchConfig::default().enabled = false` aligns with `.env.example` and the Seed profile;
  `.env.example` documents all `FLORA_SEARCH__*` keys; `docs/dev-smoke.md` records the capability/degradation
  contract and Meilisearch re-enablement gates.
- **Preserved:** tenant/membership checks, empty-query handling, type normalization, limit cap, snippets, and
  existing search/notifications/files contracts. CI `db-tests` job unchanged.

## Explicit non-claim regarding the Seed milestone

This cycle changes the search *mechanism* documentation; it does not by itself declare "Seed v0.1 verified".
The Seed verification gate and the README/constitution mechanism drift are tracked separately (see the 023 R1
research docs and the Seed reconciliation research). No frontend changes.

## NV / residual

- Meilisearch mutation hooks, delete-on-soft-delete, task-await wrapper, rebuild/backfill/blue-green swap,
  Meilisearch-gated CI and cross-engine parity remain deferred (re-enablement gates documented).
- PG FTS relevance remains NV for `pg_trgm`/typo tolerance and i18n/CJK corpora; search ACL NV.
- Outbox/tx-coupling, deterministic notification UUIDs, unassign events, mentions/push, 2vCPU/4GB load/restore,
  browser e2e remain out of scope.
