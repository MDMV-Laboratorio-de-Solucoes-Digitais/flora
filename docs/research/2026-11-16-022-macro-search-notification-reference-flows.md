# 022 — Macro reference flows: search indexing lifecycle/multi-entity parity and task assignment notifications/idempotency (80/20 for Flora)

**Date:** 2026-11-16  
**Branch/context:** `dev@af0d54170` (read-only research; this document only)  
**Objective:** Extract primary-evidence patterns from `macro-inc/macro` for Flora 022 (self-hosted SMB alternative, Rust + PostgreSQL, cheap VPS): (1) search indexing lifecycle and multi-entity parity (messages/tasks/files/docs), delete/soft-delete/update propagation, rebuild/backfill, tenant/org scoping, and API search contracts; (2) task assignment/unassignment/reassignment notifications and event/idempotency semantics. Output focuses on transferable **80/20 keep/adapt** vs **heavy/complex defer/drop** (OpenSearch/Kafka/SQS/multi-service extraction/fanout).

## 1. Method, sources, and confidence

**Method:** Read local Flora context (`AGENTS.md`, prior research `docs/research/2026-11-16-018/019/021-macro-*` for continuity and gap-check), performed EN/pt-BR/es/de/fr/zh-CN/ja web searches for Macro-specific evidence, and collected primary artifacts from the public Macro repo (`main`) via `raw.githubusercontent.com` / GitHub API (code, READMEs, service internals). Conclusions prioritize primary code/docs; where only secondary synthesis or partial snippets were available, marked **NV**.

**Primary evidence (EN, highest confidence):**
- [P1] `services/search_processing_service/README.md` — pipeline ownership, live Kafka + legacy SQS + backfills, invalidation-hint/reread model, idempotent upsert/delete, agent-session rollout/repair, call event mapping, delivery/commit/retry semantics, backfill HTTP routes and ops. https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/README.md
- [P2] `services/search_processing_service/src/inbound/kafka_consumer.rs` (+ module listing) — declared topics/subscriptions, sharding by ordering key, bounded workers, commit-after-handoff, retries/exhausted-drop, processing outcomes. https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/src/inbound/kafka_consumer.rs ; process modules index: https://api.github.com/repos/macro-inc/macro/contents/services/search_processing_service/src/process
- [P3] Document search processing: `process/document.rs`, `inbound/kafka_consumer/document.rs` — event→action mapping (`Created/Deleted/Copied/Interaction` ignored for search action, `Updated` refreshes name only when name present, `ContentUploaded` extract text, `SyncContentUpdated` extract sync, `Purged` remove), soft-deleted/missing doc handling for name refresh, extraction from S3/lexical/db. https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/src/process/document.rs ; https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/src/inbound/kafka_consumer/document.rs
- [P4] OpenSearch delete semantics (documents): delete by `entity_id` query and delete all docs for `owner_id` (parents + child chunks via `has_parent`), refresh true. https://raw.githubusercontent.com/macro-inc/macro/main/crates/opensearch_client/src/delete/document.rs
- [P5] Unified search contract/args: multi-entity composition (`agent_session`, `document`, `email`, `channel_message`, `chat`, `call_record`, `project`, `calendar_event`), per-entity args/filters/modes, shared paging/collapse/cursor/ids-only concepts, conversion to entity-specific search args. https://raw.githubusercontent.com/macro-inc/macro/main/crates/opensearch_client/src/search/unified.rs ; hit/model helpers: https://raw.githubusercontent.com/macro-inc/macro/main/crates/opensearch_client/src/search/model.rs
- [P6] Backfill API surface: internal authenticated per-entity POST endpoints (`agent-sessions`, `calls`, `chats`, `channels`, `documents`, `emails`, `properties`, `projects`, `calendar-events`) spawning tracked jobs, 202 + job id, poll status. https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/src/api/internal/backfill.rs ; router mount: https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/src/api/internal.rs
- [P7] Notification ingress/idempotency primitives: recipient filtering (sender/muted/unsubscribed/type-disabled), DB create with collapse key, idempotent create returns new/unpublished rows or `None` after publication/withdrawal preserving original recipients on retry, suppress re-alert for acted-on/deleted rows, complete publication, queue publish with state decisions. https://raw.githubusercontent.com/macro-inc/macro/main/crates/notification/src/domain/service/ingress.rs
- [P8] Task assignment (agent/session trigger path): assignment derived only from newly added bot assignees on Assignees property updates (set-difference vs previous; human/non-bot refs ignored; other properties/unchanged saves/removals ignored), replay-stable discussion/session identity per (`event_id`,`bot_id`), authorize + optional session-exists short-circuit + discussion/message + publish `AgentAssignedToTaskEvent`; assignment instructions explicitly tell agent not to post discussion progress unless asked. https://raw.githubusercontent.com/macro-inc/macro/main/crates/agent_trigger/src/domain/task_assignment.rs
- [P9] Notification event taxonomy includes `TaskAssigned` metadata variant (user-facing task assignment notification type exists in model layer). https://raw.githubusercontent.com/macro-inc/macro/main/crates/model_notifications/src/lib.rs

**Supporting/secondary (lower confidence):**
- [S1] Macro README, docs, topology/wiki, commit/PR traces surfaced via web search for context (multi-surface product, queues/services). https://github.com/macro-inc/macro , https://macro.com/docs/product/search , https://macro.com/docs/concepts/event-system
- [S2] zh-CN synthesis of SPS architecture (aligned with P1/P2 but not first-party). http://m.magformersworld.cn/news/952615

**Multilingual coverage (requested):** searches/issues/docs queries attempted in EN, pt-BR, es, de, fr, zh-CN, ja. **Finding:** Macro-specific primary technical details for these flows are overwhelmingly EN (repo code/docs). No non-EN first-party source contradicting P1–P9 was found. Non-EN results were mostly generic VPS/search/notification guidance or high-level overviews (**low incremental confidence**; conclusions anchored to EN primaries).

**Confidence legend:** **High** = directly evidenced in fetched primary code/docs; **Medium** = inferred from multiple primary signals but not fully traced end-to-end; **Low/NV** = not verified in this pass.

## 2. Search indexing lifecycle and multi-entity parity (Macro)

### 2.1 Lifecycle and ingestion model (High)

| Stage | Evidence-backed behavior | Notes |
|---|---|---|
| **Live ingestion** | SPS is hexagonal owner of search-event pipeline. Kafka consumer (durable group `search-processing-service`) handles call lifecycle (`macro.calls`) and agent-session lifecycle (`macro.agent_session_lifecycle`); “other live entities” drained by Kafka handlers and legacy SQS workers. [P1,P2] | Polyglot transport by entity maturity/path. |
| **Event semantics** | Events are **invalidation hints** (esp. agent sessions/calls): handlers reread current primary state/services and write idempotent full upserts/deletes; event payload excerpts not treated as source of truth where stated. [P1,P3] | Convergence over payload trust. |
| **Processing shape** | Poll loop shards decoded events by **ordering key** across sequential workers with bounded channels; commit offset after in-memory handoff (before processing completes). Retries (3 attempts, backoff) in-process; malformed/keyless/unsupported committed to avoid partition wedge; exhausted events logged/dropped. [P2] | Throughput/concurrency bounded; durability traded for progress. |
| **Index provisioning** | Some indices/aliases must be provisioned/verified manually before deploy/consume (e.g. agent_sessions join mapping/alias); normal writes may require alias and fail fast instead of auto-creating index; explicit `index_override` allowed for repair/backfill targets. [P1,P6] | Operational guardrails, not auto-magic. |

### 2.2 Update/delete/soft-delete propagation (High/Medium)

| Case | Evidence | Confidence |
|---|---|---|
| **Call lifecycle** | `record_archived/updated/summarized` → full upsert from Postgres/current state; `record_deleted` → remove by call id; `call.started`/`recording_ready` ignored. [P1] | High |
| **Agent sessions** | Any lifecycle event rereads metadata + effective ACP log via owning services; upsert parent+folded messages, refresh, remove older projection generations; missing session deletes entire projection (including when triggered by older event). Per-session advisory lease held through OpenSearch write. [P1] | High |
| **Documents (generic content docs)** | Kafka document actions: `ContentUploaded` → extract text/index path, `SyncContentUpdated` → extract sync path, `Updated` with name → refresh denormalized `document_name` from DB, `Purged` → remove; `Created/Deleted/Copied/Interaction` and name-less updates ignored for search action. [P3] | High |
| **Rename/name convergence** | Name refresh fetches basic document from DB (source of truth); `RowNotFound` or `deleted_at` set => no-op/skip (remove path owns deletions; not-yet-indexed will converge on first full index). [P3] | High |
| **Delete primitives** | `delete_document_by_id`: delete-by-query on `entity_id` (index or override). `delete_document_by_owner_id`: deletes parents with matching `owner_id` and child chunks via `has_parent`. [P4] | High |
| **Soft-delete semantics** | Explicit soft-delete field handling observed in name-refresh path (`deleted_at => skip`); purge event maps to remove. Full cross-entity soft-delete policy (tombstone vs hard delete timing) **NV**. [P3] | Medium/NV |
| **Out-of-order tolerance** | Idempotent full overwrites, DB-reread on invalidation, per-key ordering best-effort only (publish/rebalance/worker races acknowledged), handlers must converge. [P1,P2] | High |

### 2.3 Multi-entity parity and API search contracts (High/Medium)

| Aspect | Evidence | Confidence |
|---|---|---|
| **Unified entity surface** | `UnifiedSearchArgs` composes: agent sessions, documents, emails, channel messages, chats, call records, projects, calendar events; each mapped to entity-specific search args with shared `user_id/page/page_size/match_type/collapse/cursor/search_indices`. [P5] | High |
| **Per-entity capabilities/filters** | Rich entity-specific filters/modes (e.g. doc ids/sub_types/mode/property/tag filters; email thread/link/sender/labels/subject-only; channel ids/thread/mentions/senders; chat ids/role; call ids/channel/speakers/mode; project ids/tags; calendar ids/links/status/organizer/attendee). Some entities forced `ids_only` in unified mapping (e.g. channel messages) while others configurable. [P5] | High |
| **Parity style** | One query composition/contract over heterogeneous entities; parity expressed as unified entrypoint + explicit per-entity boundaries (not identical ranking/features per type). [P1,P5,S1] | Medium |
| **Result identity/disambiguation** | Search hit model tracks `matched_queries` and avoids relying solely on untagged source shape/index-name conventions for entity typing (per unified/model comments). [P5] | High |
| **Tenant/org scoping** | Unified args include `user_id` and access-relevant filters (`link_ids`, `channel_ids`, etc.); document deletes include owner-scoped operation; docs/wiki state permission filtering/enrichment. **No explicit `organization_id` field observed in fetched snippets** — Macro’s isolation appears user/workspace/permission-model based rather than Flora-style org-id scoping (**NV on exact enforcement layers**). [P1,P4,P5] | Medium/NV |

### 2.4 Rebuild/backfill (High)

- Internal HTTP backfills per entity: agent-sessions, calls, chats, channels, documents, emails, properties, projects, calendar-events; `202 Accepted` + `job_id`, poll `GET /internal/backfill/{job_id}`; jobs tracked in shared state (DynamoDB noted in handler comments), spawned async with progress/cancel. [P1,P6]
- **Modes:** most enqueue SQS; agent sessions/properties index directly. Targeted repair supported (e.g. specific IDs, `index_override` for scratch/replacement indices in some flows). Full scans may be bounded/page-walked and can have discovery limits (e.g. cannot discover already-deleted IDs not present). [P1]
- **Operational caveats:** interleaving with live queues in dev/shared environments, mapping/index readiness required before backfill in some changes, retained stream history not a complete historical backfill guarantee. [P1]

## 3. Task assignment/unassignment/reassignment notifications and event/idempotency semantics (Macro)

### 3.1 Assignment detection and trigger semantics (High, agent-assignment path)

From [P8] `agent_trigger::domain::task_assignment`:
- **Eligible trigger:** `EntityPropertyUpdated` for task entity where property is **Assignees** system property.
- **Delta computation:** `assigned_bots = current_bot_refs - previous_bot_refs` (bots only, extracted from entity refs of type User parsed as bot IDs). If empty → no assignment. **Removals, unchanged saves, human-only assignees, and other properties are ignored.**
- **Actor/origin:** actor resolved from `actor_user_id` or `on_behalf_of`; origin records whether person or agent acted and whether inherited from project.
- **Per-bot handling:** for each newly added bot, authorize task assignment, derive **replay-stable `discussion_id = UUIDv7(event timestamp preserved) with hash(event_id, bot_id)` in random bits**, skip if assignment/session already exists for `(discussion_id, bot_id)`, load task brief via owning service/access receipt, create/link discussion, publish `AgentAssignedToTaskEvent` (session trigger) with prompt/context.

**Implications:**
- **Assignment:** fires on *newly added bot* only.
- **Unassignment:** represented by removal from assignee set → not in added delta → **no agent-assignment trigger evidenced**.
- **Reassignment:** removing bot A and adding bot B produces add of B (triggers for B). Re-adding same bot A in a *later* property update event yields a different `event_id` and thus a different derived `discussion_id`/session identity (new session flow), subject to authorization/state checks — effectively a new assignment occurrence, not deduped to prior historical assignment except by current system/session existence checks. [P8]
- **Idempotency/replay safety:** deterministic discussion identity per (source event, bot) plus precondition checks (`assignment_has_session`) provide replay protection for the same event; cross-event dedupe is not the goal because each event represents a distinct change instance. [P8]

### 3.2 User-facing notification primitives and idempotency (High)

Macro’s notification domain/service shows general publication-grade safeguards [P7,P9]:
- Ingress filters: exclude sender, muted, unsubscribed, and users with notification type disabled.
- Persistence before publish; `create_notification(..., notification_id, apns_collapse_key)` returns **new/unpublished rows or `None` after publication/withdrawal**, preserving original recipients on retry.
- Post-create filtering suppresses re-alert for rows already acted on/deleted (`seen`/deleted state checks in code path shown); `complete_notification_publication` marks success; queue publish includes digest/state decisions and timestamps.
- `NotifEvent` includes `TaskAssigned(TaskAssignedMetadata)` (and many other event types), indicating task assignment is a modeled notification class in the broader system. [P9]

**Critical NV:** the exact producer path that emits `TaskAssigned` user notifications on task assignee mutations (and whether unassign/reassign generate distinct notification events) was **not fully traced** in fetched files. The evidenced agent-trigger path [P8] is a session/orchestration flow and should not be conflated with in-app notification fanout without verification.

### 3.3 Ordering, retries, and consistency posture (High)

- Search/event side: at-least-once delivery characteristics with possible duplicates/loss windows by design (commit-after-handoff), retries, and convergence via idempotent rereads/upserts. [P1,P2]
- Notification side: async queue-mediated ingress/egress in broader architecture with idempotent publication controls and recipient/lifecycle guards. [P7]
- Assignment trigger side: correctness anchored to source event + deterministic IDs + authorization/state checks rather than strict global ordering. [P8]

## 4. Flora-relevant keep / adapt / defer (80/20, cheap VPS)

### Keep / adapt now (high value, VPS-compatible)

| Theme | Macro principle (evidence) | Flora 80/20 adaptation |
|---|---|---|
| **Source-of-truth convergence** | Invalidation hint → reread primary state → idempotent upsert/delete; event payload not authoritative where possible. [P1,P2,P3,P8] | Postgres is truth. On mutations, update search projection/notification intent from committed DB state (in-process post-commit hook or tiny outbox-lite), not fragile payload copies. |
| **Idempotent writes and deletes** | Full upsert semantics, delete-by-entity/remove actions, replay/backfill repair. [P1,P3,P4] | Deterministic upsert/delete keyed by `(org_id/entity scope, entity_type, entity_id)` for search projections and stable dedupe keys for notifications where feasible. |
| **Multi-entity search contract (minimal)** | Unified query args over many entities with per-entity filters/boundaries. [P5] | One tenant-safe search API/contract for Flora core entities (messages/tasks/files, docs metadata if modeled) with explicit supported filters/modes; PG FTS default, optional Meilisearch behind same contract. |
| **Targeted repair/backfill** | Per-entity backfill/repair endpoints and targeted ID/index override workflows. [P1,P6] | Single-node admin command/job to reindex selected org/entity types from Postgres; no HTTP-orchestrated distributed job fleet. |
| **Assignment change detection** | Trigger on effective added assignees (delta), ignore removals/unchanged/irrelevant changes. [P8] | Generate assignment notifications only on effective transition to a new recipient (create/assign/reassign) using persisted old/new values and actor suppression. |
| **Notification idempotency mindset** | Create/publish guards prevent duplicate publication/acted-on re-alert; original recipients preserved on retry. [P7] | Minimal dedupe: transition-only + optional DB uniqueness/guard for active `(recipient,event_type,target_id[,state/version])`; document residual duplicate-on-retry risk if not enforced in storage. |
| **Tenant safety as first-class** | Access-relevant scoping/filters and owner-based operations in search/index flows. [P4,P5] | Enforce `organization_id`/membership/recipient eligibility server-side on every index row, query, and notification target (Macro’s exact model differs; Flora must be explicit). |

### Defer / drop for 022 Seed (complexity > value on ~2 vCPU/4 GB)

| Macro component/pattern | Why defer/drop for Flora VPS |
|---|---|
| **OpenSearch + lexical/index fleet management + aliases-as-ops requirement** | Extra service/RAM/heap, index ops, mappings/analyzers, failure modes. Prefer Postgres FTS/`pg_trgm`. [P1,P4,P5] |
| **Kafka + SQS + EventBridge + DLQ-heavy pipelines** | Broker ops, IAM/credentials, lag/loss-window tuning, consumer groups/rebalances unnecessary for monolith Seed. [P1,P2,P6] |
| **Commit-after-handoff durability tradeoff** | Acceptable at Macro scale with repair tooling; Flora should prefer simpler in-process/transaction-bounded semantics or explicit outbox with clear guarantees. [P2] |
| **Distributed backfill orchestration, DynamoDB job registry, multi-worker sharding infra** | Overkill; bounded local batch reindex suffices. [P6] |
| **Content extraction microservices/finalizers (PDF/DOCX/lexical sync, S3 object fanout)** | CPU/IO/storage heavy; Seed can index DB-native text/metadata only. [P3,S1] |
| **Full unified search breadth** (calls, agent sessions, calendar, projects deep modes, ids-only/collapse/cursor sophistication) | Implement only core entity search needed by product slice. [P5] |
| **Push/mobile/email/digest notification channels and egress fanout** | APNS/SNS/SES/device/presence complexity; keep persisted in-app notifications (+ optional best-effort realtime already present). [P7] |
| **Agent-session assignment orchestration graph (discussion auto-creation, project context enrichment, bot-run triggers)** | Useful for Macro’s agent-native UX, not a Seed prerequisite for basic task assignment notifications. Adopt only simple recipient notification semantics. [P8] |

## 5. Recommended 022 NV resolution targets (actionable)

1. **Search contract freeze (Flora):** define minimal parity matrix for `message|task|file` (and docs if applicable): query params, tenant filter, type filter, update/delete visibility, empty/error behavior; same contract for PG/Meilisearch adapters.
2. **Mutation wiring map:** enumerate exact Flora create/update/delete/soft-delete paths that must emit search upsert/delete and notification intents (source-of-truth reread, idempotent).
3. **Assignment semantics:** codify `old/new/actor` transition rules (notify on add/reassign-to-different-user; no notify on unassign by default unless product chooses), recipient eligibility (`org/workspace member`), and dedupe key schema.
4. **Backfill/repair:** add one scoped reindex command and document when to run it (schema/content drift, missed side effects).
5. **Observability:** structured logs/metrics for index/notify side-effect failures with entity IDs and mutation correlation (no silent drops).

## 6. NVs / limitations

- **NV-1 Macro search tenant model:** exact permission-filtering and workspace/org enforcement layers for unified search were not fully traced in service/API handlers; `user_id`/entity filters are evidenced but Flora’s org scoping is not directly equivalent. [P4,P5]
- **NV-2 Full entity handler parity:** only document processing and selected lifecycle mappings (calls/agent sessions from README) were inspected in depth; messages/tasks/files-as-documents/subtypes/projects parity across all handlers/backfills is partially inferred. [P1,P2,P3,P6]
- **NV-3 Task unassignment/reassignment notifications (user-facing):** `TaskAssigned` event type exists, but producer rules for assignee mutations (including unassign/reassign, recipient set, dedupe) were not confirmed; agent-assignment trigger path shows add-only behavior only. [P7,P8,P9]
- **NV-4 Soft-delete vs hard-delete lifecycle:** document purge/remove and deleted_at skip are evidenced; complete, cross-entity retention/soft-delete propagation contracts in OpenSearch are NV. [P3,P4]
- **NV-5 Performance/ops on VPS:** no Macro-vs-Flora benchmark claims made; PG FTS/index/write amplification on 2 vCPU/4 GB for Flora workloads remains unmeasured.
- **NV-6 Non-EN primary coverage:** limited; may miss discussions/issues not surfaced by EN queries (low likelihood of changing core architectural conclusions above).

## 7. Bottom line

Macro provides a strong reference for **convergent, idempotent search indexing** (invalidation + reread + repairable backfills) and **idempotent notification/publication discipline**, but implements them with infrastructure (Kafka/SQS/OpenSearch/extractors/multi-service workers/job registries) that is disproportionate for Flora’s Seed/VPS target. For 022, Flora should adopt only the **principles and contracts**: Postgres-sourced, tenant-scoped, multi-entity search projections with deterministic upsert/delete and simple reindex repair; and transition-based task assignment notifications with explicit dedupe/idempotency and no complex fanout. This captures most correctness/value while avoiding Macro’s heaviest operational dependencies.
