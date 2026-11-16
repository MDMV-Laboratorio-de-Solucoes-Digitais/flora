# 020 — Macro tasks, files/documents, and assignment/mention notification flows (multilingual primary-source research)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`dde602b3b`) — read-only research; this document only.  
**Task:** Primary-source analysis of `macro-inc/macro` for **task and file/document search indexing** and **task assignment/mention notification generation**: mutation triggers, data models/APIs/contracts, worker/queue vs sync behavior, and what is essential for an SMB MVP vs debloat/defer on a cheap VPS (Rust + PostgreSQL).

## 1. Sources and multilingual coverage

### Primary (EN, inspected directly)
- [S1] Repo/README and product/docs surfaces: https://github.com/macro-inc/macro , https://raw.githubusercontent.com/macro-inc/macro/main/README.md , https://raw.githubusercontent.com/macro-inc/macro/main/docs/agent-guide/tasks.md , https://macro.com/docs/product/search , https://macro.com/docs/concepts/mentions
- [S2] Search pipeline: https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/README.md , https://raw.githubusercontent.com/macro-inc/macro/main/docker/docker-compose.yml (`search_processing_service`, `document_upload_finalizer`, Kafka/DB/OpenSearch/lexical dependencies in stack)
- [S3] Notification architecture/service: https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/lib.rs , https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/domain.rs , https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/domain/ports.rs , https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/domain/models.rs , https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/domain/service/ingress.rs , https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/domain/service.rs (exported egress/ingress services), https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/inbound/worker.rs , https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/inbound/ingress_worker.rs , https://raw.githubusercontent.com/macro-inc/macro/main/services/notification_service/README.md
- [S4] Recipient-resolution/internal notification APIs in DSS: https://raw.githubusercontent.com/macro-inc/macro/main/services/document_storage_service/AGENTS.md , https://raw.githubusercontent.com/macro-inc/macro/main/services/document_storage_service/src/api/notification.rs , https://raw.githubusercontent.com/macro-inc/macro/main/services/document_storage_service/src/api/notification/get_document_notification_users.rs , https://raw.githubusercontent.com/macro-inc/macro/main/services/document_storage_service/src/api/notification/get_project_notification_users.rs

### Multilingual pass (requested: EN, pt-BR, es, de, fr, zh-CN, ja)
Targeted searches were run across all seven locales for Macro task/document/file search indexing and task assignment/@mention notification flows. **Actionable first-party technical detail was EN-only** (repo docs/code/compose). Non-EN hits were mostly generic search/notification architecture material or unrelated content, not Macro-specific mutation-to-index/notify internals. No non-EN primary contradiction found. Conclusions below cite EN primaries and mark uncertainties as **NV**.

### Method limits
Large monorepo; I did not exhaustively trace every producer call site (e.g. exact mention parsing, task-assignee mutation publishers, every SQS/Kafka topic for documents/chats/channels/files). Claims are labeled **evidenced**, **inferred**, or **NV**.

## 2. Task and file/document search indexing (Macro)

### 2.1 System shape (evidenced)
- **Dedicated pipeline, not inline request-path indexing:** `search_processing_service` (SPS) owns the search-event pipeline and indexes into **OpenSearch** [S2].
- **Ingress transports:** Kafka consumer (`search-processing-service`) for declared topics (evidenced: `macro.calls`, `macro.agent_session_lifecycle`) plus **legacy SQS workers** for remaining live entities; most backfills enqueue SQS, while agent sessions/properties backfill/index directly [S2].
- **Event semantics = invalidation/convergence:** for covered flows events are hints; handlers **reread current state from primary DB/owning services** and perform **idempotent full upserts/deletes** rather than trusting payload excerpts (explicit for calls/agent sessions) [S2].
- **File/document upload fanout in stack:** compose includes `document_upload_finalizer` (local analog of prod lambda) consuming upload events; lexical/text extraction and search-processing services are wired in the local stack, indicating object-created/finalization can feed downstream indexing [S2].
- **Entity coverage:** backfill HTTP routes exist for agent sessions, calls, chats, channels, documents, emails (`POST /internal/backfill/*`, tracked jobs). README states Kafka handlers/legacy SQS workers drain events for remaining entities [S2].

### 2.2 Mutation triggers and contracts (evidenced/inferred)
| Area | Evidenced contract/behavior | Trigger inference | Confidence |
|---|---|---|---|
| Calls | Kafka lifecycle events mapped: archived/updated/summarized → full upsert; deleted → remove; started/recording_ready ignored | Call domain mutations emit lifecycle events | Evidenced [S2] |
| Agent sessions | `macro.agent_session_lifecycle` invalidation; reread DB + folded transcript; rename/deletion converge; explicit repair/backfill | Session/prompt/response/rename/delete mutations emit lifecycle hints | Evidenced [S2] |
| Documents/files/tasks-as-docs | Documents have backfill; tasks are “documents under the hood” and appear in files/search surfaces; upload finalizer in stack | Create/update/delete/share/content-extraction-complete likely publish search upsert/delete or enqueue jobs consumed by SPS | **Inferred** (producer/event schema **NV**) [S1,S2] |
| Chats/channels | Backfill routes and “remaining entities” drained by Kafka/SQS workers | Message/channel mutations emit search events/jobs | **Inferred/NV** [S2] |

### 2.3 Consistency, reliability, and ops tradeoffs (evidenced)
- **Ordering:** per-entity sharded sequential workers in consumer instance; ordering best-effort (out-of-order publish/rebalance possible), handlers must converge [S2].
- **Commit-after-handoff:** offsets committed after in-memory handoff, before OpenSearch processing completes → loss window on crash/rebalance; 3 attempts, drops on exhaustion; malformed records committed to avoid partition wedging [S2].
- **Idempotency/repair:** idempotent overwrites/deletes, replica-lag-aware read choices for backfills vs workers in places, explicit repair/backfill endpoints and index provisioning/override discipline [S2].
- **Query surface implication:** Macro markets unified search across emails/messages/documents/tasks/calls and file contents indexed [S1]; implementation relies on external search/indexing infrastructure and extraction paths rather than PG-only search.

## 3. Task assignment and @mention notification generation (Macro)

### 3.1 Product/data-model signals (evidenced/inferred)
- **Tasks:** Linear-inspired, tightly integrated with channels/email/agents; tasks are documents and show up in files/lists/search; assignees/projects/status are first-class UI concepts [S1].
- **@mentions / graph:** bidirectional links and @mention primitive across surfaces (docs/tasks/channels/emails/companies/contacts, etc.); mentions drive sharing/permissions in some contexts and are central collaboration triggers [S1].
- **Concrete per-event notification rules:** exact mapping from “task assigned”, “user @mentioned in task/comment/doc/channel”, or “file shared/commented” to notification types/recipients is **NV** from fetched sources.

### 3.2 Notification generation, persistence, and delivery architecture (evidenced)
- **Hexagonal + queue-mediated split:** ingress receives/filters/persists/publishes; egress/delivery workers consume and deliver [S3,S4].
- **Ingress flow (`NotificationIngressService`):**
  - Filters recipients (remove sender, muted users, unsubscribed users, users with notification type disabled).
  - Persists notification + user-notification rows via repository (`create_notification`) with optional APNS collapse key; supports idempotent publication tracking (`complete_notification_publication`, withdrawn checks) and reuse of original recipients on retry.
  - Suppresses re-alerting for rows already acted on/deleted in specific states; builds channel messages for connection gateway, APNS/Android/mobile endpoints grouped per user (with exclusions/collapse handling), and per-recipient email messages.
  - Publishes to delivery queue; `SqsNotificationIngress` allows callers to enqueue type-erased ingress requests for async processing by worker [S3].
- **Ingress worker:** polls ingress queue, processes each message with timeout (15s), deletes on success, leaves failures for visibility-timeout retry [S3].
- **Egress worker:** polls delivery queue and delivers; failures logged, empty polls back off; also runs digest batching loop (claim/template/send) [S3].
- **Recipient resolution APIs (internal, DSS):** `GET /internal/notifications/document/{id}` and `/project/{id}` return user IDs from DB-backed notification-user queries (internal-auth only), showing recipient derivation is authoritative server-side data, not client-supplied fanout lists [S4].
- **Data/contracts (evidenced in models/ports):** notifications carry UUID, event type string, entity ref, sender, metadata, seen/done/viewed/deleted/sent state, collapse keys, per-user preference/mute/unsubscribe/type-disable, device endpoints (iOS/Android/VoIP), realtime status updates, digest eligibility/rate-limit hooks, and queue message envelopes for conn-gateway/mobile/email [S3].
- **Realtime/UI:** connection-gateway push and persisted queryable inbox/state are distinct concerns; stack wires `notification_service` with gateway/redis/db [S2,S3].

### 3.3 Sync vs async (evidenced)
- **Notification generation path:** can be synchronous from caller perspective into ingress service logic, but designed for **async handoff** via ingress queue and async delivery via egress queue/workers. Publication is tracked; retries/visibility timeouts are part of the contract [S3].
- **Search indexing:** async/event-driven and workerized (Kafka/SQS + SPS/OpenSearch) with explicit at-least-once-ish/duplicate-tolerant behavior and repair/backfill [S2].

## 4. Essential for SMB MVP (Flora 020) vs debloat/defer on cheap VPS (Rust + Postgres)

| Keep/adapt now (MVP, 80/20) | Defer/avoid (Macro-scale debloat) | Rationale |
|---|---|---|
| **Postgres-native search projections** for tasks/files/documents/messages (tsvector/GIN and/or `pg_trgm`, tenant-scoped) with deterministic upsert/delete on mutations | OpenSearch, Kafka/SQS, lexical/text-extraction fleet, index-alias choreography, multi-stage ingestion services | SPS proves invalidation+idempotent reread is the right *principle*; infra is wrong fit for 2 vCPU/4 GB VPS [S2] |
| **Mutation as source-of-truth reread:** derive index/doc/notification state from committed DB rows, not fragile event payloads | Cross-service stale-payload propagation, complex event schemas/versioning | Macro’s strongest reliability lesson is convergence from authoritative state [S2] |
| **Small, explicit notification rules** for high-value cases: direct task assignment/assignee change and explicit @mention to known users (if modeled), with self-actor suppression and permission/tenant checks | Broad activity fanout, digests, mobile/VoIP push, SES/email, multi-channel delivery matrices, sophisticated preference graphs | DSS recipient endpoints + notification filtering model show correctness depends on server-side recipient rules/preferences; keep scope minimal [S3,S4] |
| **Persist in-app notifications in Postgres** with idempotency/dedupe key (e.g. tenant+event+entity+recipient+actor/version), status (seen/done), and entity refs; optional best-effort WS publish | Separate ingress/egress microservice split, SNS, APNS/FCM, collapse-key infra beyond simple needs, digest state machines | Macro’s durable primitive is persisted, queryable, filterable notifications; delivery complexity is separable [S3] |
| **Simple post-commit hook or outbox table in same binary** (transactional outbox or job row polled by one lightweight worker) for bounded async side effects | Always-on distributed consumers, commit-after-handoff loss windows by design, multi-queue topologies | Need bounded latency/failure isolation without Macro ops burden [S2,S3] |
| **Repair path:** admin/reindex command scoped by org/entity/time and notification backfill/reconcile for missed deterministic rules | Full replay pipelines, lag dashboards/consumer-group ops, index migration fleet tooling | Macro invests heavily in repair because async systems drop/duplicate; MVP still needs cheap recovery [S2] |
| **Indexed recipient/query paths:** tenant + recipient + state/time indexes for inbox; entity/task/file indexes for lookup | Unindexed JSON/scan-heavy hot paths at growth | PR-level lesson class: indexed metadata matters [S1–S4 context] |

## 5. Flora 020 concrete takeaways (keep/adapt/defer)

1. **Keep (principle):** treat mutations as invalidation signals and recompute search/notification projections from canonical DB state; make writes idempotent and delete-aware. [S2]
2. **Adapt (tasks/files):** for task create/update/assign/delete and file/document upload/rename/delete, update a single Postgres search record per `(org_id, entity_type, entity_id)` in the same transaction when cheap, else via outbox/job; extract/index only metadata/text you already have (no OCR/parser fleet). [S1,S2]
3. **Adapt (assignment/mention):** generate notifications from resolved recipients only: for task assignment, notify assignee(s) excluding actor and unauthorized users; for @mentions, notify mentioned principals after parsing stored canonical content/refs, not raw client claims. Persist first, push realtime second/best-effort. [S1,S3,S4]
4. **Defer:** Kafka, OpenSearch, SQS, separate notification/search services, digests, push/email/mobile, complex fanout graphs, and multi-tenant cross-service event buses. [S2,S3]
5. **Contract to define now:** eventual-consistency SLA (same-request vs async), retry bounds, dedupe keys, and a manual reconcile command so “missed side effect” is recoverable, not silent. [S2,S3]

## 6. Risks

- **R1 Trigger ambiguity:** Macro’s exact mention/assignment producers and payload schemas are **NV**; copying guessed semantics can create noisy/wrong notifications.
- **R2 Permission leakage:** mentions/assignments in shared docs/tasks require strict recipient authorization; naive fanout can expose entities or spam users. [S1,S4]
- **R3 Dual-write/inconsistency:** if search/notification side effects run outside the core mutation transaction without outbox/idempotency, crashes create drift. [S2]
- **R4 VPS resource pressure:** Postgres FTS/trgm indexes, notification tables, and worker polling can contend on 4 GB RAM/IO if unbounded. [S2 inferred to Flora context]
- **R5 Content extraction gap:** Macro’s file-content search implies extraction pipelines; Flora MVP without extraction will under-index binary/rich files (set expectations).
- **R6 Overfitting to Macro UI semantics:** tasks-as-documents, projects, graph mentions are richer than Flora 020 may need; implement only explicit, testable rules.

## 7. NVs (needs verification)

- **NV-1** Exact Macro producers/topics/queues/payloads for document/chat/channel/file/task search mutations beyond evidenced call/agent-session cases. [S2]
- **NV-2** Exact business logic generating task-assignment and @mention notifications (types, recipient expansion, mention parsing source of truth, dedupe/collapse rules per event). [S1,S3]
- **NV-3** OpenSearch/document schemas and DB schemas for notifications/search projections in Macro (field-level). [S2,S3]
- **NV-4** End-to-end delivery guarantees, DLQ/retry policies, and failure handling across ingress/egress/realtime/email in production. [S3]
- **NV-5** Flora 020 code touchpoints/transaction boundaries for safe post-commit hooks or outbox in current `flora-api`/`flora-core`/`flora-search`/`flora-notifications` paths.
- **NV-6** Multilingual search/tokenization quality for pt-BR/es/de/fr/zh-CN/ja in Postgres for Flora content. 
- **NV-7** Throughput/latency of PG-only indexing + notification writes on target VPS workload.

## 8. Bottom line
Macro shows a **high-scale, async, idempotent, repairable** design: externalized search indexing (Kafka/SQS/OpenSearch/SPS) and queue-split notification ingress/egress with server-side recipient filtering, persistence, and multi-channel delivery. For Flora 020 on a cheap VPS, emulate only the durable invariants—**mutations derive tenant-scoped search and notification state from committed DB truth, idempotently, with explicit small rules and a repair path**—and defer Macro’s distributed indexing/delivery infrastructure entirely.
