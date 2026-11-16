# 022 — Meilisearch/PG coherence, degradation contracts, and idempotent indexing for Flora (VPS 80/20, multilingual)

**Date:** 2026-11-16  
**Repo:** `flora` @ workspace state (research-only; this document only)  
**Requested output:** `/home/luis/development/MDMV/projetos/Arquivados/flora/docs/research/2026-11-16-022-meilisearch-pg-coherence-degradation-contract-vps.md`  
**Scope:** Operating **optional Meilisearch** alongside **PostgreSQL FTS** with coherence: precedence/degradation contracts, consistency/fallback behavior on indexing lag/outage/drift, sync hooks vs async workers vs outbox/transactional-outbox-lite (no Kafka mandatory), idempotent indexing/delete operations, rebuild-from-Postgres strategies, and cheap-VPS 80/20 defaults (~2 vCPU / 4 GB). Focus: **deterministic/testable** behavior for cycle 022 candidate goals (minimal task/file mutation wiring or explicit operational degradation/parity contract; deterministic PG fallback; rebuild/reindex/delete-on-soft-delete semantics).

**Method:** Read-only code/schema/config/spec/research cross-check in this repo + primary-source web fetches (Meilisearch docs, PostgreSQL docs, microservices.io). Multilingual lens applied for **EN, pt-BR, es, de, fr, zh-CN, ja** (quality/tokenization/ops implications); no locale-specific production benchmarks on target hardware were executed (**NV**). Web search tool was unavailable in-session due quota; primary documentation was fetched directly via `webfetch` (see [S1–S6]).

---

## 1. Flora baseline (cross-check)

| Area | Evidence (repo) | Current reality | 022 relevance |
|---|---|---|---|
| Search config/state | `flora-core/src/config/search.rs`, `flora-api/src/state/app_state.rs` | `SearchConfig` (`enabled,url,api_key,default_limit,max_limit,index_template`, default enabled/url template `flora_org_{org_id}`). `AppState` initializes Meilisearch only when enabled; on init error logs and continues with `search_service: None` (degraded, boot does not fail). Capabilities logged. | Good optionality base; **read path can be healthy while index is empty/stale** (no freshness signal). |
| Meilisearch adapter | `flora-search/src/lib.rs` | Per-org index `flora_org_{org_id}`. `setup_index` sets filterable `["type","created_at_timestamp"]`. `index_message/task/file` `add_documents(..., Some("id"))`; `delete_document`; `search` does `type IN [...]` filter and returns `{id,type,snippet}` from stored `content` (untrimmed). Task `content = title + description`; file `content = name` only (no `file_type`); message includes `channel_id,sender_id`. | Primitives exist, but **document shape/snippet semantics differ from PG fallback**; async task nature of Meilisearch not surfaced. |
| API search behavior | `flora-api/src/routes/search.rs` | `GET /api/v1/search?q=&type=&limit` (auth + org membership). Empty/whitespace `q` => `200 {query,results:[]}`. `limit` default 20 capped 50. `type` CSV normalized/lowercased; aliases `message/messages|task/tasks|file/files`. **If `search_service` is `Some`: delegate to Meilisearch and return hits as-is (no PG fallback, no merge). If `None`: PG FTS fallback** (`search_pg`) per-entity queries, strict `organization_id` + `is_deleted=false`, `websearch_to_tsquery/simple` fallback `plainto_to_tsquery`, `ts_rank_cd`, 200-char normalized snippets, merge-sort `(rank desc, created_at desc, id desc)`, truncate to limit. | **Explicit precedence today:** Meilisearch-preferred when configured/initialized, else PG. But “configured” != “fresh/complete”; this creates **silent staleness risk** unless contracted/tested. |
| PG FTS schema | `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql` | Generated/stored `search_tsv` (`to_tsvector('simple', ...)`) + partial GIN `WHERE is_deleted=false` for `messages(content)`, `tasks(title||description)`, `files(name||file_type)`. | **Transactional, DB-maintained correctness source** for indexed columns; best baseline for deterministic parity/rebuild. |
| Mutation/index wiring | `flora-api/src/routes/tasks.rs`, `channels.rs`, `files.rs`; `flora-core/src/services/*`; repo-wide `rg` | No observed calls to `index_message/task/file`, `setup_index`, or `delete_document` in app mutation paths. Task/file/message soft-deletes exist (`is_deleted`), task PATCH supports tri-state/unassign semantics (021). Notification creation is best-effort/in-request in routes/services (not outbox-backed). | **Core gap for 022:** Meilisearch can drift/empty on any mutation, soft-delete, unassign, or update; PG fallback is only used when service absent. |
| Prior decisions/research | `specs/021-.../spec.md`, `docs/research/2026-11-16-021-meilisearch-task-file-search-parity-vps.md`, `018-postgres-search-outbox-vps.md`, `016-search-postgres-fts-vs-meilisearch.md`, `020-pg-fts-tasks-files-vps.md` | 021 preserved precedence (Meilisearch if present else PG), hardened PG fallback contracts, and **deferred Meilisearch task/file mutation-indexing parity/tests** (NV freshness/completeness). 018 proposed outbox-lite + SKIP LOCKED; 016/020 recommended PG-first default on 2vCPU/4GB. | 022 must either implement **minimal verifiable wiring+degradation contract** or make **operational contract explicit and testable** without overcommitting infra. |
| Runtime constraints | `.env.example`, prior VPS audits | 2 vCPU / 4 GB / SSD, single-node, Postgres-first, optional Meilisearch/Valkey profiles. | Prefer **no Kafka**, bounded memory/workers, rebuildable caches, deterministic failure modes. |

**Determinism gaps observed:** (1) backend choice depends on startup-time init, not per-request health/freshness; (2) no mutation hooks => Meilisearch completeness undefined; (3) Meilisearch indexing is async-task based in engine but adapter ignores task outcomes; (4) snippets/field coverage/order differ across engines; (5) no idempotency/dedupe contract for index side effects; (6) no rebuild/reindex command or operational SLO.

---

## 2. Primary-source findings (high-trust)

### 2.1 Meilisearch: async indexing, tasks, and operational implications [S1, S2]

- **Documents are stored in indexes and identified by primary key.** Adding/replacing documents requires a valid primary key strategy; document structure/fields drive search/display behavior. [S1]
- **Write/index operations are asynchronous and tracked as tasks** (`enqueued` → `processing` → `succeeded|failed|canceled`). Task objects expose `status`, `type`, `details` (e.g. received/indexed/deleted counts), `error`, timings. Many operations (document addition/update/deletion, settings, dumps/snapshots, etc.) return summarized task info and complete later. [S2]
- **Task queue behavior:** processed in order with batching/auto-batching under compatibility conditions; high-priority task types exist; enqueued/processing tasks differ from finished tasks; queue/storage has limits and operational maintenance considerations (task retention/deletion). Instance termination during processing is safe (processing tasks reset to enqueued on restart). [S2]
- **Implication for Flora:** `add_documents`/`delete_document` success at SDK call time does **not** imply searchable visibility. Coherence requires (a) treating Meilisearch as **eventually consistent relative to commit time**, (b) optionally tracking/waiting on task completion for stronger guarantees, or (c) preferring PG for correctness-critical reads. Idempotent upsert-by-primary-key is natural for creates/updates; deletes must be explicit on soft-delete.

### 2.2 PostgreSQL: transactional correctness, FTS primitives, and queue claiming [S3]

- `SELECT ... FOR UPDATE ... SKIP LOCKED` is supported in locking clauses, enabling **non-blocking claim/dispatch** of queued rows by concurrent workers (each worker locks only rows it can claim). [S3]
- Combined with prior Flora research/PG docs already in repo [F6,F7]: generated/stored `tsvector` + GIN gives **in-transaction index maintenance** for source columns; `websearch_to_tsquery`/`plainto_tsquery` are safer for user input; partial indexes on `is_deleted=false` reduce bloat and align semantics. PG is the single source of truth for committed domain state.

### 2.3 Transactional outbox pattern (conceptual, infra-light) [S4]

- Problem: atomically update DB state and publish side effects without distributed 2PC. [S4]
- Solution: write business change + outbox record in the **same DB transaction**; a relay publishes later from outbox. [S4]
- Consequences: guarantees “send if and only if commit” (for recorded intent), preserves ordering intent if designed so, but relay delivery can be **at-least-once** => **consumers/side effects must be idempotent**; requires outbox table/relay logic. [S4]
- For Flora on cheap VPS: outbox can be a **Postgres table + in-process poller** (no Kafka/Debezium required), using `SKIP LOCKED` for claims [S3,S4].

### 2.4 Rebuild/snapshot operational note [S5, S6 - fetch attempts]

- Direct fetches to `snapshots_vs_dumps` and `language support` pages returned 404 at attempted URLs in-session (**NV for exact current doc text/URL**). Operationally, Meilisearch provides backup/transfer mechanisms (dumps/snapshots in general product docs) and multilingual/tokenization behavior varies by language/data; for 022, treat **full rebuild-from-Postgres as the authoritative recovery path** and validate language quality per corpus. [S5,S6: NV]

---

## 3. Multilingual considerations (EN, pt-BR, es, de, fr, zh-CN, ja)

| Locale/script | PG FTS (`simple` + current schema) | Meilisearch (optional) | Coherence/degradation guidance |
|---|---|---|---|
| **EN** | Good baseline for keyword/stem-light search with `simple`; deterministic with generated `tsvector`. | Generally strong relevance/tokenization for common cases. | Safe to keep PG authoritative fallback; parity differences mostly ranking/snippets/freshness. |
| **pt-BR, es, fr, de** | `simple` avoids config drift but misses language stemming/stopwords/diacritics tuning; `unaccent`/language configs are possible follow-ups (**NV perms/quality**). | Often better UX (typo tolerance/normalization) depending on data; still engine/version-dependent. | Document **“ranking/snippet may differ by backend, correctness by tenant+soft-delete must not”**; validate with representative corpora before parity claims. |
| **zh-CN, ja** | Default tokenization is weak for CJK/JP boundaries; trigrams may help some substring cases but not a full solution (**NV**). | Tokenization quality/data-dependent; may outperform PG for UX but not guaranteed equivalent to PG fallback semantics. | Treat as **highest parity risk**: require explicit NV/validation, avoid promising equal recall; deterministic contracts should focus on visibility/invariants, not identical ranking. |
| **All** | Transactional, locale-independent storage semantics. | Async indexing => eventual visibility. | Expose capability/freshness state; ensure empty/lagging Meilisearch does not silently hide PG-correct results unless explicitly chosen. |

**Deterministic/testable cross-locale contract recommendation:** assert **tenant isolation, soft-delete exclusion, CRUD visibility after defined sync point, type filtering, and empty-query behavior**; treat **ranking order/snippet text as backend-specific unless normalized** (and snapshot tests only within same backend/mode).

---

## 4. Options and tradeoffs (sync hooks vs async workers vs outbox-lite)

### Option A — Synchronous inline hooks in mutation request path (no new table)

**How:** after successful DB mutation (and commit boundary as feasible), call `index_*`/`delete_document` and optionally wait for Meilisearch task completion.

| Pros | Cons |
|---|---|
| Smallest code delta; easy to reason about for low volume. | Couples request latency/availability to Meilisearch; failures on Meilisearch can break UX unless swallowed (then drift). Waiting on async tasks increases tail latency/timeouts. Harder to retry durably across process restarts. |
| Immediate best-effort freshness when healthy. | Not “commit-coupled” to side effect delivery unless carefully engineered; at-least-once/duplicate calls possible on retries. |

**VPS 80/20 verdict:** acceptable only as **best-effort fire-and-forget with strict degradation contract** (never block core writes on search side effects) — not sufficient alone for coherence guarantees.

### Option B — In-process async worker (channel/queue in memory) after commit

**How:** enqueue indexing jobs in-process once DB commit succeeds; background tokio task processes sequentially/bounded.

| Pros | Cons |
|---|---|
| Decouples user latency from Meilisearch; simple retry/backoff in memory; bounded concurrency fits 2 vCPU. | **Not durable** across crashes/restarts/deploys; jobs lost => drift until rebuild. Ordering/dedup must be implemented in-process. |
| No schema changes. | Harder to observe/replay deterministically in ops. |

**VPS 80/20 verdict:** better than inline blocking, but **durability gap** conflicts with rebuild-light operability; use only as stepping stone.

### Option C — Transactional outbox-lite in Postgres + single poller (recommended 80/20, no Kafka)

**How:** in same DB transaction as domain mutation/soft-delete/notification-intent, insert outbox row describing search side effect (`upsert`/`delete`, entity type/id/org, minimal payload or version/ref). Poller claims rows with `FOR UPDATE SKIP LOCKED`, applies idempotent Meilisearch operations, marks done/retries/dead. [S3,S4]

| Pros | Cons |
|---|---|
| **Atomic intent with commit** (no lost intent if tx commits); durable queue; replayable; observable lag/dead letters. | Requires migration + worker code + idempotency/dedupe design. |
| Fits single-node VPS, no extra infra; bounded batch/concurrency. | Relay is at-least-once => side effects must be idempotent. |
| Enables deterministic tests (run worker/drain outbox) and clean rebuild story. | Small write amplification/storage for outbox. |

**VPS 80/20 verdict:** **best coherence/complexity tradeoff** for 022+ when wiring mutations; can start minimal (search events only, single in-process poller).

### Comparison summary

| Dimension | A Sync inline | B Async in-proc | C Outbox-lite (PG) |
|---|---|---|---|
| Commit-to-intent atomicity | weak unless carefully staged | no (post-commit enqueue volatile) | **yes** [S4] |
| Durability across restart | no | no | **yes** |
| Request latency impact | high if awaited | low | low |
| Ops complexity | low | low-med | med (but Postgres-native) |
| Deterministic testability | medium | medium-low | **high** (drain queue deterministically) |
| Cheap-VPS fit | risky | ok | **good** with caps |

---

## 5. Coherence, precedence, and degradation contracts (make deterministic)

### 5.1 Recommended contract states

Define **effective search backend mode** as an explicit, observable contract (config + runtime health/capability), not an implicit side effect:

1. **`pg` (authoritative fallback/default):** always serve from PG FTS. Use for Seed/lean VPS and for deterministic correctness.
2. **`meili_preferred_pg_fallback` (recommended when Meilisearch enabled but parity/freshness not guaranteed):** attempt Meilisearch only if healthy/fresh **and** entity/indexing coverage is declared complete; otherwise **fall back to PG per request** (or per entity type). *Current code does not do per-request fallback; 022 must choose explicitly.*
3. **`meili_only_when_ready` (strict):** serve Meilisearch for covered entities only after readiness checks pass (index exists, no critical backlog, recent successful sync marker) else return **degraded/unavailable or PG fallback by policy** — never silently serve stale-empty as authoritative.
4. **`dual_read_compare` (test/ops diagnostic, non-prod default):** query PG and Meilisearch, compare counts/ids for sampled queries to detect drift (**NV perf cost**).

### 5.2 Deterministic precedence rules (proposed)

- **Tenant isolation and soft-delete correctness are invariant across all modes** (must match PG semantics: `organization_id` scoped, `is_deleted=false` excluded).
- **Empty/whitespace query behavior must be identical** across modes (`200`, empty results) — already true in API shape, keep contract-tested.
- **Type normalization/filtering must be identical** (accepted aliases/tokens, unknown-token policy) — normalize once in API layer before backend dispatch.
- **Freshness:** Meilisearch results are **eventually consistent** [S2]. Contract must state: *“mutations are visible in Meilisearch after indexing task success”* (if tracked) or *“within best-effort window; PG remains source of truth”*.
- **Outage/lag/drift handling:**
  - Init failure: current behavior OK (`search_service=None` -> PG).
  - Runtime request failure: **do not 5xx the whole search for recoverable backend errors**; fall back to PG when policy allows and record metric/log.
  - Detected drift/lag (backlog, task failures, rebuild needed): mark capability degraded; prefer PG until recovered.
- **Response normalization for parity:** if Meilisearch remains enabled, normalize snippet truncation/whitespace and ensure file content includes `name + file_type` equivalent to PG; consider API-level snippet builder for cross-backend determinism.

### 5.3 What must be explicit in 022 (decision/contract)

Because mutation hooks are absent today, 022 should explicitly choose one:
- **Wiring path:** implement minimal task/file (+message consistency) hooks/delete-on-soft-delete with idempotent ops and deterministic degraded fallback to PG on Meilisearch failure/lag signal; or
- **Operational contract path:** keep Meilisearch non-authoritative for task/file (or all) until hooks exist, document “PG authoritative for parity”, and add tests/capability signaling so behavior is deterministic and not silently stale.

Given `OBJETIVO/ROADMAP/spec 021`, **PG fallback must remain the deterministic parity baseline for Seed**.

---

## 6. Idempotent indexing and delete operations

### 6.1 Principles

- **Upsert by stable primary key (`id`)**: safe to replay for create/update/reassign/rename/content edits. Meilisearch `add_documents` with primary key is effectively replace/upsert semantics for same id [S1,S2].
- **Delete is idempotent at intent level**: deleting a non-existent doc should be treated as success/no-op in app logic (handle 404/not-found responses gracefully).
- **Soft-delete => index delete:** for `is_deleted=true`, search visibility must match PG partial-index behavior (exclude). Do **not** rely on Meilisearch-side `is_deleted` filtering unless schema/filterable attrs and every query path enforce it consistently.
- **Monotonic/version guard (recommended):** include `updated_at`/version/hash in payload or outbox; worker skips applying older versions to avoid out-of-order overwrite (important if async/retries).
- **Deterministic document builder:** single function per entity to build Meilisearch doc from DB row (id/type/content/snippet source/tenant metadata fields as needed) to prevent schema drift.

### 6.2 Concrete operation mapping

| Domain event | PG FTS effect | Meilisearch side effect (idempotent target) |
|---|---|---|
| Message/task/file create | generated `tsvector` updated in tx | upsert doc by `id` |
| Update content/title/desc/name/file_type/status/assignee | regenerated in tx | upsert doc by `id` (full current projection) |
| Task unassign/assign transition | persisted assignee changes | upsert doc by `id` (reflect latest state) |
| Soft-delete / delete | row filtered by `is_deleted` (or removed) | `delete_document(org_id,id)` |
| Restore/undelete (if any) | becomes visible | upsert doc by `id` |

### 6.3 Outbox event design (outbox-lite)

Minimal event envelope: `event_id`, `org_id`, `entity_type`, `entity_id`, `op` (`upsert|delete`), `entity_version/updated_at`, `payload_ref_or_inline`, `attempts`, `next_attempt_at`, `status`, `last_error`. Unique/dedupe options:
- coalesce by `(org_id,entity_type,entity_id)` to latest pending op, or
- keep append-only and apply “latest version wins” in consumer (simpler audit, needs version check).

---

## 7. Rebuild-from-Postgres strategies

**Principle:** Meilisearch index is a **derived, rebuildable projection**; PostgreSQL is the source of truth.

1. **Full rebuild per org/entity (recommended baseline):**
   - Ensure/setup index/settings for `org_id`.
   - Stream/select active rows (`WHERE organization_id=? AND is_deleted=false`) in deterministic order (e.g. `created_at,id`).
   - Batch upsert documents; optionally delete index + recreate for clean slate (if acceptable downtime/drift window) or overwrite in place.
   - Run as admin CLI/maintenance endpoint/job (not in request path); throttle batch size for 4 GB RAM.
2. **Incremental catch-up (if outbox/checkpoint used):**
   - Track last applied `updated_at/id` or outbox sequence; replay pending/outstanding events after outage.
3. **Reconciliation:**
   - Periodic sampled compare (PG ids vs Meilisearch hits for fixed queries) to detect drift; trigger rebuild on threshold.
4. **Soft-delete correctness pass:**
   - Rebuild from active rows only, or scan for indexed docs whose DB rows are deleted/missing and delete them (needs listing strategy; **NV** for cheap full-diff at scale).

**80/20 recommendation:** provide **manual/triggerable full rebuild-from-Postgres** (per org or global) + **delete-on-soft-delete in steady state**; defer continuous reconciliation automation.

---

## 8. Cheap-VPS 80/20 defaults (2 vCPU / 4 GB)

- **Default profile:** `search backend = pg` (or `meili disabled`) as authoritative, deterministic, zero extra daemon.
- **If Meilisearch enabled:** single instance, explicit memory/CPU limits, localhost/private binding, small/necessary indexes only, no heavy per-request settings churn. Keep worker concurrency = 1, small batches (e.g. 10–50), adaptive poll sleep.
- **Never block domain commits on Meilisearch.** Use fail-open side effects + PG fallback.
- **Deterministic API behavior over “smart” ranking:** normalize types/snippets and document backend-specific ranking.
- **Observability minimum:** capability flag (`configured`, `initialized`, `healthy`, `lag/backlog` if known), search backend used per request (debug/metrics), outbox depth/oldest age/dead count, Meilisearch task failure count.
- **Ops:** DB backups include search source data and (if used) outbox; Meilisearch treated as disposable/cache with rebuild runbook.
- **Avoid:** dual-write without idempotency, synchronous task waits in hot paths, Kafka/extra brokers, unbounded polling, cross-backend merged results without dedupe/consistent scoring in MVP.

---

## 9. Minimal verifiable 022 recommendations

Prioritized, testable slices consistent with `OBJETIVO.md` / `ROADMAP.md` / 021 constraints:

### R1 (P0, required): Make precedence and degradation **explicit and deterministic**
- **Code/contract:** define and enforce a single policy, e.g. **`PG-authoritative fallback for correctness`**: when Meilisearch is enabled but not proven fresh/covered for an entity, serve PG for that scope (or globally) OR expose/guarantee fallback on Meilisearch query/indexing failure. Avoid current “Meilisearch if Some => always trust” without freshness guarantees.
- **Normalization:** centralize `type` parsing/aliases and snippet normalization rules so both backends satisfy same contract invariants.
- **Verification:** DB-gated/contract tests for: empty query, type aliases, limit cap, tenant isolation, `is_deleted` exclusion, and **deterministic fallback behavior** when `search_service` unavailable (and, if implemented, when Meilisearch errors).

### R2 (P0): Idempotent mutation hooks for task/file/message with delete-on-soft-delete (minimal wiring)
- **Implement minimal hooks** at service/route transaction-commit boundaries (or immediately after successful mutation) for create/update/soft-delete in `tasks/files/messages`:
  - upsert index doc on create/update/unassign/reassign/content change,
  - delete index doc on soft-delete/delete.
- **Idempotency:** upsert by `id`, ignore not-found delete, deterministic doc builder (include `file_type` in file content to match PG fields).
- **Failure mode:** **fail-open** (log + metric) so core mutations never fail due to Meilisearch; PG remains authoritative.
- **Verification:** integration/contract tests in a mode with injectable/fake Meilisearch adapter or feature-gated test double asserting upsert/delete calls and payload invariants; DB-gated PG tests remain green.

### R3 (P1, recommended): Outbox-lite for durability (if “no drift on restart” required)
- Add `search_outbox` (or generic outbox) + in-process poller with `FOR UPDATE SKIP LOCKED`, bounded retries/backoff, dead-letter state, and idempotent consumer. [S3,S4]
- Keep optional for first wiring PR if time-constrained, but **document NV: in-request hooks are not durable across crashes**.

### R4 (P1): Rebuild-from-Postgres runbook/tooling
- Provide deterministic rebuild path (CLI/admin task) per org/all orgs from active DB rows; include `setup_index` and batched upserts.
- **Verification:** test rebuild produces searchable docs for existing non-deleted entities in Meilisearch mode; PG results unchanged.

### R5 (P1): Observability and health semantics
- Expose backend/capability state and (if outbox) lag metrics; log Meilisearch task submission/result where cheap; health should distinguish “configured but degraded/stale”.
- **Verification:** unit/integration tests for capability reporting and fallback selection logic.

### Suggested implementation order (80/20)
1. **R1 contract + tests** (prevents silent incoherence immediately).  
2. **R2 minimal idempotent hooks + delete-on-soft-delete + fail-open + doc parity fixes**.  
3. **R4 rebuild command** (operational recovery).  
4. **R3 outbox-lite** if durability/replay becomes required.  
5. **R5 metrics/health polish**.

### Explicit non-goals for 022 (keep NV/deferred)
- Kafka/Debezium/external queue infra, distributed exactly-once delivery, cross-engine ranking/snippet equality, multilingual stemming/CJK perfection, `pg_trgm`/hybrid scoring, ACL-aware global search beyond current org boundary, load/OOM benchmarks on target VPS, frontend search UX changes.

---

## 10. Risks and open verification items (NV)

| Risk | Why it matters | Mitigation / NV |
|---|---|---|
| Silent staleness when `search_service` is present but index lags/empty | Wrong UX/data completeness while API returns 200 | Prefer PG fallback or readiness gating; freshness metrics; tests for fallback contract |
| Meilisearch async task failures ignored by adapter | Upsert/delete may never apply [S2] | Capture/inspect task outcomes in worker/outbox flow; dead-letter + alert |
| Out-of-order async application | Older write can overwrite newer state | Version/`updated_at` checks, coalesced latest-wins events |
| Snippet/schema drift (file `file_type`, trimming) | Perceived inconsistency across modes | Deterministic doc builder + API normalization; contract tests |
| VPS memory/IO pressure with extra services + indexing | Instability on 4 GB | PG-default, caps, bounded batches, optional profile only |
| Multilingual relevance mismatch (esp. zh-CN/ja) | False parity assumptions | Mark NV; corpus validation; avoid ranking-equality guarantees |
| Rebuild duration/consistency window | Temporary incompleteness | Rebuild throttling, maintenance window docs, PG fallback during rebuild |
| Dedup/race in outbox-less hooks | Duplicate side-effect calls | Idempotent ops by `id`; move to outbox-lite when needed (NV) |
| Extension/tooling/SDK behavior differences | Migration/runtime surprises | Validate in CI with disposable PG and optional Meilisearch test profile (NV) |

---

## 11. Sources and citations

### Primary / high-trust (fetched)

- **[S1] Meilisearch Docs — Documents:** https://www.meilisearch.com/docs/learn/getting_started/documents (fetched 2026-11-16). Structure, fields, primary key, upload/indexing basics.
- **[S2] Meilisearch Docs — Tasks and asynchronous operations:** https://www.meilisearch.com/docs/learn/async/asynchronous_operations (fetched 2026-11-16). Async operations, task statuses/objects, queue/batching, cancel/delete behavior, eventual completion semantics.
- **[S3] PostgreSQL 18 Documentation — `SELECT`:** https://www.postgresql.org/docs/current/sql-select.html (fetched 2026-11-16). Locking clause includes `FOR UPDATE|... [NOWAIT|SKIP LOCKED]` for concurrent claim patterns.
- **[S4] Chris Richardson — Pattern: Transactional outbox:** https://microservices.io/patterns/data/transactional-outbox.html (fetched 2026-11-16). Atomic DB update + message intent, relay, ordering/at-least-once/idempotency tradeoffs.

### Primary-source fetches attempted (not available at URL; NV)

- **[S5]** https://www.meilisearch.com/docs/learn/update_and_migration/snapshots_vs_dumps — 404 on fetch attempt (2026-11-16). NV: use product docs index to confirm current backup/rebuild mechanism names.
- **[S6]** https://www.meilisearch.com/docs/learn/relevancy/language_support — 404 on fetch attempt (2026-11-16). NV: validate current language/tokenization support per target locales/corpora.

### Repo evidence (Flora cross-check)

- **[F1]** `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`
- **[F2]** `flora-api/src/state/app_state.rs`, `flora-api/src/routes/search.rs`
- **[F3]** `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `flora-core/src/migrations/sql/20261118000001_add_tasks_files_search_fts.sql`
- **[F4]** `flora-api/src/routes/tasks.rs`, `flora-api/src/routes/files.rs`, `flora-api/src/routes/channels.rs` (no Meilisearch mutation hooks observed)
- **[F5]** `specs/021-unassign-notification-dedup-search-hardening/spec.md`
- **[F6]** `docs/research/2026-11-16-018-postgres-search-outbox-vps.md`
- **[F7]** `docs/research/2026-11-16-016-search-postgres-fts-vs-meilisearch.md`
- **[F8]** `docs/research/2026-11-16-021-meilisearch-task-file-search-parity-vps.md`
- **[F9]** `OBJETIVO.md`, `ROADMAP.md`

### Research limitations

- `websearch` tool returned quota/unavailable in this session; research relied on direct `webfetch` of primary docs and in-repo sources. Additional multilingual primary articles (pt-BR/es/de/fr/zh-CN/ja publisher pages) were not fetched live; multilingual analysis is therefore grounded in engine/DB behavior and marked NV where corpus-specific evidence is required.
- No runtime benchmarks, no code changes, no external Meilisearch/Postgres instances exercised for this report.

---

**Bottom line (022, 80/20):** keep **PostgreSQL FTS as the deterministic correctness baseline**, make **precedence/fallback behavior explicit and testable**, implement **minimal idempotent task/file/message indexing hooks with delete-on-soft-delete and fail-open semantics** (or formally defer with non-stale contract), and add **rebuild-from-Postgres**. Use **transactional outbox-lite (`SKIP LOCKED`)** only when durable replay across restarts becomes a requirement; avoid Kafka and avoid trusting Meilisearch freshness without a defined readiness/lag contract. [S1–S4, F1–F9]
