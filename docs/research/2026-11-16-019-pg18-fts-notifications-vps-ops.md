# 019 — PostgreSQL 18 FTS (GIN/tsvector) + lightweight in-app notification writes on cheap VPS (Seed ops)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`3c8eddbfd`) — **research only** (no source/commit changes).  
**Scope:** Practical 80/20 guidance for running PostgreSQL 18 full-text search (stored `tsvector`, generated columns, GIN) and lightweight in-app notification writes on a single-node VPS (**~2 vCPU / 4 GB RAM / SSD**), aligned with Flora **Seed** profile (Rust + Postgres, optional Valkey/Meilisearch, small SMB workloads, low initial concurrency).  
**Cross-check (repo evidence):** `OBJETIVO.md`/`ROADMAP.md` (Seed = PG-first, MVP P0), `.env.example`/`docs/dev-smoke.md` (PG18+ for `uuidv7()`), `flora-core/src/database.rs` (pool hardcoded `max_connections(20)`; config fields not fully wired — NV/op risk), `flora-api/src/state/app_state.rs` (search degrade behavior), `flora-core/src/migrations/sql/20240818000001_init_schema.sql` (core tables incl. `notifications` indexes/RLS; no FTS indexes today), prior research `2026-11-16-seed-vps-profile-audit.md`, `2026-11-16-016-postgres-vps-backup-tuning.md`, `2026-11-16-018-postgres-search-outbox-vps.md`, `2026-11-16-019-inapp-notification-wiring-vps.md`.  
**Method:** primary PostgreSQL 18 documentation + targeted multilingual operator/practitioner sources (**EN, pt-BR, es, de, fr, zh-CN, ja**) for practical VPS/FTS patterns. No runtime benchmarks in this repo; all sizing is **starting-point guidance** to be validated with `EXPLAIN (ANALYZE, BUFFERS)`, `pg_stat_*`, logs, and `docker stats` (**NV**).

## 1. Executive 80/20 (Seed-safe defaults)

| Goal | Recommendation | Why (80/20) |
|---|---|---|
| Keep DB alive on 4 GB | Conservative memory, low connections, no global `work_mem` inflation | `work_mem` is per sort/hash op and multiplies by ops × sessions; OOM/latency spikes hurt SMB uptime more than marginal speedups. [P1][P6] |
| Postgres-native search | Stored generated `tsvector` + GIN, fixed text-search config, `websearch_to_tsquery`, hard `LIMIT` and tenant filters | Transactional, single-backup-story, no extra daemon; matches Seed “PG-first”. [P2][P3][P4] |
| Low-risk migrations/indexing | Add columns/indexes deliberately; prefer `CREATE INDEX CONCURRENTLY` in live ops runbooks (outside tx block), budget `maintenance_work_mem` per-session, avoid peak | GIN builds are sensitive to maintenance memory and can be write-heavy/long on small disks. [P4][P5][P7] |
| Bloat/autovacuum hygiene | Keep autovacuum on, monitor dead tuples and GIN pending-list behavior, size `autovacuum_work_mem` separately if raising maintenance memory | GIN pending-list cleanup and vacuum use memory/settings; default thresholds can lag on churn. [P1][P5][P8] |
| Notifications | **Postgres is source of truth** (small transactional insert), best-effort/non-fatal side effects, indexed reads, avoid chatty/duplicate writes | Durable inbox survives restarts; cheap and operable on VPS; aligns with 019 wiring research. [F6][F7] |

## 2. Memory & config hints (2 vCPU / 4 GB, co-located Seed)

**Budget assumption:** same VPS may run `flora-api` + Postgres (+ optional Valkey/Meilisearch). Reserve headroom for OS/page cache; on 4 GB, avoid stacking all optional services by default (see Seed audit). [F1][F3]

| Parameter | 80/20 start (shared 4 GB) | DB-heavier 4 GB ceiling | Notes / risks |
|---|---:|---:|---|
| `shared_buffers` | `512MB` | `1GB` | PG suggests ~25% on dedicated DB; >~40% unlikely to help because OS cache is also used. Restart required. [P1] |
| `effective_cache_size` | `1536MB` | `3GB` | **Planner hint only** (no allocation). Set to realistic cacheable RAM, not a promise. [P1] |
| `work_mem` | `4MB` (try `8MB` only if measured) | `8–16MB` with strict concurrency/pooling | Per sort/hash node, can multiply per query and per connection; hash ops may use more via `hash_mem_multiplier` (default 2.0 in PG18). Prefer per-role/per-tx overrides for rare heavy queries. [P1][P6] |
| `maintenance_work_mem` | `128MB` | `256MB` | Helps `VACUUM`/`CREATE INDEX`; GIN build time is very sensitive to it, but each autovacuum worker can consume up to it (or `autovacuum_work_mem`). Keep bounded on 4 GB. [P1][P4][P5] |
| `autovacuum_work_mem` | `-1` or `64–128MB` when maintenance raised | `128MB` | Set explicitly if `maintenance_work_mem` is increased, to cap unattended worker memory (`autovacuum_max_workers × ...`). [P1] |
| `max_connections` (Postgres) | `30` | `40` | Keep low; each backend has overhead. Pair with small app pool and/or PgBouncer only if needed. [P1][F3] |
| App pool (`flora-core`) | `5–10` target (wire config) | `10–15` max only with evidence | **Repo reality:** pool currently hardcoded to 20 in `database.rs` while config exposes fields — fix/wire in code is a low-risk operability action (NV). [F2][F3] |
| `random_page_cost` | `1.1` | `1.1` | Common SSD/NVMe planner hint; verify plans. [P1] |
| `checkpoint_timeout` / `checkpoint_completion_target` | `15min` / `0.9` | same | Spread checkpoint I/O on small disks. [P1][F3] |
| `max_wal_size` / `min_wal_size` | `1GB` / `256MB` | `2GB` / `512MB` (disk headroom required) | Tradeoff: fewer checkpoints vs WAL disk/recovery window. [P1][F3] |
| `statement_timeout` (app/search role) | `2–5s` for interactive search/API (NV) | workload-specific | Prevent pathological FTS/rank/headline queries from exhausting a tiny VPS. Set per role/session, not necessarily global for admin tasks. [P6] |

**Apply safely:** prefer Compose/postgres args or mounted config, change incrementally, restart where required (`shared_buffers`, `max_connections`), and verify with `SHOW ...`, `pg_stat_activity`, temp-file/checkpoint/autovacuum stats. Example (illustrative):  
`postgres -c shared_buffers=512MB -c effective_cache_size=1536MB -c work_mem=4MB -c maintenance_work_mem=128MB -c autovacuum_work_mem=64MB -c max_connections=30 -c random_page_cost=1.1 -c checkpoint_timeout=15min -c checkpoint_completion_target=0.9 -c max_wal_size=1GB -c min_wal_size=256MB`

## 3. PostgreSQL 18 FTS: schema, indexing, and query patterns

### 3.1 Recommended schema (80/20, Seed)

- **Use stored generated `tsvector` columns** for searchable core entities (start with existing high-value text: `messages.content`, `tasks.title/description`, `files.name/file_type`) and a **GIN index** on each search column. Generated columns keep vectors in sync automatically and are indexable. [P2][P3][P4]
- **Always pass an explicit text-search configuration** in `to_tsvector`/`to_tsquery` (e.g. `'simple'` baseline, or a chosen language config). Expression indexes and deterministic behavior require fixed config (not session-dependent `default_text_search_config`); stored generated columns with fixed config also make query/index matching straightforward. [P2][P3]
- **Coalesce/null-safety and weights:** `setweight(to_tsvector(cfg, coalesce(title,'')), 'A') || ...` so NULL fields do not null out the document and title can outrank body. [P2]
- **Query parser for end-user input:** prefer `websearch_to_tsquery(cfg, $q)` (or `plainto_tsquery`) over raw `to_tsquery` to avoid syntax errors on free text; rank with `ts_rank_cd`/`ts_rank` and generate snippets with `ts_headline` only for returned rows. [P2]
- **Tenant/selectivity first:** every query must filter `organization_id` (and channel/workspace/type/deleted flags where applicable) and `ORDER BY rank DESC, created_at DESC LIMIT $n` with a strict cap (e.g. ≤ 50). [F4][P2]
- **GIN is preferred for FTS**; GiST is lossy/special-case. GIN stores lexemes (not weight labels in the same way), so weighted ranking may involve rechecks depending on query — keep result sets small. [P4]

**Example pattern (conceptual):**
```sql
ALTER TABLE messages
  ADD COLUMN search_tsv tsvector
  GENERATED ALWAYS AS (to_tsvector('simple', coalesce(content, ''))) STORED;

CREATE INDEX CONCURRENTLY idx_messages_search_tsv
  ON messages USING GIN (search_tsv)
  WHERE is_deleted = false;

-- query (always tenant-scoped + limited)
SELECT id, ts_rank_cd(search_tsv, q) AS rank
FROM messages, websearch_to_tsquery('simple', $1) AS q
WHERE organization_id = $2 AND is_deleted = false AND search_tsv @@ q
ORDER BY rank DESC, created_at DESC
LIMIT 20;
```

### 3.2 Multilingual / locale notes (EN, pt-BR, es, de, fr, zh-CN, ja)

| Locale intent | 80/20 guidance | Evidence / NV |
|---|---|---|
| EN | `english` or `'simple'` depending on stemming needs; fixed config in index+query. | Primary PG FTS behavior/config model. [P2][P3] |
| pt-BR | `'portuguese'` config can stem/stop-word (e.g. Runstack examples show `to_tsvector('portuguese', ...)`, `websearch_to_tsquery`, `ts_headline`); validate stemming against real product vocabulary. | Practitioner pt-BR: Runstack (`postgres-full-text-search`, 2026-06-08). [S-ptBR-1] NV for Flora corpus quality. |
| es / de / fr | Use corresponding built-in configs when beneficial, but keep **one fixed config per indexed column/expression** (or a `config_name` column pattern per PG docs) to avoid mismatch; benchmark relevance. | Primary config/index constraint [P2]; multilingual practice references in EN sources. [S-multi-1] NV. |
| zh-CN / ja | Default FTS tokenization is often weak for CJK/JP boundaries; `'simple'` + `pg_trgm` may help substring cases but is not a full linguistic solution. Treat quality as **NV**; consider external engine/hybrid only with proven need. | Prior Flora 018 NV; general FTS limitation (no primary locale-specific claim here). [F4] |
| PG18 upgrade note | PG18 release notes include text-search collation-provider behavior changes in some upgrade paths; **reindex affected FTS/`pg_trgm` indexes after major upgrade when recommended** for your cluster/provider. | pt-BR practitioner note (Urhoba, 2026-09-24) citing PG18 notes context; verify against official PG18 release/upgrade docs for deployed path. [S-ptBR-2][P2] **NV**. |

## 4. Migration and index-build impact (VPS-safe)

| Topic | Practical guidance |
|---|---|
| `ADD COLUMN ... GENERATED ... STORED` | Rewrites/locks table for existing rows and computes vectors; on large tables schedule maintenance window and ensure disk headroom (text + `tsvector` + GIN can be significant). [P2][P3] |
| `CREATE INDEX` (GIN) | **Build time very sensitive to `maintenance_work_mem`**; raise per-session for the build if safe (bounded on 4 GB). Bulk load first, then create index when possible. [P4][P5] |
| `CREATE INDEX CONCURRENTLY` | Avoids write-blocking behavior of regular build in production traffic but is slower, cannot run inside a transaction block, may leave invalid indexes on failure, and involves extra scans/waits — plan cleanup/retry. For schema-migration tooling that wraps transactions, run as ops step outside tx. [P5][P7] |
| PG18 parallel GIN builds | PG18 supports parallel `CREATE INDEX` for GIN (capability), improving large builds when CPU/I/O/memory allow; not a guarantee and does not remove `CONCURRENTLY` caveats. On 2 vCPU, expect limited parallel benefit; prioritize I/O headroom. [P5][P7] |
| GIN `fastupdate` / pending list | Default `fastupdate` buffers inserts into a pending list (merged later / bounded by `gin_pending_list_limit`), smoothing writes but adding read/maintenance variability; bulk updates may justify drop/recreate or tuning. [P8][P4] |
| Disk & bloat | Track `pg_relation_size`/`pg_total_relation_size` for table+GIN; GIN can be large and bloat after heavy updates; keep backup/restore and `REINDEX CONCURRENTLY` (where appropriate) in runbook. [P4][P8] |

## 5. Autovacuum and index bloat (small VPS)

- **Keep autovacuum enabled.** It reclaims dead tuples and runs `ANALYZE`; falling behind causes bloat, worse scans, and stale plans. [P1]
- **Default triggers can be late on large/churny tables** (scale-factor based); for hot tables tune per-table (e.g. lower scale factor / threshold) carefully, observing I/O impact on 2 vCPU. [P1]
- **GIN-specific:** pending-list cleanup occurs during inserts when list grows (per `gin_pending_list_limit`) and via autovacuum/cleanup paths; memory used during cleanup relates to `work_mem` / `maintenance_work_mem` / `autovacuum_work_mem` depending on context. If pending list grows or latency fluctuates, consider tuning `gin_pending_list_limit`, autovacuum aggressiveness, `fastupdate`, or explicit maintenance — measure first. [P8]
- **Monitor:** `pg_stat_user_tables` (dead tuples, last vacuum/analyze), `pg_stat_all_indexes`, bloat estimates, index sizes, temp files, and `pg_stat_progress_create_index` during builds. [P1][P5]
- **Do not “fix” perf by disabling vacuum**; on VPS, favor steady low-cost maintenance over crisis `VACUUM FULL` (locking/heavy). [P1]

## 6. Query limits and safe query shapes

- **Always**: `organization_id` (and other tenant/visibility predicates) + small `LIMIT` + timeout (`statement_timeout` per API/search role) + parameterized inputs. [F4][P2]
- **`websearch_to_tsquery` / `plainto_tsquery`** for user text; handle empty/stopword-only results gracefully. [P2]
- **Rank/snippets only on page results:** avoid `ts_headline` over unbounded sets; fetch IDs/ranks first (`LIMIT`) then headline that slice if needed. [P2]
- **Pagination:** prefer keyset/seek for deep browsing when needed; offset pagination on large ranked sets gets expensive (NV for current scale). [P6]
- **Avoid per-row recomputation:** query stored generated `tsvector` columns instead of `to_tsvector(...)` in `WHERE` unless using a matching expression index intentionally. [P2][P3]
- **Detect spills/regressions:** `EXPLAIN (ANALYZE, BUFFERS)` for representative queries; look for disk sorts, bitmap heap rechecks, and seq scans on expected index paths. [P1][P6]

## 7. Lightweight in-app notification writes (VPS, 80/20)

Aligned with `2026-11-16-019-inapp-notification-wiring-vps.md` (single-event, PG-first): [F7]

| Principle | Concrete recommendation |
|---|---|
| Source of truth | Insert notification row in **PostgreSQL** (`notifications` table) as the durable record; reads/acks via existing APIs/repos. [F5][F7] |
| Keep writes cheap | One small row per real event (e.g. reply-to-parent-author only), no fanout in MVP, set bounded columns/URLs, avoid extra round trips beyond required lookups. [F7] |
| Transaction/consistency choice | **Preferred MVP:** persist core mutation authoritatively; create notification as **best-effort post-persist, non-fatal** with structured logs (auxiliary UX must not take down core action). **Stronger option:** same DB transaction as domain write or transactional outbox — only if refactor/requirements justify (NV). [F4][F7] |
| Idempotency/duplicates | At least design for “one emission per persisted triggering entity” (`event_type + target_id + user_id`); strongest cheap hardening is a uniqueness constraint + conflict-tolerant insert if migrations are touched (NV). [F7] |
| Real-time delivery | Optional Valkey/in-process push is best-effort after commit; clients must recover via polling/list endpoint. Do not require broker for correctness on Seed. [F4][F7] |
| Index/retention fit | Reuse existing notification indexes (`user_id, created_at`, unread partial, org); define retention/cleanup (`delete_old` or job) before unbounded growth (NV). [F5][F7] |
| Failure handling | Never block `201`/success of core mutation on notification insert failure in best-effort mode; emit warn/error with ids/org for diagnosis. [F7] |

## 8. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Memory overcommit (`work_mem`, many backends, maintenance/vacuum/autovacuum, parallel ops) | OOM kills, swap thrash, latency spikes on 4 GB | Keep global `work_mem` low, cap connections/pools, set `autovacuum_work_mem`, use per-session overrides sparingly. [P1][P6] |
| GIN write/read amplification and pending-list churn | Higher insert latency, disk growth, occasional read jitter | Minimal indexed columns, partial/needed indexes only, monitor sizes/pending behavior, bulk-load before index build. [P4][P8] |
| Bad FTS config/index mismatch or locale quality gaps (pt-BR/es/de/fr/zh-CN/ja) | Empty/irrelevant results, index not used | Fixed config in both index and query, `EXPLAIN` checks, locale-specific relevance validation (NV). [P2][F4] |
| Risky index builds/migrations on live tiny VPS | Lock/contention, long I/O, failed concurrent build leaving invalid index | Off-peak, `CONCURRENTLY` runbook outside tx, session `maintenance_work_mem`, disk checks, retry/cleanup plan. [P5][P7] |
| Notification side-effect inconsistency | Missed/duplicate notifications | PG source of truth, best-effort logging, idempotency guard or dedupe key, optional outbox later. [F4][F7] |
| Hardcoded pool/duplicate init paths in Flora | Cannot enforce small-VPS pool discipline via config alone | Wire pool settings and remove duplication in follow-up code task (NV, evidenced). [F2][F3] |

## 9. NVs (needs verification)

- **NV-1:** Actual p95 latency, RAM, disk, and write amplification for Flora’s real FTS dataset/queries on 2 vCPU/4 GB.
- **NV-2:** Best text-search config per target locale/corpus (`simple` vs `portuguese`/`spanish`/`german`/`french`, `unaccent`, zh-CN/ja feasibility) and need for hybrid Meilisearch/`pg_trgm`.
- **NV-3:** PG18 collation-provider/upgrade impact on existing FTS/`pg_trgm` indexes for deployers’ upgrade paths; reindex requirements from official release notes in-context.
- **NV-4:** Flora repo gaps impacting ops: `database.rs` pool hardcoding and config wiring, duplicate pool/migrate behavior, Meilisearch boot/optional semantics across all paths.
- **NV-5:** Whether notification delivery requires same-tx/outbox guarantees vs best-effort for Seed product acceptance; best uniqueness/dedupe strategy in current schema/traits.
- **NV-6:** Safe `statement_timeout`/pool/PgBouncer settings under real concurrency and SQLx behavior.
- **NV-7:** GIN `fastupdate`/`gin_pending_list_limit` tuning values (workload-specific; defaults often fine initially).

## 10. Sources

### Primary (PostgreSQL 18 docs)
- [P1] PostgreSQL 18 — Resource Consumption: https://www.postgresql.org/docs/18/runtime-config-resource.html
- [P2] PostgreSQL 18 — Full Text Search (tables/indexes, controls, indexes): https://www.postgresql.org/docs/18/textsearch-tables.html , https://www.postgresql.org/docs/18/textsearch-controls.html , https://www.postgresql.org/docs/18/textsearch-indexes.html
- [P3] PostgreSQL 18 — `CREATE TABLE` (generated columns): https://www.postgresql.org/docs/18/sql-createtable.html
- [P4] PostgreSQL 18 — GIN indexes: https://www.postgresql.org/docs/18/gin.html
- [P5] PostgreSQL 18 — `CREATE INDEX` (incl. `CONCURRENTLY`, build behavior): https://www.postgresql.org/docs/18/sql-createindex.html
- [P6] PostgreSQL 18 — Using `EXPLAIN` / planner statistics (operational verification): https://www.postgresql.org/docs/18/using-explain.html , https://www.postgresql.org/docs/18/planner-stats.html
- [P7] PostgreSQL 18 — Routine vacuuming/autovacuum and maintenance context: https://www.postgresql.org/docs/18/routine-vacuuming.html
- [P8] PostgreSQL 18 — GIN implementation details / fastupdate-pending behavior: https://www.postgresql.org/docs/18/gin-implementation.html

### Multilingual and practitioner evidence consulted (non-primary unless stated; used for 80/20 VPS/locale practice)
- [S-EN-1] Ajay Kumar, “PostgreSQL Performance Tuning (2026)… Postgres 18” (2026-11-10): https://www.ajayk.sh/blog/postgres-performance-tuning-explained-2026
- [S-EN-2] RE:NODE, “PostgreSQL full-text search: tsvector, tsquery, GIN” (2026-10-22): https://renode.space/blog/postgresql-full-text-search-basics
- [S-EN-3] pganalyze, “Understanding Postgres GIN Indexes…” (2021-12-02, concepts incl. pending-list cleanup memory): https://pganalyze.com/blog/gin-index
- [S-EN-4] RDP.sh / EasyHowly / rakibulinux / DCHost (small-VPS tuning heuristics, 2025–2026): https://rdp.sh/blog/postgresql-tuning-for-a-small-vps-a-practical-walkthrough , https://easyhowly.com/2026/10/10/tuning-postgresql-work_mem-and-shared_buffers-on-a-vps-without-blowing-up-ram , https://www.rakibulinux.com/blogs/postgresql-performance-tuning-small-vps , https://www.dchost.com/blog/en/postgresql-vps-memory-wal-pgbouncer
- [S-ptBR-1] Runstack, “Full-text search no PostgreSQL” (2026-06-08, pt-BR): https://runstack.com.br/postgres-full-text-search
- [S-ptBR-2] Urhoba, “Busca de Texto Completo PostgreSQL: tsvector e GIN” (2026-09-24, pt-BR; notes PG18 upgrade/reindex caution — verify with official release notes): https://www.urhoba.net/pt/artigo/busca-de-texto-completo
- [S-es-1] DCHost / DreamHost / VDS.guru guides (es, memory/PgBouncer/backup heuristics, consulted via search): https://www.dchost.com/blog/en/postgresql-vps-memory-wal-pgbouncer (en/es availability varies), https://help.dreamhost.com/hc/es/articles/…
- [S-de-1] S-EDV, “PostgreSQL-Performance-Tuning für Einsteiger” (de, 25%/work_mem/multiplication guidance, consulted): https://s-edv.de/…
- [S-fr-1] D-Open / DreamHost FR / 1VPS (fr, tuning/WAL/PgBouncer/backup guidance, consulted): https://d-open.fr/… , https://1vps.com/…
- [S-zhCN-1] 萤光云 / 程序员茄子 (zh-CN, 4GB tuning and PG18 FTS/generated-column/index practice, consulted 2026): https://www.yingguangyun.com/… , https://www.chenxutan.com/d/4551.html
- [S-ja-1] スナップリンク / Tomoda Hinata (ja, VPS tuning and work_mem/session guidance, consulted): https://www.snaplink.jp/… , https://tomodahinata.com/en/blog/postgresql-performance-tuning-production-guide

> Non-EN/practitioner sources informed 80/20 operational emphasis but normative behavior must follow primary PostgreSQL 18 docs for the deployed version/build.

### Flora (internal evidence)
- [F1] `docs/research/2026-11-16-seed-vps-profile-audit.md`
- [F2] `flora-core/src/database.rs`, `flora-core/src/config/database.rs`
- [F3] `docs/research/2026-11-16-016-postgres-vps-backup-tuning.md`, `.env.example`, `docker-compose.yml`
- [F4] `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`
- [F5] `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `flora-api/src/routes/notifications.rs`
- [F6] `flora-notifications/src/lib.rs`, `flora-core/src/repositories/notification_repository.rs`
- [F7] `docs/research/2026-11-16-019-inapp-notification-wiring-vps.md`
- [F8] `OBJETIVO.md`, `ROADMAP.md`
