# 022 — Meilisearch on cheap VPS: resource sizing, safe config, ops, and when to stay on Postgres FTS (multilingual)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` — research-only deliverable (this document only).  
**Scope:** High-reliability, multilingual (EN, pt-BR, es, de, fr, zh-CN, ja) guidance for running **optional Meilisearch** on a **single-node VPS (~2 vCPU / 4 GB RAM / SSD)** alongside **PostgreSQL + Rust/Axum API** (Flora Seed profile). Covers resource sizing and limits (indexing memory/threads, search performance, disk/LMDB considerations), safe configuration flags/env vars, startup/health behavior, rebuild/reindex impact, when to disable Meilisearch and rely on Postgres FTS, and 80/20 operational guidance for SMB self-hosting.  
**Version/context:** Flora compose pins `getmeili/meilisearch:v1.12` (`docker-compose.yml`); Meilisearch primary docs cited are current v1 documentation pages (feature names/flags can differ across v1 minors—verify against the exact image/version in use) [M1,M2,M3]. PostgreSQL references target current docs (Flora requires **PostgreSQL 18+**) [P1,P2,P3]. Flora context: 021 delivered tri-state task unassign and DB-gated PG FTS coverage; 022 is the preparation slot for minimal Meilisearch indexing hooks/parity or explicit degradation/rebuild contract [F1].

**Method:** Repo/code inspection (`.env.example`, `docker-compose.yml`, `flora-core` search config, `ROADMAP/OBJETIVO`) cross-checked with prior Flora research [F2–F7] and primary Meilisearch/PostgreSQL documentation [M1–M3,P1–P3]. Web-search tooling in this environment returned no usable results; evidence is anchored to primary docs and in-repo artifacts. **No runtime benchmarks** on 2 vCPU/4 GB were run here; environment-specific RSS/latency/throughput claims are **NV** unless explicitly sourced.

## 1. Flora cross-check (current state and 022 intent)

| Evidence | What it means for ops/sizing |
|---|---|
| `OBJETIVO.md`, `ROADMAP.md` [F1] | North star: self-hosted PME on cheap VPS, **Postgres-first**, optional components. MVP P0 = basic search via Postgres FTS; external engines optional. **022:** “paridade Meilisearch (indexação/reindex/degradação/coerência com PG)” — parity is not complete/guaranteed by default. |
| `flora-core/src/config/search.rs` [F2] | `SearchConfig { enabled, url, api_key, default_limit, max_limit, index_template }` (defaults: `enabled=true`, `url=http://localhost:7700`, env `MEILISEARCH_URL`/`MEILISEARCH_API_KEY`, index `flora_org_{org_id}`). Validation requires URL when enabled and sane limits. **NV:** boot/runtime coupling to Meilisearch in API/state must be treated as an operability risk until verified optional in all paths (prior audits flagged coupling). |
| `docker-compose.yml` [F3] | Lean Seed default: **Postgres only**; `meilisearch` is behind `profiles: ["search"]`. Pinned `v1.12`, `MEILI_ENV: development`, `MEILI_NO_ANALYTICS: "true"`, dev master key, `MEILI_MAX_INDEXING_MEMORY: 256Mb`, `MEILI_MAX_INDEXING_THREADS: "1"`, localhost bind `127.0.0.1:7700`, healthcheck `curl -fsS /health`, volume `flora_meili_data:/meili_data`, container hints `mem_limit: 768m`, `cpus: "0.75"`. |
| `.env.example` [F4] | Search section present but commented/minimal (`FLORA_SEARCH__ENABLED`, `FLORA_SEARCH__URL`, `FLORA_SEARCH__API_KEY`) — lean core can disable; operators must align canonical `FLORA_*__*` config with runtime behavior. |
| Prior research [F5,F6,F7] | PG FTS is the 80/20 correctness baseline and simplest for 4 GB; Meilisearch is a derived, rebuildable projection requiring lifecycle hooks/outbox/rebuild and must never block core DB writes or app boot; parity across backends is expensive and usually unnecessary. |
| `flora-tests` / `docs/dev-smoke.md` context [F1,F3] | Lean validation focuses on `api+postgres`; Valkey/Meilisearch are optional/rebuildable in Seed profile. |

**Working contract for 022 ops:** PostgreSQL is the **source of truth** and fallback baseline; Meilisearch, if enabled, is an **optional, capability-gated, rebuildable** accelerator with bounded resources and explicit degradation behavior.

## 2. Multilingual evidence summary (EN, pt-BR, es, de, fr, zh-CN, ja)

> Primary docs are language-neutral/EN; the table below combines primary signals with locale-oriented operational guidance for SMB/self-hosted PME deployments on constrained VPS. Production benchmarks for Flora’s exact workload/locales are **NV**.

| Lang | Primary/operational signal | Implication on ~2 vCPU / 4 GB SSD |
|---|---|---|
| **EN** | Meilisearch is configured at launch via CLI/env/config file (precedence: CLI > env > file); env vars use `MEILI_` + uppercase, flags accept `n/no/f/false/off/0` as false [M1]. Indexing/search run as asynchronous tasks; task backlog/queue and instance limits matter [M3]. Storage relies on LMDB with memory-mapped behavior (virtual memory ≠ RSS) and known hard limits (payload defaults, search queue/concurrency controls, DB/index/task-DB limits, large-batch/internal-error risks) [M2,M3]. Postgres FTS is in-DB/transactional with GIN/GiST maintenance and `pg_trgm` for fuzzy/trigram cases [P1,P2,P3]. | Treat Meilisearch as a bounded optional sidecar: cap indexing memory/threads, keep payloads/indexes small, monitor tasks/disk, and default to PG-only when headroom is tight. |
| **pt-BR** (PME/auto-hospedado) | Em VPS barata, cada serviço extra consome RAM, disco, tempo de operação e aumenta pontos de falha (backup, atualização, restauração). Banco único simplifica consistência. | Perfil enxuto **`api+postgres`** por padrão; Meilisearch só com perfil `search`, limites, healthcheck e procedimento de rebuild a partir do Postgres. |
| **es** (operación/hosting) | 2 vCPU/4 GB es habitual pero justo; DB + API ya ocupan la mayor parte del margen. Servicios auxiliares deben ser degradables. | Activar Meilisearch solo si aporta UX medible; limitar hilos/memoria de indexado y desactivar ante presión de memoria/swap. |
| **de** (Betrieb/Zuverlässigkeit) | Zusätzliche Dienste erhöhen Betriebs- und Wiederherstellungsaufwand; Konsistenz ist einfacher, wenn der Suchindex aus der führenden DB reproduzierbar ist. | PostgreSQL als führendes System/FTS-Fallback; Meilisearch nur optional, versioniert, mit klarer Reindex-/Degradationsstrategie. |
| **fr** (exploitation PME) | Sur petit VPS, privilégier la sobriété: moins de composants, moins de synchronisation, reprise plus simple. | Par défaut Postgres FTS; Meilisearch en profil dédié, borné (CPU/RAM/indexation), et jamais bloquant pour le cœur applicatif. |
| **zh-CN** (小规格 VPS 运维) | 2核4G 需优先保障数据库与 API；Meilisearch 有独立进程、LMDB 映射存储与异步任务队列，索引/重建会带来内存与 IO 峰值；PG 内建 FTS/`pg_trgm` 事务一致、运维面小 [M1–M3,P1–P3]. | 默认 **Postgres-only**；仅在明确体验收益（即时/容错/CJK 等）时启用 Meilisearch，并严格限流、可重建、可降级。 |
| **ja** (小規模VPS/運用) | 追加サービスは監視・障害点・復旧手順を増やす。Meilisearch はメモリマップド(LMDB)と非同期タスク特性があり、再インデックス時の負荷に注意；Postgres FTS は DB 内で一貫性を保ちやすい [M1–M3,P1–P3]. | まず Postgres FTS。Meilisearch は必要時のみ有効化し、インデックス資源上限・ヘルスチェック・PG からの再構築を必須化。 |

## 3. Resource sizing and limits (2 vCPU / 4 GB / SSD)

### 3.1 Planning budget (starting point, **NV** — validate with `docker stats`/RSS/IO/latency)

| Component | Lean Seed budget | Notes |
|---|---|---|
| OS + page cache/overhead | ~400–700 MB | Keep headroom; avoid swap on SSD-constrained VPS. |
| PostgreSQL 18 | ~768–1280 MB | Co-locates data + PG FTS indexes; tune connections/`work_mem`/buffers for small VPS [P1]. |
| `flora-api` (Rust) | ~256–512 MB | Bound pools/workers; app must not depend on Meilisearch for boot/core CRUD. |
| **Meilisearch (optional `search` profile only)** | **~256–512 MB practical ceiling** when co-located (**NV**) | Separate daemon with LMDB memory-mapped storage: virtual address usage can look high without equal RSS, but indexing/search/task pressure still competes for RAM/IO [M1,M2]. Flora compose hint: `mem_limit 768m` is an upper guardrail, not a target steady state. |
| Headroom for spikes/rebuild | required | Reindexing, large payloads, or concurrent search+index can exhaust 4 GB quickly. |

**Rule of thumb:** on 4 GB, prefer **`api+postgres` only**. Add Meilisearch only when corpus/UX justify it and after measuring steady-state + rebuild peaks. If swap appears or DB p99 degrades, **disable the `search` profile**.

### 3.2 Indexing memory/threads and search performance

| Control (Meilisearch v1 config) | What it does (docs) | 80/20 guidance for 2 vCPU/4 GB |
|---|---|---|
| `--max-indexing-memory` / `MEILI_MAX_INDEXING_MEMORY` [M1] | Caps RAM available to a single indexer (indexing is RAM-intensive); too low can fail/slow indexing. | **Start very low and fixed** (Flora uses `256Mb` hint) to protect Postgres/API; raise only with measured headroom. Treat as co-tenancy guard, not a guarantee of total process footprint (**NV**). |
| `--max-indexing-threads` / `MEILI_MAX_INDEXING_THREADS` [M1] | Caps threads used during indexing. | **Use `1`** (2 max only if proven safe). Prevents indexing from starving 2 vCPU shared with DB/API. |
| Async indexing tasks [M3] | Document/settings operations enqueue tasks processed asynchronously; backlog/processing state is observable via tasks API. | Do not assume instant freshness after writes; design for eventual consistency in Meilisearch and rely on PG fallback for correctness. Monitor task queue during imports/rebuilds. |
| Search concurrency/queue [M2] | Instance has search request limits (e.g. max concurrent search requests with 503/`Retry-After` behavior per known limitations). | Keep query load modest on shared VPS; avoid high fan-out/auto-complete storms unless benchmarked. |
| Interactive relevance | Meilisearch targets typo-tolerant, fast search UX (engine strengths vs PG tradeoffs are use-case dependent). | Only pay this cost when UX requires it (instant/prefix/typo/facets/CJK cases validated) [F5,F6]. |

### 3.3 Disk and LMDB considerations

| Topic | Evidence / implication | 80/20 ops guidance |
|---|---|---|
| **Storage engine** | Meilisearch uses **LMDB** with memory-mapped files [M2 context as documented behavior; confirm per exact version]. Virtual memory/address space and OS page cache behavior differ from RSS. | Size RAM caps conservatively; monitor actual RSS, page cache pressure, and IO latency—not just container `mem_limit`. |
| **SSD/IO** | Indexing, dumps/rebuilds, and task churn are IO-sensitive on small SSD VPS. | Keep indexes/data small; schedule rebuilds off-peak; ensure free disk headroom for DB WAL + Meilisearch data + snapshots. |
| **Data directory** | `--db-path` / `MEILI_DB_PATH` controls where Meilisearch stores data [M1]. Flora mounts `flora_meili_data:/meili_data`. | Persist on reliable volume; include in backup/restore plan or treat as **rebuildable from Postgres** (preferred for Seed). |
| **Payload/index limits** | Known limitations include default payload limits, max words/attributes/documents/index counts/DB sizes/task DB behavior, integer precision, filter depth, etc. [M2]. | Keep documents small (core fields/snippets/metadata), avoid unbounded payloads, and design for small multi-entity SMB corpora. |
| **Large batches** | Large indexing batches can trigger internal/resource issues (e.g. file descriptors) [M2]. | Batch conservatively; throttle rebuilds; prefer incremental/idempotent upserts over giant one-shot imports on 4 GB. |

## 4. Safe configuration flags / environment variables

### 4.1 Recommended (Seed/VPS-safe baseline)

| Setting | Flora/current example | Purpose | Recommendation |
|---|---|---|---|
| `--env` / `MEILI_ENV` | `development` in local compose [F3] | Runtime mode. | Use **`production`** (with proper master key handling) for non-local/self-hosted deployments; keep `development` only for local dev. |
| `--master-key` / `MEILI_MASTER_KEY` | dev key in compose [F3] | Protects all routes except health in production semantics; required for secured setups. | **Never reuse dev keys.** Store as secret, rotate as needed, restrict exposure. |
| `--http-addr` / `MEILI_HTTP_ADDR` | (default bind behavior; compose publishes localhost) [M1,F3] | Listen address. | Bind/publish **localhost/private network only** on single-node VPS; front with trusted reverse proxy/app network rules if exposed. |
| `--db-path` / `MEILI_DB_PATH` | `/meili_data` volume [F3] | Persistent data location. | Place on SSD with space; treat as disposable if rebuild-from-PG is guaranteed. |
| `--no-analytics` / `MEILI_NO_ANALYTICS` | `true` [F3] | Disables telemetry. | **Enable (true)** for privacy/least-surprise self-hosting. |
| `--max-indexing-memory` / `MEILI_MAX_INDEXING_MEMORY` | `256Mb` [F3] | Bounds indexer RAM. | Keep low (`256–512Mb` range to start, **NV**) and co-tune with DB/API. |
| `--max-indexing-threads` / `MEILI_MAX_INDEXING_THREADS` | `1` [F3] | Bounds indexing CPU parallelism. | **`1`** on 2 vCPU shared hosts. |
| `--http-payload-size-limit` / `MEILI_HTTP_PAYLOAD_SIZE_LIMIT` [M1] | (not set in compose) | Limits request body size. | Set an explicit, small limit aligned with expected doc/batch sizes to reduce abuse/OOM risk. |
| `--log-level` / `MEILI_LOG_LEVEL` [M1] | (default) | Observability. | `INFO` (or `WARN`) for steady ops; raise temporarily for incident/debug. |

### 4.2 Flags/options to treat with caution on 4 GB

| Setting | Why caution is needed |
|---|---|
| Disabling/loosening payload or indexing limits | Raises OOM/IO exhaustion risk [M1,M2]. |
| High `--max-indexing-threads` / high indexing memory | Can starve Postgres/API and trigger swap on 2 vCPU/4 GB [M1]. |
| `--experimental-*` (e.g. search queue/dumpless upgrade options when available in your version) [M1,M2] | Experimental behavior/operational semantics may change; validate explicitly before enabling in SMB production. |
| Snapshot/dump scheduling/features [M1] | Useful for portability but can add IO/CPU spikes; on Seed, prefer **logical source-of-truth in Postgres + rebuild** unless dump/restore is required. |
| Exposing HTTP endpoint publicly without auth/network controls | Increases attack surface; always pair with master/API keys and network restrictions. |

### 4.3 Flora app-side config knobs (must align with optionality)

| Key (canonical `FLORA_*__*`) | Intent | Ops note |
|---|---|---|
| `FLORA_SEARCH__ENABLED` | Disable search integration/profile | Default lean path should be **off/PG-only** for smallest VPS unless verified safe. |
| `FLORA_SEARCH__URL` | Meilisearch endpoint | Point to localhost sidecar only when profile enabled. |
| `FLORA_SEARCH__API_KEY` | Meilisearch auth | Keep secret; not required in unsecured local dev but required for protected instances. |
| `FLORA_SEARCH__DEFAULT_LIMIT` / `MAX_LIMIT` | Query bounding | Keep tight (e.g. 20/50–100) to cap response/search cost. |

> **NV:** exact env-key mapping/precedence between `.env.example`, `SearchConfig::default()` (`MEILISEARCH_*` fallbacks), and `FLORA_SEARCH__*` nested config should be verified in the running config loader before production rollout.

## 5. Startup and health behavior

| Aspect | Evidence | Recommended behavior (80/20) |
|---|---|---|
| **Process startup** | Meilisearch starts as a standalone service with configured DB path/options [M1]. Flora compose includes a healthcheck on `/health` [F3]. | Depend on `/health` (or equivalent) before routing search traffic; keep service optional in app orchestration. |
| **Readiness vs liveness** | `/health` indicates service responsiveness; indexing tasks may still be enqueued/processing [M3]. | Treat “healthy” as “API up”, not “index fully fresh”. Expose search capability/state separately if possible. |
| **App boot coupling** | Prior Flora audits indicated search init could be boot-critical in some paths [F5,F7] (**NV** for current HEAD). | **Target/require:** when search disabled or Meilisearch unavailable, API must still boot and serve core CRUD; search endpoint should degrade gracefully (PG fallback or explicit capability/503) without breaking health of core app. |
| **Task visibility** | Tasks API lists async operations and statuses [M3]. | On incidents, inspect task states (`enqueued/processing/succeeded/failed`) and backlog before restarting blindly. |
| **Failure modes** | OOM/IO pressure, bad master key/config, port/bind issues, corrupted/incompatible data dir, version mismatches. | Fail closed for Meilisearch feature, fail open for core platform: disable profile, fall back to Postgres FTS, rebuild index from PG after recovery. |

## 6. Rebuild/reindex impact (and mitigation)

| Scenario | Impact on 2 vCPU/4 GB | Mitigation |
|---|---|---|
| **Full reindex/rebuild from Postgres** | CPU, RAM, disk IO spike; task backlog; possible latency increase for DB/API; temporary staleness in Meilisearch results. | Run **scoped** (per org/entity) and **off-peak**; throttle batches; keep `max-indexing-threads=1` and low indexing memory; prefer one rebuild stream at a time. |
| **Initial bootstrap / backfill** | Can be heavier than steady-state if corpus/history is large. | Backfill in small chunks; monitor RSS, disk, and task completion; pause if DB pressure rises. |
| **Schema/settings changes** (searchable/filterable/sortable) | May require reindexing work inside Meilisearch. | Minimize churn: keep `filterableAttributes` minimal for current contract; batch config changes. |
| **Version upgrade/migration** | Compatibility and migration behavior depend on version; risk of downtime or rebuild needs. | Pin versions (`v1.12` today), test upgrades off-prod, keep PG as authoritative rebuild source. |
| **Delete/soft-delete/unassign updates** | Missing lifecycle hooks cause drift/stale hits [F6]. | Require idempotent upsert/delete hooks or outbox; periodic reconciliation/rebuild when drift detected. |
| **Meilisearch down during writes** | Derived index lags/misses events if not durably queued. | Never couple DB commit to Meilisearch; use outbox/async retry when enabled, or accept PG-only until rebuild. |

**Operational SLO suggestion (SMB, 80/20):** eventual consistency for Meilisearch is acceptable if **RPO for search index = rebuild-from-Postgres** and core product correctness never depends on Meilisearch freshness.

## 7. When to disable Meilisearch and rely on Postgres FTS

### 7.1 Prefer **Postgres FTS only** (default/recommended for Seed)

Disable (or do not enable) Meilisearch when any of the following hold:

- Host is **~2 vCPU / 4 GB** and must also run PostgreSQL + API reliably.
- Search needs are **basic keyword search** over core entities/metadata with simple ranking/snippets.
- **Consistency/freshness, simplicity, backup/restore, low ops** matter more than typo-tolerant instant UX [P1,P2].
- Team cannot operate an extra service (monitoring, upgrades, capacity, task backlog, rebuild playbooks).
- Indexing hooks/outbox/rebuild contract are incomplete (current 022 risk) or drift is observed.
- Memory pressure/swap/DB latency appears with `search` profile enabled.
- Multilingual needs are satisfied by Postgres dictionaries/configs and/or `pg_trgm` (**NV** per locale/corpus) [P3].

### 7.2 Consider enabling **optional Meilisearch** (non-parity, capability-gated)

Enable only with explicit profile, caps, and rebuild path when:

- Measured UX need for **instant/prefix search, typo tolerance, facets, or specific multilingual/CJK behavior** that PG cannot meet acceptably [F5,F6,F7].
- Corpus and query rate are small/bounded enough for 4 GB co-tenancy (**NV**).
- There is a documented and tested **rebuild-from-Postgres** procedure and degradation to PG/disabled search.
- Lifecycle handling (create/update/delete/soft-delete/unassign), tenant isolation, and contract normalization are implemented or explicitly scoped.

### 7.3 Avoid (for 80/20 SMB)

- **Dual-backend parity guarantees** (same ranking/snippets/results contract across PG and Meilisearch) — high test/ops cost, low payoff on 4 GB [F5,F7].
- Treating Meilisearch as authoritative or required for core writes/boot.

## 8. 80/20 operational guidance (SMB self-hosting)

1. **Default profile:** `api + postgres` only. Keep `search` (and `realtime`) opt-in compose profiles.
2. **Source of truth:** PostgreSQL. Meilisearch index/data is **derived and disposable**; design for full/partial rebuild from PG.
3. **Hard bounds:** `MEILI_MAX_INDEXING_THREADS=1`, conservative `MEILI_MAX_INDEXING_MEMORY` (start ~256Mb), explicit payload limit, localhost bind, `MEILI_NO_ANALYTICS=true`.
4. **Health & degradation:** gate on `/health`; never fail core boot on Meilisearch; on Meilisearch errors/timeouts/backlog, fall back to PG FTS or return a clear capability-limited response.
5. **Reindex safely:** scoped, throttled, off-peak, single stream; monitor tasks API, RSS, disk, DB latency; stop/retry rather than pushing through pressure.
6. **Backups:** prioritize consistent Postgres backups (including FTS structures/data as applicable). For Meilisearch, prefer rebuildability over complex snapshot ops on tiny hosts unless required.
7. **Observability (minimum):** container/process RSS, swap, disk free/IO, Meilisearch task failures/backlog, search endpoint latency/error rate, and PG query latency.
8. **Upgrade discipline:** pin image versions; read release notes; test in staging; keep rollback/rebuild plan.
9. **Security:** production env mode, strong master key/tenant-safe API keys as applicable, no public exposure without proxy/auth, dev keys never reused.
10. **Multilingual validation:** for pt-BR/es/de/fr/zh-CN/ja corpora, validate tokenization/relevance/snippet quality in both PG and (if enabled) Meilisearch on real samples before promising behavior (**NV**).

## 9. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Boot/runtime coupling to Meilisearch | Core app unavailable on small-VPS hiccups | Make search optional/degradable; PG fallback; health-gated capability [F1,F5,F7]. |
| Memory contention/OOM/swap on 4 GB | DB/API degradation or crashes | Strict indexing caps, single instance, no parallel rebuilds, disable profile under pressure [M1,M2]. |
| Index drift/stale results (no hooks/outbox) | Missing or outdated task/file/message hits | Idempotent lifecycle hooks or outbox + retries + scoped rebuild-from-PG [F6,F7]. |
| Soft-delete/unassign leakage in Meilisearch | Deleted/incorrect records searchable | Delete-on-soft-delete, reindex-on-update/unassign, reconciliation tests (**NV** implementation). |
| Tenant isolation error | Cross-org data exposure | Per-org indexes/namespacing and server-side scoping; never trust client-provided org/index identifiers [F2,F6]. |
| Disk exhaustion / task-DB pressure | Ingestion/search failures, degraded engine behavior | Monitor disk and tasks; keep corpora small; rotate/prune via supported ops and rebuild strategy [M2,M3]. |
| Multilingual relevance mismatch (CJK/locale specifics) | Poor recall/ranking vs expectations | Validate configs/dictionaries/tokenization; use `pg_trgm` or Meilisearch selectively per case [P3,F5]. |
| Config drift (`MEILI_*` vs `FLORA_*__*`, defaults) | Unexpected enablement or insecure exposure | Single documented config source, env validation at startup, secret management (**NV**). |

## 10. NVs (needs verification)

- **NV-1:** Measured RSS/CPU/IO/latency for `postgres + flora-api + meilisearch:v1.12` on 2 vCPU/4 GB SSD with representative Flora corpora (steady-state and rebuild peaks).
- **NV-2:** Safe production values for `MEILI_MAX_INDEXING_MEMORY`, payload limit, and search concurrency for Flora workloads on 4 GB.
- **NV-3:** Current Flora boot/search-path behavior when Meilisearch is disabled, slow, or down (true optionality and fallback contract) at HEAD.
- **NV-4:** Exact indexing lifecycle implementation status for messages/tasks/files (hooks/outbox/rebuild tooling), including soft-delete/unassign semantics in Meilisearch mode [F6].
- **NV-5:** Relevance/snippet/`type`-filter parity expectations across PG FTS and Meilisearch for supported locales (EN, pt-BR, es, de, fr, zh-CN, ja).
- **NV-6:** Backup/restore policy choice for Meilisearch data (snapshot/dump vs rebuild-only) and operational RTO/RPO targets for SMB installs.
- **NV-7:** Version-specific behavior/deltas between `v1.12` and latest v1 docs (flags, defaults, limits, storage details); pin and validate before upgrades.
- **NV-8:** Task/index growth limits in practice (per-org index count, task DB growth, file descriptor needs) for multi-tenant Seed deployments [M2].

## 11. Sources

### Meilisearch (primary)
- [M1] Meilisearch Docs — **Configure Meilisearch at launch** (CLI/env/config precedence, `MEILI_*`, indexing/memory/threads, payload/db-path/log-level, etc.): https://www.meilisearch.com/docs/learn/self_hosted/configure_meilisearch_at_launch
- [M2] Meilisearch Docs — **Known limitations** (payload/search limits, indexing/attribute/document constraints, task DB/index limits, large dataset/internal error guidance, concurrent search requests): https://www.meilisearch.com/docs/learn/resources/known_limitations
- [M3] Meilisearch Docs/API — **Tasks / asynchronous operations** (indexing and other operations processed as tasks; task listing/monitoring): https://www.meilisearch.com/docs/reference/api/tasks/list-tasks

### PostgreSQL (primary)
- [P1] PostgreSQL 18 Documentation — **Full Text Search**: https://www.postgresql.org/docs/current/textsearch.html
- [P2] PostgreSQL 18 Documentation — **Tables and Indexes for FTS** (`tsvector`, GIN/GiST): https://www.postgresql.org/docs/current/textsearch-tables.html
- [P3] PostgreSQL 18 Documentation — **`pg_trgm`**: https://www.postgresql.org/docs/current/pgtrgm.html

### Flora (in-repo)
- [F1] `OBJETIVO.md`, `ROADMAP.md` (Seed/Postgres-first targets; 021 delivered, 022 preparation/parity intent)
- [F2] `flora-core/src/config/search.rs` (Meilisearch/search config, env fallbacks, index template, validation)
- [F3] `docker-compose.yml` (lean profiles, `meilisearch:v1.12`, caps/healthcheck/volumes/bind)
- [F4] `.env.example` (search env toggles/URL/key)
- [F5] `docs/research/2026-11-16-016-search-postgres-fts-vs-meilisearch.md`
- [F6] `docs/research/2026-11-16-021-meilisearch-task-file-search-parity-vps.md`
- [F7] `docs/research/2026-11-16-021-optional-meilisearch-pg-coherence-vps.md`, `docs/research/2026-11-16-seed-vps-profile-audit.md`

### Notes on evidence quality
- Primary docs above are authoritative for documented behavior; some operational points (e.g. LMDB memory-mapping implications, exact footprint under co-tenancy) require version-specific interpretation and local measurement (**NV**). No non-primary web sources were cited due to unavailable/low-signal search results in this run; extend with recent reputable benchmarks/field reports if needed, clearly labeled as secondary.
