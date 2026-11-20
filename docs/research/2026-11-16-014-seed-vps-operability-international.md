# Seed VPS Operability — International Research (Cycle 014)

**Date:** 2026-11-16  
**Scope:** Low-risk operability guidance for running Flora Seed (Rust API + PostgreSQL, optional Valkey/Meilisearch/object storage) on a cheap VPS target **2 vCPU / 4 GB RAM / SSD**, single-node, low concurrency.  
**Research languages requested:** EN, pt-BR, es, de, fr (automated web retrieval was limited for non-EN sources in this environment; recommendations below are grounded in authoritative EN documentation and cross-checked against repo evidence, with multilingual operator notes/terms included).

## Evidence inspected (repo cross-check)

- `docker-compose.yml`: PG18 `postgres:18-alpine` on `127.0.0.1:5432`, volume `/var/lib/postgresql`; Valkey 8 AOF on `127.0.0.1:6379`, volume `/data`; Meilisearch v1.12 on `127.0.0.1:7700`, volume `/meili_data`, `MEILI_MASTER_KEY` dev value; all have healthchecks; **no `deploy.resources`/mem limits**.
- `.env.example`: `FLORA_DATABASE__POSTGRES_URL`, `FLORA_MESSAGING__VALKEY_URL`, search vars commented as “Optional for MVP”, local storage `/var/flora/storage`, mixed `FLORA_*` and non-prefixed OIDC/Zitadel vars.
- `flora-api/src/main.rs`: loads/validates config, `create_pool`, `run_migrations`, builds `AppState`, serves Axum; startup depends on DB+migrations and state init.
- `flora-api/src/state/app_state.rs`: creates **another** pool and always initializes `flora_search::SearchService::new(...)` (fails startup on search init error); Valkey is best-effort (`warn` + `None`) and messaging/OIDC tx store degrade gracefully.
- `flora-core/src/database.rs`: `PgPoolOptions::new().max_connections(20)` hardcoded (ignores `DatabaseConfig` pool fields).
- `flora-core/src/config/{database,app,messaging,search,storage}.rs`: SQLite fields exist but runtime is Postgres-only (confirmed by prior audit); search `enabled` defaults true and validates URL when enabled; messaging required by config validation but runtime connection optional; storage defaults local `/var/flora/storage`.
- `flora-search/src/lib.rs`, `flora-messaging/src/lib.rs`: Meilisearch client/indexing and Valkey Pub/Sub implementations (optional capabilities in product terms, but boot coupling differs).
- `docs/research/2026-11-16-seed-vps-profile-audit.md`: prior Seed VPS findings (mandatory/optional matrix, PG18/uuidv7, pool/init duplication, search boot coupling, env drift, no resource limits).

## External evidence (authoritative)

- PostgreSQL docs (EN): [19.4 Resource Consumption](https://www.postgresql.org/docs/current/runtime-config-resource.html) — `shared_buffers` default 128MB, 25% RAM reasonable start for dedicated DB with >=1GB, <1GB use smaller share; `work_mem` is per-operation/per-sort and can multiply with concurrency/complex queries; `maintenance_work_mem` default 64MB; `max_connections` affects per-client resources.
- PostgreSQL wiki (EN): [Tuning Your PostgreSQL Server](https://wiki.postgresql.org/wiki/Tuning_Your_PostgreSQL_Server) — on <1GB/low-RAM systems use ~15% for `shared_buffers` in some cases; keep `max_connections` modest or use pooling; `work_mem` must be sized against connections/parallel sort operations; `effective_cache_size` is planner hint (~50–75% RAM typical starting range, not allocation).
- PostgreSQL docs (EN): [25.1 SQL Dump](https://www.postgresql.org/docs/current/backup-dump.html) — `pg_dump`/`pg_dumpall` for logical backups, `psql`/`pg_restore` for restore, supports compression/parallel/custom formats; dumps are consistent snapshots without blocking normal operations (with some lock exceptions).
- Docker Compose spec (EN): [Deploy `resources`](https://docs.docker.com/reference/compose-file/deploy/#resources) (`limits`/`reservations` for `cpus`, `memory`, etc.); [Service `healthcheck`](https://docs.docker.com/reference/compose-file/services/#healthcheck) for dependency-aware readiness patterns.
- Meilisearch docs (EN): [Configure Meilisearch at launch](https://www.meilisearch.com/docs/learn/self_hosted/configure_meilisearch_at_launch) — instance-level runtime options/env/config; memory/indexing behavior can be constrained via launch configuration (important for small VPS).
- Valkey docs (EN): [Memory optimization](https://valkey.io/topics/memory-optimization/) — `maxmemory`/eviction policy, allocator behavior/RSS vs used memory, memory-efficient encodings; provision for peak usage and avoid assuming RSS drops immediately after deletes.

## Cross-check: mandatory vs optional (current reality)

| Component | Current status | Repo evidence | Low-risk operability stance (2 vCPU/4 GB) |
|---|---|---|---|
| PostgreSQL 18+ | Mandatory for Seed path | `main.rs` pool+migrate; `database.rs` Postgres-only; PG18/`uuidv7()` in smoke/audit; compose PG18 image/volume note | Keep mandatory; tune for small RAM and cap container memory. |
| Valkey/Redis | Optional at runtime (degraded) | `app_state.rs` warns and continues with `None`; messaging/OIDC tx store support fallback paths | **Profile: optional** on tiny single-node; enable only if realtime/shared-state features needed. |
| Meilisearch | Optional by config intent, **required by current boot** | `search.rs` `enabled` default true; `.env.example` says optional MVP; `app_state.rs` hard-fails on init | **De-bloat target:** gate init/features on `config.search.enabled` + graceful disable. Until then, provision memory or run Meilisearch. |
| Local storage | Required for default uploads | `storage.rs` local backend/path defaults; compose has no object storage | Mount persistent writable volume; include in backup scope. |
| Frontend container | Optional | separate `apps/frontend/Dockerfile`, CI frontend workflow | Run behind reverse proxy or separately; not required for API seed profile. |

## Concrete low-risk recommendations

### 1) Container resource limits, healthchecks, and optional service profiles
- **Memory/CPU limits (EN):** define conservative `deploy.resources.limits/reservations` (or equivalent runtime constraints) in a VPS-focused compose/profile. Starting envelope for 4 GB: reserve ~512 MB for OS/page cache/headroom; API 256–512 MB; Postgres 768–1024 MB; Valkey 64–128 MB (if enabled); Meilisearch 384–768 MB (if enabled/currently required). Keep Meilisearch off for lean profile once boot is decoupled. (Evidence: Docker deploy resources spec; `docker-compose.yml` currently unbounded; prior audit estimates.)
- **Healthchecks & startup (EN/pt-BR/es/de/fr):** keep DB/Valkey/Meili healthchecks; gate API readiness on Postgres (`pg_isready`/successful migrations) and treat Valkey/Meilisearch as `degraded` when intentionally disabled. *pt-BR: separar “obrigatório” de “opcional” no perfil compose; es: perfiles opcionales y dependencias degradables; de: optionale Profile und degradierte Abhängigkeiten klar trennen; fr: profils optionnels et dépendances dégradables explicites.*
- **Profiles (low risk):** `core` = `flora-api` + `postgres`; `realtime` adds `valkey`; `search` adds `meilisearch`. Bind stateful services to `127.0.0.1`/private network as today (`docker-compose.yml`).

### 2) PostgreSQL tuning for small RAM (2 vCPU/4 GB)
Use modest, safe defaults and validate under load (no aggressive overcommit):
- `shared_buffers`: **256MB–512MB** starting point (within ~6–12% of 4 GB; Postgres docs recommend smaller share on <1GB and ~25% on >=1GB dedicated servers, but containers share RAM with API/OS/optional services). Avoid large jumps without measuring.
- `work_mem`: **4MB** (default-like) to **8MB** initially; increase cautiously because it is allocated per sort/hash operation and can multiply by connections/plan nodes (Postgres docs/wiki). Pair with lower concurrency.
- `maintenance_work_mem`: **64MB–128MB** for VACUUM/index maintenance on small VPS (above default can help, but keep bounded).
- `effective_cache_size`: **2GB–3GB** as planner hint only (not reserved memory), assuming OS/file cache available (wiki guidance).
- `max_connections`: **20–40** max for this footprint; prefer app-level pooling discipline. **Important repo mismatch:** `flora-core/src/database.rs` hardcodes pool `max_connections(20)` and `main.rs` + `AppState::new` both create pools/migrate — fix/wire config fields and avoid duplicate pools to prevent connection/memory pressure (low-risk code follow-up, evidenced).
- Keep autovacuum on; monitor temp file spills/checkpoints before raising memory knobs.

### 3) Optional services: Valkey and Meilisearch debloat
- **Valkey:** if enabled, set container memory cap and configure `maxmemory` with an explicit eviction policy appropriate for cache/pubsub usage; persist AOF volume only when needed (`docker-compose.yml` already uses AOF). Treat as disposable acceleration where semantics allow (Valkey memory docs).
- **Meilisearch:** constrain instance resources at launch/config, persist `/meili_data`, and disable in Seed when search is not required once `search.enabled` is honored at boot (`.env.example` vs `app_state.rs` mismatch). Reindex-from-DB strategy reduces backup criticality.

### 4) Backup and restore (minimal, low-risk)
- **PostgreSQL (critical, RPO/RTO baseline):** scheduled `pg_dump` (logical), compressed/custom format as needed, stored off-host; test restore into a fresh PG18+ instance and then run API migrations/startup smoke (`docs/dev-smoke.md` path). For globals/roles in fuller setups use `pg_dumpall` as appropriate (Postgres backup docs).
- **Storage:** back up `FLORA_STORAGE__LOCAL_PATH` (or mounted volume) consistently with DB state where possible (default local backend).
- **Valkey:** backup AOF/RDB data volume only if durability matters; otherwise rebuildable cache.
- **Meilisearch:** snapshot `/meili_data` optionally; prefer rebuild/reindex capability from Postgres as primary recovery path.
- **Restore drill (pt-BR/es/de/fr note):** *testar restauração regularmente / probar restauraciones periódicamente / Wiederherstellung regelmäßig testen / tester régulièrement la restauration*.

### 5) Healthchecks and operability hygiene
- Expose only reverse proxy/API publicly; keep Postgres/Valkey/Meilisearch on loopback/private net (already aligned in compose).
- Add API liveness/readiness endpoints reporting DB required + Valkey/Meili status (`enabled|healthy|degraded|disabled`) to support safe optional profiles.
- Align env contract to Figment `FLORA_` + `__` nesting (e.g. database/messaging/search/storage) and replace all dev secrets/keys (`FLORA_JWT_SECRET` >=32 chars in prod per `app.rs`, Meili master key, DB creds).
- Keep `FLORA_STORAGE__LOCAL_PATH` on persistent volume with correct permissions; monitor disk usage on small SSD VPS.

## Top 5 low-risk actions for Cycle 014

1. **Decouple Meilisearch from mandatory boot:** honor `config.search.enabled` in `AppState`/routes and degrade gracefully (resolves current “optional in docs, required in binary” contradiction).
2. **Right-size and wire DB pooling:** use `DatabaseConfig` (`max/min/acquire/idle`) in `create_pool`, lower default for 4 GB, and remove duplicate pool+migration initialization path.
3. **Introduce VPS compose profiles + resource caps:** `core`/`realtime`/`search` overlays with `deploy.resources`, persistent volumes, localhost bindings, and secure env placeholders.
4. **Publish small-VPS Postgres/Valkey/Meili runtime presets:** conservative PG params (`shared_buffers`, `work_mem`, `max_connections`), optional Valkey `maxmemory`, and documented Meilisearch launch limits.
5. **Operationalize backups/restore drills:** documented `pg_dump` + storage backup schedule, off-host retention, and periodic PG18 restore smoke (including migrations/startup verification).

## Limitations / NV

- Automated multilingual source retrieval (pt-BR/es/de/fr) was limited in this run; non-EN guidance is provided as operator-facing terminology/checklist alignment and should be supplemented with locale-specific runbooks when available.
- No in-repo runtime benchmarks or measured memory profiles; all sizing values are conservative starting points for a 2 vCPU/4 GB assumption and must be validated.
- Feature-level coupling beyond boot wiring (search/messaging call sites across routes/services) was not exhaustively traced in this document. 
