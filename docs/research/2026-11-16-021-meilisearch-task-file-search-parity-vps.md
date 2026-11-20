# 021 — Minimal Meilisearch parity for task/file search vs Postgres FTS fallback (VPS, 80/20)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`0dcde353a`) — research only; this document only.  
**Scope:** Multilingual, high-reliability research (EN, pt-BR, es, de, fr, zh-CN, ja) on achieving **minimal search parity for tasks and file metadata in Meilisearch mode** alongside the existing Postgres FTS fallback in a Rust + Axum + sqlx app (optional Meilisearch, ~2 vCPU / 4 GB VPS). Covers document schema/filterable attributes, indexing hooks on create/update/delete (sync vs async), `type` filter semantics and response/snippet consistency with `GET /api/v1/search`, tenant isolation (per-org index), soft-delete/unassign/update handling, and low-ops 80/20 for SMB.

## 1. Flora baseline (cross-check)

| Area | Evidence | Implication |
|---|---|---|
| API contract | `flora-api/src/routes/search.rs`: `GET /api/v1/search?q=&type=&limit` (auth + `require_org` + membership). Empty/whitespace `q` => `200 {"query", "results":[]}`. `limit` default 20, capped 50. `type` CSV, lowercased; `include_type` accepts `None/empty` as “all”, plus aliases `message|messages`, `task|tasks`, `file|files`. Response items: `{ "type": "message|task|file", "id", "snippet" }`. If `state.search_service` is `Some` it delegates to Meilisearch and returns hits as-is; else PG fallback (`search_pg`) does per-entity queries, merge-sort by `(rank desc, created_at desc, id desc)`, truncate to `limit`. | Meilisearch path is **all-or-nothing** today (no hybrid/per-type fallback). Parity must be defined for precedence and per-entity behavior. |
| PG fallback/indexing | `flora-core/src/migrations/sql/20261118000001_add_tasks_files_search_fts.sql` adds `tasks.search_tsv` and `files.search_tsv` as `GENERATED ALWAYS AS (to_tsvector('simple', ...)) STORED` + partial GIN `WHERE is_deleted=false` (messages already in `20261117000001_*`). `search_pg` uses `COALESCE(websearch_to_tsquery('simple',$q), plainto_tsquery('simple',$q))`, tenant filter `organization_id=$1`, `is_deleted=false`, `ts_rank_cd`, snippets via `left(regexp_replace(...),200)`. | PG already provides deterministic, DB-maintained coverage for message/task/file with strict tenant + soft-delete filters. |
| Meilisearch adapter | `flora-search/src/lib.rs`: per-org index `index_template.replace("{org_id}", ...)` (default `flora_org_{org_id}` in `flora-core/src/config/search.rs`). `setup_index` sets filterable `["type","created_at_timestamp"]`. `index_message/task/file` add docs with primary key `id` (string UUID) and fields: common `id,type,content,created_at,created_at_timestamp`; message adds `channel_id,sender_id`; task adds `assignee_id,status` and `content = "{title} {description}"`; file adds `size` and `content = name` only (no `file_type`, no workspace/owner in doc). `search` builds `type IN [...]` filter when `types` non-empty, returns `{id,type,snippet}` from stored `content` (full field, untrimmed). `delete_document` exists. | Schema/filters/snippets differ from PG (file `file_type` not searchable/indexed in content, message/task snippets not 200-char normalized, task not deleted from index on soft-delete in current wiring). |
| Config/state/wiring | `SearchConfig` (`enabled,url,api_key,default_limit,max_limit,index_template`; env `MEILISEARCH_URL/API_KEY`). `AppState` initializes optional `search_service` at startup; no observed mutation-path calls to `setup_index/index_*/delete_document` in `flora-api/src/routes/{tasks,files,channels,...}` or elsewhere (`rg` shows only state/health/search/auth wiring). | **Parity gap is primarily indexing lifecycle wiring + contract normalization**, not missing Meilisearch primitives. |
| Prior research | `docs/research/2026-11-16-020-pg-fts-tasks-files-vps.md`: additive PG FTS, safe parsing, partial GIN, `UNION ALL`/merge semantics, tenant/soft-delete guards, VPS tradeoffs, NVs on relevance/perf/ACL/type tokens/snippets. | 020 is the correctness baseline/fallback to align with. |

## 2. Meilisearch design points (primary sources, EN)

- **Indexes/multi-tenancy:** an index is an isolated document/settings namespace; per-tenant indexes are a standard isolation pattern (settings/tasks scoped per index). ([Meilisearch: Indexes](https://www.meilisearch.com/docs/learn/getting_started/indexes))
- **Filterable attributes & `filter`:** attributes used in `filter` must be declared in `filterableAttributes` (reindex/update cost); `filter` supports `=`, `IN`, comparisons, boolean logic at search time. ([Filter search results](https://www.meilisearch.com/docs/learn/filtering_and_sorting/filter_search_results))
- **Documents write semantics:** add/replace is async-task oriented in general API usage; document updates require the primary key; delete-by-id removes a document from the index. (Current adapter uses `add_documents(&[doc], Some("id"))` / `delete_document(id)` via Rust SDK.) ([Documents API](https://www.meilisearch.com/docs/reference/api/documents#add-or-replace-documents), SDK behavior in `flora-search`)
- **Displayed/retrieved fields & highlighting:** stored fields are returnable subject to settings/`attributesToRetrieve`; highlighting/cropping are query-time controls (`attributesToHighlight`/`attributesToCrop`) rather than implicit in stored `content`. ([Displayed attributes](https://www.meilisearch.com/docs/learn/relevancy/displayed_searchable_attributes#displayed-fields))

## 3. Multilingual synthesis (EN, pt-BR, es, de, fr, zh-CN, ja)

| Locale/script | 80/20 guidance | Confidence |
|---|---|---|
| **EN** | Tokenization/relevance generally robust for title/name/short text; parity mainly about lifecycle, filters, and snippet shape. | High. |
| **pt-BR, es, fr, de** | Latin scripts with accents benefit from Meilisearch’s normalization/tokenization more than PG `'simple'` in many cases; keep schema language-agnostic (`content` text) and avoid locale-specific settings for MVP. Diacritics/case differences should be validated against real corpora. | Medium (corpus-dependent, **NV**). |
| **zh-CN, ja** | CJK/Japanese segmentation quality is engine/version/dictionary-dependent; short metadata (`title`, `name`, `file_type`) is lower risk than long prose, but parity with PG `'simple'` is not guaranteed. Do not promise equal recall/ordering across engines. | Low–medium (**NV**). |
| **Operational (all)** | One Meilisearch node on 2 vCPU/4 GB is feasible for small SMB indexes if writes are bounded and indexing is decoupled from request latency; memory/disk headroom and task backlog are the main constraints. | Medium (unmeasured, **NV**). |

## 4. Concrete 80/20 options

### Option A — **Minimal indexing wiring parity (recommended if Meilisearch is kept “on”)**
Make Meilisearch produce equivalent *coverage* for task/file/message mutations while keeping PG as correctness fallback.

- **Document schema (minimal, aligned):**
  - Common: `id` (string PK), `type` (`message|task|file`), `content` (searchable text), `created_at` (RFC3339), `created_at_timestamp` (int, filterable/sortable-friendly).
  - Message: `channel_id`, `sender_id` (+ keep `content` = body).
  - Task: `title`, `description` (optional but recommended as separate fields) **and/or** `content = title + " " + description` for backward-compatible searchable blob; include `status`, `assignee_id`, `workspace_id`, `organization_id` only if needed for future filters (not required by current API).
  - File (metadata parity): include `name`, `file_type` and set `content = "{name} {file_type}"` (currently misses `file_type`); include `workspace_id`, `owner_id` if useful later.
  - Keep filterable attributes minimal: `["type","created_at_timestamp"]` is sufficient for current `type` filter; add others only with measured need.
- **Index lifecycle hooks (create/update/delete):**
  - **Create/update:** after successful DB commit, call `index_message/task/file` (or unified upsert) — upsert by `id` means create/edit are the same path. For task reassignment/unassign, reindex doc (assignee/status/content may change); soft-delete is **not** a field filter in Meilisearch path today, so treat as **delete from index**.
  - **Delete:** on hard delete or soft-delete (`is_deleted=true`) call `delete_document(org_id, id)`. On undelete/restore (if supported), reindex.
  - **Sync vs async:** *80/20 sync-but-fail-open*: fire-and-log (or bounded timeout/background task) so user mutations on VPS do not depend on Meilisearch availability; accept short eventual-consistency window. *Stronger*: outbox/job worker with retries — better durability but more ops/code (defer).
  - **Setup:** ensure `setup_index(org_id)` on first org use or lazily before first write/search (idempotent task submission acceptable).
- **`type` semantics parity:** normalize requested CSV tokens identically in both backends (accept singular/plural, ignore unknown consistently or reject consistently — **pick one and test**). When Meilisearch enabled, ensure it can serve `task`/`file` (not just messages) with same token behavior as PG.
- **Response/snippet parity:** define `snippet` contract explicitly: either (1) **normalized plain snippet** (e.g. 200 chars, whitespace-collapsed, source text) for both engines, or (2) engine-native highlights in Meilisearch behind a flag while PG keeps plain snippets. For strict parity with current PG, prefer (1): trim/collapse `content` (or compose from title/name) server-side after Meilisearch search.
- **Tenant isolation:** keep per-org physical index naming (`flora_org_{org_id}`) as primary boundary; never accept index name/org id from client. Optionally add stored `organization_id` field for defense-in-depth/debug, but do not rely on shared-index filters for MVP.
- **Soft-delete/unassign/update:** delete index doc on soft-delete; reindex on content/title/description/name/file_type/status/assignee changes; unassign = reindex with `assignee_id: null`.
- **Ops (2 vCPU/4 GB):** single Meilisearch instance, cap searchable payloads, avoid heavy per-request settings changes, monitor RAM/disk/task queue; keep PG fallback authoritative when Meilisearch unavailable/misconfigured.

### Option B — **Explicit contract/precedence decision (lowest-risk 80/20 for small VPS)**
Avoid dual-write complexity by declaring backend responsibilities.

- **B1. PG-authoritative for parity, Meilisearch optional accelerator:** route `task`/`file` (and optionally all types) to PG fallback even when Meilisearch is configured, or restrict Meilisearch to message-only until hooks exist. Eliminates stale-index/delete drift for tasks/files immediately.
- **B2. Meilisearch-authoritative when enabled:** require indexing hooks (Option A core) + consistency tests; PG fallback only when `search_service` is `None` (current structure) — but document that task/file results depend on index freshness.
- **B3. Mode flag:** add explicit config (e.g. `search.provider = meilisearch|postgres|hybrid`) to remove ambiguity; **NV** if out of immediate scope, but recommended as a follow-up contract decision.
- **Why this is 80/20:** a one-line precedence/contract choice prevents “enabled but incomplete” behavior without building a full outbox.

## 5. Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| **No indexing hooks today** | Meilisearch returns stale/missing task/file/message docs (create/update/delete/soft-delete not reflected). | Option A wiring with fail-open async best-effort, or Option B1 until wired. |
| **Soft-delete leakage/staleness** | Deleted records remain searchable in Meilisearch (no `is_deleted` filter). | Delete index doc on soft-delete; periodic reconciliation/reindex job if needed. |
| **Schema/snippet drift** | File `file_type` not in Meilisearch `content`; snippets/ranking/order differ from PG; clients see inconsistent UX by backend. | Align file content/fields, normalize snippet server-side, document ordering/ranking as non-identical across engines. |
| **Tenant isolation bug/misconfig** | Cross-org leakage if shared index/filtering used incorrectly. | Keep per-org indexes, never trust client org, test isolation per backend. |
| **`type` token mismatch** | `task` vs `tasks`/unknown tokens behave differently across engines. | Single normalization/validation function and contract tests for both paths. |
| **VPS resource pressure** | Meilisearch + Postgres + app can contend for RAM/CPU/IO on 4 GB. | Make Meilisearch truly optional, lazy/small indexes, monitor, prefer PG-only for smallest deployments. |
| **Eventual consistency** | Recently mutated items missing briefly in async/fail-open mode. | Accept for SMB 80/20; expose via docs/tests; upgrade to outbox/retry if required. |
| **AuthZ scope gap** | Current search is org-membership scoped; finer workspace/channel ACLs not enforced in global query. | Preserve existing boundary; treat finer ACL as separate **NV**. |

## 6. NVs (needs verification)

- **NV-1:** Required freshness SLA (sync vs async acceptable) and whether fail-open indexing is permissible for production SMB tenants.
- **NV-2:** Canonical `type` contract across backends (accepted aliases, unknown-token handling, omitted `type` = all supported entities) and required snippet format/length/highlighting.
- **NV-3:** Meilisearch relevance/tokenization quality for **pt-BR, es, de, fr, zh-CN, ja** on real task/file/message corpora vs PG `'simple'` (+trgm) fallback.
- **NV-4:** Per-org index count/size/task-queue and p95 latency impact on 2 vCPU/4 GB; need for index eviction, quotas, or tenant caps.
- **NV-5:** Whether task/file visibility must respect workspace/channel/private ACLs beyond org membership in global search.
- **NV-6:** Need for full reindex/backfill tooling and reconciliation when wiring hooks into existing data (and safe rollout order).
- **NV-7:** SDK/client `filter`/search parameter compatibility (Rust `meilisearch-sdk` version in use) for any future `attributesToCrop/highlight` or sortable/filter extensions.

## 7. Recommended 80/20 decision

1. **Decide precedence now (Option B):** for low-ops VPS, prefer **PG fallback as parity baseline**; enable Meilisearch only where lifecycle wiring and contract tests exist (or limit to entities proven wired).
2. **If enabling Meilisearch parity (Option A):** wire fail-open upsert-on-create/update and delete-on-soft/hard-delete for message/task/file; add `file_type` to file search content; normalize `type` tokens and snippet shape; keep per-org indexes and `type` filterable.
3. **Validate with contract tests:** same cases on both modes where applicable — task/file hit, omitted/type-filter behavior, tenant isolation, soft-delete exclusion, update/unassign visibility, empty/special queries (no 5xx).

## 8. Primary sources

- Meilisearch — Indexes: https://www.meilisearch.com/docs/learn/getting_started/indexes
- Meilisearch — Filter search results: https://www.meilisearch.com/docs/learn/filtering_and_sorting/filter_search_results
- Meilisearch — Documents API (add/replace, delete): https://www.meilisearch.com/docs/reference/api/documents#add-or-replace-documents
- Meilisearch — Displayed/searchable attributes & retrieval concepts: https://www.meilisearch.com/docs/learn/relevancy/displayed_searchable_attributes#displayed-fields
- PostgreSQL 18 — Text search tables/indexes, parsing/ranking: https://www.postgresql.org/docs/current/textsearch-tables.html , https://www.postgresql.org/docs/current/textsearch-controls.html

## 9. Repo references

- `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`, `flora-api/src/state/app_state.rs`
- `flora-api/src/routes/search.rs`, `flora-api/src/routes/{tasks,files,channels}.rs`
- `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql`
- `flora-core/src/repositories/{message,task,file}_repository.rs`
- `docs/research/2026-11-16-020-pg-fts-tasks-files-vps.md`
- `flora-tests/src/contract/search.rs` (DB-gated PG contract baseline)
