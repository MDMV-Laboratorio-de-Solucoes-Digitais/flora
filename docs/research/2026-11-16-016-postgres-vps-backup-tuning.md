# Postgres 16+ on cheap VPS (2 vCPU / 4 GB RAM) — tuning, PgBouncer, WAL/checkpoints, backup/restore (Seed)

**Date:** 2026-11-16 (cycle 016)  
**Scope:** Low-risk, concrete guidance for running PostgreSQL 16+ reliably on a small single-node VPS (baseline **2 vCPU / 4 GB RAM / SSD or NVMe**) for a self-hosted Rust app, aligned with Docker Compose deployment. Focus: memory/connection tuning, PgBouncer considerations, WAL/checkpoint basics, and minimal backup/restore/retention (`pg_dump`, filesystem snapshots, local storage).  
**Non-goals:** production HA/Patroni, managed-DB migration plan, benchmark-driven tuning, PITR/WAL-archiving implementation (noted as optional next step), schema/query optimization.

## Repo cross-check (evidence)

| Area | Current state | Implication for VPS/ops |
|---|---|---|
| Compose services | `docker-compose.yml`: `postgres:18-alpine` (PG18+ required for `uuidv7()` init migration, mount `/var/lib/postgresql`), `valkey/valkey:8-alpine` (`--appendonly yes`), `getmeili/meilisearch:v1.12`; all bound to `127.0.0.1`, healthchecks present, named volumes (`flora_postgres_data`, `flora_valkey_data`, `flora_meili_data`), **no memory/CPU limits/profiles**. | Good local-dev baseline and localhost exposure; small-VPS deploy needs resource caps/profiles and secret rotation. PG18 image/layout is intentional today even though tuning guidance targets PG16+ compatibility. |
| Env/config contract | `.env.example`: `FLORA_DATABASE__POSTGRES_URL`, `FLORA_MESSAGING__VALKEY_URL`, search/storage vars, mixed `FLORA_*` and non-prefixed OIDC/Zitadel vars, dev placeholders. `flora-core/src/config/mod.rs` loads `FLORA_` + `__` nesting (Figment). | Prefer canonical `FLORA_*__*` keys for deploy docs/scripts; treat dev secrets/keys as placeholders only. |
| DB pool/runtime | `flora-core/src/database.rs`: `PgPoolOptions::new().max_connections(20)` hardcoded (ignores config). `flora-core/src/config/database.rs`: exposes `postgres_url`, `sqlite_path`, `max_connections` (default 10), `min_connections` (2), `acquire_timeout_secs` (30), `idle_lifetime_secs` (600), but runtime is Postgres-only/fields not wired into pool per prior audits. | **Mismatch/risk:** fixed pool of 20 can be high for tiny DB settings or collide with PgBouncer/backend limits; config intent and runtime diverge. Low-risk ops action: wire config and keep small defaults. |
| Migrations/boot | `flora-api` loads config, creates pool, runs migrations, builds `AppState` (per `main.rs`/dev-smoke docs). `docs/dev-smoke.md`: API applies migrations on startup; PG18+ required for Seed path; existing PG16 volume incompatible with PG18 container (disposable reset `docker compose down -v`, but **never reset non-disposable data without backup/migration**). | Backups/restores must target compatible PG version and include migration/startup verification. |
| Storage | `flora-core/src/config/storage.rs`: local backend default, `FLORA_STORAGE__LOCAL_PATH` (example `/var/flora/storage`), 100 MB upload cap. | Local uploads are stateful and must be backed up/persisted with correct permissions. |
| Existing research alignment | `docs/research/2026-11-16-seed-vps-profile-audit.md`, `2026-11-16-014-seed-vps-operability-international.md`, `2026-11-16-015-seed-vps-operability-quickwins.md`: mandatory Postgres, optional/degradable Valkey, search boot coupling today, no compose limits, duplicate pool/migration path, backup/restore drill needed. | This doc deepens Postgres tuning/WAL/backup specifics and keeps recommendations consistent with those findings. |

## Multilingual research basis (EN, pt-BR, es, de, fr, zh-CN, ja)

| Lang | Representative sources/themes consulted | Key takeaways carried into recommendations |
|---|---|---|
| EN | PostgreSQL docs (Resource Consumption, WAL/checkpoint behavior, `pg_dump`/file-system backup, continuous archiving/PITR concepts), PostgreSQL wiki tuning, Docker Compose docs (healthchecks/resources), PgBouncer docs/features/config, and small-VPS tuning guides (EasyHowly, RDP.sh, CubePath, DCHost, NetBay, VirtualServersVPS, v4vm, MassiveGRID, ServerCake, PGTune-style heuristics). | `shared_buffers` modest on small shared boxes (~25% or less when co-located), `work_mem` is per sort/hash node and multiplies with concurrency, keep `max_connections` low, prefer pooler for bursty clients, checkpoint tuning balances WAL/recovery/write spikes, `pg_dump` is the minimal logical backup, filesystem snapshots require consistency/whole-cluster care, restore drills are mandatory. |
| pt-BR | StreetHosting “Instalar PostgreSQL na VPS Ubuntu com segurança” (otimização, PgBouncer, `pg_dump`+cron, restore test). | Practical 4 GB/8 GB tables (e.g. 4 GB: `shared_buffers=1GB`, `effective_cache_size=3GB`, `work_mem=8MB`, `maintenance_work_mem=256MB` when DB-dedicated; reduce if co-located), `random_page_cost=1.1` on NVMe, PgBouncer transaction mode for many short-lived connections, never expose Postgres broadly, backup off-host and test restore. |
| es | DCHost “PostgreSQL on a VPS…” / playbook (memoria, WAL, PgBouncer), DreamHost guía (dimensionamiento, pg_dump/base backup/replicación, PgBouncer cuándo añadir), ValeByte PgBouncer guide (2 vCPU/4 GB minimum context, backup config files), VDS.guru `pg_dump`+cron. | Budget memory across services (not DB-only), start conservative and measure, PgBouncer often unnecessary at very low concurrency but useful when many short-lived connections/errors appear, back up DB dumps and config files, copy off-VPS. |
| de | S-EDV “PostgreSQL-Performance-Tuning für Einsteiger”. | `shared_buffers` ~25% (not over ~40%), `effective_cache_size` 50–75% as planner hint (no allocation), keep `work_mem` conservative because it multiplies per operation/connection, lower `random_page_cost` on SSD/NVMe, keep `max_connections` moderate and use PgBouncer, validate with `EXPLAIN (ANALYZE, BUFFERS)`, keep autovacuum on. |
| fr | D-Open déploiement VPS (tuning, WAL, `max_connections`/PgBouncer, `pg_dump`/`pg_basebackup`/WAL), DreamHost FR, Simafri dimensionnement, 1VPS PostgreSQL, VDS.guru `pg_dump`, Space-Node/usavps/dev.alexhost-style guidance surfaced in results. | 25%/75% memory heuristics, `work_mem` danger per op/connection, `max_connections` low + PgBouncer transaction pooling, WAL/checkpoint sizing for write spikes, layered backups (`pg_dump` minimum; base backup/WAL for stronger recovery), off-host copies and regular restore tests, cache hit ratio/autovacuum monitoring. |
| zh-CN | 萤光云 “云服务器 PostgreSQL：配置、备份与调优”. | 2核4G viable for small workloads, `shared_buffers` ~25%, keep `work_mem` controlled by concurrency, use connection pooler instead of raising `max_connections`, pick SSD/high IOPS, use `pg_dump`/`pg_basebackup`+WAL as needed with off-site backups and restore drills, monitor connections/cache hit/checkpoints/slow queries/autovacuum. |
| ja | スナップリンク “PostgreSQL運用深掘り ― VPSでのチューニング…”. | Start from memory parameters relative to VPS RAM, `work_mem` scales with connections×query complexity so keep modest, prefer pool sizing/PgBouncer over raising `max_connections`, regular `pg_dump` backups and verify restore (`pg_restore` test), measure/tune incrementally. |

> **NV-1 (multilingual coverage):** automated retrieval was strongest for EN and had good practical coverage for pt-BR/es/de/fr/zh-CN/ja, but some non-EN sources are operator guides rather than primary PostgreSQL project docs. Normative behavior should be verified against official PostgreSQL/PgBouncer/Docker docs for the exact deployed versions.

## Concrete low-risk recommendations (2 vCPU / 4 GB RAM, single-node, low concurrency)

### A. Memory budget and PostgreSQL tuning (starting point, measure before raising)

**Assumption:** same VPS may run API/proxy and optional services; reserve headroom for OS/page cache and avoid OOM. On a DB-heavier node, shift budget toward Postgres; on a co-located app node, use the lower/shared-box values.

| Setting | Low-risk start (shared 4 GB VPS) | DB-dedicated-ish 4 GB ceiling | Why / caveat |
|---|---:|---:|---|
| `shared_buffers` | `512MB` | `1GB` | ~12–25% of RAM on small shared host; PG docs suggest ~25% for dedicated DB >=1GB, but container/co-located services need room. Requires restart. |
| `effective_cache_size` | `1536MB` (`1.5GB`) | `3GB` | Planner hint only (no allocation). Set to realistic RAM available for cache (PG + OS), not a guarantee. |
| `work_mem` | `4MB` (can try `8MB` if measured) | `8–16MB` only with strict concurrency control | **Per sort/hash operation, can multiply per query and per connection**. Keep low globally; for rare heavy reports use per-session/transaction `SET work_mem` with timeout. |
| `maintenance_work_mem` | `128MB` | `256MB` | Helps VACUUM/index/create; keep bounded on 4 GB. |
| `max_connections` (Postgres) | `30` | `40–50` | Each backend has overhead; prefer pooler/app-pool limits. Must align with actual client/pool sizing. |
| `random_page_cost` | `1.1` | `1.1` | SSD/NVMe planner hint (common low-risk change). |
| `effective_io_concurrency` | `100–200` (if supported/storage benefits) | `200` | SSD/NVMe hint; verify effect/version support. |
| `wal_buffers` | `16MB` (or `-1`/auto) | `16MB` | Usually sufficient; explicit value is fine. |
| `huge_pages` | `try` | `try` | Safe where unsupported; enable OS huge pages only if intentionally configured. |

**WAL/checkpoint (small VPS, avoid spikes but keep recovery reasonable):**

| Setting | Low-risk start | Notes |
|---|---:|---|
| `min_wal_size` | `256MB`–`1GB` | Smaller floor on tiny disks/conservative start; raise if checkpoints too frequent. |
| `max_wal_size` | `1GB`–`2GB` (cap carefully on small SSD) | Larger reduces checkpoint frequency but increases WAL disk use/recovery window; **soft target, not hard cap**. Ensure disk headroom. |
| `checkpoint_timeout` | `10min`–`15min` | Spread writes vs recovery time tradeoff. |
| `checkpoint_completion_target` | `0.9` | Spreads checkpoint I/O (current common default/good practice). |
| `wal_compression` | `on` (if CPU headroom) | Can reduce WAL volume at some CPU cost; validate on workload/PG build options. |
| `checkpoint_flush_after` / `backend_flush_after` | defaults or `256kB` if beneficial | Optional; only tune with evidence. |

**Autovacuum:** keep enabled. On small VPS, watch bloat/dead tuples and I/O impact before aggressive tuning; do not disable as a “performance” tweak.

**Apply safely (Docker/Compose):** pass via Postgres command args or mounted config; change one group at a time, record baseline, restart when required (`shared_buffers`, `max_connections`), reload where possible, and verify with `SHOW ...;`, logs, `pg_stat_*`, and workload smoke. Example command-style flags (illustrative, adjust to image/version):  
`postgres -c shared_buffers=512MB -c effective_cache_size=1536MB -c work_mem=4MB -c maintenance_work_mem=128MB -c max_connections=30 -c checkpoint_timeout=15min -c max_wal_size=1GB -c min_wal_size=256MB -c checkpoint_completion_target=0.9 -c random_page_cost=1.1`.

### B. Connection tuning and PgBouncer considerations

**Current repo reality:** `flora-core/src/database.rs` hardcodes app pool `max_connections(20)`; config fields exist but are not wired into pool (NV in code, not changed here). This means app-side concurrency is currently capped in code at 20 regardless of `FLORA_DATABASE__MAX_CONNECTIONS` defaulting to 10 in config.

**Low-risk guidance:**
- **Without PgBouncer (smallest Seed):** keep total backend demand low: e.g. Postgres `max_connections=30`, app pool `5–10` (once wired), reserve slots for migrations/admin/maintenance. Avoid multiple app replicas/processes each opening large pools.
- **With PgBouncer (recommended when many short-lived connections, multiple workers, or connection pressure):** place PgBouncer in front of Postgres on private network/localhost; point app to PgBouncer port (not 5432 directly for app traffic except migrations/admin if preferred). 
  - **Mode:** `pool_mode = transaction` is usually best for stateless web/API workloads; **incompatible with some session features** (session `SET` outside transaction expectations, advisory locks held across transactions, `LISTEN/NOTIFY`, temp tables across transactions, server-side prepared statement patterns depending on driver/PgBouncer/Postgres version). Validate with SQLx/Rust usage and migrations.
  - **Sizing:** tune `default_pool_size` against Postgres cores/concurrency (often small: ~5–20 on 2 vCPU) and keep `max_db_connections` below Postgres `max_connections` minus admin/reserve headroom. `max_client_conn` can be higher. Do not multiply server connections by multiple PgBouncer instances accidentally.
  - **Auth/config:** use SCRAM (`scram-sha-256`) consistently with Postgres; mount minimal config/userlist or supported env/image config; log to stderr in containers; healthcheck/admin access restricted.
  - **Compose alignment:** add PgBouncer as optional profile/service (`127.0.0.1` or internal network only), `depends_on` Postgres healthy, small memory cap (~32–64 MB typical), and document app DSN switch.
- **Rust/SQLx note (NV):** confirm whether `sqlx` prepared statement behavior, transaction usage, and any session-level settings are compatible with PgBouncer transaction pooling before enabling it in production-like environments.

### C. WAL and checkpoint basics (operator view)

- **WAL purpose:** write-ahead log records changes before data files; enables crash recovery, replication, and (with archiving/base backups) PITR. More WAL/checkpoint churn increases I/O; too-infrequent checkpoints increase crash-recovery time and can create write spikes.
- **Checkpoints:** triggered by `checkpoint_timeout` or WAL growth toward `max_wal_size` (soft target). Monitor `pg_stat_checkpointer` (PG17+ view naming differs from older releases) and `log_checkpoints`; sustained “requested” checkpoints due to WAL volume may indicate `max_wal_size` too small or bursty writes.
- **Small-VPS tradeoff:** prefer moderate `max_wal_size`/`checkpoint_timeout` with enough free disk; avoid very aggressive values that fill SSD or prolong recovery.
- **Durability:** keep `fsync=on` and `full_page_writes=on` for production safety. Changing durability-related settings (`synchronous_commit`, fsync-related knobs) is **not** a low-risk default recommendation.

### D. Minimal backup / restore / retention strategy

| Data | Priority | Minimum low-risk backup | Retention (minimal) | Restore notes |
|---|---:|---|---|---|
| PostgreSQL (`flora` DB) | **Critical** | Scheduled logical backup with `pg_dump` (custom `-Fc` or directory `-Fd` for larger DBs), compressed, run off-peak; include globals if roles matter (`pg_dumpall --globals-only`). | **3-2-1 mindset, minimum:** keep recent daily copies (e.g. 7) + weekly copies (e.g. 4) **off the VPS** (object storage/remote host) and at least one local copy only for convenience, not as sole copy. Prune automatically and verify free disk. | Restore into **fresh compatible PostgreSQL 16+/18+ as required** (`createdb` + `pg_restore`/`psql`), then start `flora-api` to apply migrations and run smoke checks (`docs/dev-smoke.md`). Test restores regularly (at least monthly; after major changes always). |
| Local storage (`FLORA_STORAGE__LOCAL_PATH`) | **High** | File backup/sync of mounted path (tar/restic/rclone/rsync equivalent) after quiescing uploads if possible; coordinate timing with DB backup for consistency. | Same off-host retention policy as feasible; keep enough generations to recover accidental deletes/corruption. | Restore files with correct ownership/permissions and path before serving; verify app can read/write. |
| Valkey (`/data`) | Low–medium | Optional volume snapshot/backup only if AOF/RDB data must survive; otherwise treat as rebuildable cache/pubsub aid. | Short/local if any. | App has degraded/fallback behavior in some paths per existing audits; confirm feature impact. |
| Meilisearch (`/meili_data`) | Low | Optional volume backup; prefer reindex from DB if supported/reliable. | Short/local if any. | Rebuild index after restore when possible. |

**Filesystem snapshots (VM/volume-level):** useful as fast coarse backups but **not a substitute for tested logical DB restores** unless taken consistently. For a running Postgres data directory, prefer storage/filesystem snapshots that are crash-consistent at the volume level and understand WAL recovery on boot, or take snapshots with Postgres-aware coordination; **for whole-cluster recovery only** (per PG docs, partial file copies are unsafe). Always pair with restore boot tests. On a cheap VPS, a verified nightly `pg_dump` off-host is usually the lowest-risk baseline.

**Minimal retention/restore runbook (concrete):**
1. Nightly: `pg_dump -Fc` (+ optional globals) to staging path on persistent disk; verify non-zero output/checksum.
2. Immediately copy backup artifacts off-host; prune local/remote by retention policy (e.g. 7 daily / 4 weekly).
3. Include `FLORA_STORAGE__LOCAL_PATH` in file backup job.
4. Monthly (and after upgrade/migration changes): restore latest dump to disposable PG instance/volume, run app migrations/startup, execute API smoke (`/health`/basic seeded flow as applicable), and record RTO/RPO observations.
5. Before risky changes: take ad-hoc backup and confirm restore point.

**Docker Compose alignment notes:**
- Ensure Postgres data volume persists (`/var/lib/postgresql` for PG18 image) and backup jobs run with access to DB/network (`docker compose exec`/sidecar/host cron against published localhost port as appropriate).
- Store backups on a separate disk/object storage, not only inside the same named Docker volume/host disk.
- Add resource caps for backup/restore jobs if run on-host to avoid starving DB/API.

## NVs (needs verification)

- **NV-1:** Multilingual coverage is practical but not exhaustively primary in non-EN; confirm critical settings against official docs for deployed PostgreSQL (16.x/18.x as applicable), PgBouncer, SQLx, and Docker/Compose runtime.
- **NV-2:** No repo runtime benchmarks/profiles; all memory/connection/checkpoint values are conservative starting points for 2 vCPU/4 GB and must be validated with `docker stats`, `vmstat`/memory pressure, Postgres `pg_stat_*`, temp-file/checkpoint logs, and smoke/load tests.
- **NV-3:** `flora-core/src/database.rs` hardcodes pool size (20) and config pool fields are not wired; `main.rs`/`AppState` pool+migration duplication reported in prior audits—operational tuning may be limited until fixed (code change out of scope).
- **NV-4:** PgBouncer transaction pooling compatibility with actual Flora SQLx queries, migrations, prepared statements, and any session features not fully audited here.
- **NV-5:** Compose `deploy.resources` enforcement depends on runtime/orchestrator (`docker compose` non-Swarm vs Swarm/`--compatibility`); verify cgroup limits actually apply.
- **NV-6:** PG18 is required by current Seed migration/compose path; this guide targets PG16+ tuning semantics but deployment/restore must respect the app’s actual minimum/version-specific behavior (`uuidv7`, image data-dir layout).
- **NV-7:** Filesystem snapshot consistency depends on provider/filesystem and backup coordination; logical `pg_dump` + tested restore remains the minimum recommended baseline.
- **NV-8:** Meilisearch/Valkey durability/optional boot behavior follows prior operability findings; not re-audited exhaustively in this Postgres-focused pass.

## Low-risk implementation checklist (docs/ops-first)

1. **Set conservative Postgres runtime params** in Compose/VPS config (start: `shared_buffers=512MB`, `effective_cache_size=1536MB`, `work_mem=4MB`, `maintenance_work_mem=128MB`, `max_connections=30`, `max_wal_size=1GB`, `checkpoint_timeout=15min`, `checkpoint_completion_target=0.9`) and measure.
2. **Add/plan Compose resource limits/profiles** (core/realtime/search) and keep DB/cache/search on localhost/private network.
3. **Align app DB pool with DB/PgBouncer capacity** (wire config, reduce defaults for 4 GB, avoid duplicate pools) in a future low-risk code task.
4. **Adopt nightly `pg_dump` + off-host retention + monthly restore drill**; include `FLORA_STORAGE__LOCAL_PATH`.
5. **Introduce PgBouncer only when needed**, in transaction mode after compatibility checks, with small pool sizes and private networking.
6. **Rotate dev secrets and normalize `FLORA_*__*` env usage** for deployed environments.