# Flora Seed VPS Profile Audit — "runs on any cheap VPS"

**Date:** 2026-11-16  
**Scope:** Seed/dev-oriented profile for Flora (Rust + Postgres, optional Valkey/Meilisearch) based on repository inspection (no source changes).  
**Baseline assumption (explicit):** 2 vCPU / 4 GB RAM / SSD VPS, single-node, low initial concurrency.

## Commands and evidence inspected

- `ls .github/workflows` → `ci.yml`, `docker-staging.yml`, `rust.yml`
- `sed -n '1,120p' Cargo.toml`
- `sed -n '1,220p' docker-compose.yml`
- `sed -n '1,220p' .env.example`
- `sed -n '1,220p' Dockerfile` and `sed -n '1,220p' apps/frontend/Dockerfile`
- `sed -n '1,220p' docs/dev-smoke.md`
- `sed -n '1,220p' flora-api/src/main.rs`
- `sed -n '1,220p' flora-api/src/state/app_state.rs`
- `sed -n '1,220p' flora-core/src/database.rs`
- `sed -n '1,220p' flora-core/src/config/{mod,app,database,messaging,search,storage}.rs`
- `rg -n "VALKEY|valkey|MEILI|meilisearch|redis|Redis" flora-core flora-api flora-messaging flora-search flora-notifications`
- `rg -n "is_sqlite|sqlite|SqlitePool|SQLITE" flora-core flora-api`
- `sed -n '1,220p' flora-search/src/lib.rs`
- `sed -n '1,120p' .github/workflows/{rust,ci,docker-staging}.yml`

**Primary evidence paths:** `docker-compose.yml`, `.env.example`, `Dockerfile`, `apps/frontend/Dockerfile`, `.github/workflows/{rust.yml,ci.yml,docker-staging.yml}`, `flora-api/src/main.rs`, `flora-api/src/state/app_state.rs`, `flora-core/src/{database.rs,config/*.rs,migrations/sql/20240818000001_init_schema.sql (referenced by docs)}`, `flora-search/src/lib.rs`, `flora-messaging/src/lib.rs`, `docs/dev-smoke.md`.

## 1) Mandatory vs optional services

| Component | Seed/VPS status | Evidence | Notes |
|---|---|---|---|
| PostgreSQL 18+ | **Mandatory (current Seed path)** | `flora-api/src/main.rs` loads config, creates pool, runs migrations, builds state; `flora-core/src/database.rs` connects only via `config.database.postgres_url` with `sqlx::PgPool`; `docs/dev-smoke.md` states API applies migrations on startup and init migration uses `uuidv7()` requiring PG18+; `docker-compose.yml` uses `postgres:18-alpine`. | SQLite fields exist in config (`flora-core/src/config/database.rs`: `sqlite_path`, `is_sqlite()`), but pool/runtime path is Postgres-only (`database.rs`, `rg` shows no `SqlitePool` usage) — **SQLite fallback is not currently operative**. |
| Valkey/Redis | **Operationally optional (degraded mode)** | `flora-api/src/state/app_state.rs` tries `redis::Client::open(...).get_connection_manager()` and on failure logs warning, sets `valkey`/`messaging_service` to `None`; `OidcTransactionStore` accepts optional redis with in-memory fallback (`flora-api/src/state/oidc_transactions.rs` usage/signature). | Needed for shared Pub/Sub messaging, distributed OIDC tx/rate-limit semantics where implemented; single-node dev may run without it but loses those capabilities. `docker-compose.yml` includes Valkey service; `.env.example` sets `FLORA_MESSAGING__VALKEY_URL`. |
| Meilisearch | **Configured as optional in intent, but effectively required by current boot path** | `flora-core/src/config/search.rs` has `enabled` (default `true`) and validates URL when enabled; `.env.example` comments search vars as "Optional for MVP"; `docs/dev-smoke.md` starts meilisearch in compose. However `flora-api/src/state/app_state.rs` always constructs `flora_search::SearchService::new(...)` and returns `anyhow!("Search service init failed: {e}")` on error, with no `search.enabled` gate. | On a cheap VPS without Meilisearch, API startup currently fails unless code/config handling is adjusted. Treat as **must-have in current binary behavior**; **de-bloat target: make truly optional**. |
| Local filesystem storage | **Mandatory for default file flows** | `flora-core/src/config/storage.rs` defaults backend `local`, path `/var/flora/storage`, 100 MB upload in app config; `docker-compose.yml` does not provision object storage. | Ensure persistent writable volume/mount and permissions; S3 backend fields exist but not evidenced in compose. |
| Frontend runtime/container | **Optional for API Seed profile** | `apps/frontend/Dockerfile` and `.github/workflows/ci.yml` exist; root `Dockerfile` builds only `flora-api`. | VPS can run API-only or add frontend separately. |

## 2) Estimated resource budget (2 vCPU / 4 GB RAM, explicit assumption)

| Service | Suggested memory cap/reservation (starting point) | CPU/notes | Evidence/basis |
|---|---:|---|---|
| `flora-api` (Rust, release container) | 256–512 MB cap | shares 2 vCPU; async I/O bound at Seed scale | `Dockerfile` runs single `flora-api` binary; `flora-core/src/database.rs` hardcodes pool `max_connections(20)` (not using config fields) — can pressure small Postgres/RAM if concurrent. |
| PostgreSQL 18 | 768 MB–1.25 GB cap | primary stateful dependency; leave headroom for cache/checkpoints | Mandatory service, migrations at startup (`flora-api/src/main.rs`, `docs/dev-smoke.md`); compose has no memory tuning. |
| Valkey (if enabled) | 64–128 MB cap | lightweight when AOF/small dataset | `docker-compose.yml` enables `--appendonly yes`; optional in app state but useful for messaging/OIDC tx/rate-limit paths. |
| Meilisearch (if enabled/currently needed) | 384–768 MB cap | indexing/search spikes; can dominate small VPS RAM | Always initialized in `AppState` today; dev image `getmeili/meilisearch:v1.12` in compose with persistent `/meili_data`. |
| OS + buffers + logs | ~512 MB reserved | avoid swap thrash/OOM kills | 2 vCPU/4 GB assumption total budget envelope. |
| **Lean Seed target (Postgres-only ideal)** | **~1.5–2.0 GB used** | feasible only if search/valkey truly optional and pool tuned | Requires code/config follow-up (see actions). |
| **Current compose-like stack** | **~2.0–3.0+ GB possible** | tight on 4 GB with spikes | Includes API + PG18 + Valkey + Meilisearch per `docker-compose.yml`. |

> These are planning estimates, not measured benchmarks (NV: no runtime profiling evidence in repo).

## 3) Compose / env recommendations (cheap VPS)

| Area | Recommendation | Evidence/rationale |
|---|---|---|
| Ports/bindings | Keep DB/cache/search bound to localhost or private network (`127.0.0.1:*` as in compose) and expose only API/reverse-proxy ports publicly. | `docker-compose.yml` already binds `127.0.0.1` for postgres/valkey/meilisearch; `flora-core/src/config/app.rs` defaults host `0.0.0.0:3000`. |
| Volumes | Persist `postgres` data (`/var/lib/postgresql` for PG18 image layout), `valkey` `/data`, `meili` `/meili_data`, and API storage path (`FLORA_STORAGE__LOCAL_PATH`, default `/var/flora/storage`). | `docker-compose.yml` defines named volumes; storage config defaults/local backend in `flora-core/src/config/storage.rs`; PG18 mount note in compose/docs. |
| Healthchecks & startup order | Keep healthchecks; gate API startup on Postgres healthy/migratable and treat Valkey/Meilisearch as optional dependencies once code supports it. | Compose has healthchecks for all three services; API currently fails hard on DB/migrations and search init (`main.rs`, `app_state.rs`). |
| Memory hints | Add `mem_limit`/reservations (Compose) or equivalent deploy constraints; set Postgres/Valkey/Meili env tuning for small RAM; reduce DB pool/concurrency. | No memory limits in `docker-compose.yml`; pool hardcoded to 20 in `flora-core/src/database.rs` while config exposes `max/min_connections` unused by pool. |
| Env consistency | Use one secret namespace consistently: prefer `FLORA_*`/`FLORA_*__*` (Figment loader) and align docs/examples; set strong `FLORA_JWT_SECRET` (>=32 chars in prod). | `Config::load()` merges `Env::prefixed("FLORA_").split("__")` (`flora-core/src/config/mod.rs`); `.env.example` mixes `FLORA_DATABASE__*` with non-prefixed `OIDC_*`/`ZITADEL_*`; app validation requires long secret in production (`app.rs`). |
| Secrets/keys | Replace dev values (`postgres/postgres`, `dev-meilisearch-master-key`, example JWT/OIDC secrets) before non-local use. | `docker-compose.yml`, `.env.example`, `docs/dev-smoke.md`. |

## 4) PostgreSQL 18 notes

- **Required for documented Seed/dev-smoke path:** init migration uses `uuidv7()` defaults (native in PG18). Evidence: `docs/dev-smoke.md` prerequisites and `docker-compose.yml` comment/image `postgres:18-alpine`.
- **PG18 volume layout:** mount at `/var/lib/postgresql` (versioned subdir inside), not the pre-PG18 `/var/lib/postgresql/data` convention. Evidence: `docker-compose.yml` volume comment.
- **Upgrade caveat:** existing PG16 data volume is incompatible with PG18 container; disposable dev reset path is `docker compose down -v` then up, but **do not reset non-disposable data without backup/migration**. Evidence: `docs/dev-smoke.md`.
- **CI signal:** DB-backed job uses `postgres:18` service (`.github/workflows/rust.yml`), aligning with PG18 requirement.

## 5) Backup / restore minimal guidance

- **PostgreSQL (critical):** schedule `pg_dump` (logical) for `flora` DB; for smallest VPS, run off-peak and store artifacts off-host/object storage. Restore with `psql`/`pg_restore` into a compatible PG18+ instance, then run app migrations/startup checks. Include migration compatibility in restore drills. (Basis: Postgres is mandatory state store; `main.rs` runs migrations on startup.)
- **Storage files:** back up `FLORA_STORAGE__LOCAL_PATH` (or mounted volume) together with DB-consistent snapshots where possible; local backend is default (`storage.rs`).
- **Valkey:** if AOF enabled (`docker-compose.yml` uses `--appendonly yes`), persist `/data`; treat as cache/ephemeral messaging aid unless product semantics require durability — prioritize DB backup first.
- **Meilisearch:** persist `/meili_data`; can be rebuilt/reindexed from source-of-truth DB if reindex path exists, so backup is lower priority than Postgres/storage for minimal Seed ops.
- **Minimum runbook:** verify restore on fresh PG18 volume, confirm app boots/applies migrations, and validate seeded/smoke path (`docs/dev-smoke.md`) before declaring backup healthy.

## 6) Top 5 debloat/operability actions for next cycles

1. **Make Meilisearch truly optional at boot/runtime.** Gate `SearchService` initialization and search routes/features on `config.search.enabled` and graceful fallback; today startup fails if init fails (`flora-api/src/state/app_state.rs`), conflicting with "optional for MVP" messaging (`.env.example`, config docs).
2. **Fix DB pool/config operability.** Wire `database.max_connections/min_connections/acquire_timeout/idle_lifetime` into `create_pool` (currently hardcoded `max_connections(20)` in `flora-core/src/database.rs`) and provide cheap-VPS defaults; also remove duplicate pool+migration work (`main.rs` creates pool/migrates, `AppState::new` creates another pool).
3. **Ship a Seed compose profile with resource controls.** Add a minimal profile (api+postgres, optional valkey/meili overlays), `mem_limit`/reservations, restart policies, and documented env (`FLORA_*__*`) with secure placeholders; current `docker-compose.yml` is dev-oriented and unbounded.
4. **Align env/config contract and health semantics.** Reconcile `.env.example` names/comments with Figment keys (`FLORA_SEARCH__URL`/`MEILISEARCH_URL` mapping expectations, OIDC prefixed vs non-prefixed) and add a readiness/health endpoint that reflects DB (required) vs Valkey/Meili (degraded/disabled) state.
5. **Clarify/implement SQLite-or-lean mode honestly.** Either implement real SQLite/fallback startup path or remove/mark unsupported config fields (`sqlite_path`, `is_sqlite`) to avoid false "cheap VPS" assumptions; current runtime is Postgres-only despite config surface (`flora-core/src/config/database.rs`, `database.rs`).

## NV / limitations

- No runtime benchmarks, container memory measurements, or production traffic profiles in repo (resource budget above is assumption-based).
- Actual feature-level dependency on search/messaging at runtime may be broader than boot wiring; this audit focused on startup/config/compose evidence.
- CI has no full DB+Valkey+Meilisearch integration smoke (`.github/workflows/rust.yml`, `docs/dev-smoke.md` notes NV). 
