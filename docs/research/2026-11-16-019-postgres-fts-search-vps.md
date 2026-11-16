# 019 — PostgreSQL FTS/`pg_trgm` as default search backend for Flora (VPS, 80/20)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`3c8eddbfd`) — read-only analysis; this document only.  
**Scope:** Concrete 80/20 plan for 019 to implement **PostgreSQL full-text search (`tsvector`/`tsquery`, GIN, stored/generated columns) with optional `pg_trgm`** as the **default** search backend for a self-hosted SMB app (Rust + Axum + sqlx + PostgreSQL 18) on ~2 vCPU/4 GB VPS. Covers schema/migrations for messages (and tasks/files names/titles as trivial extensions), tenant/org scoping, ranking/snippets, query parsing, indexing/perf, integration with existing API contract, fallback/degradation when Meilisearch absent, and DB-gated testing patterns.  
**Cross-check:** `flora-api/src/routes/search.rs`, `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`, `flora-core/src/services/*`, message/task/file schemas/models/repos (`flora-core/src/migrations/sql/*`, `flora-core/src/models/{message,task,file}.rs`, `flora-core/src/repositories/{message,task,file}_repository.rs`), `flora-tests` DB-gated patterns (`flora-tests/src/contract/{auth,files,messaging,websocket}.rs`, `flora-tests/src/{contract,integration}/{search,notifications}.rs`), and prior research `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`, `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`.

## 1. Flora baseline (evidence)

| Area | Current state | 019 implication |
|---|---|---|
| API contract | `GET /api/v1/search?q=&type=&limit` (`flora-api/src/routes/search.rs`): requires org membership, empty `q` -> `200 {query, results:[]}`, `limit` default 20 capped 50, `type` CSV filter. If `state.search_service` is `None` returns **503** `"Search is disabled or unavailable..."`. Response items: `{type,id,snippet}` from `flora_search::SearchResult`. | Preserve request/response shape. Replace “503 when no Meilisearch” with **PG-first availability**; 503 only when *no* backend can serve (policy). Keep tenant check in route. |
| Search backend | `flora-search` is Meilisearch-only (`SearchService::search` on org index `flora_org_{org_id}`), index/index/delete methods unused by runtime; `SearchResult {id,item_type,snippet}`. `AppState` creates it only when `search.enabled` and URL reachable-ish at init, else `None`. | 019 needs a Postgres backend abstraction (trait or enum) usable by route/state; Meilisearch becomes optional accelerator, not default dependency. |
| Config | `SearchConfig` (`flora-core/src/config/search.rs`): Meilisearch-centric fields, `enabled=true`, validates URL when enabled, `index_template`. Default env: `MEILISEARCH_URL`. | 80/20: extend config additively (e.g. `backend/provider = postgres|meilisearch|auto`, PG-on-by-default in Seed) while keeping backward-compatible env parsing/validation; avoid breaking existing fields in this slice. **NV** exact naming. |
| Data model | `messages(id,channel_id,organization_id,sender_id,content,thread_id,is_edited,is_deleted,created_at,updated_at)` + indexes incl. `(organization_id,created_at DESC)`, partial `is_deleted=false`. `tasks(...,title,description,status,assignee_id,is_deleted,organization_id,workspace_id,...)`. `files(...,organization_id,workspace_id,name,file_type,storage_path,checksum,metadata,is_deleted,...)` with workspace added in `20261116000002`. No `tsvector`/GIN/`pg_trgm`. | Messages are the highest-value P0 FTS target; tasks/files name/title are trivial additive columns/indexes with same pattern. |
| Mutation wiring | `channels.rs`/`tasks.rs`/`files.rs` persist via repos/services and emit realtime/broadcast only (messages); no search indexing hooks. `flora-core` message/task services are thin/partly unused by routes. | Prefer **DB-native generated `tsvector`** so row writes already maintain search docs (no app hook required for correctness in 019 MVP). Optional explicit reindex/backfill later. |
| Tests | Search contract/integration tests are placeholders. DB-gated pattern exists elsewhere: read `FLORA_DATABASE__POSTGRES_URL` or `DATABASE_URL`, skip if unset, `PgPool::connect`, run migrations/app setup, isolate data (UUIDv7 IDs, seeded org/user/membership), some contract tests use `connect_lazy` only when no DB hit. | 019 search tests must be env-gated, migration-applied, deterministic, tenant-isolated, and CI-safe when DB absent. |

## 2. Primary-source foundations (80/20)

- **Stored `tsvector` + GIN:** PostgreSQL recommends `to_tsvector`/`@@` with GIN (or GiST); expression indexes require a fixed text-search configuration. A stored/generated `tsvector` column with a GIN index is fast and avoids re-parsing and config drift in queries/indexes. *PG Docs: [Tables and Indexes](https://www.postgresql.org/docs/current/textsearch-tables.html).* 
- **Query parsing for user input:** `websearch_to_tsquery` accepts unformatted web-style syntax, never raises syntax errors, and supports quotes/OR/`-`; `plainto_tsquery` is the safe AND fallback. `to_tsquery` is strict. *PG Docs: [Controlling Text Search / Parsing Queries](https://www.postgresql.org/docs/current/textsearch-controls.html#TEXTSEARCH-PARSING-QUERIES).* 
- **Ranking/snippets:** `ts_rank`/`ts_rank_cd` (cover density, needs positions) order by relevance; normalization flags control length effects. `ts_headline` generates excerpts with highlighting; uses original text and can be slow, so use with care/caps. *PG Docs: [Ranking](https://www.postgresql.org/docs/current/textsearch-controls.html#TEXTSEARCH-RANKING), [Highlighting](https://www.postgresql.org/docs/current/textsearch-controls.html#TEXTSEARCH-HEADLINE).* 
- **Generated stored columns:** `GENERATED ALWAYS AS (...) STORED` materializes on write and is indexable — ideal for `tsvector`. *PG Docs: [CREATE TABLE](https://www.postgresql.org/docs/current/sql-createtable.html#SQL-CREATETABLE-PARMS-GENERATED-STORED).* 
- **`pg_trgm`:** trigram similarity/GIN-GiST ops accelerate `%`, `LIKE/ILIKE`, regex-like and fuzzy matching on short text (names/titles); tunable thresholds; trusted extension in standard installs. *PG Docs: [pg_trgm](https://www.postgresql.org/docs/current/pgtrgm.html).*

## 3. Multilingual evidence synthesis (EN, pt-BR, es, de, fr, zh-CN, ja)

| Locale/lang intent | Practical PG guidance for 019 | Confidence / NV |
|---|---|---|
| **EN** | `'simple'` baseline is deterministic and works broadly; `english` stemming improves recall but must be fixed in generated column/index and may surprise exact-term users. 80/20: `'simple'` fixed. | High for mechanics; relevance quality workload-dependent (**NV**). |
| **pt-BR / es / de / fr** | Language dictionaries (`portuguese`, `spanish`, `german`, `french`) can help stemming/stopwords, but mixed-language org data and index/config coupling argue for fixed `'simple'` in 019; consider `unaccent` later for diacritics. `pg_trgm` helps typo/contains on names. | Medium; needs corpus eval (**NV-2**). |
| **zh-CN / ja** | Default `tsvector` parser/tokenization is weak for CJK/JP word boundaries; `pg_trgm` may help substring-like matching for short fields but is not full linguistic segmentation. For message bodies in zh-CN/ja, PG-only FTS quality is uncertain. | Low-medium; explicit **NV-2/NV-6**. |
| **Cross-locale ops (all)** | Cheap VPS favors DB-only, transactional consistency, no extra daemons; cap limits/work_mem/index footprint; avoid dual-write in MVP. | High (operational principle), sizing unmeasured (**NV-1**). |

**Takeaway:** 019 should ship a **locale-stable, deterministic `'simple'` FTS + tenant-safe GIN** baseline (works acceptably for Latin-script/mixed short queries) and add **optional `pg_trgm` on `files.name`/`tasks.title`** for fuzzy/contains UX; treat zh-CN/ja quality and per-locale dictionaries as measured follow-ups.

## 4. Concrete 80/20 recommendations for 019

### 4.1 Schema / migration (additive, sqlx)

Create one additive migration (e.g. `flora-core/src/migrations/sql/20261117000003_add_postgres_search.sql`; exact version/name per repo convention) — safe for existing rows:

1. **Extensions (guarded, optional by privilege):**
   - Prefer `CREATE EXTENSION IF NOT EXISTS pg_trgm;` in a privileged deploy path. For constrained managed PG, make extension creation an ops prerequisite and keep migration resilient (documented). **NV-7** (privileges/image). Do **not** require `unaccent` in 019.
2. **Messages (required MVP):**
   - `ALTER TABLE messages ADD COLUMN IF NOT EXISTS search_tsv tsvector GENERATED ALWAYS AS (to_tsvector('simple', coalesce(content,''))) STORED;`
   - `CREATE INDEX IF NOT EXISTS idx_messages_search_tsv ON messages USING GIN (search_tsv) WHERE is_deleted = false;`
   - Rationale: DB-maintained on insert/update, tenant filtering reuses `organization_id` predicates + existing `(organization_id, created_at DESC)` index.
3. **Tasks (trivial extension, recommended):**
   - `search_tsv` generated from `coalesce(title,'') || ' ' || coalesce(description,'')` (`'simple'`), partial GIN `WHERE is_deleted = false`.
   - Optional trigram for title fuzziness: `CREATE INDEX IF NOT EXISTS idx_tasks_title_trgm ON tasks USING GIN (title gin_trgm_ops) WHERE is_deleted = false;` (only if `pg_trgm` available).
4. **Files (trivial extension, metadata/name only):**
   - `search_tsv` generated from `coalesce(name,'') || ' ' || coalesce(file_type,'')` (`'simple'`), partial GIN `WHERE is_deleted = false`.
   - Optional trigram: `idx_files_name_trgm` on `name gin_trgm_ops`.
5. **Migration hygiene:** use fixed config `'simple'` (deterministic, index-usable, independent of `default_text_search_config`). Prefer `IF NOT EXISTS`; `CREATE INDEX CONCURRENTLY` cannot run inside a transaction block in standard migration flows — keep transactional DDL simple in sqlx migration and document concurrent index build as ops alternative for very large tables (**NV**). On 4 GB Seed datasets this is usually fine in-maintenance.

**No separate outbox/search_documents table in 019 MVP** (keep 80/20; prior 018 docs already position outbox as follow-up).

### 4.2 Query design, ranking, snippets, parsing

Implement a Postgres search repository/query service (e.g. in `flora-core` or a `flora-search` PG module) with parameterized sqlx queries:

- **Tenant scoping (mandatory):** every subquery includes `organization_id = $1` and respects soft-delete (`is_deleted = false` where applicable). Never rely on client filters.
- **Type dispatch:** if `types` empty/None => search requested set `{message,task,file}` (or all supported in 019); else intersect with allowed set and query only selected entities via `UNION ALL` (each branch returns `id::text`, `'message'|'task'|'file'`, `snippet`, `rank`, `created_at`).
- **Parse user query (safe):**
  - `q_trim = q.trim()`; empty handled by route (existing behavior).
  - Build `tsquery` with fallback: `COALESCE(websearch_to_tsquery('simple', $q), plainto_tsquery('simple', $q))` in SQL, or try `websearch_to_tsquery` and if yields null/empty fallback to `plainto_tsquery`/`NULL` -> no matches. `websearch_to_tsquery` avoids syntax errors for raw input (PG docs). Use same fixed config `'simple'` as columns.
- **Match/order:**
  - `WHERE organization_id = $1 AND is_deleted = false AND search_tsv @@ $qts`
  - `ORDER BY ts_rank_cd(search_tsv, $qts, 1) DESC, created_at DESC, id DESC` (small normalization `1` = divide by 1+log(length); tune later) with `LIMIT $limit`.
  - Cap limit at API max (50). Consider a minimum rank threshold only if noise observed (**NV**).
- **Snippets:** `ts_headline('simple', <source_text>, $qts, 'MaxWords=20, MinWords=5, ShortWord=2, MaxFragments=1, StartSel=<<, StopSel=>>')` for messages/tasks description/title concat as appropriate; or return bounded plaintext snippet and let clients highlight later. For files, snippet can be `name` (+ `file_type`). `ts_headline` reads source text (slower) — apply after `LIMIT` in an outer query/subselect to cap cost on VPS.
- **Optional `pg_trgm` path (if extension/index present):** for `type=file`/`task` and short `q`, can augment or fallback with `name ILIKE '%'||$q||'%'` / `title ILIKE ...` or `%` similarity when FTS misses typos; keep deterministic ordering and document behavior. 80/20: implement as a **small additive fallback** (e.g. when FTS returns < limit) rather than complex scoring fusion.

**Permission note:** current search route already enforces org membership. 019 must **not** broaden visibility (e.g. private-channel message content) beyond what the system intends; if channel ACLs are not yet enforced in search, document as **NV/risk** and keep org boundary strict.

### 4.3 API / service integration & fallback when Meilisearch absent

**Goal:** Postgres becomes default, Meilisearch optional.

1. **Backend selection (`auto` semantics, 80/20):**
   - If request can be served by a healthy preferred external backend (optional), use it; else use **Postgres backend** (always available when DB up) as default.
   - If Meilisearch is configured/enabled but unavailable, **degrade to Postgres** (log warn/metric) rather than 503, preserving availability on 4 GB single-node deploys.
   - Return 503 only if neither PG nor external backend is usable (e.g. DB down — already an infrastructure failure).
2. **State/wiring:**
   - Introduce a small search abstraction in `flora-search` or `flora-core` (`trait SearchBackend { async fn search(...) -> Result<Vec<SearchResult>> }`) and adapters: `PostgresSearchBackend` (sqlx pool/repos/queries), `MeilisearchBackend` (wrap existing), optional `Composite/Auto`.
   - `AppState.search_service` (or renamed `search_backend`) holds Postgres-capable backend by default; keep `SearchResult/SearchResponse` contract identical to route.
   - `flora-core/src/config/search.rs`: additive fields for provider/backend, keep Meilisearch fields for compatibility; validation: do not require Meilisearch URL when provider is `postgres/auto` and PG is default.
3. **Route behavior:** keep membership check, empty-query short-circuit, limit/type parsing/capping. No contract break for clients: same JSON keys and semantics (`results` possibly from PG). Document that snippets/highlighting may differ from Meilisearch (**expected**).
4. **Indexing hooks:** not required for generated-column FTS correctness. Keep `flora-search` Meilisearch `index_*` methods out of critical path in 019; if Meilisearch is enabled, dual-write/backfill is **defer** (risk of drift) — prefer rebuild job later.

### 4.4 Indexing/perf for small VPS (~2 vCPU / 4 GB, PG18)

- **Keep footprint minimal:** partial GIN on `is_deleted = false`, only `messages` mandatory + `tasks/files` text fields if cheap; avoid extra trgm indexes unless needed (each adds write/disk cost).
- **Query discipline:** always `organization_id` filter, `LIMIT` <= 50, snippet in outer query after ranking/limit, avoid leading-wildcard-only plans except trgm fallback.
- **PG tuning (starting points, validate):** modest `shared_buffers` (~256–512 MB), small `work_mem` (2–8 MB) to limit sort/hash memory per connection, conservative `max_connections` + app pool sizing, `maintenance_work_mem` sized for occasional index builds. Use `EXPLAIN (ANALYZE, BUFFERS)` on staging data, not assumptions.
- **Write amplification:** generated `tsvector` updates on content/title/name changes cost CPU; acceptable for SMB. Batch backfills (if any manual step) off-peak.
- **Capabilities/observability:** report effective search backend in logs/health/capability endpoint when available; track p95 latency and DB load.

### 4.5 DB-gated testing patterns (019)

Align with existing `flora-tests` patterns (see `contract/{auth,files,messaging,websocket}.rs`):

1. **Gating:** read `FLORA_DATABASE__POSTGRES_URL` or `DATABASE_URL`; `if url missing { eprintln!("skipping..."); return Ok(()); }` so CI without DB stays green. Use `#[tokio::test]` (or serial grouping only if needed).
2. **Setup:** `PgPool::connect(&url).await?`; apply migrations (`sqlx::migrate!` from core or project’s migration path used in tests) so new `search_tsv` columns/indexes exist. Prefer unique seeded entities per test (UUIDv7 org/user/channel/message/task/file IDs) to avoid cross-test interference.
3. **Fixtures:** insert minimal tenant graph required by auth/membership and target rows (org, user, membership/role as needed, channel+message or workspace+task/file). Reuse helpers/builders if present; otherwise local helpers in test module.
4. **Deterministic assertions (recommended for 019):**
   - **Message FTS:** insert message in org A, search `q` matching token -> result contains message id/type/snippet; edit content -> new term matches, old removed; soft-delete -> excluded.
   - **Tenant isolation:** same term in org B not returned for org A.
   - **Type filter:** `type=message` excludes task/file; invalid/unknown types yield empty per chosen contract (lock behavior).
   - **Parsing safety:** `websearch_to_tsquery`-style inputs (`"quoted phrase"`, `-term`, `OR`, punctuation) do not error; empty/stopword-only query -> empty/200 per contract.
   - **Fallback/degradation:** when Meilisearch unavailable/disabled, endpoint serves via PG (not 503) — test via app/router state configured with PG backend.
   - **Optional trgm (if enabled):** `name`/`title` ILIKE/typo-like case finds file/task.
5. **Test placement:** prefer `flora-tests/src/integration/search.rs` (and/or `contract/search.rs`) with real DB; keep pure unit tests for parser/helper logic near implementation.
6. **Do not require DB for all crates:** unit/contract-non-DB checks remain `connect_lazy` or pure where appropriate.

## 5. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Tenant/ACL leakage in PG queries | Critical | Mandatory `organization_id` in all branches; membership gate remains; add cross-org negative DB tests; treat channel-level visibility as explicit follow-up if absent. |
| Migration/extension privileges (`pg_trgm`, `CONCURRENTLY`) | Deploy friction | `IF NOT EXISTS`; make trgm optional in 019; document ops path for managed PG and concurrent index builds. |
| Relevance/tokenization gaps (pt-BR/es/de/fr stemming, zh-CN/ja segmentation) | Poor UX | Fixed `'simple'` baseline + trgm for names; measure on real corpora; locale-specific configs/hybrid Meilisearch later. |
| `ts_headline`/ranking cost on VPS | Latency/CPU | Headline after limit, cap results, avoid per-row heavy work on large sets, monitor p95. |
| Config/API compatibility drift | Client/ops breakage | Preserve endpoint/response; additive config; `auto` prefers availability (PG fallback). |
| Meilisearch/PG divergence if dual-used later | Stale results | 019: no dual-write; single authoritative backend path (PG default). |
| Search over sensitive/private entities | Data exposure | Enforce same authz boundaries as list/read APIs; tests for forbidden cases. |

## 6. NVs (needs verification)

- **NV-1:** p95 latency, RAM, disk/write amplification for generated `tsvector` + GIN (+trgm) on 2 vCPU/4 GB with realistic message/task/file volumes and concurrency.
- **NV-2:** Relevance/snippet quality per locale (**EN, pt-BR, es, de, fr, zh-CN, ja**), `'simple'` vs language configs, and `unaccent` benefit; zh-CN/ja may require hybrid/specialized tooling.
- **NV-3:** Final backend selection/config names and health/capability signaling to clients/ops (backward compat with existing `search.*`).
- **NV-4:** Whether tasks/files must ship in 019-S1 or messages-only first (scope vs value).
- **NV-5:** Channel/workspace-level authorization rules for search results (beyond org membership) given current route checks.
- **NV-6:** `pg_trgm` availability/privileges and whether fuzzy fallback is needed in MVP.
- **NV-7:** sqlx migration transactional constraints vs `CREATE INDEX CONCURRENTLY` operational procedure on production-sized data.

## 7. Concrete 019 delivery checklist (smallest verifiable slice)

1. Add additive migration: `messages.search_tsv` + GIN (and optionally tasks/files `search_tsv` + trgm indexes).
2. Implement PG search query path (`websearch_to_tsquery` -> `plainto_tsquery` fallback, tenant/type filters, rank, capped snippet) returning `SearchResult`.
3. Wire backend selection so PG serves by default / on Meilisearch absence; preserve `GET /api/v1/search` contract.
4. Add env-gated DB integration tests: indexing-on-write via generated column, tenant isolation, type filter, safe parsing, PG fallback (no 503).
5. Document config/ops notes and NVs; defer outbox, dual-write, CJK-specific tokenization, and permission-model expansion.

## 8. Primary sources

- PostgreSQL 18, Full Text Search — Tables and Indexes: https://www.postgresql.org/docs/current/textsearch-tables.html
- PostgreSQL 18, Controlling Text Search (parsing, ranking, highlighting): https://www.postgresql.org/docs/current/textsearch-controls.html
- PostgreSQL 18, `pg_trgm`: https://www.postgresql.org/docs/current/pgtrgm.html
- PostgreSQL 18, `CREATE TABLE` (generated stored columns): https://www.postgresql.org/docs/current/sql-createtable.html
- PostgreSQL 18, `SELECT` locking (`FOR UPDATE` / `SKIP LOCKED`, for future outbox patterns): https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE

## 9. Repo references (cross-check)

- `OBJETIVO.md`, `ROADMAP.md`
- Prior: `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`, `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`
- Code: `flora-api/src/routes/search.rs`, `flora-api/src/state/app_state.rs`, `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`, `flora-core/src/services/{mod,message,task,file,notification}_service.rs`, `flora-core/src/models/{message,task,file,notification}.rs`, `flora-core/src/repositories/{message,task,file,notification}_repository.rs`
- Schema: `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `flora-core/src/migrations/sql/20261116000002_add_workspace_id_to_files.sql`
- Tests: `flora-tests/src/contract/search.rs`, `flora-tests/src/integration/search.rs`, DB-gated patterns in `flora-tests/src/contract/{auth,files,messaging,websocket}.rs`
