# Seed v0.1 "Search" vs Meilisearch — decision-ready reconciliation

**Date:** 2026-11-16
**Repo:** `flora` — research-only; this document is the only change. No source code touched
(in particular `flora-api/src/routes/search.rs`, which carries uncommitted 023 work).
**Branch context:** `feat/023-postgres-authoritative-search` @ `d485fa634`; 023 working tree
implements the Postgres-authoritative contract but is **in progress / uncommitted** at time of
writing. The recommendation below is valid whether or not 023 lands as one commit.
**Question answered:** For Seed v0.1, what must "Search" mean, is Meilisearch required, and how
should the milestone/decision record be worded so code and docs stop contradicting each other?

---

## 0. Executive decision (answers a–d)

| # | Question | Verdict |
|---|---|---|
| **(a)** | What "Search" must exist for a credible Seed v0.1 | An authenticated, tenant-scoped, multi-entity **keyword search** over `messages`/`tasks`/`files` with a stable contract, bounded results, soft-delete exclusion, snippets, and **transactional freshness** — served by one authoritative backend. Postgres FTS already satisfies this in-tree. Typo tolerance, facets, search-as-you-type, CJK segmentation, InstantSearch, and cross-engine ranking parity are **not** Seed requirements. (§2) |
| **(b)** | Is Meilisearch required / optional / deferrable on 2 vCPU / 4 GB? | **Deferrable and dormant.** Not required for (a); the current path is unwritten (zero mutation hooks), asynchronously visible (enqueue ≠ indexed), and would be a second datastore competing with Postgres on a box already budgeted to ~3 GB. It is *optional* as a non-authoritative experiment only. The vendor's own page concedes PG FTS may be enough for precisely Flora's shape. (§3) |
| **(c)** | Re-enablement gates if Meilisearch becomes authoritative later | Four hard gates: **(G1)** mutation-indexing hooks + delete-on-soft-delete + one shared document mapper; **(G2)** task-await wrapper with explicit `is_success()`; **(G3)** deterministic PG-sourced rebuild/backfill with verified atomic swap; **(G4)** separate advisory CI job with task-status waits. Plus preconditions: resource headroom, production security/backup runbook, drift/capability signals. (§4) |
| **(d)** | How to word the milestone/decision record | Make **backend explicit** everywhere: "Search (Postgres FTS; Meilisearch deferred to Grove and gated)". Replace FASE 5 "Search + Meilisearch" with "Search (Postgres FTS) + gated Meilisearch re-enablement". Add one ADR-style decision record with re-open criteria. Never write bare "Search (Meilisearch)" next to "Seed = Flora + PostgreSQL". (§5) |

**One-line recommendation:** keep cycle 023's Postgres-authoritative contract, treat Meilisearch as a
dormant Grove-tier capability with written re-enablement gates, and fix the documentation by making
the backend name part of the milestone text rather than implying it.

---

## 1. The contradiction (evidence, not opinion)

Four sources disagree about what "Search" means at Seed:

| Source | Says | Conflict |
|---|---|---|
| `README.md:16,30,40` [F1] | Seed = "Flora + PostgreSQL"; milestone item "**Search (Meilisearch)**"; FASE 5 "**Search + Meilisearch**" | Same list puts Meilisearch in a tier the constitution reserves for Grove. |
| `.specify/memory/constitution.md:40-43` [F2] | Seed = Flora + PostgreSQL; Meilisearch appears only at **Grove** and Forest | Normative tier definition excludes Meilisearch from Seed. |
| `.specify/memory/constitution.md:85,134,142` [F2] | Header "global search (Meilisearch)"; Seed v0.1 list includes "Meilisearch"; "Phase 5: Search + Meilisearch" | The constitution contradicts its own deployment-tier table (a versioning/editorial defect). |
| `OBJETIVO.md:13,84-86`, `ROADMAP.md:57` [F3] | Postgres-first/optional components; "Meilisearch really optional by profile"; cycles 019–022 progressively made PG the fallback and deferred Meilisearch parity | Strategy docs already point away from a Meilisearch-backed Seed. |
| `specs/023-*/spec.md:9-16`, working tree [F4] | "make Postgres FTS the sole authoritative backend"; `source: "postgres"`; Meilisearch "dormant, non-authoritative"; `SearchConfig::default().enabled = false` | **Code/docs intent is now unambiguous**, while README/constitution still advertise Meilisearch. |

The root cause is that "Search" was recorded as a **feature name** ("Search (Meilisearch)") instead
of a **capability + backend contract**. The fix in §5 is therefore semantic, not cosmetic: state the
invariants once, and name the backend explicitly in every tier/milestone list.

Note on status: the 023 working tree already contains the authoritative-PG route, the additive
`source` field, the `enabled = false` default, the separator-normalization migration, a
capability note in `docs/dev-smoke.md`, and two new DB-gated contracts [F4]–[F6]. Those are
**uncommitted** as of this research; the wording proposal must not assume they are merged to
`main`. It also should not wait for them — the docs can be corrected against the intended 023
contract immediately.

---

## 2. (a) The minimum credible "Search" capability for Seed v0.1

Derived from the shipping contract in `flora-api/src/routes/search.rs` [F4] and the 019–023
specs [F7]. This is the definition of done Seed v0.1 must be able to demonstrate:

| # | Capability / invariant | Evidence in current code | Why it is non-negotiable |
|---|---|---|---|
| C1 | `GET /api/v1/search?q=&type=&limit=` exists and returns `{query, results[{type,id,snippet}], source}` | `search()` and `SearchResponse` [F4] | The public contract other slices/UI depend on. |
| C2 | Keyword match across **three** entity types (`message`, `task`, `file`), with `type` CSV aliases (singular/plural) and unknown-only → empty | `include_type()` + three PG branches [F4]; spec 020 [F7] | "Global search" means all modules; partial coverage is not Seed-complete. |
| C3 | Tenant scoping: `auth.require_org()` + membership lookup, `organization_id = $1` in every branch | `search()` lines 66-75, SQL predicates [F4] | A cross-org leak is a security defect, not a relevance issue. |
| C4 | Soft-delete exclusion + partial GIN indexes aligned with visibility | `is_deleted = false` + `WHERE is_deleted = false` [F4], migrations [F5] | Deleted content must never be discoverable. |
| C5 | Bounded, deterministic results: `limit` default 20, cap 50; `ts_rank_cd` → `created_at` → `id` tiebreak | lines 86, 282-288 [F4] | Stable pagination/tests; protects the 4 GB box from unbounded queries. |
| C6 | Snippet + empty-query behavior | 200-char whitespace-normalized `left(regexp_replace(...))`; empty query returns 200 before any backend [F4] | Contract completeness. |
| C7 | **Freshness after mutation with no sync pipeline** | generated/stored `search_tsv` updated in the same transaction; partial GIN; 023 separator normalization [F5] | This is what makes one backend *authoritative*. A sidecar that can be stale cannot be. |
| C8 | **Explicit backend/capability contract** | `source: "postgres"`; enabling `FLORA_SEARCH__ENABLED` does not change results [F4], [F6] | Kills the "configured ⇒ authoritative" defect; testable. |

**Explicitly out of scope for a credible Seed** (document as NV, not as missing scope): typo
tolerance, faceted counts, search-as-you-type (<50 ms), BM25/ranking-rule tuning, InstantSearch
adapters, CJK/JP segmentation, per-language indexes, hybrid/vector search, cross-engine ranking or
snippet parity, multi-node HA. None appears in the current API contract [F4], and the vendor's own
comparison lists exactly these as the reasons to consider a dedicated engine [M-VS-PG].

**Determinism principle to assert in tests:** test the *invariants* C1–C8, never "Meilisearch and PG
return the same order". Ranking text and snippet text are backend-specific; promising equality
creates false parity claims (§4, [R3], [R4]).

---

## 3. (b) Meilisearch: required, optional, or deferrable on 2 vCPU / 4 GB?

### 3.1 Determination

| Lens | Finding | Weight |
|---|---|---|
| Tier definition | Constitution: Seed = Flora + PostgreSQL; Meilisearch enters at Grove [F2] | **Normative** |
| Debloat principle | "Must run flawlessly on a cheap VPS"; "zero bloat, zero vendor lock-in" [F2] | **Normative** |
| Current wiring | Zero calls to `index_message/index_task/index_file/delete_document/setup_index` from any mutation route or service [F4], [R1] | **Observed** |
| Async visibility | `add_documents` returns a `taskUid` with status `enqueued`; the document is searchable only at `succeeded`; settings and document tasks cannot batch [M-ASYNC], [M-TASKS], [R2] | **Vendor primary** |
| Adapter behavior | All five mutators discard the task handle (`let _task = ...`); `search()` executes with no visibility barrier [F4] | **Observed** |
| Latent correctness bug | The Meili doc `type` field is singular (`message`/`task`/`file`) while the route passes the raw lowercased CSV tokens including plural aliases; a `?type=messages` filter matches nothing. PG branch normalizes aliases [F4], [R1] | **Observed** |
| Vendor concession | "When PostgreSQL FTS might be enough: small dataset (thousands of documents); search isn't user-facing or real-time; basic keyword matching sufficient; can't add infrastructure" — verified verbatim on 2026-11-16 [M-VS-PG] | **Vendor primary** (interest not neutral; usable as a concession) |
| Resource envelope | Indexer defaults to ≤2/3 of RAM and ≤1/2 of cores; OOM during indexing is a documented failure mode; `--max-indexing-memory`/`--max-indexing-threads` exist to restrain it [M-RAM] — verified verbatim | **Vendor primary** |
| Flora budget (planning, NV) | OS ~512 MB + Postgres 1 GB + Meilisearch 768 MB + Valkey 384 MB + API 256–512 MB ≈ **2.9–3.2 GB of 4 GB before indexing spikes**; compose caps are hints, not guarantees [F8], [R1] | **NV estimate** |
| Upgrade/security/backup | compose pins `getmeili/meilisearch:v1.12` with `MEILI_ENV=development`, a dev master key, localhost bind, no snapshot/dump schedule; production checklist requires ≥16-byte key, production env, HTTPS, search key, tenant tokens [F8], [M-SEC], [M-BACKUP], [R1] | **Observed + vendor primary** |
| CJK edge | Meilisearch Charabia segments zh-CN (jieba), ja (lindera IPA), handles German compounds, Cyrillic/Bulgarian normalization; PG `simple` has no CJK segmentation [M-LANG], [R1] | **Genuine advantage — gated, not assumed** |

**Verdict:** For Seed v0.1, Meilisearch is **deferrable and non-authoritative**. It is *optional*
only in the narrow sense that the crate and compose profile remain available for experiments; it is
**not part of the supported Seed surface**. Making it authoritative would require the full sync
pipeline of §4 — an effort whose only evidenced triggers (CJK corpus, typo tolerance as a stated
requirement, measured latency breach, faceting in the contract) are absent today [R1], [R5].

### 3.2 Why "just enable it" is the dangerous option

The 022 fail-open change made query *errors* fall back to PG [F7]. The unresolved defect is
**success-with-stale**: with no hooks, `add_documents` succeeding means "accepted into a queue",
while the index can be empty forever and the API still returns `200` with Meilisearch hits [M-ASYNC],
[R2]. The 023 decision removes that class of bug by construction rather than by monitoring:
`search_pg()` is always executed and the response declares `source: "postgres"` [F4]. This is the
single strongest argument that the current direction is correct — it converts a silent correctness
risk into an explicit capability boundary.

### 3.3 When PG FTS is weaker (honest counterweight)

- No typo tolerance in core (`pg_trgm` is a partial mitigation, best on short fields) [PG-TRGM], [R1].
- No native CJK/JP word segmentation; needs `zhparser`/`pg_jieba` or app-side jieba [M-LANG], [R1].
- Accent-insensitive search needs `unaccent()`, which is `STABLE`, not `IMMUTABLE` — generated columns
  and expression indexes require an immutable wrapper; do not silently edit the existing generated
  expressions [PG-UNACCENT], [R1]. This is a separate tested slice, not a 023 hotfix.
- If these become real requirements, §4 gates plus a larger VPS are the answer — not removing the
  capability contract.

---

## 4. (c) Concrete Meilisearch re-enablement gates

Design is already specified across `docs/research/2026-11-16-023-*.md` [R1]–[R6]. These are the
gates to write into the decision record; **all four are mandatory before Meilisearch may serve
authoritative reads**, and each has an acceptance check.

### G1 — Mutation-indexing hooks + delete-on-soft-delete (correctness of the projection)
- Idempotent upsert by stable key on `message`/`task`/`file` create/update; `delete_document` on
  soft-delete; capture the `TaskInfo` instead of discarding it [F4], [R1], [R3].
- **One shared document mapper** used by both hooks and rebuild — current drift: file docs carry
  `content = name` only, while PG includes `file_type`; snippets are untrimmed [F4], [R5].
- Fix the plural/singular `type` filter mismatch before any authoritative use [F4], [R1].
- *Acceptance:* DB-gated contract that a committed mutation (and a soft-delete) is reflected in the
  index without a manual reindex; replay of the same upsert is a no-op.

### G2 — Task-await wrapper (determinism)
- Return `TaskInfo`/`task_uid` from mutators or add `*_and_wait` helpers; use
  `TaskInfo::wait_for_completion` / `Client::wait_for_task` with explicit interval/timeout; **assert
  `is_success()`** because the SDK returns `Ok(Task::Failed)` on failure, and `unwrap_failure()`
  panics (forbidden by strict lints) [M-TASKS], [R2].
- Wait the `settingsUpdate` task before filtered queries; settings and document tasks cannot batch
  [M-ASYNC], [R2]. Default SDK timeout is 5 s — too short for a loaded 2 vCPU box; pass 15–30 s.
- *Acceptance:* no fixed sleeps anywhere; every write barrier ends in `Task::Succeeded`; failures
  and timeouts produce actionable `Error::Search` with `task_uid`/`index_uid`.

### G3 — Deterministic rebuild/backfill from Postgres (recovery path)
- New `flora-search-rebuild` binary: settings first on a shadow index, keyset batches
  (`WHERE id > $cursor ORDER BY id LIMIT 500–1000`, never `OFFSET`/`WITH HOLD`), `add_or_replace`
  for replay-safe upserts, checkpoint/resume, `stats` reconciliation, atomic `swap_indexes`,
  rollback by swap-back, delete old index only after verification [PG-SELECT], [M-SWAP], [M-STATS],
  [R5].
- Quiesce writers or use an `id <= watermark` bound around the scan and the swap ("enqueued tasks
  are left unmodified" on swap) [M-SWAP], [R5].
- *Acceptance:* two identical runs produce equal `numberOfDocuments`/`fieldDistribution`; interrupt
  + resume converges; no cross-tenant leakage; soft-deleted rows absent; failed tasks abort before
  swap; peak RSS under a **measured** budget (NV).

### G4 — CI and observability (signal, not guesswork)
- Separate **advisory** `search-tests` job with pinned `postgres:18` + pinned Meilisearch (exact tag
  or digest, never `latest`); health probe on `GET /health` for *server* readiness and task status
  for *data* readiness; per-test unique index template; serial `--test-threads=1` [GH-SVC],
  [M-HEALTH], [R4].
- **Do not** add Meilisearch to `db-tests`: GitHub blocks all steps until every health-checked
  service is healthy, so a Meili failure would fail the authoritative PG contracts [R4].
- Capability/health signals: backend used per request, `configured/initialized/healthy/fresh`,
  failed-task count, oldest pending task age, index-vs-DB count drift, periodic reconcile [R1], [R3].
- *Acceptance:* `build`/`db-tests` never require Meilisearch; gated tests skip loudly with a reason;
  promote `search-tests` to required only after a stable run window.

### Preconditions beyond the four gates
1. **Capacity:** measure RSS/latency of Meilisearch + PG + API co-located on 2 vCPU / 4 GB during a
   realistic rebuild; pin `--max-indexing-memory`/`--max-indexing-threads=1` from that measurement
   (all sizing figures here are NV) [M-RAM], [R1], [R5].
2. **Security:** production env, ≥16-byte master key, HTTPS/reverse proxy, search (not admin) API
   key, tenant tokens consistent with `organization_id`, firewall — Flora's compose is dev-grade
   today [M-SEC], [F8].
3. **Backup/DR runbook:** a second backup domain (snapshots = same-version exact copies; dumps =
   portable but full re-index) with schedule and off-server copy, or accept that the index is a
   disposable projection rebuildable via G3 [M-BACKUP], [R5].
4. **Durability decision:** outbox-lite is the correct bridge if "no drift across crash/restart/deploy"
   becomes a requirement, but it is *not* a substitute for G1–G3; the hard cost is introducing
   transaction boundaries into pool-bound, trait-object repositories [R6].
5. **Re-open triggers (any one, observed and documented):** typo/as-you-type is a product
   requirement; a CJK/JP corpus with unacceptable PG recall; a measured latency breach on the VPS;
   faceting enters the `/api/v1/search` contract [R1].

---

## 5. (d) Proposed wording for the milestone and decision record

### 5.1 README.md (replace lines 16, 30, 40)
```diff
-- 🌱 Seed: Minimal installation (Flora + PostgreSQL).
+- 🌱 Seed: Minimal installation (Flora + PostgreSQL; Postgres FTS is the sole search backend).
...
-- [ ] Search (Meilisearch)
+- [ ] Search (Postgres FTS — global keyword search over messages, tasks, files;
+       Meilisearch deferred to Grove and gated; see cycle 023 decision record)
...
-- **FASE 5**: Search + Meilisearch
+- **FASE 5**: Search (Postgres FTS) + gated Meilisearch re-enablement (Grove tier)
```

### 5.2 Constitution (amend §Processes & Milestones line 134 and Phase 5 line 142; align §Component Architecture line 85)
- Line 134 Seed v0.1 list: replace bare `Meilisearch` with
  `Postgres FTS (Meilisearch is Grove-tier and gated)`.
- Line 142: `Phase 5: Search (Postgres FTS); Meilisearch re-enablement gated (Grove)`.
- Line 85: `global search (Postgres FTS on Seed; Meilisearch on Grove)`.
- Keep the Deployment Tiers table **as-is** (it is already correct); the amendment is to make the
  milestone list consistent with it. Record the version bump (1.2.0 → 1.2.1) in the Sync Impact
  Report, per the Governance section [F2].

### 5.3 OBJETIVO.md / ROADMAP.md
- Cycle 023 entry: state the decision in one line — "PG FTS authoritative for `/api/v1/search` with
  `source: postgres`; Meilisearch dormant, non-authoritative, re-enablement gated (G1–G4)".
- Keep the existing "Postgres-first optionality" bullet but sharpen it: "Valkey, Meilisearch, OIDC
  are *optional per profile*; enabling them never changes the authoritative Seed search contract."

### 5.4 ADR-style decision record (new short section, e.g. `docs/decisions/0023-search-authority.md` or a `## Decision record` block in the 023 spec)

> **Status:** Accepted (cycle 023), supersedes any "Search (Meilisearch)" wording for Seed.
> **Context:** Seed = Flora + PostgreSQL (constitution). Cycle 023's R1 established that the
> Meilisearch path has no mutation hooks, discards task handles, and can serve stale/empty results
> while healthy; PG FTS is transactional, tenant/soft-delete safe, and CI-enforced on a 2 vCPU/4 GB
> VPS. Debloat is a first principle.
> **Decision:** `GET /api/v1/search` is always answered by Postgres FTS and returns
> `source: "postgres"`. Meilisearch is a dormant, non-authoritative Grove-tier capability; enabling
> `FLORA_SEARCH__ENABLED` does not change results. Meilisearch may become authoritative only after
> gates G1–G4 (hooks + shared mapper + delete-on-soft-delete; task-await with `is_success()`;
> PG-sourced rebuild with verified atomic swap; separate advisory CI with task-status waits) plus
> the capacity, security, and backup preconditions.
> **Consequences:** one authoritative backend and one backup domain on Seed; no silent-staleness
> class of bug; `source` is an additive response field; typo tolerance, facets, as-you-type, and
> CJK segmentation remain NV until a re-open trigger is observed and funded.
> **Re-open criteria:** typo/as-you-type as a stated requirement; validated CJK corpus with poor PG
> recall; measured latency breach; faceting in the API contract.

### 5.5 One invariant to enforce across all docs
Never write a milestone item as a bare engine name. Pattern to use everywhere:
**`Capability (authoritative backend) — optional backends and their tier/gate`**. This is what
prevents the next revision from reintroducing the same contradiction.

---

## 6. Options and tradeoffs

| Option | Scope | Pros | Cons | Verdict |
|---|---|---|---|---|
| **A — Wire hooks + rebuild now, keep Meili authoritative** [R1 Option A] | Hooks, await, outbox-lite, rebuild, CI, drift SLO | Preserves typo/facets/CJK option with a clean rebuild story | ~1+ day plus ongoing ops for an off-by-default component; competes for RAM/CPU; second backup/upgrade/security domain; partial parity still visible; contradicts debloat | **Reject for Seed v0.1** |
| **B — PG authoritative; Meili dormant + gated** [R1 Option B; 023] | Route always PG, `source` field, config default off, capability note, DB-gated contracts, written gates G1–G4 | Removes silent-staleness by construction; zero new daemon; reuses 019–022; frees budget for PG quality; matches constitution | Gives up typo/facets/as-you-type (not in contract); CJK future needs re-evaluation | **Adopt** |
| **C — "Keep alive but freeze"** [R1 Option C] | Keep code/profile compiling, serve PG, make no parity claims | Cheapest; avoids churn | Easy to mistake dead code for a supported backend; must still be documented as non-authoritative | Sub-detail of B (keep the crate), not a standalone decision |
| **D — Ambiguous docs, PG code** (status quo) | Bare "Search (Meilisearch)" in README/constitution while code is PG-authoritative | None | Strategic confusion; operators enable Meili expecting parity; violates "docs match code" and constitution governance | **Reject** — this is the defect being fixed |

---

## 7. Recommended decision + NVs

**Recommendation:** Adopt **Option B** and land the §5 wording changes together with the 023 merge
(or immediately after). Keep `flora-search` and the compose `search` profile in the tree as
dormant, deprecated code; do not delete them — that would foreclose the gated path. Add the G1–G4
gate table to the 023 spec and to `docs/dev-smoke.md` so the re-enablement cost is visible to the
next cycle.

**NVs (needs verification, not estimates):**

| ID | Item | Why it matters / disposition |
|---|---|---|
| NV-1 | No Flora PG-FTS vs Meilisearch benchmark on 2 vCPU / 4 GB | All sizing is planning estimate [R1], [R5]; run `EXPLAIN (ANALYZE, BUFFERS)` and a measured rebuild RSS before any re-enablement. |
| NV-2 | No validated CJK/JP corpus or locale mix | Decides whether a re-open trigger fires [M-LANG], [R1]; corpus test before any tokenizer work. |
| NV-3 | `unaccent()` volatility / immutable wrapper on PG 18 | Generated-column/index expressions require `IMMUTABLE`; verify `provolatile` before any accent slice [PG-UNACCENT]. |
| NV-4 | pg_trgm quality/threshold for Flora's short fields (and any CJK chars) | Trigram ignores non-word chars; prototype + corpus check [PG-TRGM]. |
| NV-5 | Language-config switching impact (pt/es/fr/de) on recall and reindex cost | Defer with corpus tests; do not bundle into 023 [R1]. |
| NV-6 | Compose caps (768 MB / 0.75 CPU) are runtime-enforcement-dependent | Verify with `docker stats` if the profile is used; not a gate for PG-only [F8]. |
| NV-7 | Meilisearch image v1.12 vs current stable; SDK 0.33 ↔ server v1.x drift | Pin exact tag/digest and smoke-test before G4; exclude from 023 [R2], [R4]. |
| NV-8 | Operators currently using the `search` profile | Add a clear deprecation note; no secret users assumed [R1]. |
| NV-9 | Drift detector for any retained Meilisearch path | Silent staleness is the core bug; accept as a known limitation of the dormant path [R1], [R3]. |
| NV-10 | Re-enablement effort sizing (G1–G4) | Written as gates, not estimates; re-estimate when a re-open trigger is actually observed. |
| NV-11 | 023 merge state | Working tree is uncommitted at time of writing; wording changes must be keyed to the *contract*, not to a specific commit landing. |

---

## 8. Sources and citations

Fetched or cross-checked 2026-11-16. Vendor comparison pages are flagged where the vendor's
interest is not neutral; normative weight rests on primary engine docs and Flora code.

### Primary — PostgreSQL (verified/canonical)
- **[PG-INTRO]** PostgreSQL 18 docs — 12.1 Introduction: full-text search, `tsvector`/`tsquery`,
  `@@`, configurations, ranking/indexing rationale (verified by direct fetch):
  https://www.postgresql.org/docs/current/textsearch-intro.html
- **[PG-FTS]** PostgreSQL 18 docs — Chapter 12 Full Text Search:
  https://www.postgresql.org/docs/current/textsearch.html
- **[PG-DICT]** PostgreSQL 18 docs — 12.6 Dictionaries (`simple`, Snowball language coverage,
  reindex-on-config-change): https://www.postgresql.org/docs/current/textsearch-dictionaries.html
- **[PG-TRGM]** PostgreSQL 18 docs — F.35 `pg_trgm` (trusted extension, similarity, GIN/GiST,
  thresholds, text-search integration): https://www.postgresql.org/docs/current/pgtrgm.html
- **[PG-UNACCENT]** PostgreSQL 18 docs — F.48 `unaccent` (trusted filtering dictionary,
  diacritics/ligatures): https://www.postgresql.org/docs/current/unaccent.html
- **[PG-SELECT]** PostgreSQL 18 docs — `SELECT` (keyset/row-value predicates, `LIMIT/OFFSET`
  drift, `FOR UPDATE … SKIP LOCKED`): https://www.postgresql.org/docs/current/sql-select.html

### Primary — Meilisearch (vendor; interest not neutral where comparing engines)
- **[M-VS-PG]** Meilisearch vs PostgreSQL search — quick comparison and the "When PostgreSQL FTS
  might be enough" concession (verified verbatim by direct fetch 2026-11-16; page now also covers
  pgvector): https://www.meilisearch.com/docs/resources/comparisons/postgresql
- **[M-RAM]** Impact of RAM and multi-threading on indexing performance — ≤2/3 RAM, ≤1/2 cores,
  OOM crashes, `--max-indexing-memory`/`--max-indexing-threads` (verified verbatim):
  https://www.meilisearch.com/docs/resources/self_hosting/performance/ram_multithreading.md
- **[M-ASYNC]** Tasks and asynchronous operations — async op list, `taskUid`, statuses, queue,
  auto-batching (settings ≠ documents): https://www.meilisearch.com/docs/learn/async/asynchronous_operations
- **[M-TASKS]** Working with tasks — `taskUid` → `GET /tasks/{uid}`, "succeeded = finished":
  https://www.meilisearch.com/docs/learn/async/working_with_tasks
- **[M-LANG]** Language / Charabia tokenization (Latin, German compounds, Cyrillic/Bulgarian,
  Chinese jieba, Japanese lindera IPA): https://www.meilisearch.com/docs/resources/help/language.md
- **[M-SEC]** Securing self-hosted Meilisearch (master key ≥16 bytes, production env, HTTPS,
  search key, tenant tokens, firewall): https://www.meilisearch.com/docs/resources/self_hosting/security/overview.md
- **[M-BACKUP]** Backing up Meilisearch data (snapshots vs dumps, schedule, off-server storage):
  https://www.meilisearch.com/docs/resources/self_hosting/data_backup/overview.md
- **[M-SWAP]** Swap indexes API (atomic; docs/PK/settings/history; "enqueued tasks left unmodified"):
  https://www.meilisearch.com/docs/reference/api/indexes/swap-indexes.md
- **[M-STATS]** Get stats of index (`numberOfDocuments`, `isIndexing`, `fieldDistribution`):
  https://www.meilisearch.com/docs/reference/api/indexes/get-stats-of-index.md
- **[M-HEALTH]** Get health API: https://www.meilisearch.com/docs/reference/api/health/get-health
- **[GH-SVC]** GitHub Docs — About service containers (fresh container per job, lifecycle,
  healthcheck gating): https://docs.github.com/en/actions/tutorials/use-containerized-services/use-docker-service-containers

### Flora code / config evidence (read-only cross-check)
- **[F1]** `README.md:14-47` — tier list, Seed milestone, FASE 5.
- **[F2]** `.specify/memory/constitution.md:38-43,52,85,131-149` — deployment tiers, stack,
  milestone list, phases, governance/versioning.
- **[F3]** `OBJETIVO.md:13,45,84-86`, `ROADMAP.md:40-44,50,57` — cycle log and strategy.
- **[F4]** `flora-api/src/routes/search.rs` (working tree) — `SearchResponse.source`,
  `search()` always `search_pg()`, `include_type()`, PG SQL invariants, and **no** Meilisearch
  branch. Also `flora-search/src/lib.rs` (discarded `_task` handles, singular `type`, `file_type`
  drift); `flora-api/src/state/app_state.rs:128-146` (conditional `Some(SearchService)`).
- **[F5]** `flora-core/src/migrations/sql/20261116000004_search_fts_separator_normalization.sql`
  (plus `20261117000001_…`, `20261118000001_…`) — generated `search_tsv` + partial GIN.
- **[F6]** `flora-core/src/config/search.rs` (`enabled: false`), `.env.example:35-46`
  (all `FLORA_SEARCH__*` keys + legacy fallbacks), `docs/dev-smoke.md` capability note.
- **[F7]** `specs/019-*/spec.md`, `specs/020-*/spec.md`, `specs/021-*/spec.md`,
  `specs/022-notification-idempotency-search-degradation/spec.md`.
- **[F8]** `docker-compose.yml:3,31-32,35,57-58,60-82` — profile-gated Meilisearch v1.12,
  dev env/key, 768 MB / 0.75 CPU, 256 Mb / 1-thread indexing caps; Postgres 1 GB / 1 CPU.
- **[F9]** `specs/023-meilisearch-freshness-rebuild/{spec,plan}.md`,
  `.specify/assessments/023-next-vertical-slice/decision.md` — PG-authoritative decision,
  scope, NVs, re-enablement gate inventory.

### Prior Flora research (cross-checked, not re-litigated)
- **[R1]** `docs/research/2026-11-16-023-search-debloat-pg-only-vps.md` — debloat decision,
  capability/degradation contract, re-open criteria.
- **[R2]** `…-023-meilisearch-task-await-determinism.md` — SDK 0.33 task API, `is_success()` trap,
  no fixed sleeps.
- **[R3]** `…-022-meilisearch-pg-coherence-degradation-contract-vps.md` — coherence/degradation
  contract, drift/rebuild obligations.
- **[R4]** `…-023-meilisearch-gated-ci-harness.md` — separate advisory CI job, two-phase readiness.
- **[R5]** `…-023-meilisearch-rebuild-backfill-pg.md` — keyset rebuild, blue-green swap, verification.
- **[R6]** `…-023-outbox-lite-indexing-pg.md` — outbox durability bridge and its transaction cost.
- **[R7]** `…-023-macro-backfill-notification-reference.md` — upstream reference patterns
  (bounded replay, atomic cutover), adapted to a single small VPS.

---

*End of research deliverable. No source code was modified; no commits were made.*
