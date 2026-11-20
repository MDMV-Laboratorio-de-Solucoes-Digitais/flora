# 020 — Extend PostgreSQL FTS to tasks and file metadata in Flora (VPS, 80/20)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`dde602b3b`) — research only; this document only.  
**Scope:** Concrete 80/20 plan to extend PostgreSQL full-text search (`tsvector`/`tsquery`, GIN, stored/generated columns, ranking/snippets) to **tasks (`title`, `description`)** and **file metadata (`name`, `file_type`)** in a Rust + Axum + sqlx + PostgreSQL 18 app on ~2 vCPU / 4 GB VPS. Covers additive migrations, tenant/org scoping and soft-delete filters, multi-entity query/API design for `GET /api/v1/search?q=&type=&limit`, `websearch_to_tsquery`/`plainto_tsquery`, index/perf tradeoffs, integration with existing PG fallback for messages (019), and DB-gated test patterns.  
**Cross-check:** `flora-api/src/routes/search.rs`, `flora-core/src/migrations/sql/*` (esp. `20261117000001_add_messages_search_fts.sql`, `20240818000001_init_schema.sql`, `20261116000002_add_workspace_id_to_files.sql`), `flora-core/src/models/{task,file,message}.rs`, `flora-core/src/repositories/{task,file}_repository.rs`, `flora-core/src/config/search.rs`, `flora-api/src/state/app_state.rs`, DB-gated search contract tests in `flora-tests/src/contract/search.rs`, and prior `docs/research/2026-11-16-019-postgres-fts-search-vps.md`.

## 1. Baseline and gap (evidence)

| Area | Current state (019) | 020 gap |
|---|---|---|
| Route/API | `GET /api/v1/search?q=&type=&limit` (`flora-api/src/routes/search.rs`): enforces auth + org membership, empty `q` => `200 {query,results:[]}`, `limit` default 20 capped 50, CSV `type`. If Meilisearch `search_service` present uses it; else PG fallback calls `search_messages_pg` only. Response shape `{type,id,snippet}`. | PG fallback must cover `task` and `file` (metadata) with same contract and multi-entity semantics when `type` is omitted/combined. |
| Migration | `20261117000001_add_messages_search_fts.sql`: `messages.search_tsv` generated as `to_tsvector('simple', coalesce(content,'')) STORED` + partial GIN `idx_messages_search_tsv ... WHERE is_deleted = false`. | No `tsvector`/GIN/trgm on `tasks` or `files`. |
| Data model | `tasks(id, workspace_id, organization_id, creator_id, assignee_id, title VARCHAR(255) NOT NULL, description TEXT NULL, status, is_deleted, created_at, updated_at)` with indexes incl. `(organization_id,created_at DESC)` and partial `is_deleted=false`. `files(id, organization_id, owner_id, workspace_id NULLABLE added in 20261116000002, file_type VARCHAR(100), name VARCHAR(255), size_bytes, storage_path, checksum, metadata, is_deleted, created_at, updated_at)` with org/owner/deleted/workspace indexes. | Searchable text is `title`+`description` (tasks) and `name`+`file_type` (files). No content/blob indexing. |
| Repositories/models | `Task`/`File` models and repos (`PgTaskRepository`, `PgFileRepository`) do SELECT/INSERT/RETURNING on fixed columns; no `search_tsv`. Reads are not `SELECT *` against row struct except typed columns, so additive column is low-risk. | Avoid coupling repos to search column; keep search SQL in search path. |
| Integration | `AppState.search_service` is optional Meilisearch; PG path is DB-native and requires no write hooks. Config is Meilisearch-centric (`SearchConfig`). | Preserve 019 degradation: Meilisearch optional, PG default fallback for supported entities. |
| Tests | `flora-tests/src/contract/search.rs` has DB-gated (`FLORA_DATABASE__POSTGRES_URL|DATABASE_URL`, `run_migrations`, unique seeded org/user/membership/workspace/channel/message/session/JWT, `#[ignore = "requires Postgres..."]` + runtime skip) cases for message match, empty/type filter, tenant isolation, deleted exclusion. | Extend with task/file fixtures, multi-entity/type combinations, and ranking/snippet/parse safety as deterministic DB tests. |

**020 objective:** additive PG FTS for tasks/files, unified typed results across `message|task|file`, strict org scoping + soft-delete filtering, safe query parsing, and VPS-appropriate indexes/perf.

## 2. Primary-source foundations (relevant to 020)

- **Stored/generated `tsvector` + GIN:** PostgreSQL recommends `to_tsvector`/`@@`; expression indexes require a fixed text-search configuration. A separate stored/generated `tsvector` column with GIN is fast, avoids re-parsing, and makes index/query config deterministic. ([PG 18: Tables and Indexes](https://www.postgresql.org/docs/current/textsearch-tables.html))
- **Generated stored columns:** `GENERATED ALWAYS AS (...) STORED` is materialized on write and indexable; generation expressions cannot reference other generated columns. ([PG 18: CREATE TABLE](https://www.postgresql.org/docs/current/sql-createtable.html))
- **Safe query parsing:** `websearch_to_tsquery` accepts unformatted/web-style input, recognizes quotes/`OR`/`-`, ignores other punctuation, and does not raise syntax errors; `plainto_tsquery` turns text into ANDed lexemes. `to_tsquery` is strict. Use the same config as indexed `tsvector` (e.g. `'simple'`). ([PG 18: Controlling Text Search — Parsing Queries](https://www.postgresql.org/docs/current/textsearch-controls.html#TEXTSEARCH-PARSING-QUERIES))
- **Ranking/snippets:** `ts_rank_cd` (cover density, uses positions) and `ts_rank` order relevance; normalization flags tune length effects. `ts_headline` returns excerpts/highlights from original text and can be slow, so apply with care (e.g. after limiting). ([PG 18: Ranking](https://www.postgresql.org/docs/current/textsearch-controls.html#TEXTSEARCH-RANKING), [Highlighting](https://www.postgresql.org/docs/current/textsearch-controls.html#TEXTSEARCH-HEADLINE))
- **`pg_trgm`:** trigram similarity and GIN/GiST ops support `%`/`<%`/`ILIKE`/regex-like indexable matching, useful for short names/titles and typo/substring UX; case-insensitive in default builds. ([PG 18: pg_trgm](https://www.postgresql.org/docs/current/pgtrgm.html))

## 3. Multilingual synthesis (EN, pt-BR, es, de, fr, zh-CN, ja)

| Locale/script | 80/20 guidance for 020 | Confidence |
|---|---|---|
| **EN** | Fixed `'simple'` gives deterministic tokenization without stemming surprises; matches 019. `english` config may improve recall but couples all rows/queries to one language and is poor for mixed data. | High (mechanics). |
| **pt-BR, es, de, fr** | `'simple'` works as a stable baseline for accented Latin scripts; stemming/stopwords need language configs and are risky in mixed-language orgs. `pg_trgm` can help name/title contains/typo cases; `unaccent` is a possible later enhancement (not required in 020). | Medium; relevance is corpus-dependent (**NV**). |
| **zh-CN, ja** | PG default parser/tokenization is weak for CJK/Japanese word boundaries; `'simple'` FTS may miss intra-string terms in free text, while short metadata fields may still benefit from substring/trigram-style matching. Treat CJK/ja FTS quality as limited in PG-only MVP. | Low–medium (**NV**). |
| **All (VPS)** | One DB, transactional consistency, no extra service: fixed-config generated columns + partial GIN, tight limits, minimal optional indexes. | High operationally; sizing unmeasured (**NV**). |

**Recommendation:** keep `'simple'` for all 020 `search_tsv` columns/indexes/queries; make `pg_trgm` **optional/additive** for `tasks.title` and `files.name` (metadata UX) rather than required for correctness.

## 4. Concrete 80/20 recommendations for 020

### 4.1 Additive schema migration

Create a single additive sqlx migration, e.g. `flora-core/src/migrations/sql/20261117000004_add_tasks_files_search_fts.sql` (exact version/name per repo convention):

1. **Tasks FTS (required):**
   - `ALTER TABLE tasks ADD COLUMN IF NOT EXISTS search_tsv tsvector GENERATED ALWAYS AS (to_tsvector('simple', coalesce(title,'') || ' ' || coalesce(description,''))) STORED;`
   - `CREATE INDEX IF NOT EXISTS idx_tasks_search_tsv ON tasks USING GIN (search_tsv) WHERE is_deleted = false;`
   - Rationale: covers requested `title` + `description`; DB-maintained on insert/update; partial index limits deleted rows and keeps footprint smaller.

2. **Files metadata FTS (required):**
   - `ALTER TABLE files ADD COLUMN IF NOT EXISTS search_tsv tsvector GENERATED ALWAYS AS (to_tsvector('simple', coalesce(name,'') || ' ' || coalesce(file_type,''))) STORED;`
   - `CREATE INDEX IF NOT EXISTS idx_files_search_tsv ON files USING GIN (search_tsv) WHERE is_deleted = false;`
   - Rationale: requested metadata fields only (`name`, `file_type`); no binary/content indexing.

3. **Optional `pg_trgm` for name/title UX (recommended if extension available):**
   - `CREATE EXTENSION IF NOT EXISTS pg_trgm;` (may require privileges; treat as env/ops prerequisite in constrained deployments).
   - `CREATE INDEX IF NOT EXISTS idx_tasks_title_trgm ON tasks USING GIN (title gin_trgm_ops) WHERE is_deleted = false;`
   - `CREATE INDEX IF NOT EXISTS idx_files_name_trgm ON files USING GIN (name gin_trgm_ops) WHERE is_deleted = false;`
   - Use only for fallback/contains/typo assistance on short fields; keep FTS as primary path.

4. **Migration hygiene:** additive `IF NOT EXISTS`, fixed `'simple'` config (index/query stability), transactional DDL compatible with sqlx migrations. Do **not** depend on `CREATE INDEX CONCURRENTLY` inside migration; document concurrent build as ops alternative for large prod tables (**NV**). No `unaccent`, no Meilisearch dual-write, no outbox in 020 MVP.

**Why not expression indexes:** generated stored columns make the search document explicit, reusable for ranking/snippet planning, and avoid repeating expressions; PG docs endorse this pattern.

### 4.2 Query design for multi-entity `GET /api/v1/search`

Keep request/response contract stable (`SearchQuery { q, type, limit }`, `SearchResponse { query, results: [{type,id,snippet}] }`) and existing route guards (auth, `require_org`, membership, empty-query short-circuit, `limit` cap 50).

**Parsing and normalization:**
- Trim `q`; reuse existing empty handling.
- Build one `tsquery` per request using fixed config `'simple'`: `COALESCE(websearch_to_tsquery('simple', $q), plainto_tsquery('simple', $q))` (matches 019). Guard `IS NOT NULL AND <> ''::tsquery` before `@@`.
- Normalize `type` CSV case-insensitively; accept singular/plural aliases already implied (`message/messages`) and add `task/tasks`, `file/files` (define exact accepted tokens and document; unknown tokens => ignored or yield no results for them — **lock behavior in tests**, NV).

**Entity selection:**
- If `types` is `None`/empty effective set: search all entities supported by 020 PG path (`message`, `task`, `file`).
- If filtered: query only requested supported entities. Preserve 019 behavior where `type=task` must not return messages (and now may return tasks).

**Tenant scoping and soft-delete (mandatory, every branch):**
- `messages`: `WHERE m.organization_id = $1 AND m.is_deleted = false AND m.search_tsv @@ $qts`
- `tasks`: `WHERE t.organization_id = $1 AND t.is_deleted = false AND t.search_tsv @@ $qts`
- `files`: `WHERE f.organization_id = $1 AND f.is_deleted = false AND f.search_tsv @@ $qts`
- Never infer org from client; always bind authenticated `org_id`. `tasks`/`files` already denormalize `organization_id` (files also have optional `workspace_id`) — prefer org predicate for global tenant-safe search consistent with 019/messages.

**Typed results via `UNION ALL` (80/20, parameterized sqlx):**
- Common projection per branch: `id::text AS id`, constant/entity `type`, `snippet TEXT`, `rank REAL/FLOAT8`, `created_at`, `id` as tiebreaker.
- Messages (existing): source `m.content`, `ts_rank_cd(m.search_tsv,$qts)` or current rank, snippet from `left(regexp_replace(content,'\s+',' ', 'g'), 200)` or `ts_headline(...)`.
- Tasks: source `coalesce(t.title,'') || E'\n' || coalesce(t.description,'')`; snippet can be `left(regexp_replace(t.title || ' ' || coalesce(t.description,''), '\s+', ' ', 'g'), 200)` or headline with options like `MaxWords=20, MinWords=5, ShortWord=2, MaxFragments=1`.
- Files: source/label from metadata `coalesce(f.name,'') || ' ' || coalesce(f.file_type,'')`; snippet should prioritize filename (and optionally file type), e.g. `left(regexp_replace(f.name, '\s+', ' ', 'g'), 200)` or `f.name`.
- Outer query: merge branches (or build dynamic-but-parameter-safe SQL by selected types), `ORDER BY rank DESC, created_at DESC, id DESC`, `LIMIT $limit`.
- Apply `ts_headline` after limiting (subselect/CTE over ranked limited rows) to cap CPU/IO on 4 GB VPS, per PG guidance that headline uses original text and can be slow.

**Optional `pg_trgm` fallback (small, additive):**
- If `pg_trgm` indexes exist and FTS yields < `limit` for selected `task/file` types, run a bounded fallback such as `name ILIKE '%'||$q||'%'` or `title ILIKE ...` / similarity operator with a conservative threshold, excluding already-returned IDs, then append until limit. Keep ranking deterministic and simple (e.g. secondary score) to avoid complex fusion in 020.
- Guard with feature detection/migration presence only if implemented; otherwise defer (**NV**).

**ACL note:** route currently enforces org membership only. Do not broaden access; if channel/workspace-level read restrictions are required beyond org membership, treat as explicit follow-up (**risk/NV**), but keep every query org-scoped and `is_deleted=false`.

### 4.3 Integration with existing PG fallback for messages (019)

- **Refactor, don’t fork behavior:** extract/replace `search_messages_pg` into a unified PG search function (e.g. `search_pg` in route module or a dedicated search module/service) that takes `(pool/state, org_id, q, types, limit)` and returns `Vec<SearchResult>` across entities. Preserve 019 message semantics: same `websearch_to_tsquery`/`plainto_tsquery` fallback, `organization_id` + `is_deleted=false`, `ORDER BY rank DESC, created_at DESC, id DESC`.
- **Backend precedence:** keep current wiring — if `state.search_service` (Meilisearch) is `Some`, use it; if `None`/unavailable-at-init, serve via PG fallback (already 019 behavior) so search works without Meilisearch. 020 should not regress to 503 on Meilisearch absence for PG-covered types.
- **Contract compatibility:** same endpoint, params, JSON fields, status codes. Snippet/ranking differences between Meilisearch and PG are expected.
- **Config/state:** no breaking change required for 020 MVP; `SearchConfig` remains Meilisearch-focused. Future additive `provider/auto` work is out of 020 unless trivial (**NV**), but code should structure PG path to be the default-capable fallback.
- **Indexing hooks:** none needed for correctness (generated columns). Avoid dual-write to Meilisearch in 020.

### 4.4 Indexing and performance tradeoffs (~2 vCPU / 4 GB, PG18)

- **Footprint (recommended minimum):** 2 new GIN indexes (`idx_tasks_search_tsv`, `idx_files_search_tsv`, partial on `is_deleted=false`) plus existing `idx_messages_search_tsv`. Add trgm GIN indexes only if fuzzy name/title UX is required and validated.
- **Write/read cost:** generated `tsvector` adds CPU/storage on insert/update of `title/description/name/file_type`; partial GIN reduces size and maintenance for soft-deleted rows. Acceptable for SMB traffic; monitor bloat/autovacuum.
- **Query plan discipline:** always bind `organization_id`, always cap `limit <= 50`, avoid unselective scans, ensure `@@` uses GIN (verify with `EXPLAIN (ANALYZE, BUFFERS)` on representative data), compute `ts_headline` post-limit, keep snippets bounded.
- **`UNION ALL` vs separate queries:** one round-trip `UNION ALL` with global `ORDER/LIMIT` is efficient and simple for 3 entities; alternatively run per-type queries and merge in Rust only if easier to test/compose — but preserve global limit and ordering semantics.
- **VPS tuning (starting points, validate):** conservative `max_connections` + pool size, small `work_mem` (few MB) to protect RAM, adequate `shared_buffers` (e.g. ~512 MB class) and `maintenance_work_mem` for index builds, regular autovacuum. Exact values are environment-specific (**NV**).

### 4.5 DB-gated test patterns (020)

Follow established `flora-tests/src/contract/search.rs` patterns:

1. **Gating/setup:** runtime skip if `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` unset; connect pool; `flora_core::migrations::run_migrations(&pool)` to apply 020 migration; `#[ignore = "requires Postgres; ..."]` for explicit DB runs. Use `Config::default()` + `make_test_state` with `search_service: None` to exercise PG fallback.
2. **Isolation/fixtures:** unique UUIDv7 entities per test; seed `User`, `Organization`, `Role`+`Membership`, `Workspace`, `Session` + JWT (reuse `seed_member`/`authed_get_json` helpers). Add helpers like `insert_task(state, workspace_id, org_id, creator_id, title, description, is_deleted)` and `insert_file(state, org_id, owner_id, workspace_id, name, file_type, is_deleted)` using direct parameterized `INSERT`s (as with channels/messages) so tests are independent of repo method details.
3. **Core 020 cases (deterministic):**
   - **Task match:** term in `title` and/or `description` returns `type=="task"` with correct id; PG fallback path (no Meilisearch).
   - **File metadata match:** term in `name` and/or `file_type` returns `type=="file"` with correct id/snippet containing filename.
   - **Multi-entity default:** omitted `type` with matching rows across message/task/file returns expected typed hits (order asserted only where deterministic, e.g. by strong rank/token uniqueness or by membership checks).
   - **Type filters:** `type=task`, `type=file`, `type=message`, and combinations like `type=task,file` include only requested types; unknown/unsupported tokens follow locked behavior.
   - **Tenant isolation:** identical searchable terms in org B must never appear for org A (tasks/files/messages).
   - **Soft-delete exclusion:** `is_deleted=true` tasks/files/messages never returned.
   - **Safe parsing/no 5xx:** inputs like quoted phrases, `OR`, `-term`, punctuation-heavy strings, and stopword-like/empty-after-parse queries return `200` with empty or valid results (no syntax errors), consistent with 019.
   - **Limit/empty query:** empty/whitespace `q` => `200` empty results; `limit` capped (existing contract).
4. **Optional cases (if implemented):** trgm fallback for near-match filename/title; snippet/highlight formatting assertions kept resilient (substring/contains) rather than exact full-text fragile matches.
5. **Placement:** extend `flora-tests/src/contract/search.rs` (HTTP contract, highest value) and/or `flora-tests/src/integration/search.rs` for DB/query-level checks. Keep unit tests for query/token/type parsing helpers near implementation if extracted.

## 5. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Cross-org leakage | Critical | Bind `organization_id` in every entity branch/CTE; keep auth+membership in route; add negative tenant-isolation DB tests for task/file/message. |
| ACL overexposure (channel/workspace visibility) | High (policy) | 020 preserves org boundary only; document and track as NV/follow-up if finer-grained read ACLs are required for global search. |
| Migration/extension privileges (`pg_trgm`) | Deploy friction | Make trgm optional; `CREATE EXTENSION IF NOT EXISTS` guarded by env/ops readiness; core FTS works without trgm. |
| Relevance/tokenization gaps (pt-BR/es/de/fr, zh-CN/ja) | Poor UX for some locales | Deterministic `'simple'` baseline; optional trgm for metadata; evaluate corpora before locale-specific dictionaries/unaccent/hybrid search. |
| Query cost on small VPS (`ts_headline`, broad `UNION ALL`) | Latency/CPU spikes | Headline/snippet formatting after `LIMIT`, cap 50, partial GIN, selective predicates, per-type sublimits only if needed, `EXPLAIN`/p95 monitoring. |
| Write amplification/index bloat | Disk/IO pressure | Minimal indexes, partial `WHERE is_deleted=false`, autovacuum tuning, avoid redundant trgm indexes unless needed. |
| Contract/behavior drift vs 019 clients | Regressions | Preserve endpoint/params/response schema and 019 cases; additive entity support and explicit type semantics tests. |
| Meilisearch vs PG result differences | Inconsistent UX across modes | Document expected differences; treat PG as correctness fallback, Meilisearch as optional accelerator. |

## 6. NVs (needs verification)

- **NV-1:** p95 latency, CPU, RAM, and disk/write amplification for added task/file `search_tsv` + GIN (+ optional trgm) on 2 vCPU/4 GB with realistic data and concurrency.
- **NV-2:** Relevance quality per locale (**EN, pt-BR, es, de, fr, zh-CN, ja**) for `'simple'` vs language configs and for CJK/ja segmentation limits; value of `unaccent` for pt-BR/es/fr/de.
- **NV-3:** Exact `type` token contract for multi-entity search (singular/plural, unknown types, mixed filters) and whether omitted `type` must include all supported entities in both Meilisearch and PG modes.
- **NV-4:** Whether 020 must include `pg_trgm` fallback in MVP or can ship FTS-only and add trgm later based on UX need/privileges.
- **NV-5:** Required authorization granularity for task/file/message visibility in global search (org-only vs workspace/channel-aware) and corresponding query constraints.
- **NV-6:** Operational procedure for large-table index builds if transactional migration DDL becomes slow (and whether `CREATE INDEX CONCURRENTLY`/maintenance window is needed).
- **NV-7:** Snippet strategy consistency (`left(...)` vs `ts_headline`) and highlighting markers/escaping expectations for API consumers.

## 7. Recommended 020 delivery checklist (smallest verifiable slice)

1. Add migration `20261117000004_add_tasks_files_search_fts.sql`: generated `search_tsv` + partial GIN for `tasks` and `files` (fixed `'simple'`); optionally add `pg_trgm` indexes if environment supports.
2. Implement unified PG search path extending 019: `websearch_to_tsquery` + `plainto_tsquery` fallback, per-entity `organization_id`/`is_deleted` filters, `UNION ALL` typed results, rank+deterministic tie-break, bounded snippets, `type` filtering, `limit` cap.
3. Wire `flora-api/src/routes/search.rs` to use unified PG fallback when Meilisearch is absent while preserving existing contract and membership checks.
4. Add/extend DB-gated contract tests for task/file hits, multi-entity/type filter behavior, tenant isolation, soft-delete exclusion, and parse safety.
5. Validate with `EXPLAIN (ANALYZE, BUFFERS)` on seeded data and record perf/NV notes; defer CJK-specific tokenization, ACL expansion, dual-write, and outbox.

## 8. Primary sources

- PostgreSQL 18 — Full Text Search, Tables and Indexes: https://www.postgresql.org/docs/current/textsearch-tables.html
- PostgreSQL 18 — Controlling Text Search (parsing queries, ranking, highlighting): https://www.postgresql.org/docs/current/textsearch-controls.html
- PostgreSQL 18 — `pg_trgm`: https://www.postgresql.org/docs/current/pgtrgm.html
- PostgreSQL 18 — `CREATE TABLE` (generated columns): https://www.postgresql.org/docs/current/sql-createtable.html

## 9. Repo references

- Prior research: `docs/research/2026-11-16-019-postgres-fts-search-vps.md`
- API/state: `flora-api/src/routes/search.rs`, `flora-api/src/state/app_state.rs`
- Search/config: `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`
- Schema/migrations: `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `flora-core/src/migrations/sql/20261116000002_add_workspace_id_to_files.sql`, `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`
- Models/repos: `flora-core/src/models/{message,task,file}.rs`, `flora-core/src/repositories/{message,task,file}_repository.rs`
- Tests: `flora-tests/src/contract/search.rs`, `flora-tests/src/integration/search.rs` (placeholder), DB-gated patterns in `flora-tests/src/contract/{auth,files,messaging,websocket}.rs`
- Planning context: `OBJETIVO.md`, `ROADMAP.md`
