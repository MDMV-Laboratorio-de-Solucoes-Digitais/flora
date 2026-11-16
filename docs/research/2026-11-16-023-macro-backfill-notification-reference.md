# 023 — Macro backfill/freshness and notification idempotency/coalescing/"epoch" depth (80/20 for Flora cycle 023)

**Date:** 2026-11-16
**Context:** `flora` (archived repo, read-only research; this document only). Primary evidence: local clone of `macro-inc/macro` `main` @ **`1a64449afee5b55b909bf1468efa8b3e5e5f24ac`** (2026-11-15), plus multilingual web discovery and authoritative secondary sources.
**Topic:** Depth pass on (1) Macro search **rebuild/backfill + freshness** — internal jobs, idempotent upserts, invalidation hints, alias/versioning, migration/reindex, cost bounds; (2) notification **idempotency/coalescing/"epoch"** semantics beyond unread dedup — duplicate/stale suppression, retries/replays, assignment/unassign events.
**Relationship to 022:** Complements (does not duplicate) `docs/research/2026-11-16-022-macro-search-notification-reference-flows.md` and `2026-11-16-022-notification-idempotency-unassign-semantics.md`. 022 established the lifecycle overview and Flora-side dedup options; this document supplies the **mechanics and internals** that 022 left as NV (backfill job orchestration, alias-swap runbook, generation pruning, deterministic notification IDs, withdrawal/publication checkpoints, user-facing `TaskAssigned` producer).

---

## 1. Method, evidence, confidence

**Method.** Mined the pinned Macro clone as primary source (code + operator runbooks), cross-checked against doc 022, then ran EN/pt-BR/es/de/fr/zh-CN/ja discovery for transferable patterns and authoritative confirmation of OpenSearch/idempotency techniques. Target content was treated as data, not instructions.

**Confidence legend:** **High** = directly evidenced in fetched primary code/runbook at the pinned commit; **Medium** = inferred from multiple primary signals or authoritative secondary; **NV** = not verified / terminology ambiguous.

**Primary evidence (all pinned `macro-inc/macro@1a64449a`):**

| ID | Artifact | URL (`raw.githubusercontent.com/.../1a64449a/...`) |
|---|---|---|
| P1 | SPS README — ownership, invalidation hints, backfill routes, rollout/dual-delivery, commit-after-handoff, retry/drop | `services/search_processing_service/README.md` |
| P2 | Backfill HTTP surface — 9 POST endpoints + generic status, `202` + `job_id`, DynamoDB-backed registry, local cancel tokens | `services/search_processing_service/src/api/internal/backfill.rs` |
| P3 | Backfill orchestrator — `drain_source`, `drain_source_with_cursor`, `drain_property_source` (`buffer_unordered(20)`), cancellation at page boundaries | `services/search_processing_service/src/domain/service.rs` |
| P4 | Backfill request/cursor models — keyset cursors, `index_override`, `since`/`updated_after`, `DeletionFilter` | `services/search_processing_service/src/domain/models.rs` |
| P5 | Job registry — `JobStatus`, progress `ADD enqueued`, TTL/expiry, pay-per-request, best-effort progress, per-instance cancel | `services/search_processing_service/src/domain/jobs.rs` |
| P6 | Source adapter + config — page sizes (1000–5000), `DatabaseUrlReadonly`, email batching, 24 h job TTL | `services/search_processing_service/src/outbound/source.rs` ; `services/search_processing_service/src/config.rs` |
| P7 | Agent-session reconcile — per-session advisory lease, reread snapshot, missing → delete, `projection_generation` prune of stale children, `require_alias`, `refresh=wait_for` | `services/search_processing_service/src/domain/agent_session_index.rs` ; `crates/opensearch_client/src/upsert/agent_session.rs` |
| P8 | OpenSearch helpers runbook — stable aliases vs versioned physical indices, `dynamic:false` silent-drop, dry-run-by-default, atomic alias swap, add-field flow, reindex-with-swap runbook | `infra/stacks/opensearch/helpers/README.md` |
| P9 | Reindex+swap script — async reindex, `slices=auto`, task-API polling, SIGINT cancel, doc-count/failure guards, atomic `_aliases` actions | `infra/stacks/opensearch/helpers/scripts/reindex_with_alias_swap.ts` |
| P10 | Alias/version constants, `create_indices.ts`, `verify_aliases.ts`, `verify_mappings.ts` — pre/post-flight checks, additive mapping convergence, type conflicts need reindex | `infra/stacks/opensearch/helpers/constants.ts` ; `.../scripts/create_indices.ts` ; `.../scripts/verify_aliases.ts` ; `.../scripts/verify_mappings.ts` |
| P11 | Notification request builders — `into_request()` = `Uuid::now_v7()`; `into_request_with_id()` = caller-supplied idempotency key (doc comment: "redelivered that event without notifying twice") | `crates/notification/src/domain/models/request.rs` |
| P12 | Ingress service — recipient filtering, `create_notification`, `None` on completed/withdrawn, acted-on suppression, original-recipient replay, publication completion, collapse keys, state decisions | `crates/notification/src/domain/service/ingress.rs` |
| P13 | Repository + withdrawal — `INSERT ... ON CONFLICT (id) DO NOTHING`, `publication_complete` checkpoint, `pg_advisory_xact_lock(hashtextextended("notification:{id}",0))`, durable `notification_withdrawal` tombstone | `crates/notification/src/outbound/repository.rs` ; `crates/notification/src/outbound/withdrawal.rs` |
| P14 | Notification state machine — `Unseen/Seen/Done`, `ACTIVE=[Unseen,Seen]`, `(Done,Reopen)->Seen`, atomic transitions, timestamps are historical metadata not state | `crates/notification_state/src/lib.rs` |
| P15 | Deterministic-ID producers — team join `Uuid::new_v5(namespace, team_id:email)`, invite reminder `Uuid::new_v5(invite_id, reminder_number)` | `crates/teams/src/domain/join_announcement.rs` ; `crates/teams/src/domain/team_service/invite_reminders.rs` |
| P16 | Replay/poison handling — ungrouped Kafka consumer (no offsets, `Latest`, drop unsupported schema versions); malformed SQS JSON acked; `notification_message_receipt ON CONFLICT (message_id) DO NOTHING` | `crates/notification/src/outbound/notification_consumer.rs` ; `.../queue.rs` ; `.../message_receipt_repository.rs` |
| P17 | **User-facing `TaskAssigned` producer** (resolves 022 NV-3) — delta vs persisted ASSIGNEES, bots excluded, internal writes skipped, actor suppressed, one message per recipient; no unassign path; per-recipient metadata/collapse key | `crates/properties/src/domain/service_impl/task_properties.rs` ; `crates/properties/src/outbound/notification_service.rs` ; `crates/model_notifications/src/metadata.rs` |
| P18 | Withdrawal use — channels completion withdraws then marks `Done` | `crates/channels/src/outbound/notification_sender.rs` |
| P19 | Fencing "epoch" elsewhere in Macro — `lease_epoch` checked in the same tx, bumped on takeover (analogy only, not notification) | `crates/whatsapp_store_pg/src/store.rs` ; `crates/whatsapp_linked/src/outbound/pg/session.rs` |

**Authoritative/secondary:** [S1] OpenSearch alias API — atomic index replacement and alias switching: https://docs.opensearch.org/latest/api-reference/alias/aliases-api and https://docs.opensearch.org/latest/im-plugin/index-alias ; Reindex API (`wait_for_completion=false`, task API, `slices`, `conflicts`, `require_alias`): https://docs.opensearch.org/latest/api-reference/document-apis/reindex . [S2] Idempotent Consumer pattern (es): https://learn.microsoft.com/es-es/azure/architecture/patterns/idempotent-consumer . [S3] Idempotency guide (es/ja): https://cloud.google.com/discover/idempotency?hl=es-419 , https://cloud.google.com/discover/idempotency?hl=ja . [S4] OneSignal duplicate-notification/collapse_id (zh-CN): https://documentation.onesignal.com/docs/cn/duplicated-notifications . [S5] JavaGuide idempotency (zh-CN) https://javaguide.cn/high-availability/idempotency.html ; idempotency-key traps (fr-query result) https://redisson.pro/glossary/idempotency.html ; 冪等性キー (ja) https://syu-m-5151.hatenablog.com/entry/2026/05/28/124943 . [S6] RFC 9562 UUID v5/v7: https://www.rfc-editor.org/info/rfc9562 .

---

## 2. Delta versus 022 (what this document adds)

| 022 said | 023 resolves/extends |
|---|---|
| "Backfills return 202 + job_id, poll status; jobs in shared state (DynamoDB noted)" | §3.1–3.2: exact job lifecycle, progress arithmetic, TTL, cancellation scope, page-size/cost bounds, read-replica policy [P2,P3,P5,P6] |
| "Index provisioning/aliases required; `index_override` for repair" | §3.4: full alias→versioned-index contract, dry-run/atomic-swap runbook, doc-count guard, write-window replay, `dynamic:false` silent-drop failure mode [P8,P9,P10] |
| "Full scan walks existing rows in bounded UUID pages" | §3.2: keyset cursors per entity, offset-vs-cursor split, bounded concurrency, replica-vs-primary [P3,P4,P6] |
| "Idempotent upserts; projection generations; lease" | §3.3: concrete `projection_generation` prune + `require_alias`/`refresh=wait_for` + advisory lease through write [P7] |
| "Notification ingress idempotency primitives" | §4.1–4.4: caller-supplied UUID as idempotency key, deterministic UUIDv5 producers, `publication_complete` checkpoint, advisory-lock create/withdraw race, withdrawal tombstone [P11–P15,P18] |
| "NV-3: producer of user-facing `TaskAssigned` not traced; unassign unknown" | §4.5: producer traced — delta-before-upsert, add-only, bots/internal/actor suppressed, no unassign notification; replay idempotency **not** enabled for this producer [P17] |
| "epoch semantics" (Flora-side Option D) | §4.6: Macro has **no literal notification epoch field**; equivalent mechanisms named; separate `lease_epoch` fencing pattern noted as analogy [P14,P19] |

---

## 3. Search: rebuild/backfill and freshness mechanics

### 3.1 Backfill job model (High)

Every indexed entity has an internally-authenticated POST on SPS; identical shape across nine entities (`agent-sessions`, `calls`, `chats`, `channels`, `documents`, `emails`, `properties`, `projects`, `calendar-events`) plus generic `GET /internal/backfill/{job_id}` [P2]. Handler does not do work: it allocates a job slot, records initial item, spawns a tokio task, returns `202 {"job_id"}`; completion is recorded once as `Completed | Failed | Cancelled` with optional error and timestamps [P2,P5].

Job registry is **DynamoDB**, deliberately: SPS scales 1–10 ECS tasks with no sticky sessions, so a status poll may land on a different instance; in-memory state would 404. Pay-per-request, native TTL cleanup (`expires_at`), 24 h default TTL [P5,P6]. Progress is `UpdateItem ADD enqueued :n` per page, best-effort — a failed update logs and continues; the next page's `ADD` reconciles [P5].

**Cancellation is explicitly best-effort and local:** tokens live in a per-instance `HashMap`, no cancel endpoint, no cross-pod propagation; SIGTERM stops only that pod's drains. Because reindexing is idempotent, a cancelled-but-unrecorded job is recoverable by simply re-kicking it [P2,P5]. This is a *deliberate* "replay over coordination" trade.

### 3.2 Bounded work and cost control (High)

The orchestrator uses three loop shapes, all checking cancellation **between pages** so a page is either completed+counted or fails without partial progress [P3]:

- `drain_source_with_cursor` — opaque seek cursor; used for calls, chats, channels, documents, projects, calendar events. Terminates on `rows_consumed == 0`; a `None` next-cursor on a non-empty page also terminates (prevents restart-from-zero) [P3].
- `drain_source` — integer offset advanced by **rows consumed**, not messages emitted (email source folds many rows into one SQS batch) [P3].
- `drain_property_source` — direct reindex of entity IDs with `stream::iter(...).buffer_unordered(20)`, a hard concurrency cap [P3].

Cost bounds evidenced:

| Bound | Value / mechanism | Source |
|---|---|---|
| DB page sizes | calls 2000, chats 5000, channels 5000, documents 1000, emails 1000, projects 2000, calendar 2000; properties fixed 5000; email batch 50; all `BACKFILL_*_PAGE_SIZE` tunable, must be > 0 | [P6] |
| Keyset pagination | `(updated_at, id)` / `(created_at, id)` / `(started_at, id)` tuples with `WHERE (col,id) > cursor`; UUID breaks timestamp ties so edits don't move rows across pages | [P4,P6] |
| Read path | Optional `DATABASE_URL_READONLY` for backfill scans (avoid primary contention); **queue workers always read primary** because replica lag would miss just-committed rows | [P1,P6] |
| Incremental filters | `updated_after/updated_before` on `updatedAt` (not `createdAt`) catch edits to old rows; `since` for emails; `DeletionFilter{Any,Active,Deleted}`; explicit IDs = one primary-key lookup per page | [P4] |
| Replay retries | Queue-backed completion = *source publication only*, not indexing/refresh; failed/stale jobs re-run with the same scope and are safe because upserts are idempotent | [P4] |
| Live-path bounds | Kafka: 10 sequential workers × bounded 16-message channels, 3 attempts (1 s/2 s backoff), poison/keyless records committed, `opensearch_client` retries dropped connections | [P1] |

**Freshness caveat (High):** retained Kafka history is *not* a complete historical backfill; the agent-session full scan deliberately ignores `modified_at` because ACP log appends need not update it; already-deleted IDs cannot be discovered by a scan [P1,P7]. Backfills are never auto-scheduled by deployment [P1].

### 3.3 Idempotent upserts and generation-based freshness (High)

- Events are **invalidation hints**: handlers reread current metadata/transcript from owning services, then write a full overwrite; delayed events cannot resurrect deleted state [P1,P7].
- Agent-session reconcile holds a **per-session Postgres advisory lease across the OpenSearch write**, with a separate pool for lease connections so waiting workers cannot starve the snapshot-loading pool [P1,P7].
- After writing parent + folded messages, the client runs `delete_by_query` pruning all docs for that `agent_session_id` whose **`projection_generation` != current** [P7]. This is the mechanism that removes stale child chunks when a transcript shrinks — a version/generation field, not a timestamp.
- Writes use `require_alias(index_override.is_none())` so a missing alias **fails** instead of auto-creating a bare index with an inferred mapping; explicit backfills may target a pre-provisioned physical index [P1,P7,P8]. `refresh=wait_for` on parent/bulk writes makes the projection immediately searchable [P7].
- Email property-only updates use update-by-query with `conflicts=proceed` and zero-match no-op: "a concurrent full reindex writes the same freshly-fetched values" [P7-adjacent, `crates/opensearch_client/src/upsert/email.rs`].

### 3.4 Index aliasing, versioning, migration/reindex (High)

**Contract:** application code reads/writes stable aliases (`channels`, `documents`, `emails`, `call_records`, `agent_sessions`, `chats`, `projects`, `calendar_events`); physical indices are versioned (`documents_v2`, `emails_v2`, …). "The alias is the contract; the physical index is an implementation detail that can be swapped without a code deploy" [P8,P10]. Every mapping body is `dynamic: 'false'`: a field present in code but absent in the live mapping stays in `_source` **but is not indexed** — searches match nothing rather than erroring; this is how `call_records_v2` shipped without `properties`/`name` and broke call tag filters (macro-2731) [P8].

**Operator discipline (all scripts `DRY_RUN=true` by default):**

1. `verify_aliases.ts` — pre/post-flight: alias exists, points at expected index, single target; flags "physical index where alias should be" and "alias points at multiple indices" (ambiguous swap) [P8,P10].
2. `verify_mappings.ts` — compares live mappings to canonical `create_indices.ts` bodies; missing paths → "add field then backfill entity", type conflicts → "needs reindex_with_alias_swap" [P8,P10].
3. Adding a field: edit canonical body → `create_indices.ts` (additive `_mapping` PUT keeps data) → **backfill the entity** so pre-existing docs pick up the field → verify. Additive only; type changes require full reindex [P8].
4. `reindex_with_alias_swap.ts` [P9]: submit `_reindex` with `wait_for_completion=false`, `refresh=true`, `slices=auto` (one sub-task per primary shard), poll task API (10 s default), aggregate child-task progress with high-water marks (completed children drop from `tasks.list`), SIGINT → `tasks.cancel`. **Refuses to swap** on any reported failure or when destination doc count < source. Then a **single atomic `_aliases` request** does `remove(old)/add(new)` — or `remove_index(bare physical)+add(alias)` in one call to avoid a writer recreating the bare index between two operations. Dry-run prints the exact actions list.
5. **Write window:** writes during reindex land only on the old index and are cut off at swap. Two choices: **replay** (default: accept the window, then run entity backfill bounded by `since=<reindex start>`, safe because idempotent on `_id`) or **pause writers** (drain SQS first; real freshness pause) [P8,P9].
6. Keep old index for rollback; delete only after confidence via `delete_indices.ts` [P8].
7. One-off repair for a wrongly auto-created bare `agent_sessions` index uses a single atomic `remove_index + add{is_write_index:true}` request (no copy, intentionally does **not** reindex), followed by targeted backfill of in-flight sessions [P8].

**Transferable 80/20:** the valuable idea is *stable logical name + versioned physical target + atomic cutover + bounded replay*, not the OpenSearch task/slice machinery. OpenSearch docs confirm the atomic `_aliases` replace and async reindex/task-API pattern are standard technique [S1].

---

## 4. Notifications: idempotency, coalescing, "epoch" semantics

### 4.1 The idempotency key is a caller-supplied notification UUID (High)

The framework default is **not** replay-safe: `SendNotificationRequestBuilder::into_request()` assigns `Uuid::now_v7()` [P11]. But `into_request_with_id(id)` exists explicitly for producers that can derive an ID from their own event; its doc comment states that creating a notification is idempotent on its id, so a redelivered event does not notify twice [P11]. Producers that actually do this derive a **deterministic UUIDv5** from stable business identity:

- team join announcement: `Uuid::new_v5(&NAMESPACE, "{team_id}:{recipient_email}")` [P15];
- invite reminder: `Uuid::new_v5(&reminder.team_invite_id, "team-invite-reminder-{n}")` — "one notification id per invite and reminder, so delivering the same reminder twice creates one notification" [P15].

This is Macro's practical equivalent of an "epoch": a **logical-identity token that stays constant across redelivery but changes for a new logical occurrence**. RFC 9562 v5 (name-based) vs v7 (time-ordered random) is the exact distinction [S6]. Cross-language guidance converges on the same rule: the key must be client/producer-generated and stable per *business operation*, not a broker delivery ID or a fresh UUID per attempt [S2,S3,S5].

### 4.2 Atomic persistence + publication checkpoint (High)

`create_notification` in one DB transaction [P12,P13]:

1. `pg_advisory_xact_lock(hashtextextended("notification:{id}", 0))` — serializes create vs withdrawal for the same id (the race that a naive "check then insert" cannot close).
2. If `notification_withdrawal` has the id → return `None` (durable suppression).
3. If `notification` row already exists **and `publication_complete`** → return `None` (already published: retry is a no-op).
4. If row exists but publication incomplete → return the **original stored recipients** (with their original state/timestamps); a retry therefore uses original recipients "including when current filters changed" [P12].
5. Else `INSERT INTO notification ... ON CONFLICT (id) DO NOTHING`; `rows_affected()==0` → `None` (concurrent winner); insert `user_notification` rows via `UNNEST`; default `state=Unseen`, `sent=false`.

After create, ingress filters rows to `state == Unseen && deleted_at.is_none()` — "do not alert again after a recipient has already acted on the stored notification"; if none remain it calls `complete_notification_publication` (sets `publication_complete=true`) and returns no recipients [P12,P13]. On the publish path it attaches state-machine decisions and DB timestamps, publishes channel messages, then marks publication complete [P12]. The checkpoint is the retry boundary: **before** completion → replay original recipients; **after** completion → no-op.

### 4.3 Withdrawal tombstone (High)

`notification_withdrawal(notification_id)` is a durable suppression table inserted with `ON CONFLICT DO NOTHING` under the same advisory lock [P13]. Used when the underlying work is resolved before/during delivery: channels' `NotificationReaderCompletion::complete` withdraws first (so delayed deliveries are dropped) then marks each recipient `Done` [P18]; document-storage access requests use the same port [P13]. This is a *tombstone/negative-cache* pattern — distinct from dedup, because it actively cancels an already-created intent.

### 4.4 State machine and reopen ("epoch" for the user) (High)

`NotificationState { Unseen, Seen, Done }`; `ACTIVE = [Unseen, Seen]` [P14]. Transition policy `apply` is intentionally asymmetric and idempotent: `(Unseen,MarkSeen)→Seen`, `(_,MarkDone)→Done`, `(Done,Reopen)→Seen`, else unchanged. "Reopening always produces `Seen`, never `Unseen`"; adapters must apply transitions **atomically against current stored state**, not against a stale read. Viewing timestamps are explicitly "historical metadata, not another representation of state" (a backfilled `Done` may never have been viewed) [P14]. Ingress `NotificationStatus::Reopen{unseen}` maps to a repository `reopen_notifications` call [P12].

### 4.5 Coalescing and replay hygiene (High/Medium)

- **Push collapse keys:** each notification metadata type implements `collapse_key(&entity)`; `TaskAssignedMetadata` uses `NotifCollapseKey::new(entity_type).append(entity_id)`; `with_apns()` hashes it into `MessageAttributes.collapse_key` [P11,P17]. On read-state changes, *all historical rows for a channel commonly share a collapse key*, so clear-push messages are **deduplicated into a HashSet of collapse keys** before enqueueing ("clear each push once, rather than an identical background push for every historical row") [P12]. Collapse keys are the platform-level coalescing layer (cf. APNs `apns-collapse-id` / FCM `collapse_key`) [S4].
- **Message-receipt dedup:** `notification_message_receipt(message_id, …)` inserted `ON CONFLICT (message_id) DO NOTHING` — a second, finer-grained idempotency layer keyed on the produced message id [P16].
- **Poison/replay handling:** notification Kafka consumer is **ungrouped**, assigns all partitions, starts `Latest`, commits no offsets — every process sees events published after it starts (MQTT-style live fanout, not a replay log) [P16]. Events with unsupported schema versions are logged and **dropped** as poison; other malformed payloads error. SQS ingress **acks malformed JSON rather than retrying to DLQ** so poison messages cannot wedge the batch [P16]. These are pragmatic "bounded retry, no infinite DLQ loop" choices.
- **No literal notification "epoch" field exists** in the notification crates (verified by repo-wide grep; the only `epoch` uses are Unix time and an unrelated WhatsApp `lease_epoch`) — the "epoch" role is distributed across the stable notification ID, `publication_complete`, withdrawal tombstone, and state machine [P11–P15,P19]. **NV-adjacent terminology note.** The `lease_epoch` pattern (fencing token checked in the same DB transaction, bumped on takeover so stale holders' writes fail) is an optional analogy for stale-writer protection if Flora ever adds a competing indexer/worker [P19].

### 4.6 Assignment / unassignment event modeling (High — resolves 022 NV-3)

The user-facing producer lives in `crates/properties`, **not** the agent-trigger path of 022 [P17]:

- `handle_task_assignees_property` is called **before** `upsert_entity_property` (`service_impl.rs:764-768` vs `:786`), so the "current" `ASSIGNEES_UUID` read from the repository is the **pre-write** value.
- Recipients = incoming refs that (a) are `EntityType::User`, (b) are **not** bot IDs (`BotIdStr` skipped — bots share the User ref namespace), (c) are **not** in the persisted pre-write assignee set, and (d) are **not the actor** (`assigned_by_user_id`). If empty → no work; internal/machine writes with no `assigned_by_user_id` → no notification.
- One `send_task_assigned` call fans out **one notification per recipient** with `TaskAssignedMetadata{task_id, task_name (document name), sub_type=Task, assigned_by, sender_profile_picture_url}`, entity = `Document(task_id)`, `sender_id=assigned_by`, APNS + conn-gateway channels [P17].
- **Unassign:** no notification path found — clearing/removing assignees produces no `TaskAssigned` event. **Reassignment:** removing A and adding B notifies B only; re-adding A later (when A is absent from the persisted set) produces a fresh notification. This matches 022's agent-trigger add-only behavior but is now confirmed for the **user-facing** notification path [P17; cross-check 022 P8].
- **Replay idempotency gap (important):** this producer calls `.into_request()` → `Uuid::now_v7()` [P11,P17], i.e. it does **not** pass a deterministic `into_request_with_id`. If the upstream property-update event were replayed after the write, the delta could be recomputed as empty (pre-write value now includes the assignee) — incidentally suppressing a duplicate — but a replay that observes an older snapshot would mint a new UUID and could notify twice. Macro's notification framework *supports* exact replay idempotency; this particular producer does not opt in. **Medium confidence** on the replay outcome; High on the ID choice.

---

## 5. Multilingual coverage

| Lang | Discovery result | Incremental value |
|---|---|---|
| **EN** | Macro repo/runbooks (primaries P1–P19); OpenSearch alias/reindex docs [S1]; idempotency-authority articles | **High** — all architectural conclusions anchored here |
| **pt-BR** | FireHydrant alert dedup (idempotency_key grouping + dedup window), Cash App/Google idempotency guidance | Medium — confirms "stable key + window" framing; no Macro-specific source |
| **es** | Microsoft *Idempotent Consumer* (es), Google Cloud idempotency (es-419), Reactor Patterns note: "la notification como side effect asíncrono… si se envía duplicada es aceptable; la clave debe ser de negocio, no `message_id`" | Medium — directly supports Flora's "business key, not transport id" rule |
| **de** | Idempotency-key practice articles (similar to EN results) | Low — no Macro-specific German source |
| **fr** | Idempotency-key pitfalls (derive from request body → collapses legitimate repeats; non-atomic check-then-write; missing TTL) [S5] | Medium — warns against Flora dedup keyed on payload hash |
| **zh-CN** | OneSignal duplicate-notification docs (`collapse_id` replaces rather than stacks), JavaGuide/腾讯云 idempotency (unique request ID + unique index + state machine + dedup table), RabbitMQ message-dedup (business key not messageID) [S4,S5] | Medium — corroborates collapse + layered-defense design |
| **ja** | Google Cloud idempotency (ja), AWS builders flash 冪等性, Qiita: include `tenant_id` in multi-tenant idempotency keys, DB unique constraint as backstop; LinkedIn: APNs `apns-collapse-id` + deterministic keys [S3,S5] | Medium — reinforces tenant-in-key and "DB constraint as final guard" |

**Finding (as in 022):** Macro-specific primary technical detail is overwhelmingly EN repo content. Non-EN sources supplied *confirmation and caution*, not contradictory evidence. No non-EN first-party Macro doc was found (**NV** on exhaustive issue/PR coverage in those languages).

---

## 6. Flora cycle 023 — keep / adapt / defer (cheap VPS, Postgres-first)

### Keep / adapt now (high value, low infra)

| Macro pattern (evidence) | Flora 80/20 adaptation |
|---|---|
| Stable logical index name → versioned physical target, atomic cutover, dry-run + doc-count guard [P8,P9] | For optional Meilisearch: keep one stable app-facing contract; rebuild into a **new named index**, verify counts, then atomically repoint config/alias. On Postgres FTS, the analog is a **new generated column/index (`_v2`) + swap in query**, not OpenSearch task machinery. |
| Invalidation hint → reread committed DB state → idempotent full upsert/delete [P1,P7] | Already Flora's stated direction (022). Make it explicit: never index from a stale payload copy; reread the row. |
| Bounded pages + keyset cursors + `since`/`updated_after` + hard concurrency cap [P3,P4,P6] | One CLI/admin reindex command with page size (e.g. 200–500), `ORDER BY (updated_at,id)` seek, org/entity/date filters, and a bounded batch (no unbounded full scan). Off-peak, resumable, idempotent. |
| Page-boundary cancellation + replay over coordination; best-effort progress [P2,P5] | Local job table (or just a log line) with rows-enqueued counter; re-running the same scope is safe; no distributed job registry needed on one node. |
| `require_alias` fail-fast; `dynamic:false` silent-drop failure mode; pre/post-flight mapping checks [P7,P8,P10] | Validate that the chosen search backend's schema/index exists and matches before writing; add a cheap "schema probe" test so a missing field fails loudly instead of returning empty results. |
| `projection_generation` prune of stale children [P7] | For any nested/projection table, store a generation/version on each row and delete rows with a stale generation after rewrite — especially relevant to unassign/reassign where child sets shrink. |
| Caller-supplied deterministic notification ID (UUIDv5 over business identity) + `ON CONFLICT DO NOTHING` [P11,P13,P15] | **Highest-value notification change:** add a `dedupe_key`/`notification_id` (UUIDv5 of `org_id + event_type + target_id + transition_identity`, e.g. assignment version / mutation id) with a DB unique constraint; use `INSERT ... ON CONFLICT DO NOTHING RETURNING` to make create atomic. This upgrades 022's racy query-before-insert. |
| `publication_complete` checkpoint + original-recipient replay [P12,P13] | If/when Flora adds an outbox or realtime publish, persist an intent row first, mark completion after publish; retries reuse the stored recipient set. Keeps best-effort UX without silent loss. |
| Withdrawal tombstone for "work resolved before delivery" [P13,P18] | Cheap negative table (`notification_withdrawal(notification_id)`) if reassignment/unassign can invalidate a pending notification; mark `Done` instead of sending. |
| Explicit state machine with atomic transitions + `Reopen → Seen` (not `Unseen`) [P14] | Flora's `is_read` boolean is a subset; if inbox UX needs "done/reopen", adopt `unseen/seen/done` + atomic `UPDATE ... WHERE state = expected`. Timestamps stay historical metadata. |
| Collapse key for push + dedup of clear messages [P12,P17] | Only if/when push is added: derive a stable `collapse_key` from `(type, target_id)`; dedup clear ops by key. Defer actual APNs/FCM. |
| Assignment delta computed **before** the write, bots/internal/actor excluded, add-only [P17] | Matches Flora's tri-state transition rule; add bot-identity and "internal write" suppression explicitly; document that unassign intentionally emits no notification. |

### Defer / drop (complexity disproportionate to 2 vCPU / 4 GB)

- Kafka/MSK consumer groups, 10-worker sharding, SQS/EventBridge/DLQ choreography, commit-after-handoff loss windows [P1] — replace with in-process post-commit hooks or a single Postgres outbox+lite worker if reliability demands it.
- OpenSearch cluster, shard/slice/reindex task management, alias choreography at runtime [P8,P9,P10] — keep Postgres FTS/`pg_trgm` primary; optional Meilisearch only behind the same contract.
- DynamoDB job registry, shared progress counters, 1–10 task scaling rationale [P5] — a local job row or log suffices.
- Async reindex `slices=auto` + task polling + write-block/replay dance [P9] — on a single small index, rebuild-then-swap with a short guarded window is enough.
- Notification egress fleet (SNS/APNS/FCM/email/digest state machines), ungrouped Kafka fanout, collapse-key infra [P12,P16] — persist in-app notifications; optional best-effort realtime already exists in Flora.
- `lease_epoch` fencing [P19] — only if two writers can genuinely race on the same projection; otherwise overkill.

---

## 7. NVs / limitations

- **NV-1 (terminology):** no literal "notification epoch" field in Macro; "epoch semantics" here = deterministic logical ID + publication checkpoint + withdrawal tombstone + state machine. Flora should not invent a literal epoch column unless it wants the lease-fencing meaning [P11–P15,P19].
- **NV-2 (replay outcome):** the exact behavior when the `TaskAssigned` upstream event is replayed against a stale snapshot was not verified end-to-end; High confidence only on "producer uses `now_v7()`" [P11,P17].
- **NV-3 (producer breadth):** deterministic `into_request_with_id` was confirmed in teams/channels/email/access-request/agent-harness producers [P11 list]; full producer-by-producer audit of all notification types was not done.
- **NV-4 (delete/retention):** Macro's cross-entity soft-delete vs hard-delete timing in OpenSearch and notification retention window were not fully traced (inherits 022 NV-4).
- **NV-5 (tenant model):** Macro's search isolation is user/permission/entity-filter based; the exact enforcement layer equivalent to Flora's `organization_id` was not re-verified here (inherits 022 NV-1). For Flora, include `org_id` in every dedupe key and index filter explicitly.
- **NV-6 (non-EN primary coverage):** Macro-specific issues/PRs in pt-BR/es/de/fr/zh-CN/ja were not exhaustively searched; non-EN sources were generic/secondary (low likelihood of changing core conclusions).
- **NV-7 (no benchmarks):** no Macro-vs-Flora performance claims; page sizes and concurrency caps above are Macro defaults, not measured for Flora's VPS workload.

---

## 8. Bottom line

Macro's backfill/freshness depth is a **convergent, bounded-replay system**: pagination is keyset and page-sized, work is cancelled only at page boundaries, source-of-truth rereads make upserts idempotent, a `projection_generation` prunes stale children, and index migrations use stable aliases + versioned physical indices with dry-run, doc-count guards and an atomic `_aliases` cutover plus a bounded `since` replay for the write window. Its notification side is **idempotent by construction at the storage boundary** (caller-supplied notification UUID + `ON CONFLICT DO NOTHING` + `publication_complete` checkpoint + advisory-locked withdrawal tombstone), with an explicit `unseen/seen/done` state machine and push coalescing — but **the user-facing `TaskAssigned` producer does not pass a deterministic ID**, so replay-safety there relies on delta recomputation rather than a stable key.

For Flora 023, capture the invariants — **stable logical identity, atomic DB-enforced idempotency, bounded resumable reindex, fail-fast schema checks, add-only assignment transitions, and replay-over-coordination** — and defer the entire Kafka/SQS/OpenSearch/DynamoDB/extractor fleet. The single most transferable upgrade over 022 is replacing racy `has_unread_for_target` with a DB-unique deterministic `notification_id` derived from the assignment transition identity, with `org_id` in the key.

---

## 9. Citation index (quick URLs)

All Macro paths below are relative to `https://raw.githubusercontent.com/macro-inc/macro/1a64449afee5b55b909bf1468efa8b3e5e5f24ac/`:

- `services/search_processing_service/README.md`
- `services/search_processing_service/src/{api/internal/backfill.rs, domain/service.rs, domain/models.rs, domain/jobs.rs, domain/agent_session_index.rs, outbound/source.rs, config.rs}`
- `crates/opensearch_client/src/upsert/agent_session.rs`
- `infra/stacks/opensearch/helpers/README.md` ; `infra/stacks/opensearch/helpers/constants.ts` ; `infra/stacks/opensearch/helpers/scripts/{reindex_with_alias_swap.ts, create_indices.ts, verify_aliases.ts, verify_mappings.ts}`
- `crates/notification/src/domain/models/request.rs` ; `crates/notification/src/domain/service/ingress.rs` ; `crates/notification/src/outbound/{repository.rs, withdrawal.rs, notification_consumer.rs, queue.rs, message_receipt_repository.rs}`
- `crates/notification_state/src/lib.rs`
- `crates/teams/src/domain/join_announcement.rs` ; `crates/teams/src/domain/team_service/invite_reminders.rs`
- `crates/properties/src/domain/service_impl/task_properties.rs` ; `crates/properties/src/domain/service_impl.rs` ; `crates/properties/src/outbound/notification_service.rs` ; `crates/model_notifications/src/metadata.rs`
- `crates/channels/src/outbound/notification_sender.rs` ; `crates/whatsapp_store_pg/src/store.rs`
- Secondary: [S1]–[S6] listed in §1.
