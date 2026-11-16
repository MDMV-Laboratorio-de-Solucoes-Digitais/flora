# 018 — Macro search, indexing, notifications & eventing: parity and 80/20 debloat for Flora (multilingual)

**Date:** 2026-11-16  
**Branch/context:** `dev@b6440c534` (read-only research; this document only, no source/commit)  
**Goal:** Deep, high-reliability synthesis of how Macro (`macro-inc/macro`) implements **search, indexing, notifications, and eventing** for files/messages/tasks/docs (and adjacent entities), cross-checked with `OBJETIVO.md`, `ROADMAP.md`, `docs/research/2026-11-16-017-macro-files-mvp-parity.md`, and Flora’s current `flora-search/`, `flora-notifications/`, routes/services — to identify the minimal **80/20 primitives Flora should keep/adapt vs defer** for a self-hosted SMB alternative on **cheap VPS (Rust + PostgreSQL, optional Meilisearch)**.

## 1. Method, sources, and multilingual coverage

**Local cross-check (read):** `OBJETIVO.md` (north star, P0/P1/P2, NVs incl. wiring busca/notificações, lacunas realtime/multi-nó/replay, OIDC/JWKS, carga/restore), `ROADMAP.md` (018 em preparação/R1: contratos DB-gated/hardening files + wiring essencial busca/notificações; P0.2 wiring busca/eventos→notificações), `docs/research/2026-11-16-017-macro-files-mvp-parity.md` (Macro Drive = ingestão/search/graph/agent-coupled; keep durable core only), `flora-search/src/lib.rs` + `flora-api/src/routes/search.rs` (Meilisearch-only service; per-org index; route requires membership but assumes available search service), `flora-notifications/src/lib.rs` + `flora-api/src/routes/notifications.rs` (DB insert + Valkey `XADD` publish; unread/read APIs via repositories), `docs/research/2026-11-16-016-search-postgres-fts-vs-meilisearch.md` (prior PG FTS vs Meilisearch/VPS analysis).

**Primary external evidence (EN, highest confidence):**
- Macro README (product surfaces: email/messages/tasks/docs/canvas/agents/calls/file storage/CRM/databases/calendar; file storage auto-imported from email/channels and fully searchable; bidirectional @-graph; channel-based permissions): https://raw.githubusercontent.com/macro-inc/macro/main/README.md
- `search_processing_service` (hexagonal; **Kafka** consumer for `macro.calls` and `macro.agent_session_lifecycle` with commit-after-handoff, sharded workers, retries/idempotent upserts; **“Other live entities: Kafka handlers and legacy SQS workers drain events for the remaining search entities”**; **SQS/backfill** HTTP jobs for most entities incl. agent sessions direct path; OpenSearch indexing; call events map to full upsert/delete/ignore): https://upd.dev/macro-inc/macro/src/branch/main/services/search_processing_service
- Service topology (local stack: Postgres/Redis/LocalStack/OpenSearch/Kafka/FusionAuth; cloud: ECS/Fargate + ALB + SQS/DLQ + EventBridge + S3/DynamoDB + RDS/Redis/OpenSearch; `search-event-queue` producers include document storage/email/cognition/auth → `search_processing_service`; document bucket Object Created → search upload/text extractor/finalizer; notification ingress/egress queues → `notification_service`; connection gateway/Redis/Dynamo): https://grok-wiki.com/public/docs/macro-inc/macro-bb988e1a448e/pages/08-service-topology.md
- Workspace events / routines (unified event bus across email/chat/docs/tasks/calls; triggers for arrivals/mentions/create-delete/task status-property/call start-end; routines run with creator permissions; agent/bot changes and historical imports don’t fire triggers): https://macro.com/docs/concepts/event-system
- Search/storage API concepts (unified `/search` and channel search with cursors/match modes/filters; keyword/literal/prefix semantics per entity class in MCP ContentSearch docs; DSS/storage/search mounted surfaces): https://docs.macro.com/AI/mcp/tools/content-search , https://grok-wiki.com/public/docs/macro-inc/macro-bb988e1a448e/pages/17-storage-and-workspace-apis.md
- Local/self-host footprint and cost signals (Nix+Docker, 6 core infra services + many app services; production self-hosting not primary focus): https://www.aibrew.io/blog/macro-workspace-self-host-guide , https://macro.atomgit.com/faq.html

**Multilingual research (requested: EN, pt-BR, es, de, fr, zh-CN, ja):** targeted searches were run in all seven locales. High-confidence, **Macro-specific** primary technical docs retrieved were predominantly **EN**. Locale signals obtained: **pt-BR** (Novidades IA overview of Macro as unified workspace with shared memory/bidirectional links/search over files; non-primary architecture depth), **es** (general self-host search/VPS/notification or adjacent tooling, not Macro internals), **de** (general self-host/collab/search infra and Macro overviews), **fr** (self-host search/VPS guidance e.g. Meilisearch hosting, not Macro internals), **zh-CN** (magformersworld.cn synthesis of Macro search pipeline: Kafka invalidation hints → reread primary DB → full upsert to OpenSearch, SPS + document_text_extractor + opensearch_client/query_builder, unified `/search`, backfill/reindex; secondary but technically aligned with EN sources), **ja** (general self-host workflow/search/RAG/VPS docs, limited Macro-specific internals). **Limitation/NV:** no new locale-primary (non-EN) first-party Macro specs contradicting/expanding EN evidence for file/message/task/doc indexing or notification delivery internals were verified; non-EN operational conclusions are triangulated with EN primary sources + prior Flora multilingual VPS research. `upd.dev` search_processing_service page returned 403 on direct fetch in this environment (NV on in-page markdown capture), but web search excerpts and topology/docs provided consistent evidence.

## 2. Macro: how it implements search, indexing, notifications, and eventing (evidence)

### 2.1 Search and indexing

| Aspect | Macro approach (evidence) | Notes for files/messages/tasks/docs |
|---|---|---|
| **Engine & topology** | OpenSearch-backed search; local stack includes OpenSearch (+ Kafka/Redis/LocalStack/Postgres/FusionAuth). Cloud uses managed OpenSearch/RDS/queues/Lambdas/ECS. | Search is a **dedicated subsystem**, not embedded in app DB. |
| **Indexing service** | `search_processing_service` (SPS), hexagonal: Kafka consumer + legacy SQS workers + HTTP backfill API; domain/outbound adapters; shared `process/` indexing. | Separation of live ingestion, backfill, and adapters. |
| **Live event ingestion** | Kafka topics (e.g. `macro.calls`, `macro.agent_session_lifecycle`) consumed by durable group `search-processing-service`; events treated as **invalidation hints** → reread primary DB/current state → idempotent full upsert/delete to OpenSearch (commit-after-handoff, sharded ordered workers per key, retries; at-least-once/loss-window tradeoff acknowledged). | Designed for convergence/idempotency over exactly-once. |
| **Other entities** | “Other live entities: Kafka handlers and legacy SQS workers drain events for the remaining search entities.” Search-event queue producers include document storage, email, cognition, auth flows. Document bucket Object Created fans out to search upload/text extractor/finalizer. | Files/messages/tasks/docs fit “other entities” via queues/events (specific per-entity handler details partly NV from public docs). |
| **Text extraction** | `document_text_extractor` Lambda/service and parsers (PDF/DOCX/MD etc. per secondary zh-CN synthesis consistent with topology) so binary/formatted content becomes searchable text. | Raises CPU/storage/queue complexity; unnecessary for metadata-only MVP. |
| **Backfill/reindex** | Internal HTTP backfill endpoints per entity (`/internal/backfill/{entity}`) returning job IDs; most enqueue to SQS, some (agent sessions/properties) index directly; index mapping management/rebuild workflows implied. | Operational maturity feature, heavy for Seed. |
| **Query surface** | Unified `POST /search` (+ simple/channel variants) with cursors, match modes, type/entity filters, permission filtering/enrichment; keyword/literal/prefix semantics vary by entity (ContentSearch: ANDed terms/prefix for docs/emails in some cases, phrase-prefix behavior for sessions/channels/calls; projects/calendar not content-indexed there). | Product-grade unified search UX/API far beyond Flora’s current single endpoint. |
| **Product coupling** | Files “auto-imported from email and channels, fully searchable”; unified search across workspace incl. attachment PDFs; agents/tools query search; bidirectional @-graph and channel-based sharing influence visibility. | Search value is amplified by ingestion, graph, permissions, and agents. |

### 2.2 Notifications

| Aspect | Macro approach (evidence) | Implication |
|---|---|---|
| **Service & delivery** | `notification_service` in topology with **notification ingress/egress queues**, SNS platform applications, push event handling; in-app notification inbox/lists and analytics events (`notification_opened` sources in-app/browser/native/home recommendation) observed in commit history. | Multi-channel (in-app + web/mobile push) with queue-driven fanout. |
| **Domain model/perf** | Commit evidence of indexed channel-message notification lookups (`metadata` JSON expression indexes) to avoid seq scans on large `notification` table; notification filtering/state-machine evolution in changelogs. | At scale, notification queries become DB/perf-sensitive and schema/index tuned. |
| **User-facing scope** | Unified inbox/notifications across email/messages/mentions/tasks in product narrative; routines/event system can react to workspace events (email/message/mention/doc/task/call changes) — triggers and automations complement notifications. | Notifications are part of a broader attention/event system, not just a table. |

### 2.3 Eventing and workspace events

| Aspect | Macro approach (evidence) | Implication |
|---|---|---|
| **Transport patterns** | **Kafka** for selected durable streams (calls, agent session lifecycle, event broker references), **SQS** for search/processing/backfills/work queues with DLQs, **EventBridge/S3 notifications** for object events, plus service-to-service HTTP. | Polyglot, cloud-native eventing; operationally heavy. |
| **Ordering/consistency** | Sharded workers with per-entity ordering within consumer instance but acknowledged out-of-order publish/rebalance gaps; handlers must be idempotent and converge by rereading source of truth. | Correctness favors idempotent reconciliation over strict global ordering. |
| **Unified workspace events** | Product-level unified event bus/routines: triggers for new email, channel message/@mention, document create/delete, task create/status/property changes, call start/end; runs with creator permissions; excludes agent/bot-originated and historical import triggers. | High-level automation layer on top of lower-level domain/infra events. |
| **Cross-surface graph** | Bidirectional @-links and channel-based permission propagation couple events to access and discoverability. | Event semantics intertwined with sharing model. |

## 3. Flora current state (cross-check)

| Area | Observed now | Gap / tension vs Macro (and vs 018 goals) |
|---|---|---|
| **Search crate/service** | `flora-search`: Meilisearch client, per-org index `flora_org_{org_id}`, index/delete for `message|task|file` (content = message text / task title+desc / file name), simple query + `type IN [...]`, snippet from `content`. | No extraction, no PG FTS, no backfill pipeline, no cursor/advanced ranking/facets, no mutation wiring guarantees; external-engine dependency. |
| **Search route/state** | `GET /api/v1/search` auth+org membership, CSV `type`, limit cap; `AppState.search_service` optional with `503` when unavailable (per route code read). | 016 research flagged boot-coupling risk historically — **NV** if fully resolved across all init paths/configs; no stable “search disabled” contract documented for all clients. |
| **Notifications crate** | `flora-notifications::NotificationService`: insert notification row + `XADD` to Valkey stream `notifications:org:{org_id}` (payload/event_type/user_id), get unread, mark read. | Publish exists but **end-to-end delivery/consumption by API/WS/clients** and wiring from core mutations are NV/incomplete; depends on Valkey availability for publish path. |
| **Notifications routes** | List (all/unread) and mark-read via repositories scoped to user ownership. | No create endpoint exposed here (server-side creation expected), no push/email channels, no event-driven generation wired broadly to core mutations (ROADMAP/OBJETIVO gap). |
| **Eventing/realtime** | In-process hub/canonical events for persisted messages (015) and WS auth/catch-up progress noted in OBJETIVO/ROADMAP; no Kafka/SQS/EventBridge. | Correct for monolith/VPS; must avoid introducing distributed brokers prematurely. |
| **Infra/profile** | Seed/Postgres-first intent, optional components, compose profiles/limits/healthchecks (016/017). | Macro’s stack (OpenSearch/Kafka/LocalStack/FusionAuth/many services) is the opposite of VPS debloat target. |

## 4. Prioritized keep / replace / defer (80/20 for cheap VPS)

Scoring lens: **user value for SMB Seed**, **operational cost on ~2 vCPU/4 GB**, **alignment with Postgres-first/optional components**, **implementation risk**, and **dependency on unresolved foundations (files contracts, tenancy, wiring)**.

### KEEP / ADAPT now (high leverage, low-moving parts)

1. **P0 — Idempotent domain events as invalidation/update signals (in-process):** keep Macro’s *principle* “event as hint, converge from source-of-truth state” for search indexing and notification generation, but implement as **transaction-adjacent application events/outbox-lite or direct post-commit hooks in monolith** (no Kafka/SQS). Deterministic, replayable from DB where possible.
2. **P0 — Postgres as source of truth + Postgres FTS metadata search path (default):** adapt Macro’s unified *idea* of searchable core entities (messages/tasks/files/docs metadata) but **replace OpenSearch with PostgreSQL FTS/`pg_trgm` (or equivalent) as Seed default**, with strict `organization_id`/scope filters and stable API contract. Keep Meilisearch only as **explicit optional profile** if ever needed (per 016 research).
3. **P0 — Minimal, tenant-safe notification primitives:** DB-persisted notifications + simple read/unread APIs + **synchronous or post-commit in-process creation** on core mutations (messages/tasks/files as prioritized). Treat real-time push as enhancement (reuse WS/Valkey only if already justified), not as a separate distributed pipeline.
4. **P1 — Bounded backfill/reindex command (single-node):** a simple admin/dev task to (re)index searchable rows from Postgres for selected org/entity types — replaces Macro’s HTTP backfill jobs/SQS orchestration for Seed scale.
5. **P1 — Contract-first wiring:** explicit, deterministic mapping from core mutations → searchable document shape / notification type+target (id, org, actor/recipient rules) with tests; prefer correctness/tenancy over coverage breadth.

### REPLACE / SIMPLIFY (Macro capability → Flora Seed equivalent)

1. **OpenSearch + `search_processing_service` + Kafka/SQS/EventBridge/Lambdas/text-extractor fleet → Postgres FTS + in-process index updater/worker-lite.**
2. **Per-entity distributed indexing pipelines and cursor-tracked backfill jobs → DB scans/batched upserts in one process/container.**
3. **Notification ingress/egress queues + SNS/mobile push + analytics fanout → in-app DB notifications (+ optional lightweight Valkey/WS fanout only behind profile) with no external push infra.**
4. **Unified event bus/routines automation platform → minimal internal event dispatch for search/notifications/audit hooks; defer user-authored routines/triggers.**
5. **Channel/@mention graph-driven permission/search visibility and cross-surface auto-import → explicit org/workspace (+owner/membership) checks and no auto-ingest from external channels/email.**
6. **Attachment/PDF parsing, agent/search tools over indexed corpus, semantic/vector search → filename/title/body/description metadata indexing only (files: name/metadata as today), content extraction deferred.**

### DEFER / DROP for Seed (until stable + demand proven)

1. OpenSearch/Meilisearch-as-default, Kafka, SQS, LocalStack, multi-service search/indexer topology, DLQ/alarms complexity.
2. Content extraction/OCR/PDF parsing pipelines, large-attachment indexing, embedding/vector/semantic/hybrid retrieval.
3. Mobile/web push (SNS/APNs/FCM), email digests, notification preference matrices, cross-device delivery guarantees.
4. Workspace routines/triggers engine, agent-driven automations, MCP/search tool parity, call/agent-session indexing.
5. Global/eventual-consistency tuning for multi-node consumers, cross-region replication, advanced relevance/facets/autocomplete/typeahead SLAs.
6. Channel/@mention-driven auto-sharing, CRM/canvas/calendar/database graph indexing, PR/email ingestion coupling.

## 5. 018 implications (actionable, evidence-bounded)

Aligned with ROADMAP 018 prep: **contratos DB-gated/hardening do slice de arquivos e wiring essencial busca/notificações**.

| Implication | Concrete 018 direction | Acceptance intent / NV |
|---|---|---|
| **A. Search: make optionality real and Postgres-first path explicit** | Decide and codify search capability model: `disabled` vs `postgres` (target default) vs `meilisearch` (optional profile). Gate init/route behavior on config; no boot failure in lean core when external search absent. If PG FTS not yet implemented in 018, at minimum harden degraded/optional contract and wire indexing hooks behind an interface. | **NV:** current boot/config truth across all deployments; schema/migration design for FTS projection and multilingual tokenization (pt-BR/es/de/fr/zh-CN/ja) unbenchmarked. |
| **B. Search wiring for core mutations (minimal set)** | Add deterministic indexing/update calls (or outbox-lite) for highest-value entities already modeled: **file metadata (post-017), messages, tasks** — respecting org/workspace/tenancy. Prefer DB-derived fields; avoid content extraction. | Verify create/update/delete visibility semantics in tests (DB-gated where valuable). **NV:** performance/index bloat on 4 GB without measurement. |
| **C. Notifications: wire events → persisted in-app notifications** | Define a tiny canonical event→notification map for core mutations (e.g. task assignment/status change, message mention/reply semantics if modeled, file shared/upload relevant events — only where recipient/permission model is clear). Create notifications transactionally/post-commit with dedupe/idempotency key where feasible. | **NV:** recipient resolution rules and mention semantics may be underspecified; avoid inventing complex fanout. |
| **D. Delivery posture (VPS-safe)** | Keep Valkey/stream publish optional; prioritize poll/read correctness via DB APIs. If WS/push used, treat as best-effort cache of DB truth (Macro-like convergence principle, simpler mechanism). | **NV:** end-to-end client consumption/realtime delivery guarantees. |
| **E. Files slice hardening dependency (017 carry-over)** | 018 file-contract DB-gated tests and workspace/membership/quota coherence are prerequisites for safe search indexing/notification targets referencing files. Do not index/link entities whose tenancy contract is ambiguous. | Ties directly to ROADMAP P0.1/P0.2 ordering. |
| **F. Operability** | Any new index tables/triggers/jobs must be migration-managed, backup-visible (pg_dump scope), and resource-bounded (batch sizes, no unbounded background scans). No new mandatory services/containers. | **NV:** restore/load/OOM drills on 2 vCPU/4 GB. |

## 6. NVs (needs verification)

- **NV-1 Macro internals depth:** exact per-entity handler implementations for messages/tasks/docs/files in SPS/SQS workers, OpenSearch mappings/analyzers, and permission-filter details not fully retrieved from primary source files in this pass (403/partial access to some code pages); conclusions rely on service docs, topology, README, and secondary aligned synthesis.
- **NV-2 Multilingual primary coverage:** limited Macro-specific first-party evidence in pt-BR/es/de/fr/zh-CN/ja; zh-CN secondary article aligns with EN architecture claims but is not an authoritative first-party spec.
- **NV-3 Flora runtime truth:** actual boot behavior, feature flags, and all call sites for `search_service`/notification publishing across current branch/runtime profiles need repository-wide verification before implementation claims.
- **NV-4 Postgres FTS design choices:** projection schema (per-table vs unified), ranking/snippet quality, `pg_trgm` need, language configs for multilingual SMB content, and write amplification on cheap VPS — unbenchmarked.
- **NV-5 Notification semantics:** canonical recipient/mention/assignment rules, dedupe, retention/cleanup, and cross-entity permission checks for 018 wiring require product/contract decisions.
- **NV-6 Delivery/consistency:** in-process vs outbox transaction boundaries, failure/retry behavior, and WS/Valkey interaction under reconnect/multi-client scenarios remain NV.
- **NV-7 Performance/ops:** no load, OOM, index build time, or pg_dump/restore measurements for added search/notification paths on 2 vCPU/4 GB.

## 7. Sources

### Primary / high-signal (EN)
- [S1] Macro README — https://raw.githubusercontent.com/macro-inc/macro/main/README.md
- [S2] `search_processing_service` overview/architecture/delivery/backfill — https://upd.dev/macro-inc/macro/src/branch/main/services/search_processing_service
- [S3] Service topology — https://grok-wiki.com/public/docs/macro-inc/macro-bb988e1a448e/pages/08-service-topology.md
- [S4] Workspace events / routines — https://macro.com/docs/concepts/event-system
- [S5] Unified search / storage APIs — https://grok-wiki.com/public/docs/macro-inc-macro-bb988e1a448e/pages/17-storage-and-workspace-apis.md , https://docs.macro.com/AI/mcp/tools/content-search
- [S6] Self-host/local stack scope — https://www.aibrew.io/blog/macro-workspace-self-host-guide , https://macro.atomgit.com/faq.html , https://docs.macro.com/faq

### Multilingual / secondary signals
- [M1] pt-BR: Novidades IA, “Macro unifica workspace com IA compartilhada” — https://novidadesia.com/noticias/de-ferramentas-isoladas-a-um-sistema-unico-macro-unifica-workspace-com-ia-compartilhada/
- [M2] zh-CN: magformersworld.cn, Macro search pipeline synthesis (SPS/Kafka/OpenSearch/unified search) — http://m.magformersworld.cn/news/952615
- [M3] es/de/fr/ja: targeted queries executed; mostly general self-host/search/VPS/notification or non-Macro-primary results (triangulated via EN + prior Flora research) — **NV for Macro-specific primary detail**.

### Flora internal (cross-check)
- [F1] `OBJETIVO.md`, `ROADMAP.md`
- [F2] `docs/research/2026-11-16-017-macro-files-mvp-parity.md`, `docs/research/2026-11-16-016-search-postgres-fts-vs-meilisearch.md`
- [F3] `flora-search/src/lib.rs`, `flora-api/src/routes/search.rs`
- [F4] `flora-notifications/src/lib.rs`, `flora-api/src/routes/notifications.rs`

## 8. Bottom line

Macro’s search/indexing/eventing is a **cloud-scale, event-driven data platform** (Kafka/SQS/OpenSearch/extractors/backfills/many services) and its notifications are **queue/push-integrated** within a broad attention/routines system — powerful, but antithetical to Flora’s Seed/VPS constraints. Flora should copy only the **durable principles**: (1) **idempotent, source-of-truth-reconciled updates**, (2) **tenant-scoped unified search over core entities**, and (3) **simple persisted in-app notifications generated from core mutations**. For 018, the 80/20 move is **Postgres-first/optional-by-config search capability + minimal mutation→index wiring for messages/tasks/file metadata + minimal mutation→notification wiring**, all in-process, migration-backed, and verifiable with DB-gated/deterministic tests — deferring brokers, extractors, push, routines, and OpenSearch-grade infrastructure.