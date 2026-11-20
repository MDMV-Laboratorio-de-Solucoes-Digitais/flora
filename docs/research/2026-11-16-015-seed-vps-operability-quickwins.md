# Seed VPS Operability Quick Wins (Multilingual Research + Codebase Cross-check)

**Date:** 2026-11-16  
**Scope:** Low-risk quick wins for Seed/VPS (2 vCPU / 4 GB RAM / SSD, single-node): truly optional Meilisearch/Valkey boot/config (degraded modes), compose resource limits/profiles/healthchecks, `FLORA_*` env/config/pool alignment, minimal backup guidance.  
**Evidence inspected:** `docker-compose.yml`, `.env.example`, `flora-api/src/main.rs`, `flora-api/src/state/app_state.rs`, `flora-core/src/database.rs`, `flora-core/src/config/{mod,app,database,messaging,search,storage}.rs`, `flora-search/src/lib.rs`, `flora-messaging/src/lib.rs`, `docs/research/2026-11-16-seed-vps-profile-audit.md`, `docs/research/2026-11-16-014-seed-vps-operability-international.md`.  
**External research (EN/pt-BR/es/de/fr):** Meilisearch docs (`--max-indexing-memory`, `MEILI_EXPERIMENTAL_SEARCH_QUEUE_SIZE`, RAM/thread guidance, memory-crash mitigation), Docker Compose docs (`deploy.resources`, `healthcheck`, `depends_on` conditions), PostgreSQL docs/wiki (small-RAM tuning, `pg_dump`/restore), Valkey docs/admin/guides (`maxmemory`, eviction, AOF/RDB, persistence tradeoffs), practical Compose examples with limits/healthchecks/profiles. Multilingual operator cues: pt-BR "perfis opcionais/degradação controlada/limites de memória"; es "perfiles opcionales, dependencias degradables, límites de recursos"; de "optionale Profile, degradierte Abhängigkeiten, Ressourcenbegrenzungen"; fr "profils optionnels, dépendances dégradables, limites de ressources".

## Current-state cross-check (high-signal)

| Area | Current behavior | Risk/opportunity |
|---|---|---|
| Postgres | Mandatory; `main.rs` creates pool+runs migrations, `AppState::new` creates another pool; pool hardcoded `max_connections(20)` in `flora-core/src/database.rs` (ignores config). | Connection/memory pressure on 4 GB; duplicate init cost. |
| Meilisearch | `SearchConfig.enabled` defaults `true` and validates URL when enabled, but `AppState` always constructs `SearchService::new(...)` and hard-fails on init; `.env.example` says "Optional for MVP". | Boot contradiction; blocks lean Seed profile. |
| Valkey | Runtime optional in `AppState` (warn + `None`), messaging/OIDC tx store can degrade; config still requires `messaging.valkey_url`. | Good degraded path at runtime, but config/env contract and profiles can be clearer. |
| Compose | PG18/Valkey/Meili all started with healthchecks and localhost binds, but no `deploy.resources`/profiles, dev secrets, and no API service. | Unbounded memory spikes on small VPS. |
| Env contract | Figment loads `FLORA_` + `__`; `.env.example` mixes names/comments (`FLORA_SEARCH__MEILISEARCH_URL` commented, config default/env also `MEILISEARCH_URL`; OIDC/Zitadel partly non-prefixed) and includes dev secrets. | Operator confusion/drift. |
| SQLite surface | `DatabaseConfig` exposes `sqlite_path`/`is_sqlite()` but runtime is Postgres-only. | False "lean fallback" expectation. |

## File-by-file low-risk checklist

### `docker-compose.yml`
- [ ] **Profiles (no behavior change to default core):** add `core` (postgres [+ api when added]), `realtime` (valkey), `search` (meilisearch) or equivalent Compose profiles/overlays so lean Seed can run without Valkey/Meili.
- [ ] **Resource caps (starting points for 4 GB):** add memory/CPU limits+reservations (Compose/Swarm-compatible keys or documented `--compatibility`/runtime-specific approach): postgres ~768M–1G, valkey ~96M–128M, meilisearch ~384M–512M (or disable), leave OS/headroom; validate with `docker stats` because enforcement differs by Compose mode.
- [ ] **Healthchecks/startup:** keep existing checks; for API service add `depends_on: postgres: condition: service_healthy` and make valkey/meili soft/optional deps when profiles enabled.
- [ ] **Meilisearch small-VPS flags:** when profile enabled, set conservative `MEILI_MAX_INDEXING_MEMORY` and/or `MEILI_MAX_INDEXING_THREADS`; pin image tags deliberately (current `v1.12` is old; upgrades are separate, higher-risk).
- [ ] **Valkey memory policy:** keep AOF if needed, add `--maxmemory` + explicit `--maxmemory-policy` (e.g. `noeviction` for correctness-sensitive streams or `allkeys-lru` only for pure cache) aligned with `flora-messaging` semantics.
- [ ] **Persistence/ports/secrets:** keep `127.0.0.1` binds, persist volumes, document backup-critical vs rebuildable data; replace dev `MEILI_MASTER_KEY`/DB creds via env/secret injection.

### `.env.example`
- [ ] **Align to `FLORA_*`/`__` contract:** document canonical keys used by Figment (`FLORA_DATABASE__POSTGRES_URL`, `FLORA_MESSAGING__VALKEY_URL`, `FLORA_SEARCH__URL`/`FLORA_SEARCH__ENABLED`/`FLORA_SEARCH__API_KEY` as supported) and note legacy/non-prefixed fallbacks only if intentionally supported.
- [ ] **Make optionality explicit:** add `FLORA_SEARCH__ENABLED=false` example for lean Seed and clarify Valkey as "optional runtime, recommended for realtime/OIDC tx fanout" (or match actual product needs) with degraded-mode note.
- [ ] **OIDC/Zitadel naming:** reconcile `OIDC_*`/`ZITADEL_*` vs `FLORA_OIDC__*` to avoid silent misconfig; keep placeholders non-secret.
- [ ] **Pool/ops knobs:** document DB pool/timeout envs once wired (`FLORA_DATABASE__MAX_CONNECTIONS`, etc.) with small-VPS defaults.
- [ ] **Backup-relevant paths:** keep `FLORA_STORAGE__LOCAL_PATH` prominent and add comment that it must be on a persistent, backed-up volume.

### `flora-api/src/main.rs`
- [ ] **Single init path:** avoid duplicate pool creation/migrations (currently pool+migrate here and `AppState::new` creates pool again). Low-risk refactor: pass pool/config-derived handles into state or centralize startup.
- [ ] **Readiness/logging:** emit startup capability summary (DB required, search enabled/disabled/failed-degraded if supported, valkey connected/fallback) to aid cheap-VPS ops.
- [ ] **No behavior break:** keep fail-fast for Postgres/migrations; only degrade for truly optional services.

### `flora-api/src/state/app_state.rs`
- [ ] **Gate Meilisearch on config:** initialize `SearchService` only when `config.search.enabled`; on init failure decide explicit policy: fail-fast if explicitly enabled-required, or log+set optional/disabled handle if chosen as degraded mode. This is the key "truly optional" quick win.
- [ ] **Optional search API shape:** change `search_service` to `Option<Arc<...>>` or a null/degraded adapter; audit call sites before merge (NV/risk: broader route coupling).
- [ ] **Pool ownership:** accept/reuse an injected pool instead of calling `create_pool` again.
- [ ] **Valkey clarity:** keep existing graceful fallback; optionally expose health/capability flags for diagnostics.

### `flora-core/src/database.rs`
- [ ] **Wire config into pool:** use `DatabaseConfig` fields (`max_connections`, `min_connections`, `acquire_timeout_secs`, `idle_lifetime_secs`) instead of hardcoded `20`.
- [ ] **Small-VPS default recommendation:** consider lowering default `max_connections` (e.g. 5–10) when paired with tuned Postgres `max_connections`; keep current default only with explicit justification.
- [ ] **Error context:** preserve existing error mapping; no schema/runtime behavior change.

### `flora-core/src/config/mod.rs`
- [ ] **Load/validation coherence:** ensure config supports `search.enabled=false` as a valid lean profile (currently `SearchConfig::validate` allows empty URL only when disabled, but `AppState` ignores `enabled`).
- [ ] **Optional: env compatibility shim:** if needed for migration safety, accept legacy env names with deprecation warnings (low-risk if additive).

### `flora-core/src/config/app.rs`
- [ ] **Ops-safe defaults:** keep JWT minimum enforcement for production; consider documenting `base_url/cors`/OTEL for reverse-proxied VPS.
- [ ] **No quick-win code change required** beyond docs/alignment unless health/readiness settings are added.

### `flora-core/src/config/database.rs`
- [ ] **Honest capability surface:** either implement/document SQLite path or mark `sqlite_path`/`is_sqlite()` as unsupported for current runtime to avoid false assumptions (docs-first is lowest risk).
- [ ] **Default URL/env precedence:** align `DATABASE_URL`/`SQLITE_PATH` legacy reads with canonical `FLORA_DATABASE__*` behavior in docs; avoid breaking existing deploys.

### `flora-core/src/config/messaging.rs`
- [ ] **Optionality contract:** currently `validate()` requires non-empty `valkey_url`; if Valkey is to be truly optional at boot, introduce `enabled` flag or allow empty/disabled value with matching `AppState` behavior. Coordinate carefully: runtime fallback exists, config currently enforces URL.
- [ ] **Env naming:** document `FLORA_MESSAGING__VALKEY_URL` as canonical; keep legacy `VALKEY_URL` fallback only if intended.

### `flora-core/src/config/search.rs`
- [ ] **Make `enabled` authoritative:** pair with `AppState` gating so `enabled=false` skips client/init and search features degrade cleanly.
- [ ] **URL/key defaults:** keep sensible dev defaults but ensure disabled mode does not require reachable Meilisearch.
- [ ] **Template/index helpers:** no change needed; low-risk.

### `flora-core/src/config/storage.rs`
- [ ] **Backup/permissions note:** reinforce persistent local path and quota awareness for small SSD VPS; validate local path writability at startup as a later improvement (NV).
- [ ] **No immediate code change required.**

### `flora-search/src/lib.rs`
- [ ] **No constructor behavior change required for quick win** except supporting optional initialization from callers; consider lightweight `is_healthy`/ping helper later for readiness.
- [ ] **Failure semantics:** keep typed errors; callers must handle disabled/degraded mode explicitly.

### `flora-messaging/src/lib.rs`
- [ ] **No change required for degraded boot:** service is already only created when Valkey connection exists.
- [ ] **Ops guidance:** if Valkey enabled on small VPS, document memory ceiling/eviction and AOF rewrite overhead; avoid assuming durability unless configured and tested.

## Minimal backup guidance (low-risk)

| Data | Priority | Minimal action | Restore note |
|---|---|---|---|
| PostgreSQL (`flora` DB) | Critical | Scheduled `pg_dump` (compressed/custom as needed), off-host retention, least-privilege creds; run off-peak. | Restore into fresh PG18+ (`psql`/`pg_restore`), run migrations/app startup, verify smoke path (`docs/dev-smoke.md`). |
| Local storage (`FLORA_STORAGE__LOCAL_PATH`) | High | Backup mounted path/files with DB-consistent timing where possible. | Restore files + verify permissions/paths before serving uploads. |
| Valkey (`/data`, AOF/RDB if enabled) | Low–medium | Backup only if product requires durable messaging/cache state; otherwise treat rebuildable. | Recreate container/volume; app already tolerates missing Valkey in some paths. |
| Meilisearch (`/meili_data`) | Low | Optional snapshot/volume backup; prefer rebuild/reindex from Postgres if path exists. | Restore volume or reindex after DB recovery. |
| Runbook | Required | Periodic restore drill on disposable PG18 volume + migration/startup smoke. | pt-BR: testar restauração; es: probar restauración; de: Wiederherstellung testen; fr: tester la restauration. |

## NVs (needs verification / non-goals in this quick-win pass)

- **NV-1:** Runtime feature coupling to search/messaging beyond boot (routes/services/UI expectations) not exhaustively traced; making search truly optional may require degraded responses or feature flags at API handlers.
- **NV-2:** No in-repo runtime benchmarks; memory caps/pool sizes are conservative starting points for 2 vCPU/4 GB and must be validated with `docker stats`, Postgres metrics, and load smoke.
- **NV-3:** Compose `deploy.resources` enforcement varies (Swarm vs non-Swarm/`--compatibility`); verify actual cgroup limits in target environment.
- **NV-4:** Multilingual web retrieval in this environment was stronger for EN authoritative docs; pt-BR/es/de/fr recommendations include terminology and operational mapping but should be complemented with locale-specific runbooks where required.
- **NV-5:** SQLite fallback, S3/object-storage production path, and full OIDC/Zitadel variable unification may exceed "low-risk quick win" depending on consumers/tests.
- **NV-6:** CI has no full DB+Valkey+Meilisearch integration smoke; quick wins touching boot/config should be validated with targeted local/manual smoke.
