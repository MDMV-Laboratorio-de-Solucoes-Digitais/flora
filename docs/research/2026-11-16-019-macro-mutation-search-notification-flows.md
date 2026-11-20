# 019 — Macro mutation → search indexing & notification generation flows (multilingual primary-source research)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`3c8eddbfd`) — read-only research; this document only.  
**Task:** Understand how Macro (`macro-inc/macro`) triggers **search indexing** and **notification generation** from mutations (messages/channels, tasks, files/documents; adjacent email/calls/agent sessions where evidenced), including event flow, data models/APIs, and background workers/queues vs sync — then derive minimal 80/20 guidance for Flora 019 on a cheap VPS (Rust + PostgreSQL, optional components).

## 1. Sources, method, and multilingual coverage

**Primary/high-signal (EN, public):**
- [S1] Macro repo overview/README: https://github.com/macro-inc/macro , https://raw.githubusercontent.com/macro-inc/macro/main/README.md
- [S2] `services/search_processing_service/README.md` (ingestion ownership, Kafka/SQS, invalidation-hint/reread model, idempotency, backfills, delivery/commit semantics, per-entity routes): https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/README.md , https://github.com/macro-inc/macro/tree/main/services/search_processing_service
- [S3] `docker/docker-compose.yml` (service topology: `search_processing_service`, `notification_service`, `document_upload_finalizer`, `email_pubsub_workers`, `connection_gateway`, lexical/sync dependencies): https://github.com/macro-inc/macro/blob/dd1eee50f222f47287a6f9afb48feb79a86c4e53/docker/docker-compose.yml
- [S4] Notification crate/service structure (hexagonal: domain ports/service, ingress/egress workers, queue/repository delivery split): https://github.com/macro-inc/macro/tree/main/crates/notification , https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/lib.rs , https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/domain.rs , `.../domain/service.rs` (`NotificationIngressService` filter/persist/publish-to-queue; `NotificationEgressService` consume-and-deliver; ingress worker polls ingress queue; delivery worker polls/delivers with retries/digests), https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/inbound/worker.rs , `.../inbound/ingress_worker.rs` , https://raw.githubusercontent.com/macro-inc/macro/main/services/notification_service/README.md
- [S5] Perf/behavior signals from PRs/releases (notification lookup/indexing on `notification`/metadata, channel-message notification query path `get_filtered_entity_notifications`; email focus worker consuming stream + sweep; routines on events/webhooks; GraphQL notification/entity surfaces): https://github.com/macro-inc/macro/pull/8889 , https://github.com/macro-inc/macro/pull/7880 , https://github.com/macro-inc/macro/pull/8730 , https://github.com/macro-inc/macro/pull/4978 , https://github.com/macro-inc/macro/pull/5074 , https://github.com/macro-inc/macro/releases

**Flora cross-check (internal):** `OBJETIVO.md`, `ROADMAP.md`, prior 018 research/docs in `docs/research/2026-11-16-018-*`, current `flora-search/`, `flora-notifications/`, API routes/services (not re-audited deeply here; 019 focuses on Macro evidence and Flora takeaways).

**Multilingual pass (requested): EN, pt-BR, es, de, fr, zh-CN, ja.** Targeted web searches were run in all seven locales for Macro search indexing/notification/event/mutation flows. **Result:** actionable Macro-specific primary technical detail was predominantly **EN** (repo docs, compose, PRs). Non-EN results were mostly general self-host/search/queue/notification guidance or product-level overviews, not first-party Macro mutation-to-index/notification internals; prior 018 note of zh-CN secondary synthesis of SPS/Kafka→OpenSearch invalidation flow remains directionally consistent with [S2] but is **not first-party**. No locale-primary contradiction found; conclusions below are anchored to EN primary sources and marked **NV** where implementation specifics are not publicly explicit.

**Limits/NV:** I did not inspect every producer call site inside each mutating service crate (large monorepo; some internals not fetched in this pass). Claims distinguish: **evidenced** (docs/compose/PR/code snippets read), **inferred** (consistent with evidenced architecture), **NV** (not verified).

## 2. Macro architecture: search indexing triggered by mutations

### 2.1 High-level model (evidenced)
Macro separates **mutation/domain services** from a dedicated **search processing** pipeline (`search_processing_service`, SPS). SPS indexes into **OpenSearch** and is explicitly described as owning the search-event pipeline ([S2]):
- **Live ingestion:** Kafka consumer (durable group `search-processing-service`) + legacy SQS workers for remaining entities.
- **Event semantics:** events are **invalidation hints** for agent sessions/calls-style flows — handlers reread current state from primary DB/owning services and perform **idempotent full upserts/deletes** (never trust event payload excerpt as final truth for covered cases). Call lifecycle mapping evidenced: archived/updated/summarized → full upsert; deleted → remove; started/recording_ready ignored ([S2]).
- **Backfills/repair:** internal HTTP endpoints (`/internal/backfill/*`) create tracked jobs (most enqueue SQS; some direct index like agent sessions/properties), used for rollout recovery, missed history, or dropped events ([S2]).
- **Index ops discipline:** aliases/mappings provisioned/verified manually before certain consumers; explicit index override for repair/backfill; no silent index creation in normal path for some flows ([S2]).

### 2.2 Transport, ordering, and reliability (evidenced)
- **Kafka:** declared subscriptions in consumer (e.g. `macro.calls`, `macro.agent_session_lifecycle`); partitioned processing sharded by entity ordering key across sequential workers with bounded channels. Ordering is **best-effort per key within consumer instance**, not globally guaranteed (out-of-order publish/rebalance possible); handlers must converge.
- **Commit/delivery tradeoff:** offset committed after in-memory handoff, **before** OpenSearch processing completes → loss window on crash/rebalance; retries (3 attempts) and dropped-after-exhaustion with logging; poison/malformed records committed to avoid partition wedging. Duplicate delivery possible/safe due to idempotent writes/deletes ([S2]).
- **SQS:** retained for legacy producers and many backfills/other live entities; shared queues (e.g. `search-event-queue-dev` noted) can interleave ingest and backfill; read-replica use for backfill reads when configured, primary for queue workers to avoid replica lag misses ([S2], [S3]).

### 2.3 Entity coverage and mutation-to-index implications

| Entity class | Evidenced path | Mutation trigger shape (evidenced/inferred/NV) |
|---|---|---|
| Calls | Kafka `macro.calls` → SPS upsert/delete; backfill endpoint | **Evidenced:** lifecycle events (`record_archived/updated/summarized/deleted`) trigger search actions ([S2]). |
| Agent sessions | Kafka `macro.agent_session_lifecycle` → reread DB+folded transcript → upsert/remove; direct backfill | **Evidenced:** invalidation events converge on persisted state, not excerpts ([S2]). |
| Chats / Channels / Documents / Emails | Backfill routes exist (`/internal/backfill/chats|channels|documents|emails`); README states “Kafka handlers and legacy SQS workers drain events for the remaining search entities” | **Inferred:** mutations in owning services publish/enqueue search events/jobs consumed by SPS or related workers; exact topic/queue names and per-mutation payloads for messages/channels/tasks/files are **NV** from fetched sources. Document parsing/extraction dependencies implied by compose/README ecosystem (lexical, text extractor/finalizer paths referenced around stack). |
| Files/uploads | Compose includes `document_upload_finalizer` worker (local analog of prod lambda) consuming upload events; stack historically fans out object-created processing to search/text extraction | **Inferred/evidenced in stack:** object/storage events participate in indexing pipeline for file/document content/metadata; concrete mutation contracts **NV**. |
| Tasks | Tasks are document/task entities in Macro product model (docs/PR context) and documents have backfill/indexing surfaces | **Inferred:** task mutations likely flow through document/task search indexing paths; exact producer/event **NV**. |

**Takeaway:** Macro optimizes for **eventual consistency, idempotent convergence, and operability at scale** (replay/backfill/repair) over synchronous in-request indexing.

## 3. Macro architecture: notification generation and delivery

### 3.1 Service and processing model (evidenced)
`notification_service`/`crates/notification` follow a **hexagonal, queue-mediated** design ([S3],[S4]):
- **Ingress (generation/request path):** callers submit notification work; `NotificationIngressService` handles validation/filtering/rate-limit concerns, persistence, and publishing to downstream queue(s) (ports: repository, notification queue, digest/state machine, ingress queue). `IngressWorker` continuously consumes ingress queue messages, calls `process_from_queue`, deletes on success, leaves failures for retry/visibility timeout, and bounds per-message processing time.
- **Egress/delivery (worker path):** `NotificationWorker`/`NotificationEgressService` polls delivery queue and delivers via outbound adapters; failures logged, empty polls back off. Supports digest batching/scheduling and other channels (ports/adapters for WebSocket gateway, SNS/push, SES/email implied by crate layout and service env wiring in compose: gateway URL, redis/last-online, DB).
- **Data/query behavior:** notifications are persisted in DB (large table in prod evidenced by [S5]/PR 8889), queried for GraphQL/entity notification edges; metadata JSON fields used for entity/message correlation (e.g. channel message lookup optimization), showing notification records carry typed/event/entity metadata beyond simple user/message rows (**specific schema fields NV beyond PR evidence**).

### 3.2 What triggers notification generation? (evidence-bounded)
- **Product/domain events produce notification intents:** mentions, channel messages, task/activity, reminders, routines, email/focus signals etc. appear across PRs/features ([S5] routines trigger on GitHub/webhook events; channel/thread mute, reminders, unread/focus flows). **Exact mutation-hook code paths per entity (message send, task assign, file share/comment) were not fully traced in this pass — NV.**
- **Async fanout favored:** queue-based ingress/egress decouples user-facing mutations from delivery fanout (push/websocket/email/digest), matching large multi-surface system needs.
- **Realtime/UI invalidation is related but separate:** client cache invalidation/refetch and gateway nudges exist in broader system; notification truth is server-persisted and queried/streamed via service APIs/WS.

### 3.3 Data model/API surface (partially evidenced)
- **Internal model:** domain models/ports for send, read, preferences/mutes, digests, device/push/voip/realtime consumers ([S4] modules).
- **APIs:** notification service exposes HTTP/health/internal surfaces in stack; GraphQL/entity interfaces expose notifications on entities and filtered notification queries (e.g. `get_filtered_entity_notifications` path optimized in PR 8889). User-facing inbox/notification state includes done/seen/mute semantics and entity references ([S5]).

## 4. Concrete takeaways for Flora 019 (minimal emulation vs avoid)

| Macro principle/mechanism | What to **emulate minimally** in Flora 019 (VPS, PG-first) | What to **avoid/defer** |
|---|---|---|
| **Event as invalidation hint; converge from source of truth** ([S2]) | On core mutations (messages/tasks/files metadata), trigger **deterministic reindex/notification derivation from current DB row/state** (not fragile event payload copies). Prefer idempotent upsert/delete semantics by `(org_id, entity_type, entity_id)` where applicable. | Cross-service stale-payload indexing, complex event versioning, distributed event sourcing. |
| **Async decoupling for heavy work** ([S2],[S4]) | Keep hot path cheap: commit core write, then **in-process post-commit hook or tiny outbox/job table** for indexing/notification side effects (single binary). For 019, synchronous best-effort or same-request-but-failure-isolated may be acceptable if bounded and tested. | Kafka, MSK, SQS, multiple workers/Lambdas, commit-after-handoff loss windows as a design choice. |
| **Idempotent upserts, explicit deletes, repair/backfill** ([S2]) | Add a **bounded reindex/reconcile command** from Postgres (per org/entity) and idempotent notification creation keys/dedupe for obvious duplicates. | Always-on stream replay ops, alias choreography, OpenSearch mapping fleet management. |
| **Separation of generation and delivery concerns** ([S4]) | Separate **"derive notification rows"** (deterministic rules) from **"deliver realtime/push"**. 019: persist in-app notifications in Postgres + existing/cheap read APIs; treat WS/Valkey fanout as optional best-effort. | Ingress/egress queue split, SNS/SES/mobile push, digest pipelines, per-device presence/rate-limit infrastructure unless already trivial. |
| **Tenant/entity-aware queryability** ([S5]) | Notifications and search docs/rows must carry `organization_id` + entity refs/targets and be queryable with scoped indexes (PG indexes/FTS) to avoid scans as data grows. | JSON-only unindexed lookup patterns on hot paths (learn from PR 8889 symptom class). |
| **Unified but scoped search surface** | One search contract over core Seed entities (messages/tasks/files) with type filters and tenant isolation; Postgres FTS/`pg_trgm` as default per prior 018 direction. | OpenSearch, content extractors/PDF parsing/OCR, semantic/vector search, multi-engine routing. |
| **Backpressure/operability honesty** ([S2]) | Document failure modes: side effects best-effort vs required, retry bounds, observability/logging, and repair path. | Hidden at-least-once complexity without operator tooling. |

### Suggested 019 implementation posture (actionable)
1. **Mutation hooks:** add a small internal dispatcher called from message/task/file mutation service paths after successful DB commit (or with explicit transaction strategy) producing: (a) search projection update (PG FTS row/tsvector refresh) and (b) notification rows for a tiny, well-defined rule set.
2. **Notification rules (minimal):** start with 1–2 high-value, recipient-clear cases (e.g. task assignment/state relevant to assignee, or direct reply/mention if model exists) with dedupe key and self-actor suppression.
3. **Search indexing (minimal):** index/update/delete for `messages`, `tasks`, `files` metadata on create/update/delete using Postgres-native structures; no external engine dependency for Seed.
4. **No new mandatory infra:** no Kafka/SQS/OpenSearch/Lambda workers; optional Valkey/WS only if already present and best-effort.
5. **Deterministic verification:** DB-gated tests for mutation→search visibility and mutation→notification row creation/visibility + tenant isolation.

## 5. Risks

- **R1 Correctness coupling:** synchronous hooks can fail or add latency; if notification/search is best-effort, user-visible state may lag or miss without repair — must define contract explicitly.
- **R2 Transaction boundaries:** deriving notifications/search from pre-commit vs post-commit state can cause ghosts/misses on rollback or partial failure (**NV** in Flora’s preferred pattern).
- **R3 Recipient/permission ambiguity:** Macro benefits from richer graph/permission models; naive fanout can leak or spam. Keep 019 rules to clearly authorized recipients.
- **R4 Dedupe/idempotency gaps:** retries or repeated hooks may create duplicate notifications or stale search rows without stable keys/upserts.
- **R5 Query/index performance on 4 GB VPS:** FTS/trgm indexes and notification list queries can bloat RAM/IO; need limits, scoped indexes, and later measurement (**NV**).
- **R6 Over-copying Macro scale patterns:** adopting queues/workers prematurely increases ops burden contrary to OBJETIVO/ROADMAP Seed posture.

## 6. NVs (needs verification)

- **NV-1 Producer specifics:** exact Macro mutation publishers for messages/channels/tasks/documents/files (topic names, SQS payload schemas, call sites) not exhaustively confirmed.
- **NV-2 Notification rule engine:** where/how business rules map mutations to recipients/templates/channels inside Macro (beyond queue/service skeleton) not fully traced.
- **NV-3 DB schemas:** full `notification`, search index/document schemas and migrations in Macro/OpenSearch mappings not inspected.
- **NV-4 Delivery guarantees:** end-to-end guarantees (in-app/WS/push/email), retry/DLQ policies, and consistency under multi-region/multi-instance not fully documented here.
- **NV-5 Flora integration points:** precise post-017 transaction/service boundaries and safest hook locations in current `flora` codebase require code-level 019 design pass.
- **NV-6 Multilingual content quality:** PG FTS tokenization/ranking for pt-BR/es/de/fr/zh-CN/ja in Flora workloads unvalidated.
- **NV-7 Performance/operability:** no benchmarks for proposed hooks/indexes on 2 vCPU/4 GB.

## 7. Bottom line

Macro evidences a **queue/event-driven, idempotent, repairable** split between mutations and search/notification pipelines (Kafka/SQS/SPS/OpenSearch; ingress+egress notification workers) suited to a large multi-surface product. Flora 019 should **not** replicate that infrastructure; it should adopt only the durable primitives: **mutations emit deterministic, idempotent, tenant-scoped updates derived from source-of-truth DB state** into **Postgres-backed search projections** and **persisted in-app notifications**, with bounded best-effort delivery hooks and a simple repair/reindex path. This preserves 80% of SMB value at a fraction of Macro’s operational complexity.
