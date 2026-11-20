# 019 — Minimal in-app notification wiring (VPS, Postgres-first)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`3c8eddbfd`) — **research only** (no source/commit changes).  
**Scope:** choose **1** high-value 019 event and design the smallest reliable wiring from core mutation → in-app notification using existing core model/repo/table and existing `GET/POST /api/v1/notifications` read/ack APIs, for self-hosted Rust + PostgreSQL on cheap VPS (~2 vCPU / 4 GB). Cross-checked with `flora-api/src/routes/{channels.rs,tasks.rs,notifications.rs}`, `flora-core/src/{models/notification.rs,repositories/notification_repository.rs,services/notification_service.rs,traits/repository.rs,migrations/sql/20240818000001_init_schema.sql}`, `flora-notifications/src/lib.rs`, and prior research `docs/research/2026-11-16-{018-search-notifications-wiring-vps,018-macro-search-notifications-parity}.md`.  
**Method:** repo inspection + synthesis of prior Flora research + multilingual evidence cues (**EN, pt-BR, es, de, fr, zh-CN, ja**) from 018 docs/primary patterns (Postgres outbox/transactional consistency, in-app notifications as durable rows, VPS debloat). No runtime benchmarks; locale/perf claims marked **NV** where extrapolated.

## 1. Current state (cross-check)

| Area | Evidence | 019 implication |
|---|---|---|
| Storage/model | `notifications` table + indexes/RLS in `20240818000001_init_schema.sql` (`organization_id,user_id,event_type,target_id,target_url,is_read,...`); `Notification::new` sets UUIDv7, unread, timestamps (`models/notification.rs`). `NotificationType` already includes `Reply`, `Assignment`, `TaskCompleted`, `Mention`, etc. | Primitive is ready; no schema change required for MVP event. |
| Persistence/read | `PgNotificationRepository` implements find/list(unread|all, user-scoped)/create/mark_read/mark_all/delete_old (`repositories/notification_repository.rs`). Trait in `traits/repository.rs` expects `create(notification)` (insert-only) and read/ack ops. | Reuse core repo/trait directly; **no dedupe/upsert method exists today** (idempotency must be designed). |
| API | `routes/notifications.rs`: `GET /api/v1/notifications?limit&unread_only` (auth user, capped) and `POST /api/v1/notifications/{id}/read` with ownership check. Response exposes `id,event_type,target_id,is_read,created_at` (not `target_url`, not org). | Producer wiring is the missing half; read/ack contract can stay stable. |
| Unwired/duplicate abstractions | `flora-core::services::notification_service::NotificationService` is empty/stub; `flora-notifications::NotificationService` inserts + Valkey `XADD` stream but is **not** used by API/routes/AppState. Mutations emit only in-process realtime (`channels.rs`) or nothing (`tasks.rs`). | 019 should wire via **core repo** in mutation path; avoid Valkey/`flora-notifications` coupling (optional, deferred). |
| Mutations | `channels.rs::post_message`: validates membership/channel/org/content, persists message (`Message::new` + `thread_id`), emits `RealtimeEvent::NewMessage`, returns 201. No notification/search hooks. `tasks.rs::create/update/delete`: persist/soft-delete only. | Reply is the cleanest single trigger (parent lookup gives recipient + tenancy anchors). |

## 2. 80/20 event choice (recommended for 019)

| Candidate | Value | Complexity / correctness risk | Verdict |
|---|---|---|---|
| **Reply in thread to message author** (`NotificationType::Reply`) | High: direct social feedback loop, common daily signal, easy to explain/test. | **Low:** trigger is explicit (`thread_id` present), recipient derivable from persisted parent message, self-reply easy to suppress, same org/channel context available. | **Choose for 019 (single event).** |
| Task assignment (`Assignment`) | High for task workflows. | Medium: `create_task` can assign, but `update_task` merges `assignee_id.or(existing)` (cannot clear/unassign cleanly) and must notify only on actual assignee change, exclude self/actor ambiguities, and resolve “assigner” semantics — larger contract/risk surface for first slice. | Defer to 019.x/020. |
| Task completed (`TaskCompleted`) | Useful closure signal. | Medium+: who is recipient (creator? assignee? watchers?) is unspecified; status transitions not fully modeled as events. | Defer. |
| Mention (`Mention`) | High value but requires parsing/normalization and user resolution. | High for 019. | Defer (parser + ambiguity NV). |

**019 contract (Reply):** when `POST /channels/{channel_id}/messages` succeeds for a message `m` with `thread_id = Some(parent_id)`, notify **`parent.sender_id`** iff: `parent` exists, `!parent.is_deleted`, `parent.channel_id == channel_id`, `parent.organization_id == org_id`, and `parent.sender_id != auth.user_id` (no self-notify). Create **one** `Notification { organization_id: org_id, user_id: parent.sender_id, event_type: Reply, target_id: m.id, target_url: <canonical thread/message deep link> }`.

Rationale: aligns with P0 “eventos→notificações para mutações core” (018), maximizes user-visible value per LOC, and stays inside existing types/APIs/schema.

## 3. Implementation design (smallest verifiable)

### 3.1 Data flow (proposed)

1. **Mutation (source of truth):** `routes/channels.rs::post_message` persists reply `created` as today (unchanged success semantics for message create).
2. **Recipient resolution (read-after-write in handler):** if `req.thread_id.is_some()` load parent via `PgMessageRepository::find_by_id(parent_id)`.
3. **Guards (tenant/validity):** emit only when parent passes all checks in §2 (same channel+org, not deleted, different user). This reuses route-known `org_id/channel_id` and prevents cross-tenant/foreign-thread leakage.
4. **Notification creation:** `PgNotificationRepository::new((*state.db_pool).clone()).create(Notification::new(org_id, recipient, NotificationType::Reply, created.id, Some(target_url)))`.
5. **Exposure:** no API change required — `GET /api/v1/notifications` already lists user notifications; `POST /{id}/read` already acks.

`target_url` format should match existing client conventions if present (**NV-1**); if unknown, use a stable, relative path encoding org/workspace/channel/thread/message identifiers available at call site (prefer including `channel_id` and thread root/reply id for deep-linking).

### 3.2 Where to wire

- **Preferred 019:** inline in `flora-api/src/routes/channels.rs::post_message` after successful `msg_repo.create(...)` and before/after realtime emit (order not user-visible except timing) — smallest diff, full request context (`org_id`, `channel_id`, `state.db_pool`, auth user), no service-layer refactor.
- **Alternative (cleaner, slightly larger):** add `create_reply_notification(...)` helper in `flora-core::services::notification_service` taking `Arc<dyn NotificationRepository + Send + Sync>` and explicit inputs; route calls it. Keep helper pure-orchestration (no HTTP types) to enable unit/integration reuse.
- **Not for 019:** `flora-notifications` crate, Valkey streams, background workers, outbox table.

### 3.3 Recipient / tenant rules

- Recipient is **exactly the parent message author** (`parent.sender_id`), never broadcast/fanout.
- Enforce **same-organization** and **same-channel** binding between parent and reply; reject/skip if mismatch (defense-in-depth even if IDs are guessed).
- Rely on existing route authn/authz: caller must be org member and channel must belong to org (already enforced). Notification write must use route-derived `org_id`, not untrusted input.
- RLS exists on `notifications` but app access patterns are pool-level; correctness must not depend on RLS alone (consistent with repo code).

### 3.4 Idempotency / duplication avoidance

Current risk: HTTP retries/client replays/timeouts can duplicate the **message create** (no idempotency key visible) and/or the **notification side effect**. `NotificationRepository::create` has no uniqueness guard.

**80/20 options (ranked):**
1. **Accept-at-least-once for 019, mitigate cheaply (recommended default):** create notification only once per successfully-created reply message instance (`target_id = created.id`, `event_type = Reply`, `user_id = parent.sender_id`) in the same code path; since `created.id` is generated per insert, duplicates only arise if the *same* reply create is re-executed — which would also duplicate the reply unless request-level idempotency is added. Document as best-effort single emission per persisted reply.
2. **DB idempotency guard (stronger, additive):** add partial/unique index, e.g. `UNIQUE (user_id, event_type, target_id)` for active rows (or include org) via migration, and make repo create tolerate conflict (`ON CONFLICT DO NOTHING RETURNING ...` / fetch existing) — **requires schema+repo/trait touch** (small but cross-cutting). Best if cheap to include in implementation cycle.
3. **Outbox/inbox keys:** defer (overkill for Seed, adds table/worker semantics).

**Recommendation:** specify (2) as preferred hardening if 019 implementation touches migrations; otherwise ship (1) with explicit NV and contract note “at-most-one per persisted reply; client retries may require message-level idempotency later”.

### 3.5 Best-effort vs transactional semantics

- **Core write (message) is authoritative:** must commit and return 201 regardless of notification outcome.
- **Notification side effect:** **best-effort, post-persist, non-fatal** in 019.
  - On failure: log structured warning/error with `reply_id`, `parent_id`, `recipient_id`, `org_id`, `channel_id`; do **not** fail the request (user already received “message posted” and realtime event).
  - Rationale (VPS/operability): avoids turning auxiliary UX signal into availability risk; matches 018 guidance “best-effort, logged failures for non-core side effects”.
- **Transactional coupling (same DB tx as message insert):** stronger consistency/no orphan-gap but requires plumbing `MessageRepository::create`/route flow into an explicit transaction and sharing it with notification insert — **non-trivial refactor** given current repo signatures (`&self.pool` per op). Defer unless product requires “never miss in-DB notification for a committed reply”.
- **Ordering:** notification may appear slightly after realtime message event; acceptable for polling-based in-app inbox.

## 4. Test plan (DB-gated, deterministic)

Use existing disposable-PG18 / env-gated integration style (as in 012–017/018 research), with isolated org/workspace/channel/users and explicit cleanup/transactions as per repo conventions.

| ID | Layer | Scenario | Assert |
|---|---|---|---|
| T1 | integration (route/service+repo, DB) | User B replies in thread to User A’s root message in same org/channel | `notifications` contains row for `user_id=A`, `event_type='Reply'`, `target_id=<reply_id>`, `organization_id=org`, `is_read=false`, non-null/expected `target_url`; `GET` for A lists it (all/unread). |
| T2 | integration | User A replies to own message | **No** notification created. |
| T3 | integration | Reply to missing/deleted parent, or parent in different channel/org (crafted/guarded case) | No notification, primary reply behavior per chosen contract (reply create may still succeed if only `thread_id` stored; notification must not emit across tenants/invalid parent). |
| T4 | integration (repo/API) | Mark-read path for created notification | `POST /notifications/{id}/read` returns 204 for owner, 403 for other user, 404 unknown; `is_read` flips. |
| T5 | unit (if helper extracted) | Recipient/guard function given (parent sender, actor, org/channel ids, deleted flag) | Pure decision matrix deterministic (incl. self, mismatch). |
| T6 | regression (if idempotency guard added) | Duplicate create attempt for same `(user,event,target)` | Single row / conflict tolerated without error. |
| T7 | best-effort failure (targeted) | Simulate repo/DB error for notification insert (fault injection or mock boundary, if feasible) | Message post still succeeds; warning logged (**NV** if no easy fault seam in current structure). |

**Determinism:** fixed UUIDs/timestamps not required if assertions use returned IDs; scope queries by user/org; avoid timing sleeps (poll API/state directly).

## 5. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Duplicate notifications on retries/replays | Noisy inbox | Prefer unique constraint or document single-emission-per-persisted-reply; consider future request idempotency keys. |
| Cross-tenant/thread mismatch | Privacy/correctness bug | Strict guards on parent `organization_id/channel_id`, not-deleted, route-derived org; DB-gated negative test. |
| Best-effort drop on transient DB error after message commit | Missed notification (eventual gap) | Structured logs/metrics hook point; optional later outbox/retry (**NV**). |
| Recipient ambiguity if “reply” semantics later include thread followers | Behavior change | Lock 019 to **parent-author-only** in tests/docs. |
| `target_url` drift vs frontend routes | Deep link broken but row usable | Define canonical relative URL in one place; add contract test on stored value. |
| Extra read (`find_by_id` parent) per reply | Minor latency on VPS | Single PK lookup; negligible vs message insert/network. |

## 6. NVs (needs verification)

- **NV-1 (deep link contract):** canonical `target_url` / client routing for opening a specific reply/thread in current frontend(s).
- **NV-2 (idempotency mechanism):** whether 019 should include unique-index hardening now vs defer (depends on migration appetite/touching trait/repo).
- **NV-3 (transactional guarantee):** product requirement for “notification row must exist iff reply committed” (drives tx refactor/outbox).
- **NV-4 (reply semantics):** whether replies to deleted parents should be allowed and/or notify (current API stores `thread_id` without shown validation).
- **NV-5 (performance/ops):** notification write overhead and table growth/retention on 2 vCPU/4 GB under real load (use `delete_old`/policy later if needed).
- **NV-6 (i18n/UX copy):** localization of `Reply` rendering is client-side; event payload stability only.
- **NV-7 (fault-injection test seam):** easiest deterministic way to prove best-effort behavior without invasive refactor/mocks in this codebase.
- **NV-8 (future events):** `Assignment` needs `assignee_id` diff/clear semantics and actor attribution not fully specified in `UpdateTaskRequest`/service logic. (pt-BR: verificar regras de atribuição; es: reglas de asignación; de: Zuweisungsregeln; fr: règles d’attribution; zh-CN: 指派规则需确认; ja: 割り当て規則の確認が必要).

## 7. Sources

**Flora (primary):**
- `flora-api/src/routes/{channels.rs,notifications.rs,tasks.rs}` (mutation/auth/read-ack behavior)
- `flora-core/src/models/notification.rs`, `repositories/notification_repository.rs`, `services/notification_service.rs`, `traits/repository.rs`
- `flora-core/src/migrations/sql/20240818000001_init_schema.sql`
- `flora-notifications/src/lib.rs`
- `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`, `docs/research/2026-11-16-018-macro-search-notifications-parity.md`
- `OBJETIVO.md`, `ROADMAP.md` (P0 wiring gap; cited in 018)

**Conceptual (from 018 synthesis):** PostgreSQL as system-of-record, in-app notifications as durable rows, best-effort side effects for non-core paths, optional outbox/idempotency for reliability (see 018 sources: PostgreSQL FTS/`pg_trgm` docs, transactional outbox pattern, Meilisearch comparison). Multilingual takeaway preserved: keep DB-first/simple wiring for low-RAM VPS (**EN/pt-BR/es/de/fr/zh-CN/ja**, evidence strength as in 018).