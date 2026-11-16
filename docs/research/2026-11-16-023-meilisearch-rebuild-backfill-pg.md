# 023 — Deterministic rebuild/backfill of the optional Meilisearch index from PostgreSQL (Flora, Seed/VPS 80/20, multilingual)

**Date:** 2026-11-16
**Repo:** `flora` @ `023bd2467` — **research only**; this document only (no source changes, no commits).
**Target:** Seed/Postgres-first, optional Meilisearch on ~2 vCPU / 4 GB RAM / SSD single VPS.
**Question answered:** How to deterministically rebuild/backfill the *derived* search index from PostgreSQL as the authoritative source, with bounded memory/IO, resumability, progress signals, and a safe cutover — and when a plain admin CLI is enough vs when an outbox is warranted.

## 1. Scope and method

Covered: keyset/batched reads of `is_deleted = false` rows; per-org scoping; idempotent upsert by primary key; low-memory streaming; progress/observability; operation under CPU/RAM/IO limits; interruption/resume; alias/blue-green swap; CLI/backfill vs outbox decision.

**Method (high-reliability, multilingual):** Flora code/schema/config cross-check (primary evidence for current behavior) + primary-source docs from PostgreSQL and Meilisearch + one reputable engineering writeup where PG docs are terse (keyset pagination). Findings are tagged EN, pt-BR, es, de, fr, zh-CN, ja; corpus-dependent relevance claims are **NV**. Web search was quota-limited in-session; all primary pages were fetched directly by URL (see §10).

**Definition used here:** a *deterministic rebuild* is a run that, given the same committed PG state and the same config, produces the same document set in the derived index, is replayable after interruption, and has an explicit, verifiable cutover point. It is **not** a promise of identical ranking to PG FTS (prior research: 021/022).

## 2. Flora cross-check (baseline)

| Area | Evidence (repo) | Baseline / implication for 023 |
|---|---|---|
| Authoritative source | `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql`: generated/stored `search_tsv` + partial GIN `WHERE is_deleted = false` for `messages`, `tasks`, `files`. | PG already holds a transactional, tenant-safe search representation. PG is the rebuild source; Meilisearch is a derived projection. |
| Source columns | `20240818000001_init_schema.sql`: all three tables have `id UUID PRIMARY KEY DEFAULT uuidv7()`, `organization_id UUID NOT NULL`, `is_deleted BOOLEAN NOT NULL DEFAULT FALSE`, `created_at`, `updated_at`. `files` adds `name`, `file_type`, `size_bytes`; `tasks` adds `title`, `description`, `status`, `assignee_id`; `messages` adds `content`, `channel_id`, `sender_id`. | UUIDv7 IDs are time-sortable *and* unique → `id` is a natural, index-backed keyset cursor with a deterministic tie-break built in. `updated_at` is **not** indexed (relevant only to incremental sync, not full rebuild). |
| Existing indexes | `idx_messages_organization_created`, `idx_tasks_organization_created` on `(organization_id, created_at DESC)`; **no** equivalent on `files`; PK index on `id` for all; partial `idx_*_is_deleted`. | Keyset by bare `id` works uniformly via the PK and needs no new index. Per-org keyset by `(organization_id, id)` would benefit from a small partial index (NV cost). Avoid `(created_at, id)` unless adding a `files` index. |
| Meilisearch adapter | `flora-search/src/lib.rs`: per-org index via `index_template.replace("{org_id}", …)`; `setup_index` sets filterable `["type","created_at_timestamp"]`; `index_message/task/file` call `add_documents(&[doc], Some("id"))` **one document per task**; `delete_document`; `search`; no batch add, no swap, no stats, no task monitoring. `file` doc `content = name` only (no `file_type`), snippets untrimmed. | **Gap:** no bulk ingestion, no cutover, no progress. A rebuild of N rows with current code enqueues N tasks — dangerous on a small instance (task DB limits, §3.2). Must add batch add + swap + stats in the same change. |
| Config/state | `flora-core/src/config/search.rs`: `enabled`, `url`, `api_key`, `default_limit`, `max_limit`, `index_template` (`flora_org_{org_id}`). `AppState::new` degrades to `search_service: None` on init failure; route delegates to Meilisearch when present, else PG fallback. | “Configured” ≠ “fresh/complete”. During an in-place rebuild, results are partial unless we fall back to PG or use a shadow index. Blue-green avoids this. |
| Mutation wiring | No calls to `index_*`/`delete_document` from mutation routes (022 confirmed, deferred). | A rebuild fixes a snapshot; it does **not** keep it fresh. Current determinism caveat: concurrent writes during the scan can be missed. Must be contracted (§6.3). |
| Tooling | `clap = { version = "4.5", features = ["derive","env"] }` in workspace `Cargo.toml`; `flora-api` has `[[bin]] name = "flora-api"`. `meilisearch-sdk = "0.33"`. | Cheapest home is a second `[[bin]]` (e.g. `flora-search-rebuild`) in `flora-api`, reusing `flora_core::config::Config` + pool + `flora_search`. |
| Prior research | 018 (outbox-lite + `SKIP LOCKED`), 021 (PG-first, rebuildable projection), 022 (hooks/reindex parity, drift contract) all name “rebuild from PG” as mandatory but do not specify mechanics. | This document fills that specific gap; it does not re-litigate backend precedence. |

## 3. Primary-source findings

### 3.1 PostgreSQL: batched/keyset reads and streaming

- **Cursors.** `DECLARE … CURSOR [ NO SCROLL ] … FOR query` retrieves a small number of rows at a time; `FETCH FORWARD n` pulls n rows. `WITHOUT HOLD` (default) confines the cursor to a transaction; **`WITH HOLD` copies the cursor's rows into a temporary file or memory area** so they survive commit. [S1, S2]
  - Implication for 4 GB: a long `WITH HOLD` cursor materializes the whole result set (temp file/memory) and holds resources. Prefer stateless keyset batches (below); if a cursor is used, keep it `WITHOUT HOLD` inside a short transaction and `FETCH FORWARD n`.
- **Keyset/seek pagination.** PG supports row-value predicates (`(a,b) < (?,?)`) with proper index access since 8.4, and `OFFSET`/`FETCH FIRST` come with documented costs: OFFSET must count/skip all prior rows (growing cost) and **pages drift when rows are inserted**. Deterministic paging requires a unique, deterministic sort key. [S3, S4]
  - Implication: use `WHERE id > $last ORDER BY id ASC LIMIT $n`. Never `OFFSET`. UUIDv7 PK gives a unique total order for free.
- **Queue claims (for the outbox comparison only).** `SELECT … FOR UPDATE … SKIP LOCKED` is supported. [S3] (Prior 018 research already leverages this.)
- **Consistency choice.** A single repeatable-read snapshot would be ideal but implies a long transaction/`WITH HOLD` materialization (bad on small VPS). Practical compromise: keyset batches under read-committed plus either a write quiescence window or an `id <= watermark` bound (§6.3). **NV:** exact freshness guarantee under concurrent writes.

### 3.2 Meilisearch: tasks, batching, swap, stats, backup

- **Everything expensive is an async task.** Document add/update/delete, settings, index create/delete/**swap**, dumps/snapshots all return a summarized `taskUid` (`enqueued`) and are processed in order. Statuses: `enqueued → processing → succeeded|failed|canceled`. [S5]
  - Auto-batching groups *consecutive compatible* tasks: same index, same type, same content type; settings updates never batch with document adds. [S5]
  - **Task DB hard edges:** queue ~10 GiB (`no_space_left_on_device`), 1M stored tasks (auto-deletes oldest 100K finished). Thus “one task per row” backfill is an operational hazard; batch documents into few tasks. [S5]
  - **Interruption safety:** terminating the instance mid-task is safe; `processing` tasks reset to `enqueued` on restart. [S5]
- **Idempotent upsert by PK.** `add-or-replace` (SDK: `add_or_replace(&[docs], Some("id"))`) overwrites the whole document for an existing `id`, **removing fields absent from the new payload**; a missing index is created; default HTTP payload cap ~100 MB; `primaryKey` only effective on first add. [S8, S15]
  - This is exactly the replay-safe primitive a resumable backfill needs: re-sending a batch is a no-op in final state.
- **Progress/verification signals.** `GET /indexes/{uid}/stats` returns `numberOfDocuments`, `isIndexing`, `indexSize`, and **`fieldDistribution`** (per-field doc counts). Task details expose `receivedDocuments`/`indexedDocuments`. `GET /tasks/{uid}` and list/cancel/delete routes allow per-batch confirmation; `GET /tasks/stream` and `/batches/stream` are experimental (need `tasksStreamingRoute`) and are live feeds, not history. `customMetadata` can tag document tasks (raw HTTP; **SDK 0.33's `add_or_replace` does not expose it**). [S6, S9, S15]
- **Native blue-green: `POST /swap-indexes`.** Swaps documents, primary key, **settings, and task history** of two indexes; multiple pairs per request; **atomic** (all or none); global task; optional `rename` (rename first index to second instead of swapping). SDK 0.33 exposes `client.swap_indexes([&SwapIndexes { indexes: (a,b), rename: None }])` and `wait_for_completion`. [S7, S15]
  - **Caveat:** “enqueued tasks are left unmodified.” A mutation enqueued just before the swap can land on the index that changes identity. Quiesce writers around the swap (or route mutations through the shadow during cutover).
- **Settings before data.** Configure settings *before* adding documents; changing settings after indexing triggers a **full reindex**. Smaller docs (drop non-searchable fields) index faster and use less RAM. Too many indexes cost resources; separate indexes per language improve relevancy. [S10]
- **Resource behavior.** The indexer defaults to ≤2/3 of RAM and ≤1/2 of CPU cores; `--max-indexing-memory` and `--max-indexing-threads` override. Smaller batches reduce peak memory; as a rule the dataset should fit in RAM during indexing; settings attributes add memory cost. Experimental caps `--experimental-max-number-of-batched-tasks` and `--experimental-limit-batched-tasks-total-size` (default half RAM, max 10 GiB) and `--experimental-reduce-indexing-memory-usage` (`MDB_WRITEMAP`) exist. [S11, S12]
  - On 2 vCPU, default indexing threads = 1; with PG co-located, pin `--max-indexing-memory` well below total RAM (NV exact value) and `--max-indexing-threads=1`.
- **Backups are not rebuilds.** Snapshots = exact same-version copy (fast restore); dumps = portable cross-version blueprint (slow, requires full re-index). Dump/snapshot staging uses `TMPDIR` (often `/tmp`), so a small `/tmp` can fail even with free dump/snapshot space. Neither replaces a logical rebuild from PG when the index drifts. [S12, S13]
- **Vendor's own PG migration guide:** states PG remains source of truth; its sample uses `LIMIT/OFFSET` and accumulates all rows in memory (explicitly **not** acceptable for Flora's RAM budget), and recommends a server-side cursor or `COPY` for very large tables. It lists sync options: app-level sync, `LISTEN/NOTIFY`, periodic `updated_at` sweep, CDC. [S14]

## 4. Multilingual considerations (EN, pt-BR, es, de, fr, zh-CN, ja)

Rebuild mechanics are language-agnostic; relevance and index-layout choices are not.

| Locale | Rebuild-relevant signal | 80/20 guidance | Confidence |
|---|---|---|---|
| **EN** | PG `'simple'` + Meilisearch tokenizers both handle short ASCII metadata; divergence is mostly ranking/typo tolerance. | Rebuild for *coverage* and invariants, not ranking parity. Validate `type`/snippet contract. | High (engine behavior) |
| **pt-BR** | Acentuação e plurais podem ranquear diferente; `simple` não usa dicionário PT. | Included `name`/`title`/`description`/`file_type` unchanged in docs; validate with accented queries post-rebuild. | Medium, **NV** corpus |
| **es** | Acentos y singular/plural divergen entre motores. | Misma regla: reconstruir cobertura; medir recall con consultas acentuadas. | Medium, **NV** |
| **de** | Komposita/Umlaute; `simple` no stemming alemán. | Para metadatos cortos basta; no prometer ordenación igual. | Medium, **NV** |
| **fr** | Diacríticos/élisiones; tokenización distinta. | Mantener PG como referencia determinística; verificar tras rebuild. | Medium, **NV** |
| **zh-CN** | 中文分词对引擎/版本敏感；PG `simple` 对 CJK 弱，Meilisearch 常有优势但不等价。 | 短字段风险较低；优先保证「可重建 + 不漏租户」，召回质量用真实语料验证。 | Low–Medium, **NV** |
| **ja** | 漢字/かな混在で分かち書き差が出る。 | 同上。再構築手段と不変条件を優先し、品質は実データで検証。 | Low–Medium, **NV** |
| **Cross-locale ops** | Meilisearch recommends separate indexes per language, but Flora already uses per-org indexes; a second language dimension multiplies index count on a small VPS. [S10] | **Do not** add per-language indexes in 023. Accept mixed-language relevancy; log it as NV. | Medium |

## 5. Concrete minimal rebuild design (recommended for 023/024)

**Shape:** new `[[bin]] flora-search-rebuild` in `flora-api` (or a `--rebuild` subcommand), using `clap`, `flora_core::config::Config::load()`, the existing PG pool, and a small extension of `flora-search` (batch add, index stats, swap, delete index). One-shot, operator-run, synchronous; **no daemon, no outbox**.

### 5.1 Preflight
1. Load config; require `search.enabled` and a reachable Meilisearch (`GET /health`). Fail closed on misconfiguration (this is an admin job, not a request path).
2. Resolve scope: `--org <uuid>` (single tenant) or `--all-orgs` (loop `SELECT id FROM organizations ORDER BY id`). Never accept an index name from input; always derive via `SearchConfig::index_name(org_id)`.
3. Generate `run_id` (UUIDv4). Live index `L = flora_org_<org>`; shadow index `S = flora_org_<org>__rebuild_<run_id>` (UID-safe: alnum/`-`/`_`).
4. Decide mode: **blue-green** (default) or **in-place** (`--in-place`, low-disk fallback).

### 5.2 Prepare shadow (blue-green)
- Apply settings to `S` **first** — the same set `setup_index` uses (`filterableAttributes = ["type","created_at_timestamp"]`; optionally explicit `searchableAttributes = ["content"]`) — then wait for the `settingsUpdate` task to `succeeded`. Creating settings creates the index. [S8, S10, S15]
- SDK note: `wait_for_completion(client, None, None)` defaults to **5 s timeout** — too short on this VPS. Pass an explicit `Duration` (e.g. 5–15 min) or poll `get_task` in a bounded loop. [S15]

### 5.3 Stream from PG (keyset, per entity, per org)
For `E ∈ {messages, tasks, files}`, cursor `$2` starts at the zero UUID (all UUIDv7 rows sort above it):

```sql
SELECT id, content                                   -- messages
FROM messages
WHERE is_deleted = false
  AND organization_id = $1
  AND id > $2
ORDER BY id ASC
LIMIT $3;
-- tasks:      SELECT id, title, description, status, assignee_id, created_at, ...
-- files:      SELECT id, name, file_type, size_bytes, created_at, ...
```

- Use `$3 = 500–1000` (tune; byte cap ≤ ~16–32 MB, far below the 100 MB payload limit). [S8, S12]
- Execute via `sqlx` with `fetch()`/`try_next()` and keep **only one batch** of `serde_json::Value` in memory. No `OFFSET`, no `fetch_all`, no `WITH HOLD`. [S1–S4]
- Optional stability bound: capture `max(id)` per entity at start and add `AND id <= $watermark` so inserts during the run cannot extend the scan. Updates to already-scanned rows still need a follow-up (§6.3).
- Map rows to documents with **one shared mapper used by both hooks and rebuild** (fix the known `file_type` drift here). Use `id` as PK exactly as today. [F: `flora-search/src/lib.rs`]
- Bulk upsert: `index(S).add_or_replace(&docs, Some("id"))` → one `documentAdditionOrUpdate` task per batch, not per row. Wait or record `task_uid` and poll later. [S5, S8, S15]
- Advance the checkpoint to the batch's last `id` only **after** the task is confirmed `succeeded` (or after all queued tasks are confirmed at the end). Replay is safe because of full-document overwrite. [S8]

### 5.4 Checkpoint / resume
- Lightweight: a JSON file at a configured path, written atomically (temp file + rename), keyed by `(run_id, org_id, entity) → {last_id, rows_done, task_uids}`. Durable alternative: small PG table `search_rebuild_checkpoint` (requires a migration; better queryability).
- On restart with `--resume <run_id>`: skip completed `(org, entity)` pairs and continue from `last_id`. Because upsert is idempotent, replaying the last batch is harmless. [S5, S8]
- Interrupted mid-task is safe on the Meilisearch side (`processing → enqueued` on restart); re-query the recorded `task_uid`s. [S5]

### 5.5 Verify, cut over, finish
1. Wait for all document tasks to finish; fail on any `failed` task (log `error.code`/`message`).
2. Reconcile per org: PG `count(*) WHERE is_deleted=false AND organization_id=$1` per entity vs `GET /indexes/S/stats` → `numberOfDocuments` and `fieldDistribution.type` (the index mixes the three entity types). Investigate mismatches; do not swap on mismatch. [S9]
3. Cutover: `client.swap_indexes([&SwapIndexes { indexes: (S, L), rename: None }])`, wait for `indexSwap` to `succeeded`. [S7, S15]
4. Post-swap: re-check `stats` on `L`; if wrong, **swap back** (atomic, same primitive) and keep the shadow for diagnosis.
5. Delete the old index (now `S`) only after verification; update checkpoint `status = completed`.
6. **In-place mode:** `delete_all_documents` on `L`, then stream; during the window mark the search capability degraded and serve PG fallback. Simpler, but partial visibility and no rollback — use only when disk cannot hold two indexes. [S15]

### 5.6 Safety envelope (2 vCPU / 4 GB)
- Meilisearch launch flags: `--max-indexing-memory` pinned (start ~512 MiB–1 GiB, **NV**), `--max-indexing-threads=1`, `--no-analytics`, loopback bind, `MEILI_ENV=production`, explicit small `TMPDIR` on a volume with free space if dumps/snapshots are ever used. [S12]
- Optional experimental caps if task queue pressure appears: `--experimental-max-number-of-batched-tasks`, `--experimental-limit-batched-tasks-total-size`. [S12]
- Job PG pool `max_connections = 2`; run off-peak; throttle between batches if `stats.isIndexing` stays true or DB latency rises. [S9, S11]
- Do **not** run dumps/snapshots concurrently with a rebuild (staging copies compete for disk/RAM). [S12, S13]
- Per-tenant isolation: derive index name from authenticated/admin config only; keep the API's `require_org` + membership checks unchanged; the job is an admin tool, never a client-facing route.

## 6. 80/20 options

| Option | Description | Pros | Cons | Verdict |
|---|---|---|---|---|
| **A — In-place batched backfill** | Keyset batches into live index, no shadow. | Smallest code delta; one index on disk. | Partial results during run; no atomic cutover/rollback. | Acceptable only for a brand-new empty index or maintenance window. |
| **B — Shadow + verify + atomic swap (CLI only)** | §5 without checkpoint table. | Deterministic cutover; live index untouched until swap; restart = rerun. | Needs ~2× index disk briefly; must pause writers around swap. | **Recommended baseline for 023/024.** |
| **C — B + checkpoint/resume + bounded caps + verification** | Full §5. | Survives interruption; observable; VPS-safe. | Slightly more code/config. | **Recommended if real data exists.** |
| **D — Transactional outbox + replay worker** | Persist index intent with domain write; drain idempotently. | Durable freshness for *ongoing* mutations; auditable lag. | Migration + worker + retries; not needed for a one-shot rebuild. | **Defer to 024+**, only if Meilisearch becomes a real profile. |
| **E — CDC / Debezium / full platform** | Stream WAL. | Strongest near-real-time sync. | Far beyond 2 vCPU/4 GB Seed. | **Reject.** |

**CLI vs outbox decision rule:** a rebuild is a rare, operator-triggered, finite job with a natural completion boundary — a synchronous CLI is deterministic, trivially testable and needs no new runtime infra. An outbox solves a *different* problem: durability of per-write side effects between commits. Use CLI for bootstrap/restore/schema-change/drift recovery; add outbox (or at minimum fail-open hooks + periodic `updated_at` sweep, which Meilisearch itself lists [S14]) only when continuous freshness is required. A rebuild remains mandatory alongside either strategy. [S14; prior 018/021/022]

## 7. Risks and NVs

| Risk | Impact | Mitigation |
|---|---|---|
| Concurrent writes during scan | Missing/extra docs; non-deterministic result. | Quiesce writes (maintenance window) or `id <= watermark`; document the residual need for incremental catch-up; **NV** for exact guarantee. |
| Swap with enqueued tasks | “Enqueued tasks are left unmodified” → mutation lands on wrong index. [S7] | Pause writers/route to shadow around swap; verify with `stats` post-swap; swap back on failure. |
| One task per row | Task DB exhaustion (`no_space_left_on_device`, 1M cap). [S5] | Batch `add_or_replace`; record few `task_uid`s. |
| Peak indexing RAM | OOM / search latency collapse. [S11] | Small batches, explicit `--max-indexing-memory`, 1 thread, no concurrent dumps. |
| Settings changed after data | Full reindex unexpectedly. [S10] | Settings first on shadow; version settings in code/config. |
| Doc-shape drift (file `file_type`, snippet) | PG/Meilisearch contract mismatch persists after rebuild. | Single shared mapper; contract tests in both modes. |
| ID collision across entity types | One doc overwrites another in the shared per-org index. | UUIDv7 collisions negligible; consider namespaced `id = "<type>:<uuid>"` later (breaking change; defer). |
| Many org indexes | Index metadata/RAM growth; vendor warns against too many indexes. [S10] | Scope `--org` where possible; **NV** org-count ceiling. |
| `customMetadata` unavailable in SDK 0.33 | Weaker per-batch task tagging. | Raw HTTP for the query param, or log `task_uid`→batch mapping locally. **NV.** |
| Dump/snapshot staging in `/tmp` | Disk full during backup. [S12] | Set `TMPDIR` to a roomy volume; never overlap with rebuild. |

**NVs (needs verification, not estimates):**
1. **NV-1** Real RSS/latency of Meilisearch + PG + API co-located on 2 vCPU/4 GB during a 100k-row rebuild; choose batch size and `--max-indexing-memory` from measurement.
2. **NV-2** Safe org/index count ceiling on this VPS.
3. **NV-3** Relevance/recall deltas for pt-BR/es/de/fr/zh-CN/ja on real Flora corpora (not a rebuild blocker).
4. **NV-4** Whether a `partial (organization_id, id) WHERE is_deleted = false` index is worth its write cost (PK keyset already works).
5. **NV-5** Exact concurrency contract during swap (pause hooks vs accept rare drift).
6. **NV-6** Whether SDK 0.33 `wait_for_completion` polling interval/timeout is stable enough; else explicit `get_task` loop.
7. **NV-7** Checkpoint placement: file (no migration) vs PG table (migration + durability).

## 8. Recommendation for cycle 023

- **023 is research-only:** no code. Record the design above as the agreed rebuild contract.
- **For 024 (if Meilisearch is kept enabled):** implement **Option C** — `flora-search-rebuild` CLI with blue-green shadow, batched UUIDv7 keyset reads, idempotent `add_or_replace`, checkpoint/resume, task + `stats` verification, atomic `swap_indexes`, rollback-by-swap — plus the shared document mapper (fix `file_type`). Keep PG authoritative and PG fallback intact.
- **Explicitly defer:** outbox/CDC, per-language indexes, cross-engine ranking parity, hybrid query merge, multi-node HA.
- **Acceptance criteria for that implementation:** (a) two identical runs produce equal `numberOfDocuments`/`fieldDistribution` per org; (b) interrupting and resuming converges to the same state; (c) no tenant leakage (active rows of org A never appear in index B); (d) `is_deleted = true` rows absent; (e) failed tasks abort before swap; (f) peak RSS stays below the measured budget (NV-1).

---

## 9. Sources and citations

### Primary — PostgreSQL
- **[S1]** PostgreSQL Docs: `DECLARE` (cursor types; `WITH HOLD` materializes rows into temp file/memory) — https://www.postgresql.org/docs/current/sql-declare.html
- **[S2]** PostgreSQL Docs: `FETCH` (`FETCH FORWARD n`, cursor positioning) — https://www.postgresql.org/docs/current/sql-fetch.html
- **[S3]** PostgreSQL Docs: `SELECT` (`LIMIT/OFFSET`, row values, `FOR UPDATE … SKIP LOCKED`) — https://www.postgresql.org/docs/current/sql-select.html
- **[S4]** Markus Winand, *Use The Index, Luke!* — “Paging Through Results” (offset drift/cost; seek method; deterministic sort order; PostgreSQL row-value index support) — https://use-the-index-luke.com/sql/partial-results/fetch-next-page

### Primary — Meilisearch
- **[S5]** Meilisearch Docs: Tasks and asynchronous operations (async op list, statuses, auto-batching, task DB limits, safe restart behavior) — https://www.meilisearch.com/docs/learn/async/asynchronous_operations
- **[S6]** Meilisearch Docs: Working with tasks (`get_task`, `customMetadata`, streaming routes, error object) — https://www.meilisearch.com/docs/capabilities/indexing/tasks_and_batches/monitor_tasks.md
- **[S7]** Meilisearch Docs: Swap indexes API (atomic, swaps docs/PK/settings/history, `rename`, “enqueued tasks left unmodified”) — https://www.meilisearch.com/docs/reference/api/indexes/swap-indexes.md
- **[S8]** Meilisearch Docs: Add or replace documents (full overwrite/field removal, `primaryKey`, payload limit) — https://www.meilisearch.com/docs/reference/api/documents/add-or-replace-documents.md
- **[S9]** Meilisearch Docs: Get stats of index (`numberOfDocuments`, `isIndexing`, `fieldDistribution`) — https://www.meilisearch.com/docs/reference/api/indexes/get-stats-of-index.md
- **[S10]** Meilisearch Docs: Good practices (settings before data; full reindex on settings change; batching; language indexes; too many indexes) — https://www.meilisearch.com/docs/getting_started/good_practices.md
- **[S11]** Meilisearch Docs: Impact of RAM and multi-threading on indexing performance — https://www.meilisearch.com/docs/resources/self_hosting/performance/ram_multithreading.md
- **[S12]** Meilisearch Docs: Configuration reference (`--max-indexing-memory`, `--max-indexing-threads`, payload cap, batch caps, dump/snapshot `TMPDIR` staging, webhooks) — https://www.meilisearch.com/docs/resources/self_hosting/configuration/reference.md
- **[S13]** Meilisearch Docs: Backing up Meilisearch data (snapshots vs dumps) — https://www.meilisearch.com/docs/resources/self_hosting/data_backup/overview.md
- **[S14]** Meilisearch Docs: Migrating from PostgreSQL full-text search (PG as source of truth; OFFSET sample; server-side cursor/`COPY` note; sync options) — https://www.meilisearch.com/docs/resources/migration/postgresql_migration.md

### SDK / repository evidence
- **[S15]** `meilisearch-sdk` 0.33.0 crate source (local registry): `Client::swap_indexes`, `Index::add_or_replace`, `Index::delete_all_documents`, `Index::get_stats`, `TaskInfo::wait_for_completion` (default 5 s timeout) — `~/.cargo/registry/src/*/meilisearch-sdk-0.33.0/src/{client,indexes,task_info}.rs`
- **[F]** Flora: `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`, `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql`, `flora-api/src/state/app_state.rs`, `flora-api/src/routes/search.rs`, `Cargo.toml`
- **[P]** Prior Flora research: `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`, `…-021-optional-meilisearch-pg-coherence-vps.md`, `…-022-meilisearch-indexing-hooks-reindex-parity-vps.md`, `…-022-meilisearch-pg-coherence-degradation-contract-vps.md`

*End of research deliverable (cycle 023).*
