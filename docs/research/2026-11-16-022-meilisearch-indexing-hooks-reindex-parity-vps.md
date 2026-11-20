# 022 — Minimal, cheap-VPS-safe Meilisearch parity for task/file/message search (indexing hooks, reindex, isolation)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`0dcde353a`) — **research only**; this document only (no source code changes outside this doc, no commits).  
**Audience/target:** Seed/Postgres-first, optional Meilisearch on ~2 vCPU / 4 GB RAM / SSD single VPS.

## 1. Scope and method

**Scope:** Achieve **minimal, cheap-VPS-safe Meilisearch parity** for `message | task | file` search in a multi-tenant (org-scoped) app. Covered: document schema and filterable/displayed attributes, indexing hooks on create/update/delete/soft-delete/restore (sync vs async vs outbox-lite), delete-on-soft-delete semantics, rebuild/reindex/backfill strategies, per-org/tenant index isolation, `type`-filter and snippet/response-shape consistency, and 80/20 tradeoffs for Seed.

**Method (high-reliability, multilingual):**
- **Flora cross-check:** repository code/config/migrations and prior `docs/research` (primary evidence for current behavior and constraints).
- **Primary sources:** official Meilisearch documentation (indexes, documents, tasks/async operations, filtering, displayed/searchable attributes, dumps, snapshots) and PostgreSQL documentation (full-text search tables/indexes). PostgreSQL/Meilisearch docs are primarily in English; multilingual findings below combine primary-engine semantics with locale/script-specific operational interpretation for **EN, pt-BR, es, de, fr, zh-CN, ja** and mark environment/corpus-dependent claims as **NV**.
- **Constraints:** web environment/tooling may limit exhaustive non-EN primary pages; claims are cross-checked against primary docs where available and not extrapolated beyond them.

## 2. Flora cross-check (baseline)

| Area | Evidence (repo) | Baseline / gap |
|---|---|---|
| API contract | `flora-api/src/routes/search.rs`: `GET /api/v1/search?q=&type=&limit` (auth + `require_org` + membership). Empty/whitespace `q` => `200 {"query":"…","results":[]}`. `limit` default 20, capped 50. `type` is CSV, lowercased; PG path treats `None/empty` as all and accepts aliases `message|messages`, `task|tasks`, `file|files`. Response: `{ "query", "results": [ { "type": "message|task|file", "id", "snippet" } ] }`. If `state.search_service` is `Some` it delegates to Meilisearch and returns hits as-is; else PG fallback merges per-entity results by `(rank desc, created_at desc, id desc)`. | **Precedence is Meilisearch-first when configured** (all-or-nothing path today). Parity requires either equivalent coverage/shape in Meilisearch mode or an explicit precedence/contract change. |
| PG FTS fallback (authoritative DB path) | `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql` add generated `search_tsv` (`to_tsvector('simple', …) STORED`) + partial GIN `WHERE is_deleted=false` for messages/tasks/files. `search_pg` uses `COALESCE(websearch_to_tsquery('simple',$q), plainto_tsquery('simple',$q))`, strict `organization_id` + `is_deleted=false`, `ts_rank_cd`, snippets as `left(regexp_replace(...,'\s+',' ','g'),200)`. Task text: `title + description`; file text: `name + file_type`. | PG is tenant+soft-delete safe, deterministic by DB state, and already covers the three entities. This is the parity/correctness baseline. |
| Meilisearch adapter | `flora-search/src/lib.rs`: per-org index `index_template.replace("{org_id}", org_id)` (default `flora_org_{org_id}`, `flora-core/src/config/search.rs`). `setup_index` sets filterable `["type","created_at_timestamp"]`. `index_message/task/file` upsert docs via `add_documents(&[doc], Some("id"))` with fields: common `id,type,content,created_at,created_at_timestamp`; message `channel_id,sender_id`; task `assignee_id,status`, `content = "{title} {description}"`; file `size`, `content = name` only (**no `file_type`**). `search` builds `type IN [...]` when `types` non-empty and returns stored `content` as `snippet` (untrimmed, not highlight/crop). `delete_document` exists. | **Schema/snippet drift:** file misses `file_type`; snippets/normalization differ from PG; no displayed/searchable attribute policy beyond defaults; no `organization_id` stored in doc (isolation relies on per-org index). Upsert by UUID `id` is fine if IDs are globally unique per entity. |
| Config/state/wiring | `SearchConfig` (`enabled,url,api_key,default_limit,max_limit,index_template`; env `MEILISEARCH_URL/API_KEY`). `AppState::new` (`flora-api/src/state/app_state.rs`) creates optional `search_service` when enabled and **degrades to `None` on init failure** (fail-open boot). `rg` shows no mutation-path calls to `setup_index/index_*/delete_document` in `flora-api/src/routes/{tasks,files,channels,...}` (observed wiring limited to state/health/search/auth/tests). | **Primary parity gap:** missing indexing lifecycle hooks + contract normalization, not missing Meilisearch primitives. When enabled, index can drift/miss create/update/soft-delete. |
| Prior research | `docs/research/2026-11-16-021-meilisearch-task-file-search-parity-vps.md`, `2026-11-16-021-optional-meilisearch-pg-coherence-vps.md`, `2026-11-16-020-pg-fts-tasks-files-vps.md`, `2026-11-16-018-postgres-search-outbox-vps.md`, etc. | Confirms: PG-first fallback, optional Meilisearch profile, dual-write/outbox as stronger but heavier option, need explicit precedence/consistency contract. |

## 3. Primary-source findings (Meilisearch / PostgreSQL)

### 3.1 Meilisearch behavior relevant to parity

- **Asynchronous task model:** most write operations (including document addition/update and index/settings changes) are processed asynchronously via Meilisearch’s task system; clients receive task information and can monitor completion. This means “hook called” != “immediately searchable” unless the caller waits for task success. (Source: [Meilisearch — Tasks and asynchronous operations](https://www.meilisearch.com/docs/learn/async/asynchronous_operations), [List tasks API](https://www.meilisearch.com/docs/reference/api/tasks).)
- **Indexes as isolated namespaces:** an index groups documents and settings; per-tenant indexes are a natural isolation boundary (separate settings, document sets, and task effects per index). (Source: [Indexes](https://www.meilisearch.com/docs/learn/getting_started/indexes), [Indexes API](https://www.meilisearch.com/docs/reference/api/indexes).)
- **Filterable attributes are required for filters:** fields used in `filter` must be declared in `filterableAttributes` (settings update/reindexing cost implications). `filter` supports equality/comparison/IN/boolean logic at search time. (Source: [Filter search results](https://www.meilisearch.com/docs/learn/filtering_and_sorting/filter_search_results).)
- **Displayed/searchable attributes control retrieval/search fields:** by default documents are searchable/displayed broadly, but explicit configuration improves control/performance and response shaping; `attributesToRetrieve`/highlight/crop are query-time tools for shaping results. (Source: [Displayed and searchable attributes](https://www.meilisearch.com/docs/capabilities/full_text_search/how_to/configure_displayed_attributes), [Displayed/searchable settings](https://www.meilisearch.com/docs/learn/relevancy/displayed_searchable_attributes).)
- **Dumps and snapshots (backup/migration/restore):**
  - **Dump:** portable export/import for migration/backup workflows (not a low-latency live-sync primitive). (Source: [Dumps](https://www.meilisearch.com/docs/learn/data_backup/dumps).)
  - **Snapshot:** exact database copy, faster restore for same-version recovery; not for version migration and can overwrite/import semantics matter operationally. (Source: [Snapshots](https://www.meilisearch.com/docs/learn/data_backup/snapshots).)
  - **Implication for parity:** neither replaces application-level backfill/reconciliation from PostgreSQL when index state drifts.

### 3.2 PostgreSQL behavior relevant to baseline parity

- **`tsvector` + index maintenance:** FTS can be computed at query time or stored (e.g. generated/stored column) and indexed with GIN/GiST; expression indexes or stored columns enable efficient `@@` searches. Stored/generated columns keep representation in sync with source rows per DB semantics. (Source: [PostgreSQL: Tables and Indexes for Text Search](https://www.postgresql.org/docs/current/textsearch-tables.html), [Controlling Text Search](https://www.postgresql.org/docs/current/textsearch-controls.html).)
- **Implication:** PG fallback in Flora already benefits from in-DB maintenance and strict tenant/soft-delete predicates, making it the low-ops correctness anchor on small VPS deployments.

## 4. Multilingual findings (EN, pt-BR, es, de, fr, zh-CN, ja)

| Locale/script | Primary/semantic considerations | Practical 80/20 guidance (cheap VPS) | Confidence / NV |
|---|---|---|---|
| **EN** | Meilisearch provides strong general tokenization/typo-tolerant relevance for short text; PG `'simple'` config in Flora is intentionally minimal/language-agnostic. | Parity work should prioritize **lifecycle + filters + snippet/shape consistency** over trying to equalize ranking. | High (engine behavior), **NV** for exact relevance deltas on Flora corpus. |
| **pt-BR** | Acentuação, variações morfológicas e termos compostos podem ranquear diferente entre Meilisearch e PG `'simple'`. Títulos/nomes curtos tendem a ter menor risco que texto longo. | Manter schema textual simples (`title/description/name/file_type`), validar buscas reais com acentos/stopwords implícitos; não prometer mesma ordenação entre backends. | Medium, **NV** (corpus real). |
| **es** | Similar a pt-BR/fr: normalización de acentos y stemming implícito (según motor/config) pueden divergir de `'simple'`. | Usar `type` y normalización de snippets consistentes; pruebas con consultas acentuadas y plurales/singulares de `type`. | Medium, **NV**. |
| **de** | Compuestos, mayúsculas y umlauts pueden comportarse distinto; PG `'simple'` no aplica diccionario alemán. | Para metadatos cortos suele bastar; si la calidad alemana es crítica, medir antes de depender de Meilisearch o ajustar estrategia PG (fuera de 022 mínimo). | Medium, **NV**. |
| **fr** | Diacríticos, élisions y tokenización difieren entre motores/configs. | Validar ejemplos representativos; mantener fallback PG como referencia determinística en caso de dudas de recall. | Medium, **NV**. |
| **zh-CN** | 中文分词/召回对引擎与版本敏感；PG `'simple'` 对 CJK 的匹配行为与专用分词器不同，Meilisearch 在多语言/CJK 场景常有优势但非“等价”。 | 对 `title/name/file_type` 等短字段风险较低；在 2c/4GB 上优先保证索引一致性与可重建性，不承诺与 PG 相同排序/召回。需用真实语料验证。 | Low–Medium, **NV**. |
| **ja** | 日本語（漢字/かな混在）でも分かち書き・正規化の差が結果に影響。短いメタデータは比較的安定だが本文検索は差が出やすい。 | 同上：ライフサイクル整合と再構築手順を優先し、品質は実データで検証。 | Low–Medium, **NV**. |
| **Cross-locale ops (all)** | Small VPS means resource headroom dominates reliability; async indexing introduces eventual consistency. | Bound payloads, fail-open hooks for UX if chosen, and a deterministic backfill/rebuild path from PostgreSQL. | Medium, **NV** for sizing. |

**Source anchors:** [Meilisearch async tasks](https://www.meilisearch.com/docs/learn/async/asynchronous_operations), [filtering](https://www.meilisearch.com/docs/learn/filtering_and_sorting/filter_search_results), [displayed/searchable attributes](https://www.meilisearch.com/docs/learn/relevancy/displayed_searchable_attributes), [PostgreSQL FTS tables/indexes](https://www.postgresql.org/docs/current/textsearch-tables.html). Locale/script outcomes are **NV** without Flora-specific corpora/benchmarks.

## 5. 80/20 design options for cycle 022

### Option A — Minimal wiring + rebuild contract (recommended if Meilisearch remains enabled)

**Goal:** make Meilisearch *coverage* converge toward PG for supported entities with low operational complexity.

1. **Document schema (minimal parity-oriented)**
   - Keep common fields: `id` (UUID string, PK), `type`, `content`, `created_at`, `created_at_timestamp`.
   - **Message:** `content = body` (or normalized body), plus `channel_id`, `sender_id` as today if useful.
   - **Task:** `content = trim(title + " " + coalesce(description,""))`; keep `status`, `assignee_id`; reindex on title/description/status/assignee/soft-delete changes.
   - **File:** **`content = trim(name + " " + coalesce(file_type,""))`** to match PG searchable metadata; keep `size` if desired.
   - Optional (defer): store `organization_id` for defense-in-depth/debug; **do not** replace per-org index isolation with shared-index filters in MVP.
2. **Index settings (minimal)**
   - `filterableAttributes`: keep `["type","created_at_timestamp"]` for current API needs; add others only when required by a concrete filter/sort feature.
   - `searchableAttributes`: prefer explicit `["content"]` (or include structured text fields if split) to avoid accidental over-search/noise; **NV** confirm desired behavior against current defaults.
   - `displayedAttributes`: include only fields needed to build API response (at least `id,type,content` or equivalent) to reduce payload; **NV** align with SDK/request shape.
   - Ensure `setup_index(org_id)` is applied idempotently/lazily on first write/search path for an org (accept async task nature).
3. **Indexing hooks (create/update/delete/soft-delete/restore)**
   - **Create/Update (including unassign/reassign/content edits):** upsert/index after successful DB mutation (post-commit). `add_documents(..., primary_key="id")` replaces existing doc state.
   - **Soft-delete / hard delete:** **delete index document** (`delete_document(org_id, id)`) when `is_deleted` becomes true or row is removed.
   - **Restore/undelete:** reindex current row state.
   - **Sync vs async vs outbox-lite:**
     - *Sync (inline):* simplest but adds latency and couples request success to Meilisearch availability; risky on 4 GB.
     - *Async fail-open (recommended 80/20):* enqueue/best-effort background task after commit, log-and-drop on failure/timeout; gives eventual consistency without blocking core CRUD.
     - *Outbox-lite (transactional intent in PG + async drain):* persist indexing intent/event in same DB transaction, worker retries idempotently; better durability/operability than hooks-only, moderate implementation cost.
     - *Full outbox/queue platform:* best correctness/retry semantics, higher ops/code cost (defer for Seed).
4. **Delete-on-soft-delete semantics (explicit)**
   - Because PG fallback excludes `is_deleted=false` rows, Meilisearch parity should treat soft-deleted entities as **absent** from search results: prefer **physical removal from index** over adding an `is_deleted` filter (keeps docs small and behavior simple). Ensure hooks fire on all soft-delete paths.
5. **`type` filter and response/snippet consistency**
   - Normalize `type` tokens identically in both backends (accept singular/plural aliases; define behavior for unknown/empty tokens and test).
   - Normalize Meilisearch `snippet` server-side to match PG contract when needed (e.g. collapse whitespace, trim to ~200 chars, derive from `content/title/name`) instead of returning raw stored field.
   - Accept that **ranking/order may differ** across engines; keep deterministic merge/sort only in PG fallback unless hybrid merge is explicitly designed.
6. **Backfill/reindex/rebuild strategy**
   - **Initial backfill:** scan active (`is_deleted=false`) messages/tasks/files per org from PostgreSQL and upsert into Meilisearch; run off-peak, batched, throttled.
   - **Rebuild:** for drift/corruption/schema change, drop/recreate org index (or delete all docs) then backfill from PG source of truth.
   - **Operational tooling:** provide a scoped command/procedure (per-org or global) and record completion; dumps/snapshots are backup aids, not substitutes for logical reindex from PG.
7. **VPS safety**
   - Single Meilisearch instance, local bind, strict resource caps (indexing memory/threads) as deployment policy (**NV** exact values), avoid dual-query modes, monitor task queue/RAM/disk, keep PG-only as fallback profile.

### Option B — Explicit degradation / precedence & rebuild contract (lowest-risk Seed default)

**Goal:** avoid “Meilisearch enabled but stale/incomplete” behavior without full lifecycle plumbing.

- **B1. PG-authoritative for parity:** when `search_service` is configured but hook/rebuild guarantees are incomplete, either disable Meilisearch for affected entity types or prefer PG fallback for `task|file` (and/or all) until parity is verified. Keep Meilisearch as optional accelerator only for proven coverage.
- **B2. Meilisearch-authoritative only with guarantees:** if Meilisearch path is selected, require: hooks (or outbox-lite) for create/update/delete/soft-delete/restore, initial backfill completed, periodic reconciliation/rebuild procedure documented, and contract tests passing in that mode.
- **B3. Explicit mode/capability contract:** expose/configure backend mode and capability (e.g. `postgres | meilisearch | hybrid-future`) and make precedence observable; prevents ambiguous behavior across environments.
- **Rebuild contract:** “If Meilisearch index state is unknown/stale, treat results as best-effort and rebuild from PostgreSQL before relying on it for parity-sensitive flows.”

## 6. Per-org/tenant index isolation

- **Current model:** one index per org (`flora_org_{org_id}`) — strong logical isolation boundary in Meilisearch (separate docs/settings/task scope per index). (Source: [Indexes](https://www.meilisearch.com/docs/learn/getting_started/indexes).)
- **Requirements for safety:** never accept index/org identifiers from untrusted client input for cross-tenant access; derive from authenticated `org_id`; preserve API-level membership checks (already present in route).
- **Tradeoffs (4 GB VPS):** many orgs/indexes increase metadata/memory/operational overhead; for small SMB Seed this is acceptable if bounded, but **NV** for upper org/index counts and task backlog behavior.
- **Alternative (not MVP):** shared index with mandatory `organization_id` filter + `filterableAttributes` — reduces index count but increases leakage risk if filters misapplied; not recommended as 80/20 change now.

## 7. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Missing mutation hooks (current) | Stale/missing results in Meilisearch mode. | Option A hooks (fail-open/outbox-lite) or Option B precedence limiting. |
| Soft-delete not propagated | Deleted entities remain searchable (PG excludes them). | Delete-on-soft-delete hook + backfill reconciliation. |
| Meilisearch async task lag/failure | Eventual consistency; write “succeeded” in app but not yet searchable. | Do not block core writes; monitor task status where needed; rebuild/backfill from PG. |
| Schema/shape drift | Different snippets, missing file metadata (`file_type`), `type` token mismatch, ranking divergence. | Align content fields, normalize snippet/tokens server-side, define/document non-goal for identical ranking. |
| VPS resource pressure (RAM/CPU/IO) | API/DB degradation, swap/OOM, slow indexing. | PG-only default for smallest profile, throttle/batch indexing, cap Meilisearch resources, single rebuild stream. |
| Tenant isolation mistake | Cross-org leakage. | Keep per-org indexes, server-derived org, contract tests for isolation in both backends. |
| Backup mistaken for parity repair | Restored dump/snapshot may not reflect latest PG truth or desired schema state. | Treat PG as source of truth; use logical reindex/backfill after restore/drift. |
| Multilingual relevance mismatch (pt-BR/es/de/fr/zh-CN/ja) | Recall/ranking differs from expectations or from PG. | Validate on representative corpora; keep deterministic PG fallback; document **NV**. |

## 8. NVs (needs verification)

- **NV-1:** Required freshness SLO (sync vs eventual) and acceptability of fail-open indexing for Seed tenants.
- **NV-2:** Canonical `type` token contract (aliases, unknown tokens, empty/omitted behavior) and required snippet format (plain/length/highlight) across backends.
- **NV-3:** Whether Meilisearch `searchableAttributes`/`displayedAttributes` must be explicitly set now for parity/perf and exact fields to expose.
- **NV-4:** Meilisearch relevance/tokenization quality for **pt-BR, es, de, fr, zh-CN, ja** on real Flora message/task/file corpora vs PG `'simple'`.
- **NV-5:** Safe resource caps and observed latency/task backlog on 2 vCPU/4 GB with co-located Postgres+API+Meilisearch.
- **NV-6:** Existence of restore/undelete flows and all soft-delete/hard-delete code paths that must emit hooks.
- **NV-7:** Need for finer-grained authorization filtering (workspace/channel/private visibility) beyond org membership in global search.
- **NV-8:** Rust `meilisearch-sdk` version/behavior in use (task waiting helpers, filter/highlight/crop options) and compatibility with desired API usage.
- **NV-9:** Expected org/index scale in Seed and whether per-org index strategy remains viable.

## 9. Recommendations for cycle 022 (small, verifiable slice)

**Preferred 80/20 slice (verifiable, VPS-safe): “contract + minimal correctness wiring”, not full distributed reliability.**

1. **Freeze/declare precedence & degradation (must-do):**
   - Keep PG fallback as the **parity baseline**. If Meilisearch is enabled without verified hooks/backfill, document it as **best-effort/eventually-consistent** (or restrict to entities proven wired) to avoid silent correctness claims.
2. **Implement minimal hooks (small scope):**
   - Add fail-open, post-commit best-effort upsert for **create/update** and **delete-from-index on soft-delete/hard-delete** for the entities touched in this cycle (start with tasks/files or all three if trivial), including task unassign/field edits and file metadata changes.
   - Call `setup_index(org_id)` lazily/idempotently where needed.
3. **Fix highest-value schema/shape mismatches:**
   - Include `file_type` in Meilisearch file `content` (and keep task `title+description`).
   - Normalize `snippet` and `type` token handling in the Meilisearch response path to match PG contract as closely as practical.
4. **Add rebuild/backfill procedure (operational, minimal):**
   - Provide a one-shot scoped backfill/reindex path from PostgreSQL source-of-truth (script/command/runbook step) and state: “run after enabling Meilisearch, after restore, or on suspected drift”.
5. **Verification (deterministic where possible):**
   - Contract tests for PG fallback remain mandatory.
   - For Meilisearch mode (when env/service available): verify create/update/delete/soft-delete visibility, `type` filtering, tenant isolation, and empty query behavior; allow for async-task timing by polling/waiting within bounded timeout in tests (**NV** harness choice).
6. **Defer (explicit):**
   - Transactional outbox worker/retries, hybrid query merging, identical cross-engine ranking, advanced multilingual analyzers/tuning, and multi-node HA.

**Decision summary:** for Seed on ~2 vCPU/4 GB, **PG-only/PG-authoritative** is the safe default; **optional Meilisearch** is acceptable in 022 only with minimal fail-open hooks + normalized contract + documented rebuild-from-PG path.

## 10. Sources and citations

### Primary sources
- Meilisearch — Tasks and asynchronous operations: https://www.meilisearch.com/docs/learn/async/asynchronous_operations
- Meilisearch — Tasks API (list): https://www.meilisearch.com/docs/reference/api/tasks
- Meilisearch — Indexes: https://www.meilisearch.com/docs/learn/getting_started/indexes
- Meilisearch — Indexes API: https://www.meilisearch.com/docs/reference/api/indexes
- Meilisearch — Filter search results: https://www.meilisearch.com/docs/learn/filtering_and_sorting/filter_search_results
- Meilisearch — Displayed/searchable attributes (relevancy): https://www.meilisearch.com/docs/learn/relevancy/displayed_searchable_attributes
- Meilisearch — Configure displayed attributes: https://www.meilisearch.com/docs/capabilities/full_text_search/how_to/configure_displayed_attributes
- Meilisearch — Documents API (add/replace/delete): https://www.meilisearch.com/docs/reference/api/documents
- Meilisearch — Dumps: https://www.meilisearch.com/docs/learn/data_backup/dumps
- Meilisearch — Snapshots: https://www.meilisearch.com/docs/learn/data_backup/snapshots
- PostgreSQL — Full Text Search: Tables and Indexes: https://www.postgresql.org/docs/current/textsearch-tables.html
- PostgreSQL — Controlling Text Search: https://www.postgresql.org/docs/current/textsearch-controls.html

### Flora repository evidence (cross-check)
- `flora-api/src/routes/search.rs`, `flora-api/src/state/app_state.rs`, `flora-api/src/routes/{tasks,files,channels}.rs`
- `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`
- `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `flora-core/src/migrations/sql/20261118000001_add_tasks_files_search_fts.sql`
- `docs/research/2026-11-16-021-meilisearch-task-file-search-parity-vps.md`, `docs/research/2026-11-16-021-optional-meilisearch-pg-coherence-vps.md`, `docs/research/2026-11-16-020-pg-fts-tasks-files-vps.md`, `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`
- `OBJETIVO.md`, `ROADMAP.md`

*End of research deliverable (cycle 022).*
