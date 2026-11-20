# 018 — Postgres-native search and lightweight eventing/outbox for Flora on cheap VPS (multilingual)

**Date:** 2026-11-16  
**Repo:** `flora` (read-only research; this document only)  
**Scope:** 80/20, Postgres-first design for **~2 vCPU / 4 GB RAM / SSD, single-node**: PostgreSQL FTS (`tsvector`/`tsquery`, GIN, generated columns), `pg_trgm`, hybrid with optional Meilisearch, and simple outbox/transactional hooks for indexing + in-app notifications in Rust/sqlx services.  
**Method:** Code/schema/config cross-check + primary PostgreSQL documentation. Multilingual query intent covered for **EN, pt-BR, es, de, fr, zh-CN, ja** (no authoritative locale-specific 4 GB benchmarks found; operational sizing is NV unless measured).

## 1. Flora current state (cross-check)

| Area | Evidence | Gap / implication |
|---|---|---|
| Search impl | `flora-search` uses Meilisearch only (`SearchService`: per-org index template, `index_message/task/file`, `search/delete_document`). `flora-core/src/config/search.rs` defaults `enabled=true`, requires URL when enabled. `flora-api/src/state/app_state.rs` now gates init on `config.search.enabled` and degrades to `search_service: None` on init failure; `GET /api/v1/search` (`flora-api/src/routes/search.rs`) returns 503/unavailable when `None`, preserves `{query, results[{type,id,snippet}]}` contract. No DB search backend. | Direction matches “optional external search”, but **no Postgres FTS path** yet (P0 gap in `OBJETIVO.md`). Boot/contract improved vs older audits; capability must stay explicit. |
| Searchable core data | `messages` (`content`, `organization_id`, `channel_id`, `sender_id`, `thread_id`, soft-delete flags), `tasks` (`title`, `description`, `workspace_id`, `organization_id`, `status`, `assignee_id`, soft-delete), `files` (`name`, `file_type`, metadata, `organization_id`, `workspace_id` added in later migration), all indexed by org/created/lookup keys in `20240818000001_init_schema.sql`. No `tsvector`/GIN/`pg_trgm` in migrations scanned. | Best 80/20 targets for “basic search integrated with core objects”. |
| Notifications | `notifications` table exists with org/user/read/target indexes and RLS enabled. `flora-notifications::NotificationService` writes row then `XADD notifications:org:{org_id}` to Valkey (coupled to Redis). `flora-core` has `PgNotificationRepository` + minimal `services::notification_service::NotificationService` (placeholder). API reads/marks via repo. No unified in-app notification dispatch abstraction across crates. | Two notification paths (core repo vs notifications crate) and **no DB outbox**; Valkey publish is best-effort/infra-coupled and can diverge from committed state. |
| Eventing/index hooks | No `index_*` call sites found outside search crate/tests/docs; no outbox/job table; messaging is optional Valkey Pub/Sub in app state. | Indexing + notifications lack transactional, replayable hooks. |

## 2. Primary-source foundations (what to rely on)

- **FTS indexing/querying:** PostgreSQL recommends `to_tsvector`/`to_tsquery` (`@@`) with GIN (or GiST) indexes; expression indexes require fixed text-search config; stored/generated `tsvector` column + GIN index enables fast queries and can avoid re-computing `to_tsvector` at query time and/or config coupling in queries. Prefer `websearch_to_tsquery`/`plainto_tsquery` for end-user input; `ts_headline` for snippets. [PG FTS tables/indexes](https://www.postgresql.org/docs/current/textsearch-tables.html), [FTS intro/controls](https://www.postgresql.org/docs/current/textsearch.html)
- **Generated columns:** `GENERATED ALWAYS AS (...) STORED` materializes expression results on write and can be indexed; good fit for `tsvector` search documents. [CREATE TABLE generated columns](https://www.postgresql.org/docs/current/sql-createtable.html#SQL-CREATETABLE-PARMS-GENERATED-STORED)
- **`pg_trgm`:** trigram similarity and GIN/GiST ops accelerate `%`, similarity, and `LIKE`/`ILIKE`/regex-like pattern searches; useful for fuzzy name/title matching and misspelling tolerance without a separate engine. Extension is trusted-installable where available. [pg_trgm](https://www.postgresql.org/docs/current/pgtrgm.html)
- **Transactional claim/dispatch:** `SELECT ... FOR UPDATE SKIP LOCKED` lets workers claim queued rows without blocking each other, enabling simple concurrent pollers. [SELECT locking clause](https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE)

## 3. Concrete 80/20 recommendations for Flora

### 3.1 Schema: Postgres-native search (default backend)

**Goal:** tenant-safe, transactional, low-ops search for `message|task|file` with current API contract.

**Recommended MVP (single migration, additive):**
1. **Enable extensions (if permitted):** `CREATE EXTENSION IF NOT EXISTS pg_trgm;` optionally `unaccent` later (NV by image/permissions/locales).
2. **Choose representation:** start with **per-entity stored `tsvector` + GIN** (least abstraction) or a thin **`search_documents` projection** for uniform ranking/snippets. 80/20 favors per-entity columns first to avoid sync logic:
   - `messages.search_tsv` `tsvector GENERATED ALWAYS AS (to_tsvector('simple', coalesce(content,''))) STORED`
   - `tasks.search_tsv` generated from `title || ' ' || coalesce(description,'')`
   - `files.search_tsv` generated from `name || ' ' || coalesce(file_type,'')` (metadata text later if needed)
3. **Indexes:**
   - `CREATE INDEX CONCURRENTLY idx_messages_search_tsv ON messages USING GIN (search_tsv) WHERE is_deleted = false;` (and analogous filtered indexes for tasks/files) — create non-concurrently inside transactional migrations if required by migration tooling; prefer CONCURRENTLY in ops runbooks.
   - Keep/ensure tenant/time filter support: existing `(organization_id, created_at DESC)` and entity FKs remain primary filters.
4. **Optional fuzzy boost (defer until needed):** `GIN (name gin_trgm_ops)` or `title` trigram indexes for “contains/typo-ish” UX, especially for files/tasks titles.
5. **Language/config:** default to `'simple'` or a fixed config for deterministic indexes (expression/generated indexes must be stable vs `default_text_search_config` per PG docs). Per-locale configs (`portuguese`, `spanish`, `german`, `french`, etc.) are a Phase 2 NV (zh/ja tokenization often needs non-default tooling).

**Why this fits 2 vCPU/4 GB:** no extra daemon, no sync pipeline, search index lifecycle tied to DB backup/restore, and storage/CPU cost paid incrementally on writes.

### 3.2 API and Rust/sqlx service design

1. **Backend abstraction (small, explicit):**
   - Introduce `trait SearchBackend: Send + Sync` with `search(org_id, query, types, limit)` and optional index/delete hooks.
   - Implementations: `PostgresSearchBackend` (default in Seed), `MeilisearchSearchBackend` (adapter over existing `flora-search`), `NoopSearchBackend`.
   - `AppState.search_service` becomes backend-typed optional/capability-aware; keep route response stable.
2. **Query SQL (80/20):**
   - Build per-type `UNION ALL` subqueries or dispatch by requested types; always `WHERE organization_id = $1 AND (is_deleted = false where applicable)`.
   - Rank: `ts_rank_cd(search_tsv, query)` or `ts_rank`; order by rank desc, `created_at` desc; `LIMIT $n` (cap 50 as today).
   - Query parser: `websearch_to_tsquery('simple', $q)` fallback to `plainto_tsquery` on syntax/empty parse.
   - Snippet: `ts_headline('simple', source_text, query, 'MaxFragments=1, MaxWords=20, MinWords=5')` or bounded substring fallback.
3. **Optional hybrid:** if `search.provider=meilisearch` and healthy, delegate; on request failure **do not** fail boot but return degraded error/capability (policy) and allow operator fallback to Postgres backend in config. Do not dual-write in MVP.
4. **sqlx notes:** use runtime queries or checked queries with offline cache per repo practice; keep statements tenant-scoped and parameterized; add indexes only with measured justification on VPS.

### 3.3 Lightweight outbox / transactional hooks (indexing + in-app notifications)

**Principle:** write business state and side-effect intent in the **same DB transaction**; deliver asynchronously with idempotent consumers.

**Recommended schema (`domain_outbox` or `event_outbox`), additive:**
- `id UUID PK` (v7), `organization_id UUID NULLABLE/required by event`, `aggregate_type TEXT`, `aggregate_id UUID`, `event_type TEXT`
- `payload JSONB NOT NULL` (minimal: ids, actor, target, search document fields or refs)
- `status TEXT` (`pending|processing|done|dead`) or `available_at/processed_at`
- `attempts INT DEFAULT 0`, `next_attempt_at TIMESTAMPTZ`, `last_error TEXT NULL`, `created_at TIMESTAMPTZ`
- Indexes: `(status, next_attempt_at, created_at)` for claim scans; `(organization_id, created_at)` for ops; optional unique dedupe key `(aggregate_type, aggregate_id, event_type, dedupe_hash)` if needed.

**Producer (transactional hook, 80/20):**
- In service/repository methods that mutate messages/tasks/files or create notification-worthy events, accept/propagate a transaction (`sqlx::Transaction<'_, Postgres>` or executor abstraction) and insert outbox row(s) in the same tx.
- Events to start: `search.upsert_message/task/file`, `search.delete_*`, `notification.created` (or derive notification insert + enqueue in one tx).
- Keep payloads small (IDs + canonical fields) to avoid large rows and simplify retries.

**Dispatcher/consumer (simple, cheap):**
- **Single-node in-process poller** (tokio task in `flora-api` or separate lightweight worker binary later): every N ms/seconds claim batch:
  - `BEGIN; SELECT ... FROM outbox WHERE status='pending' AND next_attempt_at <= now() ORDER BY created_at LIMIT $batch FOR UPDATE SKIP LOCKED; mark processing; COMMIT;`
  - handle each event idempotently (upsert/delete search projection or Postgres FTS source already current; send in-app notification), then mark `done` (or reschedule with backoff, cap attempts -> `dead`).
- **Idempotency:** `ON CONFLICT DO UPDATE`/delete-by-id for search side effects; notification delivery keyed by notification/event id to avoid duplicates on retry.
- **In-app notifications (80/20):** make **Postgres the source of truth** (insert `notifications` row transactionally). Optional real-time push: best-effort publish to existing Valkey stream or in-process realtime hub **after commit** (or via outbox consumer) — loss of push must be recoverable by polling `GET /api/v1/notifications` (cursor/since-id can be added later; NV).

**What to defer:** full PGMQ pipeline, exactly-once distributed consumers, external brokers, LISTEN/NOTIFY dependency for correctness (can be a wake-up hint only), Meilisearch sync workers beyond optional rebuild tooling.

### 3.4 Hybrid with optional Meilisearch (when to enable)

- **Stay on Postgres-only** for Seed/PME: basic keyword search, low write/read volume, strict 4 GB budget, desire for single backup story.
- **Enable Meilisearch profile** only if product needs justify: instant/typeahead UX, strong typo tolerance, multilingual ranking/tokenization quality beyond PG config, facets, or measured PG relevance/latency gaps. Run with pinned version, memory/CPU limits, localhost binding, rebuild-from-Postgres job, and health/capability reporting.
- **Keep contracts stable:** same search endpoint/response; backend selected by config/capability.

### 3.5 Perf/ops guidance for ~2 vCPU / 4 GB (NV unless benchmarked)

| Area | 80/20 guidance |
|---|---|
| Process budget | Reserve headroom for OS/page cache; keep core to `flora-api` + Postgres. Add search engine/Valkey only via explicit profiles with caps; expect pressure if all enabled. |
| Postgres sizing (starting point) | Conservative: `shared_buffers` ~256–512 MB, `work_mem` small (2–8 MB) to avoid per-sort/hash blowups, `maintenance_work_mem` modest for index builds, connection cap low (pool sized accordingly). Tune with `pg_stat_statements`/EXPLAIN on real data. |
| FTS/query perf | Always filter `organization_id` (+ workspace/channel/type) first; cap limits; avoid unbounded `OR`-heavy `ts_rank` on huge result sets; prefer selective filters and pagination. Use generated `tsvector` to avoid per-row parsing in selects. |
| Index/write cost | GIN/trgm indexes add write amplification and disk; index only needed columns and consider partial indexes (`WHERE is_deleted = false`); batch backfills off-peak. |
| Outbox worker | Small batch (e.g. 10–50), short tx for claim/update, exponential backoff, bounded concurrency (1 worker on 2 vCPU), metrics/logs for lag/dead letters. Prevent hot polling (adaptive sleep when empty). |
| Backups/DR | DB backups include FTS columns/index definitions and outbox/notifications; validate restore. If Meilisearch enabled, treat its data as rebuildable cache unless separately backed up. |
| Observability | Expose readiness/capabilities (db, search backend, valkey), outbox depth/oldest age, search latency/error counts; alert on sustained lag. |

### 3.6 Multilingual implementation notes (EN, pt-BR, es, de, fr, zh-CN, ja)

- **EN/pt-BR/es/de/fr:** Postgres dictionaries/stemming configs can improve FTS recall but require explicit config choice and index consistency; `'simple'` is safest cross-locale baseline. `unaccent` (optional) can help diacritics-heavy queries.
- **zh-CN/ja:** default `tsvector` tokenization is often poor for CJK/JP word boundaries; `pg_trgm` may help substring-like matching in some cases but is not a full linguistic solution. If these locales are primary, validate quality early and treat Meilisearch (or specialized PG extensions/tooling) as a **targeted hybrid** (NV).
- **API/UX:** accept Unicode queries, normalize case/whitespace, keep snippets safe/truncated, and return stable type/id fields regardless of locale.

## 4. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Tenant leakage in search SQL/backend | Critical data exposure | Mandatory `organization_id` predicate in every query; tests per entity/type; avoid cross-org caches. |
| Index/write amplification on small VPS | Latency spikes, disk growth | Minimal columns/indexes, partial indexes, measure write rates, maintenance windows. |
| Poor relevance/tokenization for some languages (esp. zh-CN/ja) | Bad UX, false confidence in PG-only | Start with `'simple'` + explicit NV; locale-driven evaluation; enable hybrid Meilisearch selectively. |
| Outbox lag or dead letters | Stale search, missed push side effects | SKIP LOCKED worker, backoff/retry cap, DLQ/`dead` status, lag metrics, replay/runbook. |
| Dual-path notifications (core repo vs `flora-notifications` crate/Valkey) | Inconsistency/duplicate logic | Converge on one transactional create path + outbox-driven delivery hooks. |
| Backend/config drift (`search.enabled`, provider, env naming) | Ops confusion, accidental dependency | Single capability model in config/state/health/docs; explicit profiles. |
| Migration/tooling constraints | `CREATE INDEX CONCURRENTLY`, extension perms | Provide safe migration + ops alternative; document privileges. |

## 5. NVs (needs verification)

- **NV-1:** Real p95 latency/QPS and RAM/disk impact of FTS+trgm indexes on Flora’s actual dataset at 2 vCPU/4 GB.
- **NV-2:** Best config per locale (`simple` vs language dictionaries vs `unaccent`) for pt-BR/es/de/fr and feasibility for zh-CN/ja without extra extensions/services.
- **NV-3:** Unified projection table vs per-entity generated columns given evolving schema (`files.workspace_id`, future entities/permissions/channel scoping).
- **NV-4:** Exact transactional boundaries in current Rust services/repositories (executor/tx propagation) for inserting outbox rows atomically with domain writes.
- **NV-5:** Whether in-app notifications should standardize on `flora-core` repo path or `flora-notifications` crate and how Valkey/realtime hub should be driven post-commit.
- **NV-6:** Meilisearch relevance/features still required by any active client beyond `q/type/limit` and rebuild/ops cost acceptability on target VPS.
- **NV-7:** Extension availability/privileges (`pg_trgm`, `unaccent`) in managed/self-hosted Postgres images used by deployers.

## 6. Sources

**Primary (PostgreSQL):**
- [S1] Full Text Search — Tables and Indexes: https://www.postgresql.org/docs/current/textsearch-tables.html
- [S2] Full Text Search (chapter): https://www.postgresql.org/docs/current/textsearch.html
- [S3] `pg_trgm`: https://www.postgresql.org/docs/current/pgtrgm.html
- [S4] `CREATE TABLE` (generated columns, indexes/constraints context): https://www.postgresql.org/docs/current/sql-createtable.html
- [S5] `SELECT` locking (`FOR UPDATE` / `SKIP LOCKED`): https://www.postgresql.org/docs/current/sql-select.html#SQL-FOR-UPDATE-SHARE

**Repo/internal (evidence):**
- [F1] `OBJETIVO.md`, `ROADMAP.md`
- [F2] `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`
- [F3] `flora-api/src/state/app_state.rs`, `flora-api/src/routes/search.rs`, `flora-api/src/routes/notifications.rs`
- [F4] `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `20261116000002_add_workspace_id_to_files.sql`
- [F5] `flora-notifications/src/lib.rs`, `flora-core/src/models/notification.rs`, `flora-core/src/repositories/notification_repository.rs`
- [F6] `docs/research/2026-11-16-016-search-postgres-fts-vs-meilisearch.md`, `docs/research/2026-11-16-016-operability-codebase-audit.md`
