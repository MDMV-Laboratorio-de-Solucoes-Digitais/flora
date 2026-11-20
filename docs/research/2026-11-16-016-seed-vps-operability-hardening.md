# Seed/VPS Operability Hardening — Quick Wins (Multilingual Research + Codebase Cross-check)

**Date:** 2026-11-16  
**Scope:** Low-risk operability quick wins for running Flora on a Seed/VPS profile (target assumption: **2 vCPU / 4 GB RAM / SSD**, single-node, low concurrency): make Meilisearch/Valkey truly optional at boot/config (degraded modes), add Compose resource limits/profiles/healthchecks, align `FLORA_*` env/config/pool settings, improve startup resilience, and provide minimal backup/restore guidance for PostgreSQL + local storage.  
**Constraint:** Research-only deliverable; **no application source modified**.

## Evidence inspected (repo)

- `docker-compose.yml` — PG18 (`postgres:18-alpine`), Valkey 8 AOF, Meilisearch v1.12; localhost binds, healthchecks, named volumes; **no profiles/resource limits/API service**; dev secrets (`postgres/postgres`, `dev-meilisearch-master-key`).
- `.env.example` — Figment-oriented `FLORA_*`/`FLORA_*__*` keys mixed with non-prefixed `OIDC_*`/`ZITADEL_*`; search vars commented as optional MVP; storage default `/var/flora/storage`.
- `flora-api/src/main.rs` — load/validate config, create pool, run migrations, build `AppState`, serve Axum; **pool+migrations duplicated vs `AppState::new`**.
- `flora-api/src/state/app_state.rs` — creates a second pool; **always initializes `SearchService::new(...)` and fails startup on error** (ignores `config.search.enabled`); Valkey connection is best-effort with warn+`None` fallback; `messaging_service`/OIDC tx store degrade.
- `flora-core/src/database.rs` — hardcoded `PgPoolOptions::new().max_connections(20)`; ignores `DatabaseConfig` pool/timeout fields.
- `flora-core/src/config/{mod,app,database,messaging,search,storage,oidc,notifications}.rs` — Figment merge defaults + `FLORA_`/`__`; DB config exposes unused pool knobs and unsupported-at-runtime SQLite surface (`sqlite_path`/`is_sqlite`); messaging validation requires non-empty `valkey_url`; search has `enabled` (default true) but boot path not gated; storage local default `/var/flora/storage`.
- `flora-search/src/lib.rs` — Meilisearch client wrapper/index/search operations; constructor can fail; no optional/degraded adapter in crate.
- `flora-messaging/src/lib.rs` — Valkey Pub/Sub publish service; only instantiated when connection exists.
- Prior audits: `docs/research/2026-11-16-seed-vps-profile-audit.md`, `docs/research/2026-11-16-014-seed-vps-operability-international.md`, `docs/research/2026-11-16-015-seed-vps-operability-quickwins.md`.

## Multilingual internet research cross-check (EN, pt-BR, es, de, fr, zh-CN, ja)

| Theme | EN (authoritative/practical) | pt-BR | es | de | fr | zh-CN / ja (operator cues) | Low-risk implication for Flora |
|---|---|---|---|---|---|---|---|
| Optional services / profiles | Docker Compose `healthcheck`, `depends_on` conditions, and deploy/resource controls support optional overlays and readiness gating. | *perfis opcionais, dependências opcionais, degradação controlada* | *perfiles opcionales, dependencias degradables* | *optionale Profile, degradierte Abhängigkeiten* | *profils optionnels, dépendances dégradables* | zh-CN: 可选服务/降级模式/健康检查; ja: オプションサービス/段階的縮退/ヘルスチェック | Keep Postgres mandatory; make Valkey/Meilisearch profile-gated and boot-degradable where semantics allow. |
| Resource limits (small VPS) | Compose `deploy.resources.limits/reservations`; container memory caps prevent one service starving others; Meilisearch/Valkey docs recommend explicit memory ceilings. | *limites de memória/CPU, evitar OOM* | *límites de memoria/CPU* | *Ressourcenbegrenzungen, OOM vermeiden* | *limites de ressources, éviter l’OOM* | zh-CN: 内存/CPU 限制; ja: メモリ/CPU 制限 | Add conservative caps for 4 GB envelope and validate with `docker stats`; treat Meilisearch as highest-risk memory spike when enabled. |
| Meilisearch operability | `MEILI_ENV`, `MEILI_MASTER_KEY`, `MEILI_NO_ANALYTICS`, `MEILI_MAX_INDEXING_MEMORY`, `MEILI_MAX_INDEXING_THREADS`, `MEILI_EXPERIMENTAL_SEARCH_QUEUE_SIZE` help constrain small instances; health endpoint `/health` usable in Compose. | *limitar memória de indexação, chave mestra fora de dev* | *limitar memoria de indexación* | *Indexierungsspeicher begrenzen* | *limiter la mémoire d’indexation* | zh-CN: 限制索引内存/线程; ja: インデックス用メモリ/スレッド制限 | Gate init by `search.enabled`; when enabled on VPS, set indexing memory/thread limits and keep data volume persistent. |
| Valkey/Redis memory & persistence | Valkey admin/docs/guides: set `maxmemory` + explicit eviction policy, size for overhead/fragmentation/fork spikes (AOF rewrite/RDB), enable swap-awareness/monitoring; `valkey-cli ping` healthcheck. | *definir `maxmemory` e política de expulsão; AOF consome pico* | *configurar `maxmemory` y política de expulsión* | *`maxmemory` und Eviction-Policy setzen* | *définir `maxmemory` et la politique d’éviction* | zh-CN: 设置 maxmemory/淘汰策略; ja: maxmemory と退避ポリシー設定 | For Seed, prefer small cap + explicit policy; treat as rebuildable unless messaging/OIDC state requires durability. |
| PostgreSQL small-RAM tuning & backup | PostgreSQL docs/wiki: modest `shared_buffers`/`work_mem`/`maintenance_work_mem`, controlled `max_connections`, regular `pg_dump`/`pg_restore` (or `pg_dumpall --globals-only`), restore drills; in Docker avoid TTY on dumps/restores and keep data on persistent volume. | *backup lógico `pg_dump`, teste de restauração* | *copias `pg_dump`, probar restauración* | *logische Sicherung, Wiederherstellung testen* | *sauvegarde logique, tester la restauration* | zh-CN: 定期备份与恢复演练; ja: 定期バックアップと復元テスト | Critical path = Postgres + storage backup; keep PG18 volume layout (`/var/lib/postgresql`) and test restore on fresh PG18 before trust. |

**Sources consulted (selection):** PostgreSQL docs/wiki (resource config, SQL dump/restore), Docker Compose docs (deploy resources/healthcheck), Meilisearch self-hosting/config docs and runtime flags, Valkey docs/guides on memory/admin/persistence, and practical VPS-oriented Compose/backup guides cross-checked against repo behavior. NV: automated retrieval/validation depth varied by locale (pt-BR/es/de/fr stronger than zh-CN/ja primary-source parity in this run); locale notes should be validated in operator runbooks where compliance/process language matters.

## File-by-file low-risk checklist

### `docker-compose.yml`
- [ ] **Profiles/overlays (lean core):** add `core` (postgres + optional api), `realtime` (valkey), `search` (meilisearch) so Seed can run without optional services.
- [ ] **Resource caps (starting envelope, 4 GB):** postgres ~768M–1G, valkey ~96M–128M, meilisearch ~384M–512M (or off), API ~256M–512M; reserve ~512M OS/headroom. Verify enforcement mode (`deploy.resources` vs non-Swarm/`--compatibility`/runtime-specific keys) with `docker stats`.
- [ ] **Healthchecks & dependencies:** keep service healthchecks; API should `depends_on` postgres healthy; valkey/meilisearch must be optional deps when enabled by profile.
- [ ] **Meilisearch hardening:** keep localhost bind/persistent volume; set `MEILI_ENV`, strong non-dev `MEILI_MASTER_KEY`, `MEILI_NO_ANALYTICS`; for small VPS add `MEILI_MAX_INDEXING_MEMORY` and optionally `MEILI_MAX_INDEXING_THREADS` / `MEILI_EXPERIMENTAL_SEARCH_QUEUE_SIZE`.
- [ ] **Valkey hardening:** keep localhost bind/persistent `/data`; for constrained VPS set `--maxmemory` + explicit policy aligned to semantics (`noeviction` if correctness-sensitive, cache policy only if safe); account for AOF rewrite fork memory spikes.
- [ ] **Postgres hardening:** keep PG18 image/volume at `/var/lib/postgresql`; optional small-VPS command/env tuning (`shared_buffers`, `work_mem`, `maintenance_work_mem`, `max_connections`) documented as conservative starting points.
- [ ] **Secrets/ports:** remove/override dev credentials before non-local use; never publish postgres/valkey/meilisearch publicly.

### `.env.example`
- [ ] **Canonicalize `FLORA_*` contract:** document Figment keys/prefix split (`FLORA_` + `__`) for database/messaging/search/storage/app/notifications; mark legacy env names as compatibility-only if supported.
- [ ] **Optionality toggles:** add explicit `FLORA_SEARCH__ENABLED=false` lean example and clarify Meilisearch/Valkey degraded-mode expectations; align comments with actual boot behavior after code fix.
- [ ] **OIDC/Zitadel alignment:** reconcile `OIDC_*`/`ZITADEL_*` vs `FLORA_OIDC__*` to prevent silent misconfig.
- [ ] **Pool/timeout knobs:** add `FLORA_DATABASE__MAX_CONNECTIONS`, `MIN_CONNECTIONS`, `ACQUIRE_TIMEOUT_SECS`, `IDLE_LIFETIME_SECS` once wired, with small-VPS defaults.
- [ ] **Storage/backup note:** emphasize persistent writable `FLORA_STORAGE__LOCAL_PATH` and include it in backup scope.
- [ ] **No real secrets:** placeholders only; require strong `FLORA_JWT_SECRET` (>=32 chars in production).

### `flora-api/src/main.rs`
- [ ] **Single startup path:** avoid duplicate pool creation/migration (currently created here and again in `AppState::new`); pass shared pool/handles into state.
- [ ] **Fail-fast boundaries:** keep hard-fail for config validation, Postgres connectivity, and migrations; degrade only truly optional services.
- [ ] **Startup capability logging:** emit DB required/ready, search `enabled|disabled|degraded`, valkey `connected|degraded` for VPS diagnostics.
- [ ] **Readiness (future, low-risk additive):** expose/plan liveness vs readiness reflecting dependency criticality.

### `flora-api/src/state/app_state.rs`
- [ ] **Gate Meilisearch by config:** initialize only when `config.search.enabled`; if disabled, skip init cleanly. If enabled, choose explicit policy: fail-fast or log-and-degrade (documented) — required to make search truly optional.
- [ ] **Optional search abstraction:** support `Option<Arc<SearchService>>` or null/degraded adapter; audit route/service call sites before rollout (NV coupling).
- [ ] **Reuse pool:** accept injected pool instead of creating another pool.
- [ ] **Valkey remains degradable:** keep warn+continue behavior; surface connection state for health/capability reporting.

### `flora-core/src/database.rs`
- [ ] **Wire pool config:** apply `max_connections`, `min_connections`, `acquire_timeout`, `idle_timeout` from `DatabaseConfig` (currently hardcoded `max_connections(20)`).
- [ ] **Seed/VPS-safe defaults:** recommend lower concurrency (e.g. 5–10) aligned with Postgres `max_connections` and 4 GB RAM envelope.
- [ ] **Preserve error mapping:** no behavior change beyond configurability.

### `flora-core/src/config/mod.rs`
- [ ] **Validation coherence:** ensure `search.enabled=false` is a fully valid lean profile end-to-end once `AppState` honors it.
- [ ] **Compatibility shims (optional/additive):** accept legacy env aliases with deprecation warnings only if needed for migration safety.

### `flora-core/src/config/app.rs`
- [ ] **No risky logic change:** keep production JWT/session validation; document reverse-proxy/base URL/CORS/OTEL considerations for VPS.
- [ ] **NV:** consider explicit readiness/health toggles later if endpoints added.

### `flora-core/src/config/database.rs`
- [ ] **Honest runtime contract:** SQLite fields (`sqlite_path`, `is_sqlite`) imply fallback but runtime is Postgres-only; lowest-risk fix is docs/deprecation marking until implemented.
- [ ] **Env precedence clarity:** align legacy `DATABASE_URL`/`SQLITE_PATH` behavior with canonical `FLORA_DATABASE__*` docs.

### `flora-core/src/config/messaging.rs`
- [ ] **Make optionality representable:** runtime can run without Valkey, but config validation requires non-empty URL. Low-risk options: add `enabled` flag, allow empty/disabled sentinel, or validate conditionally by profile/policy — must match `AppState` and docs.
- [ ] **Canonical env:** prefer `FLORA_MESSAGING__VALKEY_URL`; keep legacy fallback only intentionally.

### `flora-core/src/config/search.rs`
- [ ] **Authoritative `enabled`:** disabled mode must not require reachable Meilisearch; pair with boot gating and degraded feature handling.
- [ ] **Safe defaults:** keep dev defaults but make production/VPS require explicit URL/key when enabled.
- [ ] **Index template helpers:** no change required.

### `flora-core/src/config/storage.rs`
- [ ] **Operational checks:** document persistent mount, permissions, quota/disk monitoring on small SSD; optional startup writability check is a later improvement.
- [ ] **No immediate code change required.**

### `flora-core/src/config/oidc.rs`, `flora-core/src/config/notifications.rs`
- [ ] **Env/doc alignment:** ensure OIDC/notification settings map clearly to `FLORA_*` names and degraded external dependencies are documented (OIDC fallback behavior, notification delivery expectations).
- [ ] **No quick-win functional change required.**

### `flora-search/src/lib.rs`
- [ ] **Support optional callers:** keep constructor/errors; consider lightweight health/ping helper for readiness diagnostics (additive).
- [ ] **No forced behavior change:** degraded handling belongs at integration boundary (`AppState`/routes).

### `flora-messaging/src/lib.rs`
- [ ] **Keep degraded-friendly design:** service instantiated only with live connection.
- [ ] **Document VPS constraints:** when enabled, set Valkey `maxmemory`/policy and persistence according to semantics; monitor latency targets/logs.

## Compose/config examples (low-risk, illustrative)

> Examples are intentionally conservative for 2 vCPU / 4 GB and assume non-Swarm local Compose unless noted; adjust to actual runtime and verify limit enforcement.

### Example A — Core profile (Postgres only, lean Seed)

```yaml
# docker-compose.seed.yml (illustrative)
services:
  postgres:
    image: postgres:18-alpine
    restart: unless-stopped
    environment:
      POSTGRES_USER: ${POSTGRES_USER:-postgres}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD:-change-me}
      POSTGRES_DB: ${POSTGRES_DB:-flora}
    ports:
      - "127.0.0.1:5432:5432"
    volumes:
      - flora_postgres_data:/var/lib/postgresql
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER:-postgres} -d ${POSTGRES_DB:-flora}"]
      interval: 5s
      timeout: 5s
      retries: 10
    # Non-Swarm Compose may require runtime-specific keys or `--compatibility` for deploy.*
    deploy:
      resources:
        limits:
          cpus: "1.00"
          memory: 1G
        reservations:
          memory: 512M

volumes:
  flora_postgres_data:
```

### Example B — Optional Valkey profile (degraded/realtime)

```yaml
services:
  valkey:
    image: valkey/valkey:8-alpine
    profiles: ["realtime"]
    restart: unless-stopped
    ports:
      - "127.0.0.1:6379:6379"
    volumes:
      - flora_valkey_data:/data
    command:
      [
        "valkey-server",
        "--appendonly", "yes",
        "--appendfsync", "everysec",
        "--save", "",
        "--maxmemory", "96mb",
        "--maxmemory-policy", "noeviction"
      ]
    healthcheck:
      test: ["CMD", "valkey-cli", "ping"]
      interval: 5s
      timeout: 3s
      retries: 10
    deploy:
      resources:
        limits:
          cpus: "0.50"
          memory: 128M
volumes:
  flora_valkey_data:
```

### Example C — Optional Meilisearch profile (search)

```yaml
services:
  meilisearch:
    image: getmeili/meilisearch:v1.12
    profiles: ["search"]
    restart: unless-stopped
    environment:
      MEILI_ENV: ${MEILI_ENV:-production}
      MEILI_MASTER_KEY: ${MEILI_MASTER_KEY:?set-strong-master-key}
      MEILI_NO_ANALYTICS: "true"
      MEILI_MAX_INDEXING_MEMORY: ${MEILI_MAX_INDEXING_MEMORY:-256Mb}
      # Optional: MEILI_MAX_INDEXING_THREADS: "1"
      # Optional: MEILI_EXPERIMENTAL_SEARCH_QUEUE_SIZE: "200"
    ports:
      - "127.0.0.1:7700:7700"
    volumes:
      - flora_meili_data:/meili_data
    healthcheck:
      test: ["CMD", "curl", "-fsS", "http://127.0.0.1:7700/health"]
      interval: 5s
      timeout: 5s
      retries: 10
    deploy:
      resources:
        limits:
          cpus: "1.00"
          memory: 512M
volumes:
  flora_meili_data:
```

### Example D — `FLORA_*` env alignment (illustrative `.env`)

```dotenv
# App
FLORA_ENV=production
FLORA_APP__HOST=0.0.0.0
FLORA_APP__PORT=3000
FLORA_APP__BASE_URL=https://flora.example.com
FLORA_APP__JWT_SECRET=<32+ chars random>
FLORA_APP__SESSION_TTL_SECS=3600

# Database (Postgres required)
FLORA_DATABASE__POSTGRES_URL=postgresql://flora:change-me@127.0.0.1:5432/flora
# After wiring pool config in create_pool():
FLORA_DATABASE__MAX_CONNECTIONS=8
FLORA_DATABASE__MIN_CONNECTIONS=1
FLORA_DATABASE__ACQUIRE_TIMEOUT_SECS=30
FLORA_DATABASE__IDLE_LIFETIME_SECS=600

# Optional services / degraded modes
FLORA_SEARCH__ENABLED=false
# FLORA_SEARCH__URL=http://127.0.0.1:7700
# FLORA_SEARCH__API_KEY=<meili-master-or-search-key>

# If messaging made truly optional by config contract:
# FLORA_MESSAGING__ENABLED=false
FLORA_MESSAGING__VALKEY_URL=redis://127.0.0.1:6379

# Storage (persistent, backed up)
FLORA_STORAGE__BACKEND=local
FLORA_STORAGE__LOCAL_PATH=/var/flora/storage
```

## Startup resilience (low-risk targets)

1. **Dependency classification:** required = Postgres/migrations/config; optional/degradable = Meilisearch, Valkey (subject to feature semantics). Reflect in boot and health/readiness.
2. **Idempotent single init:** one pool, one migration run, one state assembly path.
3. **Graceful degradation:** `search.enabled=false` or unavailable optional search should not prevent API boot if routes can degrade; Valkey failures should continue to log and expose degraded state.
4. **Bounded resources:** pool size + container caps + service memory knobs (`shared_buffers/work_mem`, `MEILI_MAX_INDEXING_MEMORY`, `--maxmemory`) reduce OOM/cascade risk.
5. **Operational visibility:** startup summary and (additive) readiness endpoint reporting `required/optional` dependency states.

## Minimal backup/restore guidance (PostgreSQL + local storage)

| Asset | Priority | Minimal backup | Minimal restore | Notes |
|---|---|---|---|---|
| PostgreSQL (`flora`) | **Critical** | Scheduled `pg_dump -Fc` (or plain SQL if preferred) + periodic `pg_dumpall --globals-only` for roles if used; run off-peak, store off-host, least privilege. In Docker use `docker compose exec -T` (no TTY). | Restore with `pg_restore`/`psql` into fresh **PG18+** instance (respect `template0`/create flow as needed), then run Flora migrations/startup smoke (`docs/dev-smoke.md`). | Test restores regularly (pt-BR: *testar restauração*; es: *probar restauración*; de: *Wiederherstellung testen*; fr: *tester la restauration*; zh-CN: 恢复演练; ja: 復元テスト). |
| Local storage (`FLORA_STORAGE__LOCAL_PATH`) | **High** | Back up mounted path/files with consistent schedule; include permissions/ownership metadata when possible. | Restore files to persistent path, verify writability/ownership, then validate uploads/downloads. | Coordinate timing with DB backups for consistency where feasible. |
| Valkey `/data` | Low–Medium | Snapshot volume/AOF only if state must survive restart; otherwise treat rebuildable. | Restore volume or start empty; confirm app degraded/healthy behavior. | Set `maxmemory`/policy; AOF rewrite can spike memory. |
| Meilisearch `/meili_data` | Low | Optional volume snapshot; prefer rebuild/reindex from Postgres source of truth if supported. | Restore volume or reindex after DB recovery. | Constrain indexing memory on small VPS. |

**Minimal runbook:** provision fresh VPS/container host → restore Postgres to PG18 volume → restore storage path → start core profile → verify migrations/health/smoke → enable optional profiles only after core is stable.

## NVs (needs verification / follow-up)

- **NV-1 Boot/feature coupling:** route and service call sites may assume non-null search/messaging; making them truly optional may require degraded responses/feature flags beyond `AppState` gating.
- **NV-2 No runtime benchmarks:** memory caps, pool sizes, and Postgres/Meilisearch/Valkey knobs are conservative starting points for 2 vCPU/4 GB and must be validated under representative load.
- **NV-3 Compose limit enforcement:** `deploy.resources` behavior differs by environment (Swarm/non-Swarm/`--compatibility`/cgroup config); verify actual limits and OOM behavior.
- **NV-4 Config contract migration:** changing `messaging.valkey_url` validation or env naming can break existing deployments/tests; prefer additive compatibility shims and explicit release notes.
- **NV-5 SQLite/object storage claims:** config/docs imply capabilities not fully operative (SQLite fallback, full S3 path evidence in compose); avoid overstating support without implementation/tests.
- **NV-6 Multilingual primary-source parity:** pt-BR/es/de/fr/zh-CN/ja guidance here is cross-checked from authoritative EN docs and operator terminology; locale-specific compliance/runbook wording may require native-source confirmation.
- **NV-7 CI coverage:** no full DB+Valkey+Meilisearch integration smoke in workflows; operability changes need targeted local/manual smoke and restore drills.
