# 016 — Search strategy for Flora on cheap VPS: PostgreSQL FTS vs Meilisearch (multilingual)

**Date:** 2026-11-16  
**Repo:** `flora` (read-only analysis; this document only)  
**Scope:** Search strategies for self-hosted SMB apps on cheap VPS (**2 vCPU / 4 GB RAM / SSD**, single-node): PostgreSQL full-text search (`tsvector`/GIN, `pg_trgm`) vs Meilisearch, operational cost, relevance/latency tradeoffs, and a pragmatic 80/20 path for Flora (Rust + PostgreSQL) to debloat heavy search dependencies while keeping useful search.  
**Method:** Repo/code inspection + cross-check with existing Flora research/goals + multilingual web research (**EN, pt-BR, es, de, fr, zh-CN, ja**). No runtime benchmarks in this repo, no application source changes, no commits. Claims grounded in primary docs/benchmarks where cited; environment-specific performance and memory figures are estimates/hypotheses unless directly benchmarked (**NV**).

## 1. Cross-check: Flora current state and goals

| Source | Relevant evidence |
|---|---|
| `OBJETIVO.md` (2026-11-15/16) | North star: self-hosted Macro alternative for PMEs on cheap VPS; debloat, **Postgres-first, optional components**. MVP P0 includes **busca básica: Postgres FTS integrada aos objetos núcleo; motores externos opcionais**. P2 defers heavy surfaces. Gaps: search/notification wiring, optionality/debloat. |
| `ROADMAP.md` (2026-11-16) | P0/P1 priorities include wiring essential search/notifications and making Valkey/Meilisearch truly optional by profile/boot with limits/healthchecks/backup; Seed operable on modest VPS. |
| `flora-search/Cargo.toml`, `src/lib.rs` | Dedicated crate depends on `meilisearch-sdk`; `SearchService` builds Meilisearch client, per-org index `index_template.replace("{org_id}", org_id)` (default template `flora_org_{org_id}` in config), indexes/searches/deletes `message|task|file` documents. No Postgres/SQL search path. |
| `flora-core/src/config/search.rs` | `SearchConfig { url, api_key, default_limit, max_limit, index_template, enabled }`, default `enabled=true`, URL default `http://localhost:7700`, validates URL required when enabled. |
| `flora-api/src/state/app_state.rs` | **Always** constructs `Arc<flora_search::SearchService>::new(...)` and fails startup with `"Search service init failed: {e}"` (no `config.search.enabled` gate). `search_service` is non-optional field. |
| `flora-api/src/routes/search.rs` | `GET /api/v1/search` (`q`, optional `type` CSV, `limit` capped 50) requires auth+org membership, delegates to `state.search_service.search(&org_id, &q, types, limit)`. Errors propagate from search client. |
| `flora-core/src/migrations/sql/20240818000001_init_schema.sql` | Initial schema only; no `tsvector`/GIN FTS columns/indexes or `pg_trgm` found in migration inspection (**NV** for future migrations beyond this scan pattern, but current path is Meilisearch-only). |
| Prior audits: `docs/research/2026-11-16-seed-vps-profile-audit.md`, `...-015-seed-vps-operability-quickwins.md`, `...-016-seed-vps-operability-hardening.md` | Consistently identify Meilisearch as **configured optional in intent but effectively required by boot** and a debloat target for lean Seed/VPS. |

**Alignment verdict:** Flora’s stated Seed direction (Postgres FTS default, external engines optional) conflicts with current implementation (mandatory Meilisearch at boot, no PG FTS). This is a high-leverage debloat and operability gap for 2 vCPU/4 GB VPS targets.

## 2. Multilingual evidence summary (EN, pt-BR, es, de, fr, zh-CN, ja)

> Note: strongest primary/benchmark evidence retrieved in EN; zh-CN includes a practitioner comparison article; pt-BR/es/de/fr/ja signals combine targeted-language queries, regional/hosting context, and translated operational guidance from primary sources. Locale-specific production benchmarks on 2 vCPU/4 GB were **not** found as authoritative studies (**NV**); conclusions triangulated.

| Lang | Key evidence / query focus | Practical takeaway for Flora Seed VPS |
|---|---|---|
| **EN** | PostgreSQL docs (FTS, GIN/GiST, `pg_trgm`); Meilisearch docs comparison (typo tolerance, facets, search-as-you-type, CJK support, hybrid/embedders; notes pgvector/FTS tradeoffs); benchmarks/reports (e.g. Fabwaseem 2026: on 8 vCPU/15 GB VM, Postgres BM25 extensions matched Elasticsearch ranking within ~0.05 nDCG@10, low-ms latency, **new rows searchable 2–4 ms in Postgres vs 61.9 ms Meilisearch async task vs 705.8 ms ES refresh**; Meilisearch index much larger in that test, 1.89 GB DB for 171k docs; built-in `ts_rank` weaker for NL ranking but fast/transactional). Practitioner consensus: start with Postgres FTS+`pg_trgm` for <~few million docs / search-as-feature. [S1–S8] | On 4 GB, avoid an always-on extra search service unless UX requires it. PG gives transactional consistency, no sync pipeline, simpler backup, and predictable co-tenancy with existing DB. |
| **pt-BR** | Consultas: “busca full-text PostgreSQL tsvector pg_trgm vs Meilisearch VPS barata 4GB RAM custo operacional auto-hospedado PME”. Sinal consistente com EN: reduzir componentes em VPS pequena, usar Postgres como fonte única, reservar Meilisearch para busca instantânea com tolerância a erros quando for diferencial de produto. | Para PMEs/self-host, **menos infraestrutura = menor custo operacional**; perfil lean `api+postgres` é o mais viável. |
| **es** | Consultas: “búsqueda de texto completo PostgreSQL pg_trgm Meilisearch servidor VPS 4GB comparación costos latencia”. Contexto regional de hosting (Kimsufi/Hostinger etc.) reforça que 2 vCPU/4 GB é entrada comum, mas serviços extras (índices/brokers/JVM-like footprints) pressionam RAM. | Priorizar búsqueda integrada en Postgres y activar motores externos solo por perfil/demanda. |
| **de** | Consultas: “PostgreSQL Volltextsuche tsvector GIN pg_trgm vs Meilisearch selbst gehostet VPS 4GB RAM Betriebskosten Latenz”. Leitlinie: optionale Profile, degradierbare Abhängigkeiten, Ressourcenbegrenzungen. | Betriebskosten/Nutzungsaufwand steigen mit jedem Zusatzdienst; FTS im DB-Kern minimiert Betriebsfläche. |
| **fr** | Consultas: “recherche plein texte PostgreSQL tsvector pg_trgm vs Meilisearch auto-hébergé VPS 4 Go coûts latence pertinence”; contexte VPS FR (OuiHeberg/Korben/Virtua) aligne faible RAM avec stacks limitées. | Garder Meilisearch optionnel/profilé; défaut Postgres pour recherche “fonction” plutôt que “produit”. |
| **zh-CN** | 文章与查询（如 “PostgreSQL 全文搜索 tsvector GIN pg_trgm 与 Meilisearch 自托管 2核4G VPS 内存 占用 延迟 对比”）: PG 内建 `tsvector/tsquery`+GIN 与 `pg_trgm` 适合数据已在 PG、需求不复杂（博客/产品名/用户/后台检索）；Meilisearch 开箱即用容错/即时搜索、资源需求低于 ES 但仍为独立服务；ES 留给日志/聚合/TB 级。决策倾向“先 PG，不够再加”。 [S9] | 2核4G 下优先 **单库方案**，避免同步与额外进程；按需再启用 Meilisearch。 |
| **ja** | Consultas: “PostgreSQL 全文検索 tsvector GIN pg_trgm Meilisearch セルフホスト VPS 4GB メモリ 遅延 比較”。 運用上の示唆: 追加サービスは監視/バックアップ/障害点を増やすため、小規模 VPS では DB 内検索を基本に。 | まず Postgres FTS、タイポ許容や即時検索 UX が必須になった時点で Meilisearch をプロファイル追加。 |

## 3. PostgreSQL FTS (`tsvector`/GIN, `pg_trgm`) vs Meilisearch on 2 vCPU/4 GB

### 3.1 Capability and tradeoff matrix

| Dimension | PostgreSQL (`tsvector` + GIN, `websearch_to_tsquery`/`plainto_tsquery`, `ts_rank[_cd]`, `pg_trgm`) | Meilisearch (current Flora integration) | Implication for cheap VPS Seed |
|---|---|---|---|
| **Infra/ops** | No extra service; uses existing DB process/storage/backup. Extensions `pg_trgm` (and optionally `unaccent`) typically available. | Separate daemon/container, HTTP API, index storage, API key, health/upgrade lifecycle, sync/indexing path. | **PG wins strongly** on fixed RAM/ops surface. |
| **Consistency/freshness** | In-transaction/indexed visibility; no external sync pipeline. (BM25 extensions also show ~ms freshness in cited benchmarks.) | Async indexing/tasks and client-server boundary; eventual/operational sync concerns. | PG better for “search reflects recent core writes” with less glue. |
| **Memory/disk** | Index + table overhead inside Postgres (GIN/`tsvector` can add space; tune `shared_buffers`/`work_mem`). | Own memory-mapped/index footprint + process overhead; can be substantial relative to corpus/features (e.g. large DB size in some benchmarks; Flora audit budgets 384–768 MB cap when enabled). | On 4 GB, Meilisearch competes with DB/API headroom; PG co-locates cost. |
| **Latency (typical)** | Good for exact/stemmed keyword and filtered SQL; `ts_rank` OR over broad matches can degrade (p50 ~239 ms on 171k-doc NL case in cited benchmark) — schema/query design matters. | Optimized for interactive/search-as-you-type with low-latency responses in common cases. | For Flora’s current non-typeahead, bounded org-scoped search, PG FTS is usually sufficient; optimize query shape if needed. |
| **Relevance** | Stemming/language configs, weights, phrase/prefix patterns; ranking simpler than BM25 engines unless extensions added (not assumed). | Tunable ranking rules, typo tolerance, prefix behavior out of the box. | PG adequate for “basic useful search” (P0); Meilisearch better for polished instant/typo UX. |
| **Typo/fuzzy** | `pg_trgm` similarity/index for names/titles and “did you mean” fallback; not same as engine-native typo-aware ranking. | Built-in typo tolerance. | 80/20: add trigram fallback on key fields if needed later. |
| **Facets/filters** | SQL `GROUP BY`/counts possible but can be costly/complex at scale. | Native facets/filters APIs. | Flora P0 does not require rich faceting. |
| **Multilingual/CJK** | Depends on dictionaries/config/tokenization; CJK often needs extra setup/extensions; `pg_trgm` can help character-level matching in some cases. | Documented strong multilingual/tokenization support including CJK/Arabic/Hebrew in docs comparison. | **Risk for global/JP/CN-heavy content** on plain `tsvector`; assess actual user locales/data before committing to PG-only. |
| **Security/tenancy** | Same DB, easy to enforce `org_id` predicates/RLS-aligned patterns in query; no cross-service auth surface. | Per-org indexes in Flora design; must secure Meilisearch endpoint/key and keep index boundaries correct. | PG simplifies tenant isolation model. |
| **Backup/restore** | Included in `pg_dump`/DB restore drills (plus reindex cheap). | Separate data dir/snapshot or rebuild from source of truth. | Fewer moving parts for SMB ops. |
| **Current Flora fit** | Not implemented; needs schema/index/service/contract work. | Implemented but boot-coupled/mandatory and not aligned with “optional” Seed intent. | Short-term debloat can start by making Meilisearch optional; medium-term by adding PG FTS backend. |

### 3.2 Operational cost on 2 vCPU/4 GB (planning, **NV** without local benchmarks)

- **Lean viable envelope:** reserve ~512 MB for OS/buffers; **`flora-api` + PostgreSQL** as core. Prior Flora audits suggest conservative caps around API 256–512 MB and Postgres 768–1024 MB (tuned), leaving little room for extra stateful services under spikes.
- **With Meilisearch:** add process + index/cache overhead (plan 384–768 MB cap only when enabled, and constrain indexing threads/memory); risk of swap/OOM or DB starvation on small corpus operations. Acceptable only as explicit `search` profile with limits and rebuildable data.
- **With Valkey:** smaller footprint possible but still optional; keep disabled in lean Seed unless required.
- **Human/maintenance cost:** each added service increases config, secrets, upgrades, monitoring, and failure modes — disproportionately expensive for PME/self-hosted installs.

## 4. Pragmatic 80/20 path for Flora (Rust + Postgres) to debloat search

Ordered by value/risk for Seed/VPS, preserving useful search:

### Phase 0 — Immediate debloat (no UX regression required if handled explicitly)
1. **Make `config.search.enabled` authoritative** in `flora-api/src/state/app_state.rs`: skip init when disabled; represent search as optional capability (`Option<Arc<...>>` or `SearchBackend` enum with `Disabled/Unavailable/Meilisearch`).
2. **Degrade gracefully at API boundary:** `GET /api/v1/search` returns a clear, documented response when search is disabled/unavailable (e.g. empty results + capability flag, or `501/503` per chosen contract) instead of failing app boot. Audit all call sites/tests that assume non-optional search.
3. **Compose/env profiles:** default lean `core` (`api`+`postgres`), optional `search` overlay for Meilisearch with resource caps, localhost binds, healthchecks, and rebuildable-volume guidance; align `.env.example` to canonical `FLORA_*__*` keys and state Meilisearch as optional.
4. **Fix adjacent operability blockers (from audits):** wire DB pool config and avoid duplicate pool/init paths so search coexistence does not amplify memory/connection pressure.

### Phase 1 — Postgres FTS MVP for core objects (P0-aligned)
5. **Introduce a search backend abstraction** in Rust (trait/service) with two implementations: `MeilisearchBackend` (existing behavior) and `PostgresBackend` (new default for Seed).
6. **Schema/index (minimal):** add search representations for P0 entities already indexed in Flora (messages, tasks, files/metadata) scoped by `organization_id` (+ workspace/channel where applicable). Practical options:
   - generated/stored `tsvector` columns per searchable table or a unified `search_documents` projection table keyed by `(org_id, entity_type, entity_id)` with `tsvector` + metadata/snippet source;
   - `GIN` index on `tsvector`; `btree`/composite indexes for tenant/time filters;
   - optional `pg_trgm` GIN/GiST on title/name fields for fuzzy fallback (defer unless needed).
7. **Query strategy:** `websearch_to_tsquery` for user-friendly input, rank with `ts_rank_cd`/`ts_rank` (or hybrid scoring), strict `WHERE organization_id = $org` and type filters; `LIMIT` capped; highlight/snippet via `ts_headline` or lightweight excerpting.
8. **Indexing/write path:** simplest 80/20 is DB-native maintenance (generated columns + triggers, or transactional upsert into projection on mutations) to avoid a new worker pipeline; ensure idempotent updates/deletes and org isolation.
9. **Keep API contract stable where possible:** preserve `GET /api/v1/search?q=&type=&limit=` response shape (`query`, `results[{type,id,snippet}]`) so `apps/frontend` is minimally affected.

### Phase 2 — Parity and selective re-enable of Meilisearch
10. **Wire mutations to chosen backend(s):** if Meilisearch enabled, keep async/outbox-style indexing decoupled from request latency and document rebuild-from-Postgres procedure.
11. **Profile-based selection:** `search.backend = postgres | meilisearch | none` (or `enabled`+`provider`) with safe defaults for Seed.
12. **Improve relevance only when justified:** add trigram fallback, language configs, or (later, NV) Postgres BM25 extensions if hosting allows and benchmarks show need.

### Defer / non-goals (for 80/20 Seed)
- Rich faceted/aggregated search UI, cross-entity semantic/vector search, autocomplete-as-you-type as a hard requirement, multilingual stemming perfection, Meilisearch clustering, and dedicated indexing workers — until demand/evidence justifies complexity.

## 5. Recommendations (actionable)

1. **Adopt “Postgres FTS default, Meilisearch optional profile” as the target architecture** consistent with `OBJETIVO.md`/`ROADMAP.md` and cheap-VPS operability.
2. **Unblock lean deploys now:** decouple app boot from Meilisearch availability; treat search as a degradable capability with explicit operator-visible status.
3. **Implement a minimal `PostgresBackend` for messages/tasks/files metadata** behind the existing route contract; prioritize tenant correctness, deterministic behavior, and small migrations over ranking sophistication.
4. **Constrain resources when Meilisearch is enabled:** memory/thread limits, pinned version upgrades tested separately, and no assumption that it fits a 4 GB host alongside default DB/API without validation.
5. **Measure before tuning:** add lightweight query/index metrics or EXPLAIN-based checks on representative VPS-sized datasets; keep NVs explicit until benchmarked.

## 6. NVs (needs verification)

- **NV-1 Runtime sizing:** No Flora-specific latency/memory benchmarks for PG FTS vs Meilisearch on 2 vCPU/4 GB; caps in this doc are planning estimates from prior audits and external context.
- **NV-2 Ranking/query shape:** `ts_rank` behavior on Flora’s real NL/short-query mix untested; OR-rewrite/performance and snippet quality need prototype validation.
- **NV-3 Multilingual/tokenization:** Quality for pt-BR, es, de, fr, zh-CN, ja content with default Postgres configs/extensions is **NV**; may require per-language config or alternative strategy for CJK-heavy tenants.
- **NV-4 Data model fit:** Best schema for search projection (per-table `tsvector` vs unified table, workspace/channel scoping, attachment/file-content indexing) depends on evolving domain/contracts.
- **NV-5 Migration/compat:** Introducing `pg_trgm`/FTS indexes and backend switching must be validated against CI (`db-tests`), existing clients (`apps/frontend`, legacy `apps/web` artifacts), and API error semantics.
- **NV-6 Relevance parity:** Meilisearch features currently exercised by any client (beyond simple query/type/limit) are **NV** in this pass; generated legacy web client types suggest possible broader historical contract.
- **NV-7 External benchmark transferability:** Cited benchmarks use different datasets/hardware/versions; treat as directional, not guarantees for Flora’s workload.

## 7. Sources

### Primary / documentation
- [S1] PostgreSQL Documentation: Full Text Search — https://www.postgresql.org/docs/current/textsearch.html
- [S2] PostgreSQL Documentation: Preferred Index Types for Text Search (GIN/GiST) — https://www.postgresql.org/docs/current/textsearch-indexes.html
- [S3] PostgreSQL Documentation: `pg_trgm` — https://www.postgresql.org/docs/current/pgtrgm.html
- [S4] Meilisearch Docs: “Meilisearch vs PostgreSQL search” — https://www.meilisearch.com/docs/resources/comparisons/postgresql

### Benchmarks / technical analyses (EN, directional)
- [S5] Waseem Anjum, “Postgres Full-Text Search vs Elasticsearch vs Meilisearch: BM25 Benchmarked (2026)” — https://fabwaseem.com/blog/postgres-full-text-search-vs-elasticsearch (method: 8 vCPU/15 GB VM, BEIR datasets; freshness/index/latency/nDCG comparisons)
- [S6] AIDToolStack, “Adding search to a web app: Postgres FTS vs search engines” — https://aidtoolstack.com/blog/building-search-functionality-web-apps
- [S7] Pavle Aleksic, “Postgres Full-Text Search vs a Dedicated Search Engine” — https://pavlealeksic.com/blog/postgres-fulltext-vs-search-engine
- [S8] StitchDB, “Postgres Full-Text Search vs Elasticsearch (and Meilisearch)” — https://stitchdb.com/blog/postgres-full-text-search-vs-elasticsearch

### Multilingual practitioner signal
- [S9] Terry Yao (zh-CN), “全文搜尋：Elasticsearch、Meilisearch、還是 PostgreSQL 就夠了？” — https://terryyaowork.cc/common/foundations/database/05-search-engine

### Flora internal (cross-check)
- [F1] `OBJETIVO.md`, `ROADMAP.md`
- [F2] `flora-search/{Cargo.toml,src/lib.rs,src/config/meilisearch_template.json}`
- [F3] `flora-core/src/config/search.rs`, `flora-core/src/config/mod.rs`, `flora-core/src/migrations/sql/20240818000001_init_schema.sql`
- [F4] `flora-api/src/state/app_state.rs`, `flora-api/src/routes/search.rs`
- [F5] `docs/research/2026-11-16-seed-vps-profile-audit.md`
- [F6] `docs/research/2026-11-16-015-seed-vps-operability-quickwins.md`, `docs/research/2026-11-16-016-seed-vps-operability-hardening.md`, `docs/research/2026-11-16-014-seed-vps-operability-international.md`, `docs/research/2026-11-15-macro-alternative-80-20-research.md`
