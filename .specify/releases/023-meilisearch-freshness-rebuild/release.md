# Release: 023 — Postgres-Authoritative Search Contract

**Date:** 2026-11-16  
**Branch/PR:** `feat/023-postgres-authoritative-search` -> `dev` (pending)  
**Baseline:** `dev@023bd2467`

## Delivered (slice S1)
- **PG-authoritative search:** `GET /api/v1/search` always executes Postgres FTS; the Meilisearch request branch
  was removed. `SearchResponse` includes `source: "postgres"` (additive capability contract; no canonical FE consumer).
  `flora-search` and the compose `search` profile stay dormant/non-authoritative.
- **PG FTS hardening:** additive migration `20261116000004_search_fts_separator_normalization.sql` normalizes
  `-`, `.`, `_` in generated `tsvector` columns for messages/tasks/files (and restores partial GIN indexes), so
  hyphenated names match sub-token queries.
- **Config/docs:** `SearchConfig::default().enabled = false` aligns with `.env.example` and the Seed profile;
  all `FLORA_SEARCH__*` keys documented; `docs/dev-smoke.md` records the capability/degradation contract and
  Meilisearch re-enablement gates.
- **Tests:** two new DB-gated contracts (PG authority even with an unreachable Meili configured; hyphenated-name
  sub-token match), written first and verified red before implementation. All six CI DB-gated suites green locally.
- **R1:** 8 background research agents produced 6 cycle docs plus a Seed-milestone research round; assessment
  and decision artifacts recorded. No frontend changes; no new services.

## Evidence
- Local: fmt, strict clippy (core/tests all-targets; api lib), unit tests and all six DB-gated suites green on a
  disposable PostgreSQL 18 database — details in `.specify/validations/023-meilisearch-freshness-rebuild/validate.md`.
- CI: PR result to be recorded here after the required checks are green.

## Seed milestone note
This cycle closes the *search mechanism* question for Seed (PG FTS authoritative; Meilisearch deferred to a gated
Grove tier). It does **not** by itself declare "Seed v0.1 verified": the remaining decisive gap is OIDC ID-token
verification, selected as cycle 024. See `docs/research/2026-11-16-seed-*.md` and
`.specify/assessments/024-next-vertical-slice/decision.md`.

## NV / follow-ups (024+)
- OIDC signature/JWKS/`iss`/`aud`/`exp`/`nonce` verification (024); Phase 0 CI enforcement; Seed decision ADR/docs
  reconciliation; honest OTel wiring (`opentelemetry-otlp 0.22` does not exist on crates.io); notification transport
  fork; Meilisearch hooks/await/rebuild/CI; `pg_trgm`/i18n/CJK relevance; browser e2e/Tauri.
