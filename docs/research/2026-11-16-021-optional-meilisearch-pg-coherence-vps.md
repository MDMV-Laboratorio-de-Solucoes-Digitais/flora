# 021 — Optional Meilisearch alongside PostgreSQL FTS: coherence, indexing, and 80/20 defaults for cheap VPS (multilingual)

**Date:** 2026-11-16  
**Repo:** `flora` (research-only deliverable; no source changes, no commit)  
**Scope:** Operating **optional Meilisearch alongside PostgreSQL FTS** for small multi-entity search (messages/tasks/files) on a cheap single-node VPS (**~2 vCPU / 4 GB RAM / SSD**). Focus: indexing strategy (sync hooks vs rebuild), consistency/fallback behavior, resource limits, 80/20 defaults for Seed/SMB, and when to prefer **PG-only** vs **dual-backend parity**.  
**Method:** Cross-check with Flora goals/state + multilingual primary-doc research (**EN, pt-BR, es, de, fr, zh-CN, ja**). Web-search tooling in this run returned no usable result pages; evidence is anchored to primary documentation and Flora-internal research. Environment-specific sizing/latency on 2 vCPU/4 GB is **NV** unless benchmarked.

## 1. Flora cross-check (why this matters)

| Evidence | Implication |
|---|---|
| `OBJETIVO.md` / `ROADMAP.md` [F1] | Seed target: self-hosted PME on cheap VPS, Postgres-first, optional components; MVP P0 states basic search via Postgres FTS with external engines optional. |
| `flora-search` + `flora-core/src/config/search.rs` [F2,F3] | Meilisearch SDK/client, per-org indexes (`flora_org_{org_id}`), index/search/delete for `message|task|file`, `enabled` config exists. |
| `flora-api/src/state/app_state.rs`, routes [F4] | Search service initialized unconditionally and boot-coupled in current implementation (prior audits); API exposes `GET /api/v1/search`. |
| Prior search research [F5,F6] | PG FTS identified as viable 80/20 default; Meilisearch adds ops/memory/sync surface; lean profile should be `api+postgres`. |

**Verdict:** Dual-backend operation is only sensible if Meilisearch is a truly optional, degradable profile with a clear source of truth (Postgres) and a rebuild path.

## 2. Multilingual evidence summary (EN, pt-BR, es, de, fr, zh-CN, ja)

| Lang | Primary/operational signal | Takeaway for 4 GB Seed/SMB |
|---|---|---|
| **EN** | Postgres FTS uses `tsvector`/`tsquery`, dictionaries/configs, and GIN/GiST indexes with in-DB transactional maintenance [S1,S2]; `pg_trgm` adds trigram similarity/fuzzy matching and indexable `LIKE/ILIKE`/regex support [S3]. Meilisearch is a separate engine with memory-mapped LMDB storage, async indexing tasks by nature of a service boundary, and instance options including `--max-indexing-memory` / `--max-indexing-threads` [S4,S5]; comparison highlights typo tolerance, instant search, facets, and CJK tokenization strengths vs PG search tradeoffs [S6]. | Keep PG as source of truth and default search; add Meilisearch only as bounded optional profile. |
| **pt-BR** (orientação operacional) | Em VPS pequena, cada serviço extra aumenta custo de memória, backup e falhas; priorizar “banco único” reduz superfície. | Perfil enxuto: `api+postgres`. Meilisearch só quando UX justificar (busca instantânea/tolerância a erro). |
| **es** (operación/hosting) | 2 vCPU/4 GB es común pero ajustado; colocalizar base de datos y API deja poco margen para daemons adicionales. | Búsqueda en Postgres por defecto; Meilisearch opcional con límites y reconstruible. |
| **de** (Betrieb/Qualität) | Zusatzdienste erhöhen Betriebsaufwand (Monitoring, Updates, Restore). Konsistenz ist einfacher, wenn der Suchindex aus der führenden DB ableitbar ist. | Primärsystem PostgreSQL, Meilisearch nur als optionales Profil mit klarer Rebuild-Strategie. |
| **fr** (exploitation) | Sur petit VPS, limiter les composants et privilégier la cohérence transactionnelle; un moteur externe impose synchronisation et capacité de reprise. | FTS Postgres par défaut; Meilisearch profilé, capé, et secondaire. |
| **zh-CN** | PG 内建 `tsvector/tsquery`+GIN 与 `pg_trgm` 适合数据已在 PG、需求不复杂的场景；Meilisearch 开箱即用容错/即时搜索但仍是独立服务；小规格机器优先单库、避免同步复杂度 [S7]. | 2核4G 优先 PG-only；仅在体验收益明确时启用 Meilisearch。 |
| **ja** (運用/多言語) | 追加サービスは監視・障害点・復旧手順を増やす。Postgres FTS の品質は辞書/トークナイズ設定に依存し、CJK は設計確認が必要；Meilisearch は多言語/CJK 対応の強みがある [S6]. | まず Postgres、CJK/即時検索要件が強い場合のみ Meilisearch を限定的に追加。 |

## 3. Indexing strategy: sync hooks vs rebuild (concrete)

**Source of truth:** PostgreSQL for messages/tasks/files. Any Meilisearch index is a **derived, rebuildable projection**.

| Strategy | How | Pros | Cons | Recommendation (4 GB, small multi-entity) |
|---|---|---|---|---|
| **DB-native PG FTS maintenance** (generated/stored `tsvector` + GIN, triggers or transactional upsert to projection table) | Writes update search representation in same transaction/DB workflow | Strong consistency, no extra worker, simple backup/restore | Ranking/features simpler than engine; schema/migration work | **Default 80/20.** Best for Seed/SMB. [S1,S2,S3] |
| **App-layer sync hooks to Meilisearch** (on create/update/delete) | Service call after DB commit or inline in request path | Fast to add when SDK exists | **Risky:** dual-write inconsistency on partial failure/retry/timeout; couples latency/availability to Meilisearch | **Avoid as primary.** If used temporarily, make idempotent, retry-safe, and never block core writes. |
| **Transactional outbox (PG) + async indexer** | Persist domain change/outbox row in same DB tx; worker drains to Meilisearch with idempotent upserts/deletes by `(org_id,type,id,version)` | Decouples request latency, supports retries, auditable lag, rebuild-friendly | More moving parts (worker/scheduling) | **Preferred when Meilisearch is enabled.** Right consistency/ops balance. |
| **Periodic/full rebuild from Postgres** | Reindex all entities per org or globally on deploy/drift detection | Simplest correctness guarantee; recovers from drift | Rebuild window, load spike; stale until complete | **Mandatory companion** to any async/hook strategy. Support scoped rebuild (per org/entity) and run off-peak. |
| **Hybrid** | Outbox for steady state + on-demand rebuild + PG fallback for reads | Practical resilience | Requires capability flags and observability | **Target design** for optional dual-backend. |

**Practical rules:** never make Meilisearch availability a precondition for DB commit; include monotonic version/updated-at to drop stale updates; deletes must be first-class events; per-org isolation must be preserved in index naming/filtering.

## 4. Consistency and fallback behavior

| Scenario | Desired behavior | Notes |
|---|---|---|
| Meilisearch healthy | Optional fast path: query Meilisearch **or** PG per configured backend; do not require dual-query by default | Dual-query adds latency/load on 4 GB. |
| Meilisearch lagging (async/outbox) | Accept eventual consistency for Meilisearch results; expose/metric `index_lag` (**NV** impl) | Document that PG FTS is fresher/transactional [S1,S5]. |
| Meilisearch down / misconfigured / indexing paused | **Fallback to Postgres FTS** if PG backend available; else degrade gracefully (clear 503/feature-disabled or empty+capability flag per contract) — **never break boot/core CRUD** | Aligns with optional-component goal [F1,F5]. |
| Drift detected (counts mismatch, missing docs) | Trigger scoped rebuild from Postgres; keep serving fallback | Rebuild is cheap insurance for small corpora. |
| Reindex/rebuild in progress | Serve PG results or last-known Meilisearch snapshot depending on mode; throttle rebuild concurrency | Avoid starving Postgres/API on 2 vCPU. |

**Consistency model to state explicitly:** PG = authoritative and transactionally searchable (when PG FTS enabled); Meilisearch = eventually consistent derived index unless proven otherwise (**NV** for Flora’s exact task semantics).

## 5. Resource limits on ~2 vCPU / 4 GB VPS

> Planning guidance; validate with local metrics (**NV**).

| Component | 80/20 budget (lean Seed) | Limits/controls |
|---|---|---|
| OS + overhead | ~300–600 MB | Leave headroom; avoid swap. |
| PostgreSQL | ~768–1280 MB | Tune `shared_buffers`/`work_mem`/connections for small VPS; co-locate FTS indexes. [S1] |
| `flora-api` | ~256–512 MB | Cap concurrency/pool to match DB. |
| **Meilisearch (optional profile only)** | ~256–512 MB practical ceiling when co-located (**NV**) | Set `--max-indexing-memory` and `--max-indexing-threads` low; prefer `--no-analytics`; isolate data volume; bind locally; keep dataset small. Meilisearch uses LMDB memory-mapped storage (OS-managed cache behavior; virtual memory != RSS) [S5]. |
| Indexing jobs | off-peak / throttled | One rebuild/index stream at a time; backoff on pressure. |

**Operational caps (concrete starting points, tune after measurement):** `--max-indexing-threads=1` (or 2 max), `--max-indexing-memory` to a small fixed value (e.g. 256–512 MiB range depending on co-tenancy, **NV**), single instance, no HA. If RSS+DB pressure causes swap/OOM: **disable Meilisearch profile** (PG-only).

## 6. 80/20 defaults for Seed/SMB

1. **Default backend = `postgres` (or `none` until PG FTS ships), Meilisearch off.** One stateful core: Postgres.
2. **Scope:** search over core fields/snippets for messages, tasks, files/metadata; strict `organization_id` scoping; simple ranking (`ts_rank`/`ts_rank_cd`) + recency tie-break where useful.
3. **Fuzzy fallback:** add `pg_trgm` only on high-value fields (title/name) if users need typo-like matching [S3].
4. **Indexing:** DB-native `tsvector` maintenance; no external queue in Seed.
5. **API contract:** keep stable `q/type/limit` and result shape; expose capability/status so UI can degrade cleanly.
6. **Optional `search` profile (Meilisearch):** enable only with explicit env/profile, resource caps, healthcheck, and documented **rebuild-from-Postgres** command/procedure.
7. **Multilingual note:** for pt-BR/es/de/fr, configure appropriate text search configs/dictionaries as needed (**NV**); for **zh-CN/ja/CJK-heavy** tenants, validate PG tokenization quality early — Meilisearch may be justified despite cost [S6,S7].

## 7. When to prefer PG-only vs dual-backend parity

| Prefer | Choose when | Rationale |
|---|---|---|
| **PG-only** | Seed/SMB on 2–4 GB; basic keyword search; freshness/consistency matters; minimal ops; few tenants/corpus modest; engineering capacity limited | Lowest RAM/ops/sync risk; transactional visibility; simpler backup/restore. [S1,S2,F1] |
| **Meilisearch-only (derived, optional)** | Product UX requires typo-tolerant instant/prefix search, facets, or CJK behavior that PG cannot meet acceptably **and** corpus/traffic justify it | Better interactive relevance/features out of box [S6], but keep PG authoritative and rebuildable. |
| **Dual-backend parity** (same features/results contract across PG and Meilisearch) | **Rarely.** Only if you must support both deployments and guarantee equivalent user-visible behavior/ranking semantics across them | High cost: two relevance models, two indexing paths, double testing/migrations/observability. On 4 GB, parity increases failure modes without proportional value. **Avoid for 80/20.** |

**Decision rule:** default **PG-only** → promote to **optional Meilisearch (non-parity, capability-gated)** on measured UX need → consider **parity** only with explicit product requirement, benchmarks, and ops budget.

## 8. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Dual-write/hooks without outbox | Lost/duplicate/stale index entries | Outbox + idempotent consumer + rebuild. |
| Boot/runtime coupling to Meilisearch | Core app unavailable on VPS hiccups | Optional init, health-gated capability, PG fallback. |
| Memory contention/OOM on 4 GB | DB latency, swaps, crashes | Strict caps, `--max-indexing-memory/threads`, disable profile under pressure. [S5] |
| Multilingual/CJK relevance gaps in PG | Poor recall for ja/zh or mixed locales | Validate dictionaries/configs; use trigram selectively; consider Meilisearch for CJK-heavy cases (**NV**). [S3,S6] |
| Rebuild/load spikes | Search staleness or API/DB degradation | Scoped, throttled, off-peak rebuilds. |
| Tenant isolation bugs in derived index | Data leakage across orgs | Deterministic per-org keys/indexes/filters; tests on rebuild and delete paths. |

## 9. NVs (needs verification)

- **NV-1:** Real RSS/latency of Meilisearch + Postgres + API co-located on 2 vCPU/4 GB with Flora-sized corpora.
- **NV-2:** Optimal `--max-indexing-memory`/threads for safe co-tenancy (OS/page cache effects). [S5]
- **NV-3:** PG FTS ranking/snippet quality for messages/tasks/files in pt-BR/es/de/fr/zh-CN/ja with chosen configs.
- **NV-4:** Need for `pg_trgm` vs Meilisearch for fuzzy/CJK use cases in actual tenant data. [S3,S6]
- **NV-5:** Best outbox schema/worker placement in Flora crates and impact on migrations/CI (`db-tests`).
- **NV-6:** Any existing client expectations beyond simple `q/type/limit` contract when backend/capability changes.
- **NV-7:** Transferability of external comparisons/benchmarks to Flora workloads and Meilisearch/PG versions in use.

## 10. Sources

### Primary documentation
- [S1] PostgreSQL Documentation: Full Text Search — https://www.postgresql.org/docs/current/textsearch.html
- [S2] PostgreSQL Documentation: Full Text Search Introduction — https://www.postgresql.org/docs/current/textsearch-intro.html
- [S3] PostgreSQL Documentation: `pg_trgm` — https://www.postgresql.org/docs/current/pgtrgm.html
- [S4] Meilisearch Docs: Configure Meilisearch at launch (incl. indexing/memory-related options) — https://www.meilisearch.com/docs/learn/self_hosted/configure_meilisearch_at_launch
- [S5] Meilisearch Docs: Storage / LMDB, memory mapping — https://www.meilisearch.com/docs/learn/engine/storage
- [S6] Meilisearch Docs: Comparison to alternatives (incl. PostgreSQL) — https://www.meilisearch.com/docs/resources/comparisons/alternatives

### Multilingual practitioner signal
- [S7] Terry Yao (zh-CN), 「全文搜尋：Elasticsearch、Meilisearch、還是 PostgreSQL 就夠了？」 — https://terryyaowork.cc/common/foundations/database/05-search-engine

### Flora internal
- [F1] `OBJETIVO.md`, `ROADMAP.md`
- [F2] `flora-search/Cargo.toml`, `flora-search/src/lib.rs`
- [F3] `flora-core/src/config/search.rs`
- [F4] `flora-api/src/state/app_state.rs`, `flora-api/src/routes/search.rs`
- [F5] `docs/research/2026-11-16-016-search-postgres-fts-vs-meilisearch.md`
- [F6] `docs/research/2026-11-16-seed-vps-profile-audit.md`, `docs/research/2026-11-16-015-seed-vps-operability-quickwins.md`, `docs/research/2026-11-16-016-seed-vps-operability-hardening.md`
