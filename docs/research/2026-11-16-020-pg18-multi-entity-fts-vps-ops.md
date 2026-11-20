# 020 — Multi-entity PostgreSQL 18 FTS + notification writes on a cheap VPS (2 vCPU/4 GB)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`dde602b3b`) — **research-only deliverable** (no source/commit changes).  
**Scope:** Operating **multi-entity PostgreSQL 18 full-text search** (messages/tasks/files with stored/generated `tsvector` + GIN, partial indexes) and **frequent in-app notification writes** on a single cheap VPS (**~2 vCPU / 4 GB RAM / SSD**, co-located app+DB typical for Seed/SMB). Focus: write amplification, index count/bloat, autovacuum tuning, `UNION` query planning/limits, migration/index-build strategy, and safe 80/20 defaults.

**Repo baseline (evidence):** `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql` already adds `messages.search_tsv` (`GENERATED ALWAYS AS (to_tsvector('simple', coalesce(content,''))) STORED`) + partial GIN `idx_messages_search_tsv ... WHERE is_deleted = false`. `tasks` (`title`, `description`, `is_deleted`) and `files` (`name`, `file_type`, `is_deleted`) exist without search columns/indexes. `notifications` table/indexes exist in init schema; notifications are lightweight transactional rows. `flora-core/src/database.rs` currently hardcodes pool sizing (`max_connections(20)`) — config wiring is an operability **NV**. Prior related notes: `019-pg18-fts-notifications-vps-ops.md`, `019-postgres-fts-search-vps.md`, `018-postgres-search-outbox-vps.md`.

**Method:** Primary PostgreSQL 18 docs + multilingual practitioner/operator sources (**EN, pt-BR, es, de, fr, zh-CN, ja**) for 80/20 patterns. No production benchmarks in this repo; numbers are **starting defaults to validate** with `EXPLAIN (ANALYZE, BUFFERS)`, `pg_stat_*`, `pg_relation_size`, logs, and host metrics (**NV**).

## 1. Executive 80/20 (Seed/SMB-safe)

| Area | Default | Rationale |
|---|---|---|
| Multi-entity FTS shape | One **stored generated `tsvector` per searchable entity** + one **partial GIN** per entity (`WHERE is_deleted = false`), fixed config `'simple'` initially | DB-maintained on write, deterministic index/query match, tenant/delete selectivity; avoids expression recompute and config drift. [PG-FTS][PG-GIN][PG-CT] |
| Entity rollout | Keep **messages** first; add **tasks** (`title||' '||description`) and **files** (`name||' '||file_type`) only when needed | Each extra GIN adds write/index-maintenance and disk; on 2 vCPU/4 GB, index count is a budget item. [PG-GIN][PG-CI] |
| Query shape | Tenant + visibility predicates, `websearch_to_tsquery('simple',$q)`/`plainto_tsquery` fallback, `search_tsv @@ q`, `ORDER BY ts_rank_cd(...) DESC, created_at DESC LIMIT n` (≤50), `ts_headline` only on final slice | Safe parsing, bounded CPU/IO, index-friendly, snippet cost controlled. [PG-FTS] |
| Multi-entity search SQL | `UNION ALL` of per-entity selective branches, each `LIMIT`ed, then outer `ORDER BY rank/ts LIMIT n` | Preserves entity indexes/pushdown; avoids global dedup cost of `UNION` when IDs/types are disjoint. [PG-UNION][MyDBA] |
| Notifications | **PostgreSQL is source of truth** (small transactional insert), best-effort non-fatal side effects/realtime, idempotent emission key, indexed reads, retention | Durable and cheap on VPS; avoids coupling core mutations to aux delivery. [Repo][PG-MVCC] |
| VPS posture | Low connections, low `work_mem`, bounded maintenance/autovacuum memory, incremental config changes, observe before tuning | Prevents OOM/swap and long maintenance stalls on 4 GB. [PG-RES][PG-VAC] |

## 2. Multi-entity schema & indexing pattern (concrete)

**Messages (existing):** keep `idx_messages_search_tsv ON messages USING GIN (search_tsv) WHERE is_deleted = false`.

**Tasks (additive, when required):**
```sql
ALTER TABLE tasks
  ADD COLUMN IF NOT EXISTS search_tsv tsvector
  GENERATED ALWAYS AS (
    setweight(to_tsvector('simple', coalesce(title,'')), 'A') ||
    setweight(to_tsvector('simple', coalesce(description,'')), 'B')
  ) STORED;
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_tasks_search_tsv
  ON tasks USING GIN (search_tsv) WHERE is_deleted = false;
```

**Files (additive, metadata/name search):**
```sql
ALTER TABLE files
  ADD COLUMN IF NOT EXISTS search_tsv tsvector
  GENERATED ALWAYS AS (
    setweight(to_tsvector('simple', coalesce(name,'')), 'A') ||
    setweight(to_tsvector('simple', coalesce(file_type,'')), 'B')
  ) STORED;
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_files_search_tsv
  ON files USING GIN (search_tsv) WHERE is_deleted = false;
```

**Rules:** fixed config in index expression/column and every query (`'simple'` baseline, or one chosen config consistently); always `coalesce`; include `organization_id` (and workspace/channel/type filters) in every branch; keep partial predicate aligned with real filters. Avoid per-row `to_tsvector(...)` in `WHERE` except a matching expression index. [PG-FTS][PG-CT]

**Optional (defer unless measured):** `pg_trgm` GIN on `tasks.title`/`files.name` for typo/substring UX (extra write/index cost, privilege/extension management). Not a full CJK/JA tokenizer. [PG-TRGM][NV]

## 3. Write amplification, index count, and bloat

| Mechanism | What happens | VPS impact | 80/20 control |
|---|---|---|---|
| Generated `tsvector` on write | Every insert/update that changes source text recomputes lexemes and stores column bytes | CPU + heap growth proportional to text/churn; updates create dead tuples (MVCC) | Index only high-value fields; avoid indexing large volatile blobs; measure row/text growth. [PG-FTS][PG-CT] |
| GIN inverted entries | One row can touch many lexeme posting entries; GIN uses `fastupdate` pending list (buffered inserts, later merged) by default | Faster individual inserts but pending-list scans/cleanup bursts possible; index can be sizable fraction of data | Keep `fastupdate` default initially; load/bulk-update then build index; monitor pending behavior and index size. [PG-GIN] |
| Index count (messages+tasks+files = 3 FTS GINs, plus existing B-trees/notification indexes) | Each qualifying write/update must maintain every relevant index | On 2 vCPU, extra random IO/CPU and WAL; diminishing value per index | **Minimize indexes**: 1 partial FTS GIN/entity max for MVP; drop/merge unused; avoid redundant trgm unless proven. [PG-CI][PG-GIN] |
| Bloat (heap + GIN) | Dead tuples/pages accumulate if vacuum lags; GIN pages/postings can bloat after churn | Disk pressure, slower scans, backup/restore time on SSD-limited VPS | Autovacuum health, track `pg_stat_user_tables`, `pg_relation_size/pg_total_relation_size`, occasional `REINDEX CONCURRENTLY` in maintenance window if needed. [PG-VAC][PG-GIN] |

**Verification queries (ops):** `SELECT pg_size_pretty(pg_total_relation_size('messages'));`, index sizes per relation, `n_dead_tup/last_autovacuum`, `pg_stat_all_indexes`, temp files/checkpoints. Treat >~1–2× expected growth or rising dead tuples as a tuning signal, not a panic metric (**NV** thresholds).

## 4. Autovacuum tuning (small VPS, GIN-aware)

- **Keep autovacuum on**; vacuum reclaims dead tuples, updates visibility/stats, and drains/merges GIN pending entries as part of maintenance paths. [PG-VAC][PG-GIN]
- **Defaults can be late for hot/churny tables** (scale-factor triggers). For high-write FTS/notification tables consider **per-table** tuning (not global first): e.g. lower `autovacuum_vacuum_scale_factor` (0.02–0.05) and/or threshold so cleanup is frequent-but-small; validate IO impact on 2 vCPU. [PG-VAC]
- **Cost throttling:** default autovacuum is gentle; on SSD if vacuum falls behind, cautiously raise `autovacuum_vacuum_cost_limit` (or lower delay) and/or workers — but remember workers share budget and memory. [PG-RES][PG-VAC]
- **Memory caps:** set `autovacuum_work_mem` explicitly (e.g. `64–128MB`) when raising `maintenance_work_mem`, to bound `autovacuum_max_workers × autovacuum_work_mem`. GIN pending-list cleanup memory relates to work/maintenance/autovacuum memory by context. [PG-RES][PG-GIN]
- **GIN-specific knobs (measure before changing):** `gin_pending_list_limit` (index/table storage parameter) controls pending growth/cleanup threshold; `fastupdate=off` trades steadier reads for slower writes; explicit cleanup possible but usually unnecessary if autovacuum is healthy. [PG-GIN]
- **Never** disable vacuum or rely on emergency `VACUUM FULL` as routine on a live small VPS (locks/heavy IO). [PG-VAC]

## 5. `UNION` / `UNION ALL` planning and limits (multi-entity search)

**Planning facts (primary behavior):** `UNION` removes duplicates (HashAggregate/Unique over sorted input above Append) and must process the combined result; `UNION ALL` returns all rows (Append) and can be cheaper/simpler when branches are semantically disjoint. Cost scales with input rows, not just duplicates found. [PG-UNION][MyDBA]

**Recommended pattern for Flora-like multi-entity FTS:**
```sql
(
  SELECT 'message'::text AS type, m.id::text AS id,
         ts_headline('simple', left(m.content, 500), q, 'MaxFragments=1,MaxWords=20') AS snippet,
         ts_rank_cd(m.search_tsv, q) AS rank, m.created_at
  FROM messages m, websearch_to_tsquery('simple', $2) q
  WHERE m.organization_id = $1 AND m.is_deleted = false AND m.search_tsv @@ q
  ORDER BY rank DESC, m.created_at DESC
  LIMIT $3
)
UNION ALL
(
  SELECT 'task', t.id::text,
         ts_headline('simple', coalesce(t.title,'') || ' ' || coalesce(t.description,''), q, 'MaxFragments=1,MaxWords=20'),
         ts_rank_cd(t.search_tsv, q), t.updated_at
  FROM tasks t, websearch_to_tsquery('simple', $2) q
  WHERE t.organization_id = $1 AND t.is_deleted = false AND t.search_tsv @@ q
  ORDER BY rank DESC, t.updated_at DESC
  LIMIT $3
)
-- + files branch similarly
ORDER BY rank DESC, created_at DESC
LIMIT $3;
```

**Guidance/limits:**
- Use **`UNION ALL`** (entities/types/IDs disjoint). Use `UNION` only if real cross-entity duplicate semantics are required (rare; extra sort/hash memory). [PG-UNION][MyDBA]
- **Push selectivity into every branch**: mandatory `organization_id`, soft-delete/visibility, `@@ q`, and **per-branch `LIMIT`** to cap work before Append. Outer `ORDER/LIMIT` bounds final result. [PG-FTS]
- **Memory:** dedup/sorts use `work_mem` (and hash paths can use more via `hash_mem_multiplier`); on 4 GB keep global `work_mem` low and avoid broad unbounded unions. Detect spills (`Sort Method: external merge`, hash `Batches/Disk`) via `EXPLAIN (ANALYZE, BUFFERS)`. [PG-RES][PG-EXPLAIN]
- **Planner behavior:** do not assume outer predicates are/aren’t pushed identically in all forms — **verify with `EXPLAIN`** on representative data. Keep branches simple and sargable. [MyDBA][PG-EXPLAIN]
- **Complexity cap:** 2–4 entity branches is fine for Seed; beyond that (or very loose matches) consider staged search/app merge or re-evaluate need (**NV**).

## 6. Migration & index-build strategy (VPS-safe)

| Step | Recommendation | Notes/risks |
|---|---|---|
| Schema change | Additive `ALTER TABLE ... ADD COLUMN IF NOT EXISTS ... GENERATED ALWAYS AS (...) STORED` in sqlx migration | Computes vectors for existing rows and rewrites/locks table scope per PG behavior; schedule low-traffic window and ensure disk headroom (heap + tsvector + GIN). [PG-CT][PG-FTS] |
| Index creation (prod traffic) | Prefer `CREATE INDEX CONCURRENTLY` as an **ops/runbook step outside transaction blocks**; keep migration SQL transaction-compatible or split | `CONCURRENTLY` avoids write-blocking build but is slower, needs extra scans, cannot run in a transaction block, and can leave **invalid** indexes on failure — detect/drop/retry. [PG-CI] |
| Build resources | Set session `SET maintenance_work_mem = '128MB'`–`'256MB'` for build only (bounded on 4 GB); GIN build is sensitive to maintenance memory | Do not raise globally without capping `autovacuum_work_mem`. [PG-RES][PG-GIN] |
| PG18 capability | Parallel `CREATE INDEX` for GIN exists in modern PG; benefit limited on 2 vCPU/shared IO | Treat as opportunistic, not a reason to over-parallelize. [PG-CI] |
| Bulk/backfill | If loading data, **load first then create GIN**; for large content rewrites consider controlled maintenance window | Reduces write amplification and pending churn. [PG-GIN] |
| Rollout order | Messages (done) → tasks/files only with measured need → optional trgm | Minimizes cumulative index count/bloat. [PG-CI] |
| Post-upgrade | On major PG upgrades, follow release notes for FTS/`pg_trgm` index reindex requirements if applicable | **NV** for specific upgrade paths/collations. [S-ptBR-2] |

## 7. Frequent notification writes (operability on 2 vCPU/4 GB)

| Concern | 80/20 recommendation |
|---|---|
| Correctness | Persist notification as a **Postgres row** (durable inbox/source of truth). [Repo] |
| Write path | Keep payload minimal (ids/type/target/url/timestamps), **one emission per real event** (e.g. reply-to-parent-author), avoid fanout in MVP. [Repo] |
| Consistency mode | **Preferred:** core domain mutation committed first; notification insert **best-effort, non-fatal** with structured logs. **Stronger (NV):** same-tx or transactional outbox only if product requires no-miss guarantees. [Repo] |
| Duplicates/idempotency | Enforce at app boundary now; strongest cheap hardening is uniqueness/dedupe key (`event_type,target_id,user_id` or equivalent) + conflict-tolerant insert if schema touched (**NV**). |
| Indexes/reads | Reuse existing `(user_id, created_at)`, unread partial, org indexes; paginate, cap list sizes. [Repo] |
| Realtime/aux | Valkey/pubsub/websocket best-effort after commit; clients recover by polling list endpoint. [Repo] |
| Retention | Periodic cleanup (`delete_old`/job) to prevent unbounded growth and vacuum debt. [Repo][PG-VAC] |
| Failure behavior | Notification failure must not fail core 2xx mutation in best-effort mode; alert on sustained insert errors. |

## 8. Safe 80/20 defaults for ~2 vCPU / 4 GB SSD (Seed/SMB)

| Setting | Starting value | Notes |
|---|---:|---|
| `shared_buffers` | `512MB` (DB-heavier max ~`1GB` with evidence) | Restart required; leave RAM for OS cache/app/Valkey. [PG-RES] |
| `effective_cache_size` | `1536MB` (hint) | Planner-only, not allocation. [PG-RES] |
| `work_mem` | `4MB` global (max `8MB` only if measured) | Per sort/hash node × operations × connections; prefer per-session override for rare admin/report query. [PG-RES] |
| `hash_mem_multiplier` | default `2.0` | Keep default unless spill evidence justifies change. [PG-RES] |
| `maintenance_work_mem` | `128MB` (build/vacuum window: `256MB` session-scoped) | Improves index/vacuum; cap because autovacuum can consume it. [PG-RES][PG-GIN] |
| `autovacuum_work_mem` | `64–128MB` (or `-1` if maintenance kept low) | Bounds background memory. [PG-RES] |
| `max_connections` (PG) | `30` | Small VPS; use app pool discipline (PgBouncer only if needed). [PG-RES] |
| App DB pool (Flora) | target `5–10` effective | **NV:** wire config in `database.rs` (currently hardcoded). [Repo] |
| `random_page_cost` | `1.1` | SSD hint; verify plans. [PG-RES] |
| `checkpoint_timeout` / `completion_target` | `15min` / `0.9` | Smooth IO. [PG-RES] |
| `max_wal_size` / `min_wal_size` | `1GB` / `256MB` | Needs disk headroom. [PG-RES] |
| `statement_timeout` (search/API role) | `2–5s` (NV) | Guardrail for pathological FTS/UNION/snippet queries. [PG-RES] |
| `autovacuum_vacuum_scale_factor` (hot FTS/notify tables) | `0.02–0.05` per table (NV) | Earlier, smaller vacuums if churn warrants. [PG-VAC] |
| `autovacuum_vacuum_cost_limit` | `200` default; raise cautiously (e.g. `400–1000`) only if lagging on SSD | Measure IO latency impact. [PG-VAC] |
| `gin_pending_list_limit` | default (4MB) initially | Tune per index only with evidence. [PG-GIN] |

**Operational checklist:** apply incrementally, restart only where required, confirm with `SHOW`, track `EXPLAIN (ANALYZE, BUFFERS)`, dead tuples, index/table sizes, temp files, checkpoints, and app p95.

## 9. Risks

| Risk | Why it matters on 2 vCPU/4 GB | Mitigation |
|---|---|---|
| Memory overcommit (`work_mem` × nodes × sessions, maintenance/autovacuum, parallel build) | OOM kill, swap, cascading latency | Low global `work_mem`, cap connections/pools, cap `autovacuum_work_mem`, session-scoped maintenance settings. [PG-RES] |
| Cumulative index count/write amplification (3 FTS GINs + existing indexes) | Slower writes, higher WAL/CPU/disk, notification+search contention | Add entities incrementally, partial indexes only, no speculative trgm, measure write p95. [PG-GIN][PG-CI] |
| GIN bloat/pending-list bursts | Read jitter, disk growth | Healthy autovacuum, monitor sizes/pending behavior, bulk-load-before-index, rare `REINDEX CONCURRENTLY`. [PG-GIN][PG-VAC] |
| Bad `UNION`/unbounded queries | Large sorts/hashes, temp spills, timeouts | `UNION ALL`, per-branch predicates+`LIMIT`, global `LIMIT`, `statement_timeout`, `EXPLAIN` verification. [PG-UNION][PG-RES] |
| Migration/index build on live small VPS | Lock/contention or long IO; invalid concurrent index | Off-peak, `CONCURRENTLY` runbook outside tx, bounded session maintenance memory, disk checks, cleanup plan. [PG-CI] |
| Config/index-query mismatch (language/config, locale tokenization) | Index not used or poor relevance (esp. zh-CN/ja segmentation) | Fixed config everywhere, `EXPLAIN`, locale corpus validation; treat CJK/JA as **NV**/possible hybrid later. [PG-FTS] |
| Notification duplication/loss semantics | UX inconsistency or write overhead | PG source of truth, idempotent emission, non-fatal aux path, optional outbox/unique constraint by requirement. [Repo] |
| Repo pool/config drift | Cannot enforce VPS-safe concurrency | Wire DB pool config and remove hardcoded/duplicate init paths (follow-up). [Repo] |

## 10. NVs (needs verification)

- **NV-1:** Real write/read p95, WAL/disk growth, and GIN pending/size behavior for Flora’s actual message/task/file/notification rates on 2 vCPU/4 GB.
- **NV-2:** Locale quality for `'simple'` vs language configs (`english`, `portuguese`, `spanish`, `german`, `french`) and feasibility for **zh-CN/ja** (tokenization); whether `pg_trgm` materially helps without excess cost.
- **NV-3:** Best per-table autovacuum thresholds/cost and `gin_pending_list_limit`/`fastupdate` for observed churn.
- **NV-4:** Production-safe migration procedure in this codebase (sqlx transactional migrations vs `CREATE INDEX CONCURRENTLY` ops split) and extension privileges for `pg_trgm`.
- **NV-5:** Search authorization scope beyond `organization_id` (e.g. channel/workspace/private visibility) before exposing broader multi-entity results.
- **NV-6:** Notification consistency contract for Seed (best-effort vs same-tx/outbox) and concrete dedupe uniqueness strategy.
- **NV-7:** Effective app pool sizing after wiring config and interaction with any external pooler.

## 11. Sources

### Primary (PostgreSQL 18 / current docs)
- [PG-FTS] Tables and Indexes / Controlling Text Search / Preferred Index Types: https://www.postgresql.org/docs/current/textsearch-tables.html , https://www.postgresql.org/docs/current/textsearch-controls.html , https://www.postgresql.org/docs/current/textsearch-indexes.html
- [PG-GIN] GIN Indexes (fastupdate, pending list, tips): https://www.postgresql.org/docs/current/gin.html
- [PG-CT] `CREATE TABLE` (generated stored columns): https://www.postgresql.org/docs/current/sql-createtable.html
- [PG-CI] `CREATE INDEX` (`CONCURRENTLY`, build considerations): https://www.postgresql.org/docs/current/sql-createindex.html
- [PG-RES] Resource Consumption (`shared_buffers`, `work_mem`, `hash_mem_multiplier`, maintenance/autovacuum memory): https://www.postgresql.org/docs/current/runtime-config-resource.html
- [PG-VAC] Routine Vacuuming / autovacuum: https://www.postgresql.org/docs/current/routine-vacuuming.html
- [PG-EXPLAIN] Using `EXPLAIN`: https://www.postgresql.org/docs/current/using-explain.html
- [PG-UNION] `SELECT` / combining queries (`UNION`, `UNION ALL`): https://www.postgresql.org/docs/current/queries-union.html
- [PG-MVCC] Concurrency Control (MVCC/write behavior context): https://www.postgresql.org/docs/current/mvcc.html
- [PG-TRGM] `pg_trgm`: https://www.postgresql.org/docs/current/pgtrgm.html

### Multilingual / practitioner evidence consulted (for VPS 80/20 and locale practice; validate against primary docs for deployed version)
- **EN:** RE:NODE, “PostgreSQL full-text search: tsvector, tsquery, GIN” (2026-10-22): https://renode.space/blog/postgresql-full-text-search-basics ; pganalyze, “Understanding Postgres GIN Indexes…” (2021-12-02): https://pganalyze.com/blog/gin-index ; myDBA.dev, “Is UNION ALL faster than UNION in PostgreSQL?” (2026-09-17): https://mydba.dev/blog/postgres-union-vs-union-all ; Ajay Kumar, “PostgreSQL Performance Tuning (2026)” (2026-11-10): https://www.ajayk.sh/blog/postgres-performance-tuning-explained-2026 ; GitLab Docs, “Tune PostgreSQL autovacuum”: https://docs.gitlab.com/administration/postgresql/vacuum
- **pt-BR:** Runstack, “Full-text search no PostgreSQL” (2026-06-08): https://runstack.com.br/postgres-full-text-search ; [S-ptBR-2] Urhoba, “Busca de Texto Completo PostgreSQL: tsvector e GIN” (2026-09-24, upgrade/reindex caution — verify with official release notes): https://www.urhoba.net/pt/artigo/busca-de-texto-completo
- **es:** DCHost, “PostgreSQL en una VPS…” (memory/WAL/PgBouncer heuristics, consulted): https://www.dchost.com/blog/es/ (topic page; validate specifics)
- **de:** S-EDV, “PostgreSQL-Performance-Tuning für Einsteiger” (consulted, 25% buffers/work_mem multiplication heuristics): https://s-edv.de/
- **fr:** D-Open, “PostgreSQL : performances, tuning et WAL sur VPS” (consulted): https://d-open.fr/
- **zh-CN:** 萤光云, “4G内存VPS PostgreSQL优化指南” (consulted): https://www.yingguangyun.com/ ; 程序员茄子, PG18 FTS/generated-column practice (consulted): https://www.chenxutan.com/d/4551.html
- **ja:** スナップリンク, PostgreSQL/VPS tuning (consulted): https://www.snaplink.jp/ ; Tomoda Hinata, “PostgreSQL Performance Tuning: A Production Guide” (consulted): https://tomodahinata.com/en/blog/postgresql-performance-tuning-production-guide

> Practitioner sources inform 80/20 ops emphasis; authoritative behavior/settings for the running cluster must be confirmed against PostgreSQL 18/current docs and local measurements.
