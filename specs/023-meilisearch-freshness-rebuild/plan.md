# Plan: 023 — Postgres-Authoritative Search Contract

## Inputs
- Assessment: `.specify/assessments/023-next-vertical-slice/{intake,research,problem,concept,decision}.md`
- Research: `docs/research/2026-11-16-023-*.md` (6 docs), codebase/frontend/ops audits
- Prior: 022 artifacts, constitution (Seed = Flora + PostgreSQL)

## Design
1. **Route (`flora-api/src/routes/search.rs`)**: remove the `if let Some(search_service)` branch; always `search_pg(&state, org_id, &query, types.as_deref(), limit).await?`. Add `pub source: String` to `SearchResponse`, populated with `"postgres"` for both empty and normal responses.
2. **Migration (`flora-core/src/migrations/sql/20261116000004_search_fts_separator_normalization.sql`)**: for each of `messages`, `tasks`, `files`, drop the partial GIN index and generated `search_tsv` column, re-add with `replace(replace(replace(coalesce(...,''),'-',' '),'.',' '),'_',' ')` normalization (files also concatenate `file_type`), recreate the partial index.
3. **Config (`flora-core/src/config/search.rs`)**: `enabled: false` default; update doc comment. `.env.example`: document `DEFAULT_LIMIT`, `MAX_LIMIT`, `INDEX_TEMPLATE`, legacy fallbacks.
4. **Tests (`flora-tests/src/contract/search.rs`)**: add `db_search_pg_authoritative_with_meilisearch_configured` (construct `SearchService::new("http://127.0.0.1:1", ...)` → `Some(Arc::new(...))` in `AppState`, assert `source == "postgres"` and PG results) and `db_search_pg_matches_hyphenated_names` (file/task fixture with hyphens). Keep existing suites intact.
5. **Docs**: `docs/dev-smoke.md` capability/degradation note; planning artifacts; `OBJETIVO.md`/`ROADMAP.md` for 023→024.

## Verification
- `cargo fmt --all -- --check`
- `cargo clippy -p flora-core -p flora-api -p flora-tests --all-targets -- -D warnings`
- `cargo test -p flora-core --lib`, `-p flora-api`, `-p flora-tests --lib`
- All six DB-gated suites on disposable PG18 (env-gated locally), serially `--test-threads=1`
- CI green including `db-tests`

## Risks/NVs
- Generated-column recreation on existing DBs: derived data only; verify on fresh DB in CI.
- Additive response field: no FE consumer; contract tests updated.
- Meilisearch re-enablement and relevance improvements deferred.
