# 021 — Macro search parity (messages/tasks/files) & task assignment/unassignment notification dedupe (multilingual)

**Date:** 2026-11-16  
**Branch/context:** `dev@0dcde353` (read-only research; this document only)  
**Focus:** (1) **multi-entity search (messages/tasks/files) parity across search backends/modes** in Macro vs what Flora needs on a cheap VPS (Rust + Postgres, optional Meilisearch); (2) **task assignment/unassignment notification generation and deduplication** in Macro and minimal 80/20 contracts for Flora 021.  
**Cross-check (local):** `OBJETIVO.md`, `ROADMAP.md`, `docs/research/2026-11-16-{018-macro-search-notifications-parity,019-macro-mutation-search-notification-flows,020-macro-tasks-files-assignment-flows,020-task-assignment-notification-vps,020-pg-fts-tasks-files-vps}.md`, `flora-search/src/lib.rs`, `flora-api/src/routes/search.rs`, `flora-notifications/src/lib.rs`, `flora-core/src/models/{notification.rs,task.rs}`.

## 1. Sources, method, and multilingual coverage

**Primary (EN, inspected):**
- [S1] Repo/product: https://github.com/macro-inc/macro , https://raw.githubusercontent.com/macro-inc/macro/main/README.md
- [S2] Search pipeline/ops: https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/README.md (`macro-inc/macro@main`, ingests Kafka `macro.calls`/`macro.agent_session_lifecycle` + legacy SQS/backfills, invalidation hints → reread source-of-truth → idempotent OpenSearch upsert/delete, commit-after-handoff loss window, repair/backfill endpoints)
- [S3] Unified search/query model: `crates/opensearch_client/src/search/unified.rs` (UnifiedSearchArgs across agent sessions/documents/emails/channel messages/chats/call records/projects/calendar events; per-entity args, modes/filters, ids-only, collapse/cursor/page fields, query JSON building)
- [S4] Product search surface: https://raw.githubusercontent.com/macro-inc/macro/main/apps/docs/product/search.mdx (unified search across email/tasks/docs/call transcripts/agents/files; type/tag filters; same index used by agents)
- [S5] Task API semantics: `packages/sdk/src/entities/tasks/namespace.ts` (`assignees` defaults to creator; `Pass [] to leave it unassigned`; `create` sets initiative/assignees/due in one request)
- [S6] Notification ingress/domain: `crates/notification/src/domain/service/ingress.rs`, `crates/notification/src/domain/ports.rs`, `crates/notification/src/domain/models.rs`, `crates/notification/src/lib.rs` (filter sender/muted/unsubscribed/disabled types; persist before publish; `create_notification` returns new/unpublished rows or `None` after publication/withdrawal and preserves original recipients on retry; `complete_notification_publication`; suppress re-alert for acted-on/deleted rows; queue fanout, APNS collapse key handling, realtime/mobile/email channels)
- [S7] Task-assigned evidence: `crates/model_notifications/src/lib.rs` (`NotifEvent::TaskAssigned(TaskAssignedMetadata)`), generated notification schemas/types in web/sdk (`TaskAssignedMetadata*`, `initiativeDiscussionReason` includes `assignee`), PR/repo references to task-assigned flows/agent triggers; no `TaskUnassigned` notification variant found in searched code/docs (**NV-3**).

**Multilingual pass (requested: EN, pt-BR, es, de, fr, zh-CN, ja):** searches run in all seven locales. Actionable Macro-specific primary internals remained **EN-first** (repo/docs/code). Non-EN hits were mostly generic search/notification/VPS guidance or non-primary overviews; no locale-primary contradiction to [S1–S7] found. Operational conclusions are triangulated with prior Flora multilingual VPS research and EN primaries.

**Limits:** monorepo not exhaustively traced; exact producer call sites/payloads for every task-assignment mutation and all entity search handlers/mappings are partially **NV**.

## 2. Macro evidence

### 2.1 Multi-entity search and “parity across backends/modes”

| Dimension | Macro (evidenced) | Parity meaning |
|---|---|---|
| **Indexing substrate** | Dedicated OpenSearch + `search_processing_service` (Kafka/SQS/backfill), extraction/finalizer services in stack [S1–S3]. | Search is decoupled from primary DB; writes converge asynchronously. |
| **Ingestion contract** | Events treated as invalidation hints; handlers reread primary state and perform idempotent full upsert/delete; repair/backfill endpoints [S2]. | Correctness over time despite retries/lag/duplicates. |
| **Unified query surface** | Single product search over many entities; `UnifiedSearchArgs` composes per-entity args (docs/email/channel/chat/call/project/calendar/agent sessions) with shared paging/cursor/collapse/match options and entity-specific filters/modes [S3,S4]. | One client contract, heterogeneous entity semantics. |
| **Modes/filters** | `ids_only`, collapse, entity/type selection, per-entity filters (ids, labels, sender/recipients, speakers, roles, thread/channel, tag/property, time/status where applicable), term/match behavior variants [S3,S4]. | “Parity” is API-level consistency + explicit per-entity capability boundaries, not identical ranking/features for every type. |
| **Consistency/ops** | At-least-once-ish delivery, commit-before-processing loss window, ordered sharding best-effort, explicit backfill/repair, index provisioning discipline [S2]. | Operational parity requires replay/repair, not only query parity. |

**Implication:** Macro achieves cross-entity search parity by unifying query composition and convergent indexing over a specialized backend, at high infra/ops cost.

### 2.2 Task assignment/unassignment notification generation and deduplication

| Dimension | Macro (evidenced / NV) | Notes |
|---|---|---|
| **Assignment mutation model** | Task create can set assignees; assignees can be cleared by passing empty list in SDK contract [S5]. Exact server-side mutation handlers for all task surfaces **NV** in this pass. | “Unassign” is represented as absence/empty assignee set in API semantics. |
| **Notification types** | `TaskAssigned` event/metadata exists [S7]; initiative/project discussion notifications include `assignee` as a delivery reason in metadata/schemas/docs context [S7]. | Assignment is an explicit, recipient-directed notification class. |
| **Unassignment notify** | No explicit `TaskUnassigned` variant/reason found in inspected/search results (**NV-3**). | Do not assume Macro emits unassign notifications; treat as product decision. |
| **Recipient filtering** | Ingress removes sender, muted, unsubscribed, and users with notification type disabled [S6]. | Reduces noise and enforces preference boundaries. |
| **Persistence/publication idempotency** | `create_notification(..., notification_id, ..., apns_collapse_key)` returns new/unpublished rows or `None` after publication/withdrawal; retries reuse original recipients/lifecycle; `complete_notification_publication` marks success; withdrawn checks; suppress re-alert for already acted-on/deleted rows [S6]. | Strong dedupe/retry safety at ingress/publication boundary. |
| **Delivery fanout** | Queue-published messages for conn-gateway/APNS/Android/email, grouping/collapse handling, digest/state-machine hooks [S6]. | Delivery complexity is separable from generation correctness. |
| **Discussion/mention adjacency** | Recipient derivation and notification reasons (mention/reply/assignee/owner) appear in message/discussion designs [S7]. | Assignment is one case in a broader, server-authoritative recipient system. |

## 3. Flora current state (relevant)

| Area | Observed | Gap vs 021 target |
|---|---|---|
| **Search** | `flora-search`: Meilisearch, per-org index, index/delete for message/task/file, simple query + type filter + snippet [F]. `flora-api` search route: auth+membership, CSV `type`, limit cap, uses Meilisearch when configured else PG FTS fallback over messages/tasks/files with `websearch_to_tsquery/plainto_tsquery` and per-entity merged/ranked results [F]. | No unified backend abstraction/contract tests for parity, limited modes (no cursor/facets/ids-only), ranking/snippet semantics differ by backend, delete/update wiring completeness **NV**. |
| **Notifications** | `flora-notifications::NotificationService`: DB insert + Valkey `XADD`; read/unread/mark-read APIs exist; `NotificationType::Assignment` exists; no dedupe/idempotency key or publication ledger [F]. 020 research identifies PATCH assignee-clear contract gap and recommends transition-only, best-effort assignment notify in routes [F]. | No generation wiring for task assignment transitions yet (per 020/ROADMAP), no dedupe guard, no unassign policy, no recipient preference/actor model beyond simple fields. |
| **Task model** | `assignee_id: Option<Uuid>`; update paths historically ambiguous for clear vs omit (020) [F]. | Must define effective transition detection for assign/reassign/unassign before notification parity claims. |

## 4. Keep / adapt / defer for Flora 021 (cheap VPS)

### 4.1 Search parity (messages/tasks/files)

| Decision | Keep/adapt (021) | Defer/drop |
|---|---|---|
| **Contract (parity target)** | Define one search contract for **messages, tasks, files**: tenant-scoped by `organization_id` + membership, `q`, optional `type` set, `limit`, stable result shape `{type,id,snippet}` (+ optional rank/metadata only if cheap/stable). Same empty-query/empty-result/error behavior across backends. | Unified search across all Macro entities (email/calls/calendar/agents/projects/channels), tag/property graph filters, cross-entity blended relevance tuning. |
| **Backends** | **Adapt:** backend-agnostic service trait/adapter with **Postgres FTS as default** (Seed/VPS) and **Meilisearch optional profile**. Parity = same mutation visibility invariants and API contract, not identical lexical ranking. | OpenSearch, Kafka/SQS ingestion fleet, extraction/indexer microservices as requirements. |
| **Modes** | **Keep minimal modes:** keyword search + type filter + bounded limit; document/index only metadata/text already in DB (message content, task title/description, file name/metadata). | `ids_only` APIs, cursor pagination, advanced match modes (literal/prefix/phrase semantics per entity), collapse, highlights/facets/autocomplete, content extraction/OCR/embeddings. |
| **Indexing/update semantics** | **Adapt Macro principle:** mutation-driven, idempotent upsert/delete derived from current DB state (transactional or post-commit with bounded repair), including delete/soft-delete convergence. | Commit-after-handoff loss windows, distributed workers, complex replay orchestration. |
| **Parity verification** | DB-gated/contract tests run against the selected backend contract: create/update/delete of message/task/file becomes searchable/non-searchable correctly, tenant isolation, type filters, deterministic bounds. | Cross-backend ranking equality, load/perf SLAs without benchmarks. |

### 4.2 Task assignment/unassignment notification generation and deduplication

| Decision | Keep/adapt (021) | Defer/drop |
|---|---|---|
| **Generation trigger** | **Adapt:** emit on **effective assignment transition** only: notify recipient when `new_assignee == Some(u)` and `old_assignee != Some(u)` and `u != actor`, on create/assign/reassign paths where old+new are known post-persist. Persist in-app `NotificationType::Assignment` with `target_id=task.id`, tenant/org from trusted context, stable `target_url` if available. | Fanout to previous assignee/watchers/mentions/subscribers, multi-channel push/email/digest, workflow/routine automations. |
| **Unassignment** | **Default 021:** **no `Assignment` notification on unassign** (`Some -> None`) unless product explicitly defines recipient/meaning (former assignee? actor confirmation?). Record as NV/product decision; avoid silent semantic invention. | Implicit unassign notifications, dedicated unassign event type without contract/tests. |
| **Deduplication** | **MVP (Macro-inspired, simplified):** (1) transition-only emission (primary dedupe for no-op/repeated patches), (2) self/tenant/permission guards, (3) best-effort non-fatal side effect after successful mutation. **Hardening (preferred if cheap):** idempotency key/dedupe guard in DB for assignment notifications, e.g. suppress duplicate active unread row for same `(user_id, event_type, target_id)` within current assignment intent or store a monotonic assignment/version marker; model retries explicitly. Do **not** claim exactly-once without storage support. | Full ingress/egress publication ledger, delivery receipts per channel, collapse-key/mobile push infra, digest dedupe state machines. |
| **Retry/idempotency semantics** | Document: core mutation success independent of notification insert; duplicate possible on client-replayed mutations that create distinct logical requests unless request idempotency key exists — acceptable only if bounded/visible. Prefer deterministic insert guard over hidden assumptions. | Distributed exactly-once guarantees. |
| **Recipient correctness** | Server-authoritative recipient from persisted task state; validate org/workspace/member eligibility where cheap; never trust client recipient list. | Preference center complexity (muted/unsubscribed/type-disable matrices) except trivial future extension. |

## 5. 021 recommended contract (concise)

1. **Search parity contract:** one API/query behavior over PG-FTS|Meilisearch for `message|task|file`; identical auth/tenancy/type filtering/limits; backend differences isolated behind adapter; add parity-focused tests before claiming equivalence.
2. **Assignment notify contract:** `old/new/actor` -> notify only newly assigned non-actor user; unassign => no notification by default; insert best-effort post-persist with warn logs and mutation-id correlation fields in logs.
3. **Dedupe contract:** transition-only required; add cheap DB-level dedupe/idempotency guard if schema/repo touch is acceptable in 021, else document residual duplicate-on-retry risk explicitly.
4. **Operability:** no new mandatory services; migrations/indexes bounded; reindex/reconcile path remains manual/admin and tenant-scoped.

## 6. Risks

| Risk | Why it matters | Mitigation |
|---|---|---|
| **R1 False search parity** | Same endpoint but different ranking/snippet/delete visibility across PG and Meilisearch breaks user trust and tests. | Define explicit parity matrix; test invariants, not scores. |
| **R2 Assignment transition bugs** | Ambiguous patch semantics or stale reads can mis-detect assign/unassign/reassign. | Use effective persisted old/new values; fix/clarify clear-assignee contract (020 NV-1). |
| **R3 Duplicate/missed notifications** | Retries, races, or best-effort failures create noise or silence. | Transition-only + optional DB dedupe key; structured warnings; repair/reconcile only if needed. |
| **R4 Privacy/tenancy leakage** | Wrong recipient or cross-org search/notify scope. | Server-derived recipient, org/workspace checks, membership validation, tenant-filtered queries/tests. |
| **R5 VPS cost** | FTS indexes/triggers/notification writes contend on 2 vCPU/4 GB. | Bounded indexes, metadata-only indexing, no background fleets, measure later (NV). |
| **R6 Over-copying Macro** | Queues/OpenSearch/push semantics add ops burden without SMB value. | Keep Postgres-first, in-process, in-app only. |

## 7. NVs (needs verification)

- **NV-1 Search parity specifics:** exact ranking/snippet/tokenization expectations per locale (pt-BR/es/de/fr/zh-CN/ja) and acceptable backend deltas for Flora.
- **NV-2 Flora mutation wiring truth:** all create/update/delete/restore paths for messages/tasks/files that must trigger index upsert/delete across PG and Meilisearch.
- **NV-3 Macro unassign semantics:** whether Macro emits any notification or side effect on task unassignment/assignee removal in production paths (no explicit variant found).
- **NV-4 Dedupe key design:** safe uniqueness for legitimate re-assignment over time (simple unique row vs epoch/version vs time-window) compatible with current schema/migrations.
- **NV-5 Task clear-assignee contract:** API representation for unassign in Flora (`null` meaning clear vs omit) and migration/compat impact.
- **NV-6 Recipient eligibility:** assignee must be active org/workspace member at notify time?
- **NV-7 Delivery/read contract:** should `target_url`/metadata be exposed in notification list API for deep-linking; client expectations.
- **NV-8 Performance:** index/write latency and storage growth on target 2 vCPU/4 GB VPS under representative load.

## 8. Sources

**Primary (Macro, EN):** [S1] https://github.com/macro-inc/macro , https://raw.githubusercontent.com/macro-inc/macro/main/README.md ; [S2] https://raw.githubusercontent.com/macro-inc/macro/main/services/search_processing_service/README.md ; [S3] `crates/opensearch_client/src/search/unified.rs` (repo source); [S4] https://raw.githubusercontent.com/macro-inc/macro/main/apps/docs/product/search.mdx ; [S5] `packages/sdk/src/entities/tasks/namespace.ts`; [S6] `crates/notification/src/domain/{ports.rs,service/ingress.rs,models.rs}`; [S7] `crates/model_notifications/src/lib.rs`, generated notification metadata/schemas and discussion-reason references (`assignee`).

**Flora internal:** [F] `OBJETIVO.md`, `ROADMAP.md`, `docs/research/2026-11-16-018-macro-search-notifications-parity.md`, `docs/research/2026-11-16-019-macro-mutation-search-notification-flows.md`, `docs/research/2026-11-16-020-macro-tasks-files-assignment-flows.md`, `docs/research/2026-11-16-020-task-assignment-notification-vps.md`, `flora-search/src/lib.rs`, `flora-api/src/routes/search.rs`, `flora-notifications/src/lib.rs`, `flora-core/src/models/{notification.rs,task.rs}`.

**Multilingual:** EN primary above; pt-BR/es/de/fr/zh-CN/ja locale-targeted searches performed (Macro-specific first-party detail limited/NV; no contradictions found).
