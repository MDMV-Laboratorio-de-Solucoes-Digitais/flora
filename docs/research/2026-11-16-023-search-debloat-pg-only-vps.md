# 023 — Search debloat decision: keep optional Meilisearch vs. PostgreSQL-FTS-only on the Seed VPS

**Date:** 2026-11-16  
**Repo:** `flora` (research-only; this document is the only change)  
**Requested output:** `docs/research/2026-11-16-023-search-debloat-pg-only-vps.md`  
**Cycle context:** `ROADMAP.md` §ciclo 023 candidate; `OBJETIVO.md` §5; `specs/023-meilisearch-freshness-rebuild/` (empty at time of writing)  
**Question answered:** For cycle 023, should Flora (a) wire minimal Meilisearch mutation-indexing hooks + rebuild, or (b) deprecate/disable Meilisearch and make Postgres FTS the sole Seed search backend?

---

## 0. Executive recommendation (read first)

**Recommendation: choose (b) — make PostgreSQL FTS the sole *supported* search backend for the Seed profile in 023, and demote Meilisearch to a dormant, explicitly non-authoritative opt-in. Do NOT spend 023 on mutation hooks + rebuild.**

Rationale in one paragraph: Flora's search contract is a keyword search over three small tables with a type filter and a bounded result count — precisely the workload PostgreSQL FTS handles transactionally with zero extra infrastructure [P1–P5, C1–C3]. Meilisearch is already default-off and profile-gated, has **no mutation indexing hooks**, and its async task model means that enabling it can silently return stale/empty results while the API still returns `200` [F1–F4, M2]. The cost of keeping it "supported" is not the binary; it is the sync pipeline (outbox/worker/rebuild/drift detection), separate backups, a second security surface, and ~200–500 MB idle / up to 2⁄3 of RAM during indexing on a 4 GB box that already budgets 1 GB for Postgres [M2, M3, M5, C1, C3]. Even Meilisearch's own documentation concedes PG FTS may be enough for small datasets, basic keyword matching, and cases where you cannot add infrastructure [M8]. Practitioner guidance is consistent: "most projects never need to graduate" beyond database-native FTS, and the graduation trigger should be observed relevance/latency complaints — not the availability of an engine [C1–C3].

**What 023 should actually do** (§8 gives the full slice): flip precedence so PG is authoritative even when Meilisearch is configured; add an explicit backend/capability contract and fail-open tests; keep the `flora-search` crate and `search` profile as a *deprecated, best-effort* path (no parity claim, no hooks, no rebuild investment); and spend the saved budget on PG FTS quality (trigram fallback, optional accent handling, snippet quality, a small EXPLAIN-based benchmark). **Revisit (a) only if** a concrete product signal appears — search-as-you-type, typo tolerance, faceting, or CJK-heavy corpora become requirements — with a larger VPS and a funded sync-pipeline slice (§8.4).

---

## 1. Flora baseline (code-anchored cross-check)

| Area | Evidence in this checkout | 023 implication |
|---|---|---|
| Search config | `flora-core/src/config/search.rs`; `.env.example:35-38` sets `FLORA_SEARCH__ENABLED=false`, URL `http://localhost:7700`. | Already lean-by-default. Deprecation is a small, low-risk change; no "make it optional" work remains. |
| Compose | `docker-compose.yml:60-82`: `meilisearch` behind `profiles: ["search"]`, image **v1.12**, `MEILI_ENV=development`, dev master key, `MEILI_MAX_INDEXING_MEMORY=256Mb`, `MEILI_MAX_INDEXING_THREADS=1`, `mem_limit: 768m`, `cpus: "0.75"`, localhost bind, healthcheck, volume `flora_meili_data`. Postgres is capped at `1g` / `1.00` CPU (`:31-32`); Valkey `384m` / `0.50` (`:57-58`). | Enabling `search` on a 2 vCPU/4 GB host adds a 768 MB / 0.75 CPU competitor to a 1 GB Postgres. No snapshot schedule, no off-server backup target, and `development` env + dev key are **not production-grade** per Meilisearch's own checklist [M4, M5]. Image is ~2 years behind current v1.56 (released 2026-10-26) [M1]. |
| Runtime wiring | `flora-api/src/state/app_state.rs:128-129`: constructs `SearchService` only when `config.search.enabled`; init failure logs and continues with `search_service: None`. | Good optionality. But "configured" ≠ "fresh"; see below. |
| API precedence | `flora-api/src/routes/search.rs:90-120`: if `search_service` is `Some`, return Meilisearch hits as-is; **only on `Err`** log a warning and fall back to `search_pg`. 022 added that fail-open. | Success-with-stale is still trusted. This is the core unresolved correctness risk (§6). |
| PG FTS backend | `search_pg()` in the same file (`:135-305`): per-entity queries with `websearch_to_tsquery('simple', …)` / `plainto_tsquery`, `ts_rank_cd`, tenant-scoped `organization_id`, `is_deleted = false`, 200-char whitespace-normalized snippets, deterministic merge-sort `(rank DESC, created_at DESC, id DESC)`, limit capped at 50. | Already tenant-safe, deterministic, multi-entity, and the 022 fail-open path. It is the natural sole backend. |
| PG schema | `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql`: stored generated `search_tsv` via `to_tsvector('simple', …)` + partial GIN `WHERE is_deleted = false` for `messages(content)`, `tasks(title‖description)`, `files(name‖file_type)`. | In-transaction index maintenance; no worker, no drift. PG docs explicitly note stored generated columns supersede the old trigger pattern [P4]. |
| Meilisearch adapter | `flora-search/src/lib.rs`: per-org index `flora_org_{org_id}`; filterable `["type","created_at_timestamp"]`; `add_documents(..., Some("id"))`; `delete_document`; search filters by `type`. Document payloads differ from PG: file `content = name` only (PG includes `file_type`), snippets untrimmed. Adapter does not inspect async task outcomes. | Even a perfect hook implementation would need doc-shape normalization for cross-backend parity — extra work with no stated user requirement. |
| Mutation hooks | Repo-wide `rg`: no calls to `index_message/task/file`, `setup_index`, or `delete_document` from mutation paths (tasks/files/channels/services). | Confirms user's baseline: enabling Meilisearch yields stale results. Rebuilding would need new outbox/worker/rebuild code (Option A). |
| Prior decisions | cycles 019/020/021/022 made PG the authoritative fallback and deferred hooks/rebuild; `docs/research/2026-11-16-022-meilisearch-pg-coherence-degradation-contract-vps.md` recommended hooks **or** an explicit non-stale contract; `...-016-search-postgres-fts-vs-meilisearch.md` recommended PG-first default. | 023 is the cycle where the deferred choice must be made. This document makes it. |

**Determinism gap that remains after 022:** Meilisearch is trusted on *success*, but success of `add_documents` is only task *enqueue*, not indexed visibility [M2]. With no hooks, "success" can mean "index is empty/stale forever". 022 fixed query-error fail-open; it did not fix success-with-stale.

---

## 2. Operational burden of the extra service (memory, CPU, disk, upgrades, security, observability)

### 2.1 Memory and CPU — the hard constraint on 2 vCPU / 4 GB

- Meilisearch's indexer is **memory-intensive and multi-threaded by default**. Official docs: it uses up to **two thirds of available RAM** for indexing, and on multi-core machines tries to use **up to half the cores**; `--max-indexing-memory` / `--max-indexing-threads` exist precisely to restrain this [M2]. On 2 vCPU that means one core can be consumed by indexing, contending with Postgres and the API. OOM kills during indexing are an explicitly documented failure mode, with the guidance that "your machine should have enough RAM to hold the full dataset in memory during indexing" and that lower-end machines should batch more finely [M2].
- Third-party planning figures: ~200–500 MB idle for a moderate index, and Meilisearch "typically uses 1–3× the data size in RAM" [C3]. These are estimates, **NV** on Flora's hardware/corpus, but directionally consistent with the official warning [M2].
- Flora's compose caps (768 MB / 0.75 CPU) are **hints, not engine guarantees** (the file itself says limits "depend on runtime"; prior HQ notes echo this). `MEILI_MAX_INDEXING_MEMORY=256Mb` is far below the documented 2⁄3-of-RAM default, which is good, but a 256 MB indexing budget on a growing org index means slow/failed batches — exactly the conditions that produce lag and drift [M2, C1].
- Budget arithmetic (planning, **NV**): OS + buffers ~512 MB; Postgres 1 GB; Meilisearch 768 MB; Valkey (if `realtime` also on) 384 MB; API 256–512 MB → **~2.9–3.2 GB of 4 GB committed before indexing spikes**. There is little headroom for the 2⁄3-RAM indexing behaviour that Meilisearch documents [M2].

### 2.2 Disk and backups

- Meilisearch stores an LMDB database (`data.ms`) with its own data directory; snapshots are **exact DB copies** (larger, same-version only) and dumps are **portable but slow** (full re-index on import) [M3]. This is a second backup domain with its own schedule, off-server copy, and restore drill — above and beyond `pg_dump`/WAL for Flora's source of truth [M3, C3].
- Flora's compose has a `flora_meili_data` volume but **no snapshot schedule and no dump/backup destination**; operators enabling the profile inherit a backup obligation they are unlikely to meet. By contrast, PG FTS indexes are re-derivable from the same tables already in Flora's existing backup/restore drills.
- Postgres FTS disk overhead is bounded and local: a GIN FTS index is typically ~30–100% of indexed text size [C3]; the `pg_trgm` and `unaccent` extensions are trusted and installable by non-superusers [P5, P6].

### 2.3 Upgrades

- `getmeili/meilisearch:v1.12` is pinned in compose while the current release is **v1.56** (2026-10-26) — roughly two years and dozens of releases behind [M1, F2]. Meilisearch's own backup guidance says to **create a dump before upgrading** and that snapshots only restore into the same version [M3]; the migration guide frames version moves as a dump/import exercise [M1]. That is a recurring maintenance task Flora would own for a component that is not on the critical path of the product.
- Postgres FTS uses core functions and `contrib` extensions already shipped with the Postgres 18 image Flora runs [P1, P5, P6]. No separate upgrade cadence.

### 2.4 Security surface

Meilisearch's own production checklist requires: a ≥16-byte master key, `env = "production"`, HTTPS via reverse proxy or direct SSL, use of the **search** (not admin) API key in clients, tenant tokens for multi-tenant isolation, and firewall rules [M5]. Flora's compose currently sets `MEILI_ENV=development`, `MEILI_MASTER_KEY=dev-meilisearch-master-key`, and relies on a localhost bind [F2]. That is acceptable for local development, not for a self-hosted SMB deployment. Enabling search therefore adds: a key to manage/rotate, a second auth model (keys + tenant tokens) that must be kept consistent with Flora's org membership checks, and a new network-exposed endpoint to harden [M5, M9].

Postgres FTS, by contrast, needs no new auth surface: the existing `organization_id` predicate in `search_pg()` is the tenancy boundary, and search results are restricted by the same DB access the API already uses [F2, C2]. (Note: Flora uses per-org *queries*, not Meilisearch tenant tokens today — a per-org index is not by itself a security boundary if the API key can read all indexes [M9, C3].)

### 2.5 Observability

- With Meilisearch, correctness depends on an **async task queue**: document adds/deletes are tasks that transition `enqueued → processing → succeeded/failed/canceled`, with batching and a bounded task DB ([M2]; task DB cap 20 GiB, auto-cleanup after ~1M finished tasks [M7]). A production operator must watch task failures, backlog, and drift. Flora currently consumes none of this: the adapter ignores task outcomes and the API exposes no freshness signal [F1, F2].
- Observability minimum for a *supported* Meilisearch path would be: backend used per request, capability state (`configured/initialized/healthy/fresh`), enqueued/failed task counts, oldest pending task age, index-vs-DB count drift, and a periodic reconcile job [C1, C3]. That is a non-trivial slice in its own right — and it is the slice 023 was supposed to choose between, not add on top of hooks + rebuild.

**Burden summary:** the extra service is not a single daemon; it is a second datastore with its own resource envelope, backup domain, upgrade cycle, auth model, and failure telemetry [M2–M5, M7, C1, C3]. For a lean Seed profile these costs are disproportionate to the benefit of typo tolerance and facets that Flora's API contract does not even expose.

---

## 3. What PostgreSQL FTS covers for Flora's collaboration app

### 3.1 Capability fit against the actual contract

| Contract element (`GET /api/v1/search`) | PG FTS today | Verdict |
|---|---|---|
| `q` keyword match over message/task/file | `tsvector @@ tsquery` over generated columns; `websearch_to_tsquery` accepts quoted phrases and `-exclusions`, does not throw on stray punctuation [P1, C2] | Covered |
| `type` CSV filter (`message|task|file`, singular/plural aliases) | Three per-entity queries gated by `include_type()`; aliases normalized in API layer [F2] | Covered |
| `limit` default 20, cap 50 | `LIMIT` per entity + global `truncate` [F2] | Covered |
| Snippet | 200-char whitespace-normalized `left(regexp_replace(...))` [F2]. `ts_headline()` is available if better highlighting is wanted later [P4, C2] | Covered |
| Ordering / determinism | `ts_rank_cd` + `created_at` + `id` tiebreak; stable across runs [F2] | Covered, strong |
| Tenant isolation / soft-delete | `organization_id = $1 AND is_deleted = false` in every branch; partial GIN index aligns with visibility [F2, F3] | Covered, strong |
| Empty/whitespace query | `200 {query,results:[]}` before any backend call [F2] | Covered |
| Freshness after mutation | generated `tsvector` updated in the same transaction; no pipeline [F3, P4] | **Covered, and better than Meilisearch** |
| Typo tolerance | Not in core; `pg_trgm` similarity as a fallback on short fields (names/titles) [P5, C1–C3] | Gap, mitigable |
| Faceted counts, search-as-you-type, BM25 ranking, frontend InstantSearch | Not native; SQL `GROUP BY`, prefix queries, `ts_rank`; no InstantSearch adapter | Gap — **but not in Flora's P0 contract** [M8, C2, C3] |

**Conclusion:** for a "search is a feature, not the product" collaboration app with a few thousand to a few hundred thousand rows per org, PG FTS covers the contract and its correctness properties are *strictly better* than an unsynced sidecar [C1–C3, M8]. Meilisearch's advantages (typo tolerance, facets, instant search, tuned ranking) are exactly the features Flora's API does not currently promise [F2].

### 3.2 Language configs and the `simple` choice

- `to_tsvector('simple', …)` lowercases and keeps every token; it does **no stemming and no stopword removal** in the default `pg_catalog.simple` configuration [P3]. This is deterministic and dump/restore-safe because the config is named explicitly — PG docs warn that omitting the explicit config in triggers can make results change after dump/restore [P4]. Flora names `'simple'` explicitly in both migrations and queries [F3, F2]. Good.
- For Latin languages Flora cares about (pt-BR, es, fr, de, en), `simple` gives literal substring-ish token matching — predictable but no morphological recall ("cantar" won't match "cantei"; "Laufen" won't match "lief"). Language configs (Snowball stemmers) exist for these languages [P3, P7], but switching config means **reindexing** and introduces query/document config drift risk. For 023 this should stay **NV/avoided**, not bundled.
- Diacritics (ação/açao, hábito/habito, für/fur): `unaccent` is a trusted filtering dictionary that removes diacritics and expands ligatures, complete with a ready `unaccent.rules` for European languages [P6]. **Important gotcha for Flora's generated-column design:** `unaccent()` is declared `STABLE`, not `IMMUTABLE`, and generated columns / expression indexes require immutable expressions; the standard workaround is a small `IMMUTABLE` wrapper that names the schema and dictionary explicitly [P6, C8]. So accent-insensitive search cannot be added by merely editing the existing generated expression (NV: verify `provolatile` in the target PG 18 and validate wrapper). This is a reason to keep it as a separate, tested slice — not a 023 hotfix.
- Trigram fallback: `pg_trgm` is a trusted extension providing `similarity`, `%`, GiST/GIN opclasses, and a documented "text search integration" recipe (build a word table from `ts_stat(..., 'simple')`, index it with `gin_trgm_ops`, and suggest spellings for misspelled terms) [P5]. Defaults: `pg_trgm.similarity_threshold = 0.3`, `word_similarity_threshold = 0.6`, `strict_word_similarity_threshold = 0.5` [P5]. Best on short fields (names, titles), poor for fuzzy matching inside long bodies [P5, C1]. It ignores non-word characters when extracting trigrams, so CJK behaviour is limited and needs corpus validation (**NV**) [P5].
- Hard limits are generous for this use case: max lexeme < 2 kB, max `tsvector` data < 1 MB, position ≤ 16,383 [P2]. Flora's 200-char snippets are far below these.

### 3.3 CJK and non-Latin caveats

- The default PG parser does not provide dictionary-based word segmentation for Chinese, Japanese, or Korean. `to_tsvector('simple', <CJK text>)` tends to produce whole-token lexemes rather than segmented words, so substring-style queries and relevance are weak without `zhparser`/`pg_jieba` or application-side segmentation [M8, C6]. Chinese practitioners report exactly this: the common advice is to add `zhparser` or `pg_jieba`, with complaints about installation complexity, custom-dictionary upkeep, and cloud-extension restrictions; one widely read Zhihu post recommends **application-side jieba segmentation** with weighted tokens instead of an extension [C6]. Japanese searches surface the same "start with PG FTS, add a service when needed" framing (**NV**: no authoritative ja benchmark retrieved) [C6].
- Meilisearch's tokenizer **Charabia** handles these explicitly: Chinese via jieba dictionary segmentation, Japanese via lindera IPA, Korean via lindera KO, with kvariant/decomposition normalization [M6]. This is a genuine relevance advantage **if** Flora has or expects CJK-heavy content. It is also the one multilingual case where "PG FTS plus a trigram fallback" does not close the gap. Treat as the strongest argument for keeping a sidecar option alive — but only when backed by a real corpus requirement (**NV**: Flora has no validated CJK tenant/corpus today).

---

## 4. Capability / degradation contract for an optional component

A reusable pattern for Flora: **never let "dependency present" imply "dependency authoritative".** Define explicit modes and invariants:

| Mode | Meaning | When to use |
|---|---|---|
| `pg` (recommended Seed default) | PG FTS is the only search backend. `search_service` is not constructed or is ignored. | All lean/Seed deploys; baseline for deterministic contract tests. |
| `meili_shadow` (deprecated, opt-in) | Meilisearch may be configured for experiments, but **PG remains authoritative**: reads are served from PG; Meilisearch results are never returned as correctness-bearing. Optionally use for dual-read diagnostics off the request path. | Developers evaluating an upgrade; no production parity promise. |
| `meili_authoritative` (**not recommended in 023**) | Meilisearch serves reads; requires proven freshness/completeness (hooks + outbox + rebuild + drift SLO) and resource headroom. | Only after §8.4 gates are met. |

**Invariants that must hold in every mode** (test these, not ranking equality): tenant scoping; `is_deleted` exclusion; empty-query behaviour; type alias normalization; limit cap; deterministic result identity; no 5xx on recoverable backend failure (022 already does query-error fail-open [F2]).

**Freshness contract language for a supported optional component:** "Meilisearch results are eventually consistent with committed DB state; visibility is guaranteed only after the indexing task for the relevant document reports `succeeded` [M2]. Until then PG is authoritative." Flora currently has no such statement wired into code or docs — another reason not to claim support.

**Drift contract:** a sidecar index is a *derived, rebuildable projection*; PG is the source of truth. Any supported sidecar needs (a) idempotent upsert/delete by stable key, (b) delete-on-soft-delete, (c) rebuild-from-Postgres, (d) periodic count/ID reconciliation with an alert threshold (practitioners suggest >1% sustained divergence) and a nightly/weekly reindex even with CDC [C1, C3]. Implementing all four is the Option A cost that 023 should decline.

---

## 5. Real-world guidance: when to drop (or never add) a search service

- **Vendor's own escape hatch:** Meilisearch's comparison page says to consider PostgreSQL FTS when you have a small dataset (thousands of documents), search is not user-facing/real-time, basic keyword matching is sufficient, you cannot add infrastructure, or your managed Postgres restricts extensions [M8]. Flora's Seed profile matches several of these.
- **Practitioner rule of thumb:** "If your app already runs on Postgres and you need a search box over a few hundred thousand rows, start with Postgres full-text search plus `pg_trgm`… Until one of those [typo tolerance/as-you-type/facets] is a real requirement, a second datastore is mostly a second thing to keep in sync" [C1]. The same article prices the honest alternative: an outbox + worker + periodic reindex is "roughly a day of work plus ongoing monitoring", versus an afternoon for PG FTS [C1].
- **Decision guides:** "Most projects never need to graduate" from database-native FTS; the graduation path is PG FTS → tune (`pg_trgm`, languages) → sidecar when latency/relevance complaints accumulate → cluster/hosted later [C3]. "Skipping steps is common and usually wrong" [C3]. StitchDB's balanced comparison: "start in Postgres and move later… you only build a sync pipeline once you know you need one"; PG FTS suits SaaS apps under a few million rows where freshness and per-user permissions matter more than tuned relevance [C2].
- **Sync failure modes are quiet:** stale docs (deleted rows still returned), missing docs, wrong content, phantom IDs — "the failure mode is quiet: a deleted record that still shows up in results, or a new one that never appears" [C1, C3]. Flora has no drift detector today; this is precisely the risk of enabling Meilisearch without hooks.
- **Data-shape gate:** "Your data updates constantly (chat messages, social feed): sync cost is the dominant constraint" [C3]. Flora's primary searchable entity is **messages** — the highest-churn table. This makes a sidecar uniquely unattractive here.
- **Operational gate:** "You have no one to operate another service → Postgres" [C1]. A self-hosted SMB operator is the canonical case.

**When dropping is justified (reverse triggers to keep Meilisearch):** only keep/graduate when (i) typo-tolerant instant search or faceting is a *stated, observed* user requirement; (ii) CJK-heavy corpora need real segmentation; (iii) a measured PG query budget is exceeded on VPS-sized data; or (iv) search becomes the product. None is evidenced in Flora's current specs [F5, F6].

---

## 6. Multilingual depth (EN, pt-BR, es, de, fr, bg, zh-CN, ja+)

> Method note: `websearch` returned no results in this session; discovery used direct primary-doc fetches and DuckDuckGo HTML result pages, then fetched the strongest pages. Locale-specific benchmarks on 2 vCPU/4 GB were not found (**NV**). Where local-language technical literature was thin (notably **bg**, **pt-BR**, **de**, **fr**, **ja**), the analysis is grounded in the engines' documented tokenization behaviour and marked NV; **zh-CN** had substantive practitioner material [C6].

| Locale | PG FTS (`simple` today) | PG upgrade path | Meilisearch / Charabia | Flora guidance |
|---|---|---|---|---|
| **EN** | Good deterministic keyword search; `websearch_to_tsquery` handles phrases/exclusions; `ts_rank_cd` ranking [P1, C2]. | `english` Snowball config for stemming; `pg_trgm` for typos [P3, P5]. | Strong out of the box; typo tolerance + tuned ranking [M6, M8]. | PG sufficient for Seed; parity gap is ranking polish only. |
| **pt-BR** | `simple` keeps all tokens; accent variants (`ação`/`acao`) do not match without `unaccent` [P3, P6]. | `portuguese` stemmer + `unaccent` (via immutable wrapper) + `pg_trgm` [P3, P5, P6, C8]. | Latin pipeline: CamelCase segmentation, decomposition, lowercase, **nonspacing-mark removal** (diacritics) [M6]. | Real but modest UX gap; close with `unaccent` in a later slice, not by adding a service. |
| **es** | Same as pt-BR; `simple` is diacritic-sensitive. | `spanish` stemmer + `unaccent` + `pg_trgm` [P3, P5, P6]. | Same Latin pipeline as above [M6]. | Same as pt-BR. Spanish-language practitioner pages found mainly recipe-level indexes, not comparative operational evidence [C7]; **NV** for local benchmarks. |
| **de** | `simple` misses compounds and stemming. | `german` Snowball config; compound splitting is limited without custom dictionaries [P3]. | Latin + **compound word decomposition** specifically for German [M6]. | This is one of the few Latin cases where Meilisearch has a documented edge over plain PG. Still below the bar of "user-visible requirement" for Seed (**NV** corpus validation). |
| **fr** | `simple` + accents issue; `french` config available [P3, P6]. | `french` stemmer + `unaccent` + `pg_trgm` [P3, P5, P6]. | Latin pipeline + nonspacing-mark removal [M6]. | Same as pt-BR/es. Meilisearch docs illustrate French accent-insensitive FTS via `unaccent`, showing the pattern is portable to PG [P6]. |
| **bg (Bulgarian)** | `simple` lowercases Cyrillic but does not stem; PG/Snowball ships **no Bulgarian stemmer** in core [P3, P7]. | Custom Hunspell/Ispell dictionary or `simple` + `pg_trgm` [P3, P5]. | Charabia: **Cyrillic/Georgian (explicitly incl. Bulgarian)** with default segmentation and decomposition + lowercase normalization [M6]. | **Notable finding:** Charabia's "Bulgarian support" is normalization/segmentation, *not* stemming — the relevance delta vs PG `simple` is smaller than marketing implies. Typo tolerance remains the main advantage. Bulgarian-language comparative sources were not found in search (**NV**). |
| **zh-CN** | Weak with `simple`: no dictionary segmentation, poor substring/recall; needs `zhparser`/`pg_jieba` or app-side jieba [C6, M8]. | Extension or application-side segmentation; custom dictionaries, cloud-extension limits, config drift [C6]. | **jieba-based dictionary segmentation** + kvariant conversion [M6]. | Strongest technical case for a sidecar. But no validated CJK requirement in Flora today (**NV**); if it appears, re-open §8.4 with a corpus test rather than pre-building it now. |
| **ja** | Similar to zh-CN: default parser lacks lindera-style word segmentation [M8]. | Third-party tokenizer needed (**NV**). | **lindera IPA** dictionary segmentation [M6]. | Same gate as zh-CN. |
| **Other** | Latin/whitespace languages use the default pipeline; unlisted non-whitespace languages degrade [M6]. | Per-language config/`unaccent` as needed. | Greek, Arabic, Hebrew, Turkish, Thai, Khmer, Korean have dedicated pipelines [M6]. | Document as capability, not guarantee; keep NV for unvalidated locales. |

**Cross-locale contract principle:** assert identical **invariants** (tenant isolation, soft-delete, empty query, type filter, limit, deterministic IDs) across backends; treat **ranking order and snippet text as backend-specific** and never promise cross-engine equality [F4]. This keeps tests honest and avoids false parity claims.

---

## 7. Options for 023 and tradeoffs

### Option A — Wire minimal hooks + rebuild (keep Meilisearch as a maintained backend)
**Scope:** idempotent upsert/delete hooks on task/file/message mutations, delete-on-soft-delete, async task outcome handling or `waitForTasks`, outbox-lite (`FOR UPDATE SKIP LOCKED`) for durability, rebuild-from-Postgres command, drift/count reconciliation, caps and health/capability signalling, doc-shape parity (`file_type`, snippet trimming) [F4, C1, C3].

| Pros | Cons |
|---|---|
| Preserves Meilisearch UX option (typo tolerance, future facets/CJK). | ~1+ day of engineering plus ongoing monitoring for a component that is off by default [C1]. |
| Clean rebuild story if a CJK/typo requirement appears. | Still competes for RAM/CPU on 4 GB; indexing can hit documented OOM conditions [M2, C3]. |
| | Adds a second backup/upgrade/security/observability domain [M3–M5]. |
| | Parity is only partial without ranking/snippet normalization; users still see differences [F1, F4]. |
| | Directly contradicts "one service less" debloat goal and the high-churn message table [C3, F6]. |

### Option B — Deprecate/disable Meilisearch; double down on PG FTS (recommended)
**Scope:** PG authoritative always; demote `flora-search` + `search` profile to deprecated opt-in with an explicit non-authoritative contract; capability/fail-open tests; invest in PG FTS quality and a small benchmark; document the re-enable gates.

| Pros | Cons |
|---|---|
| Removes the silent-staleness correctness risk by construction. | Gives up typo tolerance/facets/instant search (not in current contract) [F2]. |
| Zero new daemon, sync pipeline, backup domain, key rotation, or task queue [C1–C3]. | CJK-heavy future tenants would need re-evaluation (§8.4) [M6, C6]. |
| Reuses 019–022 work (tenant-safe, deterministic PG fallback, 022 fail-open) [F2]. | Requires a clear deprecation note so operators of the opt-in profile are not surprised. |
| Frees 023 budget for real quality work (trigram, accent, snippets, benchmarks). | |
| Aligns with stated Seed strategy and practitioner consensus [F6, C1–C3]. | |

### Option C — "Keep alive but freeze" (middle ground, rejected as the primary 023 slice)
Keep the crate/profile compiling and optionally present, but make no parity/freshness claims, add no hooks or rebuild, and serve PG always. This is effectively Option B with dead code retained; it is cheap and can be a *sub-detail* of B (keep code to avoid churn), but it must not be mistaken for a supported backend.

---

## 8. Concrete 023 recommendation and verifiable slice

### 8.1 Primary decision
Adopt **Option B**. Make `pg` the authoritative and only *supported* Seed backend. Keep `flora-search` and the `search` compose profile in the tree, marked **deprecated / best-effort / non-authoritative**, with no new feature work. Concretely:

1. **Precedence change** in `flora-api/src/routes/search.rs`: serve `search_pg()` as the default path; when `search_service` is configured, treat it as an optional shadow/diagnostic only — do not return its hits as authoritative results in the Seed profile. (Simplest implementation: gate the Meilisearch branch behind an explicit opt-in flag such as `search.backend = meili_shadow`, default `pg`.)
2. **Capability contract**: expose a small backend/capability indicator in logs/metrics (`backend=pg`, `meili_configured=true/false`, `meili_authoritative=false`); document that Meilisearch is a derived, non-authoritative experiment and that PG is the source of truth.
3. **Tests** (DB-gated / contract, consistent with 022 CI): empty query, type aliases, limit cap, tenant isolation, `is_deleted` exclusion, deterministic ordering, and **backend selection** (PG used by default even when a Meilisearch service is present; no 5xx if it errors). These are cheap and directly reduce the silent-staleness risk.
4. **Docs/config**: set `.env.example` and compose comments to state PG is the sole supported Seed backend and the `search` profile is deprecated; note that enabling it today can return stale results and is not covered by parity contracts. Consider pinning/upgrading the image only if the profile is kept for experiments (currently v1.12 vs v1.56) [M1, F2] — but do not make upgrades a 023 deliverable.

### 8.2 PG FTS quality work funded by the saved budget (keep tightly scoped)
- **Trigram fallback** for short fields (task titles, file names) when `tsvector` returns nothing: `pg_trgm` GIN + `%`/`similarity`, with documented threshold choice [P5, C1, C3]. Keep it behind a small, testable query branch; measure before enabling globally.
- **Accent handling as NV-extension, not 023 code**: document the `unaccent()` STABLE/immutable-wrapper constraint and validate on PG 18 before any generated-column change [P6, C8]. Do not silently edit the existing generated expressions.
- **Snippet quality**: optionally switch from 200-char `regexp_replace` to `ts_headline()` for query-term highlighting [P4, C2] — low risk, but only if contract tests confirm stable output.
- **One small benchmark**: `EXPLAIN (ANALYZE, BUFFERS)` on representative org-sized data on the VPS target to record p50/p99 and index-size baseline; keep all performance numbers **NV until measured** [C1, C3, F6].

### 8.3 Explicit non-goals for 023
Meilisearch hooks/delete-on-soft-delete, outbox/tx-coupling, waitForTasks, rebuild/reindex CLI, drift-reconciliation daemon, cross-engine ranking/snippet parity, Meilisearch-gated deterministic CI, CJK tokenizers (`zhparser`/`pg_jieba`), language-config switching, hybrid/vector search, frontend changes, 2 vCPU/4 GB load/OOM/restore benchmarks beyond the single EXPLAIN check.

### 8.4 Re-open criteria (when to reverse this decision)
Revisit Option A only when at least one of these is **observed and documented**, not assumed:
1. Users explicitly request/complain about missing typo tolerance or search-as-you-type, and it is a product requirement.
2. A tenant/corpus with substantial zh-CN/ja content exists and a corpus test shows PG `simple` recall is unacceptable [M6, C6].
3. A benchmark on the target VPS shows PG FTS exceeding the agreed latency budget on real data.
4. Faceting/filters become part of the `/api/v1/search` contract.
If triggered, fund the full sync-pipeline slice (outbox + worker + rebuild + drift SLO + caps + security/backup runbook) and size the VPS accordingly [C1, C3, M2–M5].

---

## 9. Risks and open verification items (NV)

| ID | Item | Why it matters | Disposition |
|---|---|---|---|
| NV-1 | No Flora benchmark of PG FTS vs Meilisearch on 2 vCPU/4 GB. | All sizing figures here are planning estimates [M2, C3]. | Run the §8.2 EXPLAIN check; keep numbers NV until then. |
| NV-2 | No validated CJK/JP corpus or locale mix for Flora. | Decides whether §8.4 trigger 2 fires [M6, C6]. | Corpus test before any CJK work. |
| NV-3 | `unaccent()` volatility on the target PG 18 and wrapper correctness. | Generated-column/index expressions need `IMMUTABLE` [P6, C8]. | Verify `provolatile`; test wrapper in a migration; never edit generated expressions blindly. |
| NV-4 | pg_trgm quality/threshold for Flora's short fields and any CJK chars. | Trigram ignores non-word chars; thresholds need tuning [P5]. | Prototype query branch + corpus check. |
| NV-5 | Language-config switching impact (pt/es/fr/de/bg) on ranking and reindex cost. | Stemming improves recall but changes tokens and requires reindexing [P3, P4]. | Defer; separate spec with corpus tests. |
| NV-6 | Operators currently using the `search` profile. | Deprecation could surprise them. | Check issues/docs; add a clear deprecation note; no secret users assumed. |
| NV-7 | Compose resource caps are runtime-enforcement-dependent. | 768 MB/0.75 CPU may not bind on all runtimes; HQ notes warn of this. | Verify with `docker stats` if the profile is used; not a 023 gate for PG-only. |
| NV-8 | Current Meilisearch image v1.12 vs v1.56; upgrade/dump path untested. | Relevant only if the profile stays alive [M1, M3]. | Document; exclude from 023 scope. |
| NV-9 | Local-language (bg/pt/es/de/fr/ja) comparative benchmarks absent. | Multilingual conclusions rest on engine docs + thin local literature [M6, C6, C7]. | Mark as such; do not claim measured relevance parity. |
| NV-10 | Drift detector for any retained Meilisearch path. | Silent staleness is the core bug [C1, C3]. | Accept as known limitation of the deprecated path; do not claim support. |

---

## 10. Sources and citations

Fetched 2026-11-16 unless noted. Vendor sources are flagged where their interest is not neutral.

### Primary — PostgreSQL (high trust)
- **[P1]** PostgreSQL 18 docs — Full Text Search (chapter): https://www.postgresql.org/docs/current/textsearch.html
- **[P2]** PostgreSQL 18 docs — 12.11 Limitations (lexeme/tsvector/tsquery size limits): https://www.postgresql.org/docs/current/textsearch-limitations.html
- **[P3]** PostgreSQL 18 docs — 12.6 Dictionaries (simple, synonym, thesaurus, Ispell, Snowball; language coverage; reindex on thesaurus changes): https://www.postgresql.org/docs/current/textsearch-dictionaries.html
- **[P4]** PostgreSQL 18 docs — 12.4 Additional Features (`setweight`, `ts_headline`-adjacent functions, generated-column guidance superseding triggers, explicit-config warning): https://www.postgresql.org/docs/current/textsearch-features.html
- **[P5]** PostgreSQL 18 docs — F.35 `pg_trgm` (trusted extension, similarity operators, GIN/GiST, thresholds, text-search integration): https://www.postgresql.org/docs/current/pgtrgm.html
- **[P6]** PostgreSQL 18 docs — F.48 `unaccent` (trusted filtering dictionary, diacritics/ligatures, rules file): https://www.postgresql.org/docs/current/unaccent.html
- **[P7]** Snowball — stemming algorithms list (no Bulgarian/Chinese/Japanese stemmers): https://snowballstem.org/algorithms/

### Primary — Meilisearch (vendor docs; interest not neutral where comparing engines)
- **[M1]** Meilisearch docs index (`llms.txt`) — current page map and changelog (v1.12 = 2024-12-23; v1.56 = 2026-10-26): https://www.meilisearch.com/docs/llms.txt
- **[M2]** Impact of RAM and multi-threading on indexing performance (2⁄3 RAM, up to half of cores, OOM crashes, `--max-indexing-memory`/`--max-indexing-threads`): https://www.meilisearch.com/docs/resources/self_hosting/performance/ram_multithreading.md
- **[M3]** Backing up Meilisearch data (snapshots vs dumps; schedule snapshots; dump before upgrade; off-server storage): https://www.meilisearch.com/docs/resources/self_hosting/data_backup/overview.md
- **[M4]** Running Meilisearch in production (dedicated user, config file, systemd, reverse proxy, HTTPS/cert renewal; Cloud recommended): https://www.meilisearch.com/docs/resources/self_hosting/deployment/running_production.md
- **[M5]** Securing self-hosted Meilisearch (master key ≥16 bytes, production env, HTTPS, search key, tenant tokens, firewall): https://www.meilisearch.com/docs/resources/self_hosting/security/overview.md
- **[M6]** Language / Charabia tokenization table (Latin incl. pt/es/fr; German compounds; Cyrillic incl. Bulgarian; Chinese jieba; Japanese lindera IPA; Korean lindera KO): https://www.meilisearch.com/docs/resources/help/language.md
- **[M7]** Known limitations (10-word query cap; task DB 20 GiB / ~1M-task cleanup; 1000 concurrent searches; LMDB constraints): https://www.meilisearch.com/docs/resources/help/known_limitations.md
- **[M8]** Meilisearch vs PostgreSQL search (vendor comparison; admits PG FTS may be enough for small datasets, basic keyword search, no extra infra, no native CJK): https://www.meilisearch.com/docs/resources/comparisons/postgresql.md
- **[M9]** Multitenancy and tenant tokens (per-index filters, short-lived client tokens): https://www.meilisearch.com/docs/learn/security/api_keys

### Practitioner / case-study / secondary (directional, not benchmarks)
- **[C1]** Libme, “Postgres Full-Text Search vs Meilisearch vs Typesense: Do You Need a Search Service for a Small App?” (DEV, 2026-11-09) — second datastore = second thing to sync; outbox+reindex ≈ a day + monitoring; decision table: https://dev.to/libme/postgres-full-text-search-vs-meilisearch-vs-typesense-do-you-need-a-search-service-for-a-small-app-5hd2
- **[C2]** StitchDB, “Postgres Full-Text Search vs Elasticsearch (and Meilisearch)” (2026-11-04) — no second system to sync; RLS/freshness; "start in Postgres and move later": https://stitchdb.com/blog/postgres-full-text-search-vs-elasticsearch
- **[C3]** Big Iron, “Adding full-text search to an existing app in 2026: the three patterns” — cost table (PG setup/sync/ops = zero; sidecar = real); “most projects never need to graduate”; 200–500 MB / 1–3× RAM estimates; drift/reconcile guidance: https://www.bigiron.cc/guides/adding-full-text-search-to-an-existing-app-the-three-patterns
- **[C4]** devmanue — “native PostgreSQL already handles 95% of search use cases at a fraction of the cost” (search-result snippet only; **NV** full text): https://devmanue.com/blog/postgresql-full-text-search-tsvector-vs-elasticsearch/
- **[C5]** stackpractices.com/es — Spanish-language FTS recipe overview (Elasticsearch/Meilisearch/PostgreSQL tsvector): https://stackpractices.com/es/recipes/full-text-search/
- **[C6]** zh-CN practitioner material surfaced via DuckDuckGo: Zhihu “关于 PostgreSQL 全文检索的实战 —— 中文分词…” (app-side jieba recommendation) https://zhuanlan.zhihu.com/p/523233840 ; CSDN “tsvector、tsquery 与中文分词” (zhparser vs pg_jieba, config/dictionary/cloud limits) https://blog.csdn.net/qq_45239623/article/details/169365748 ; 51CTO jieba-based PG Chinese FTS https://blog.51cto.com/u_16213681/14591077 . Snippets translated/summarized; full articles not fetched (**NV**).
- **[C7]** Regional-language discovery: pt-BR, es, de, fr, ja searches mostly surfaced the same English primary sources; substantive local-language operational comparisons were not found (**NV** for bg/pt/es/de/fr/ja local benchmarks).
- **[C8]** `unaccent()` immutability constraint — Cybertec (functional indexes require immutable results) https://www.cybertec-postgresql.com/en/unaccent-getting-rid-of-umlauts-accents-and-special-characters/ ; Stack Overflow “unaccent() preventing index usage” (unaccent is STABLE, wrapper pattern) https://stackoverflow.com/questions/28899042/unaccent-preventing-index-usage-in-postgres . Discovered via DuckDuckGo; treat as secondary, verify on target PG [NV-3].

### Flora repo evidence
- **[F1]** `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`
- **[F2]** `flora-api/src/routes/search.rs`, `flora-api/src/state/app_state.rs`
- **[F3]** `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql`
- **[F4]** repo-wide absence of Meilisearch mutation hooks in `flora-api/src/routes/{tasks,channels,files}.rs` and services
- **[F5]** `specs/022-notification-idempotency-search-degradation/spec.md`, `plan.md`, `tasks.md`; `specs/023-meilisearch-freshness-rebuild/` (empty)
- **[F6]** `OBJETIVO.md`, `ROADMAP.md`; prior research `docs/research/2026-11-16-016-search-postgres-fts-vs-meilisearch.md`, `...-022-meilisearch-pg-coherence-degradation-contract-vps.md`, `...-022-meilisearch-indexing-hooks-reindex-parity-vps.md`
- **[F7]** `docker-compose.yml`, `.env.example`

---

**Bottom line:** For cycle 023, the cost/benefit strongly favours **deprecating Meilisearch from Flora's supported Seed contract and standardising on PostgreSQL FTS**. The remaining Meilisearch advantages are unexercised by the API contract and are dwarfed by the sync/ops/security/backup burden on a 2 vCPU/4 GB self-hosted box. Keep the door open with explicit re-enable gates (typo tolerance, CJK, measured latency, faceting), but do not pay for the sync pipeline until a real requirement forces it [P1–P7, M2–M8, C1–C3, F1–F7].
