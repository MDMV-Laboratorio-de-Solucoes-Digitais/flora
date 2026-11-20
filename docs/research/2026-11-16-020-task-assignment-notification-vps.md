# 020 — Minimal in-app task assignment notification (VPS, Postgres-first)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`dde602b3b`) — **research only** (no source/commit changes).  
**Scope:** design a **minimal, high-reliability** in-app notification for **task assignment** in a self-hosted Rust + Axum + PostgreSQL app on a cheap VPS (~2 vCPU / 4 GB). Focus: detecting assignment on task create/update/reassign/unassign (actor vs assignee), recipient/tenant rules, avoiding self-notify and obvious duplicates (idempotency without heavy infra), best-effort vs transactional semantics, `target_url`/payload shape for the existing notification model/API, and DB-gated deterministic tests.

**Cross-check (primary):** `flora-api/src/routes/{tasks.rs,channels.rs,notifications.rs}`, `flora-core/src/models/{task.rs,notification.rs}`, `flora-core/src/repositories/{task_repository.rs,notification_repository.rs}`, `flora-core/src/services/{task_service.rs,notification_service.rs}`, `flora-core/src/traits/repository.rs` (`TaskRepository`, `NotificationRepository`), `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `flora-tests/src/contract/notifications.rs` (019 reply DB-gated pattern), and 019 research: `docs/research/2026-11-16-019-{inapp-notification-wiring-vps,db-gated-tests-search-notifications}.md`.

**Method:** codebase inspection + synthesis of existing 019 implementation/research + multilingual evidence cues (**EN, pt-BR, es, de, fr, zh-CN, ja**) from established patterns: durable in-app rows in Postgres (authoritative store), event emission on state transition, self-suppression, idempotent/at-most-once-by-key where cheap, best-effort side effects for non-core paths, and deterministic DB-gated tests. Performance/locale/UX claims are marked **NV** where not directly evidenced.

---

## 1. Current state (facts)

| Area | Evidence | 020 implication |
|---|---|---|
| Task mutation paths | `routes/tasks.rs`: `POST /` creates task (sets `assignee_id` from req), `PATCH /{id}` loads existing, computes `updated` with `title/description` overridden when provided but `assignee_id: req.assignee_id.or(existing.assignee_id)` and `status` defaulting to existing, then `task_repo.update(id, updated)`; `DELETE` soft-deletes. Routes do **not** call `TaskService` (service exists with `create_task/update_task/assign_task` but is bypassed). | Assignment detection must live where old+new state are both known (route after load, or service/repo transaction boundary). Current PATCH merge **cannot express unassign/clear** (`Some -> None`) because `None` in request is indistinguishable from “unchanged” and `.or(existing)` preserves old value. |
| Task model | `Task { id, workspace_id, organization_id, creator_id, assignee_id: Option<Uuid>, status, is_deleted, ... }` (`models/task.rs`). `UpdateTaskInput.assignee_id: Option<Uuid>` same ambiguity. | `assignee_id` transition is the assignment signal; `organization_id` and `workspace_id` available for tenant/deep-link context. |
| Notification primitive | `notifications` table (org/user/event_type/target_id/target_url/is_read, indexes, RLS) in init migration; `Notification::new(org,user,event_type,target_id,target_url)`; `NotificationType::Assignment` already exists; `PgNotificationRepository::{create,find_by_id,find_by_user_id,find_unread_by_user_id,mark_as_read,...}`; trait requires `create(notification)` with **no dedupe/upsert**. | No schema change required for MVP payload; idempotency must be application-level or added via optional unique constraint/repo change. |
| Notification API | `GET /api/v1/notifications` lists user notifications, returns `id,event_type,target_id,is_read,created_at` (omits `target_url`, `organization_id`); `POST /{id}/read` enforces ownership. | Producer can store `target_url`, but current read contract will not expose it until API is extended (**NV**). Tests can assert DB row and/or API-visible fields. |
| 019 reference implementation | `channels.rs::post_message` creates reply, then **best-effort** `maybe_notify_reply(...)`: loads parent, guards `!deleted && same org/channel && parent.sender_id != sender`, inserts `NotificationType::Reply` with `target_id = reply_id`, `target_url = None`, logs warning on failure, never fails message create. `flora-tests/src/contract/notifications.rs` provides DB-gated patterns (`db_state`, seed users/org/workspace/channel/session, `authed_json`, reply/self-reply assertions). | 020 should mirror 019’s operational posture (best-effort, non-fatal, tenant/self guards, DB-gated deterministic tests) and reuse its harness style. |

---

## 2. 80/20 event semantics (recommended)

### 2.1 Goal
Notify a user when a task becomes **newly assigned to them** by someone else, with minimal moving parts and no heavy infra.

### 2.2 Canonical rule (single event type: `NotificationType::Assignment`)
Emit **at most one logical assignment notification per effective (re)assignment transition** to a recipient, when all hold:

1. **Transition detected:** `new_assignee = Some(uid)` and `old_assignee != Some(uid)` (covers unassigned→assigned and reassigned A→B).  
2. **Not self-notify:** `uid != actor_user_id` (the authenticated actor performing the mutation).  
3. **Recipient/tenant valid:** `uid` is the intended assignee within the same `organization_id` as the task and the actor is an authorized org member for that mutation (route already enforces actor membership/org match). Stronger check: assignee is an active member of the org (and, if required, has access to `workspace_id`) — **NV-2**.  
4. **Target valid:** task is not being soft-deleted in the same operation; notification `target_id` is the task id.

**Do not emit** for: create/update with no effective assignee change; assignment to `None` (unassign) as an “Assignment” event to the former assignee (different semantics, no current event type/recipient clarity) — treat unassign as **no notification in 020 MVP**; self-assignment (`actor == assignee`); cross-org or unauthorized contexts.

### 2.3 Per-mutation semantics

| Mutation | Old state | New/effective state | 80/20 behavior |
|---|---|---|---|
| **Create task** (`POST /tasks`) | none | `req.assignee_id` (or `None`) | Notify `assignee_id` iff `Some(uid)` and `uid != actor`. (pt-BR: notificar novo responsável; es: notificar asignado; de: Zuständigen benachrichtigen; fr: notifier l’assigné; zh-CN: 通知被指派人; ja: 担当者に通知). |
| **Update task** (`PATCH /tasks/{id}`) — assign/change assignee | `existing.assignee_id` | requested/effective assignee after applying patch | Notify **new** assignee only when transition condition holds (`Some(new) != old` and `new != actor`). No notification to previous assignee on reassignment/unassign in MVP. |
| **Update task** — unassign | `Some(old)` | `None` | **No Assignment notification in MVP** (needs explicit clear support + product decision). |
| **Reassign** (A→B) | `Some(A)` | `Some(B)` | Notify **B** only (if `B != actor`); do not notify A. |
| **Delete task** (`DELETE`) | any | soft-deleted | No assignment notification. |
| **Status-only/other-field updates** | same assignee | same assignee | No notification. |

### 2.4 Critical contract gap (must be acknowledged)
`PATCH` currently does `assignee_id: req.assignee_id.or(existing.assignee_id)` (`routes/tasks.rs`, similarly `TaskService::update_task`). Therefore:
- client cannot explicitly **unassign** a task today, and
- server cannot reliably distinguish “set to None” from “omit field”.

**020 design implication:** implement transition detection against the *effective* persisted/updated assignee. To fully support unassign/reassign semantics, introduce an explicit tri-state in the API/service contract (e.g. `assignee_id` with presence semantics via `Option<Option<Uuid>>`/`serde(default, deserialize_with = ...)` wrapper, or a dedicated action/flag) — **recommended follow-up in same implementation cycle if feasible**; otherwise document that unassign-driven cases are out of scope for deterministic correctness (**NV-1**).

### 2.5 Actor identity
Use `auth.user_id` from `AuthExtractor` as `actor`. This is the only authenticated principal available in routes; “creator” vs later “editor” does not create a separate notifier identity in current model (acceptable for 80/20).

---

## 3. Implementation design (smallest verifiable)

### 3.1 Placement (preferred 020)
**Inline in `flora-api/src/routes/tasks.rs`**, mirroring 019 in channels:
- `create_task`: after successful `task_repo.create(...)`, compare `created.assignee_id` vs actor and emit best-effort.
- `update_task`: capture `old_assignee = existing.assignee_id` (and ids/org/workspace) **before** building `updated`; after successful `task_repo.update(...)`, compare `result.assignee_id` (effective new) to `old_assignee` and actor.
- Keep response/status semantics unchanged for the core mutation.

**Rationale:** full request context (`org_id`, `state.db_pool`, `auth.user_id`), minimal diff, no service refactor. `TaskService` remains bypassed by routes today; if routes are later refactored to use service, move notification orchestration behind a small helper that takes explicit `(old,new,actor,org,task_id,workspace_id)` inputs.

**Not for 020:** `flora-notifications` crate, Valkey streams, `PgListener`, background workers, outbox table.

### 3.2 Proposed helper (pure orchestration)
```text
fn assignment_recipient(old: Option<Uuid>, new: Option<Uuid>, actor: Uuid) -> Option<Uuid>
  -> Some(new) iff new.is_some() && new != old && new != actor

async fn maybe_notify_task_assignment(state, org_id, workspace_id, task_id, old, new, actor) -> Result<()>
  - if let Some(recipient)=assignment_recipient(...) create Notification::new(org_id, recipient, Assignment, task_id, target_url)
  - best-effort: caller logs warn on Err, does not propagate failure to HTTP response
```
(No code added in this research; shape for implementation clarity.)

### 3.3 Recipient / tenant rules
- **Recipient:** effective new assignee only.
- **Tenant binding:** always persist `organization_id` from route-validated context (`auth.require_org()` / task’s org), never from client body. Before notify on update, ensure `existing.organization_id == org_id` (already enforced) and operate on the loaded task record.
- **Membership validation (recommended hardening):** verify assignee belongs to `org_id` (and workspace access if applicable) before creating notification to avoid notifying a non-member/orphan UUID; cheapest via `PgMembershipRepository::find_by_user_and_organization` (**NV-2**). MVP may rely on upstream assignment validation if present — verify.
- **RLS:** table has RLS, but app uses pool-level access patterns; correctness must not depend on RLS alone.

### 3.4 `target_url` / payload shape (existing model & API)
- **Model support:** `Notification { event_type, target_id, target_url: Option<String>, organization_id, user_id, is_read, timestamps }` — sufficient for 020 without schema change.
- **`event_type`:** `NotificationType::Assignment` (already defined/serialized as `"Assignment"`).
- **`target_id`:** `task.id` (stable identifier of the assigned resource).
- **`target_url` (recommended, deterministic, relative):** encode enough context to deep-link without extra lookups:  
  `"/workspaces/{workspace_id}/tasks/{task_id}"` (preferred if workspace-scoped task routes/views exist) or `"/tasks/{task_id}"` fallback. Include org context only if frontend routing requires it (**NV-3**). 019 currently stores `None`; 020 should prefer setting a stable URL now (value is persisted even if API omits it).
- **Current API response:** `NotificationResponse { id, event_type, target_id, is_read, created_at }` — does **not** include `target_url` or `organization_id`. For 020 this is acceptable if clients can resolve by `target_id + event_type`; exposing `target_url` would be a small, backward-compatible response extension (**NV-3**, optional follow-up).
- **No extra payload column/table:** avoid JSON blobs/heavy schema; use existing fields (80/20).

### 3.5 Idempotency / duplicate avoidance (without heavy infra)

**Risk sources:** HTTP retries/timeouts/client replay causing duplicate mutation requests; user flipping assignee back-and-forth intentionally (each real transition is a legitimate event).

**80/20 strategy (ranked):**
1. **Transition-only emission (required):** compute recipient from `old -> new` diff; emit only when `new` is a *newly assigned* user different from actor. This removes duplicates from no-op updates and most repeated PATCH payloads that do not change effective assignee.
2. **At-most-once per successful create path:** create emits once per persisted task row (unique `task.id`), so duplicates only occur if create is retried at HTTP level and creates multiple tasks (separate core idempotency concern).
3. **Cheap DB guard (recommended hardening, additive):** partial unique index to prevent identical unread/active duplicate signals for the same current assignment target, e.g. conceptually `UNIQUE (user_id, event_type, target_id)` or include a recency/epoch marker — but reassignment back to same user later may be intentionally notifiable. A safer minimal guard is **application-level recent duplicate suppression**: before insert, check for an existing unread `Assignment` for same `(user_id, target_id)` created recently (e.g. short window) or simply rely on transition-only logic; both are **NV-4** trade-offs. Given no repo method for conditional insert, schema+repo change is needed for true idempotent insert.
4. **Outbox/inbox/request idempotency keys:** defer (overkill for cheap VPS MVP).

**Recommendation:** ship (1)+(2) as MVP contract (“notify on effective assignment transition; no-op updates do not notify”). If implementation can add a migration, prefer `ON CONFLICT DO NOTHING` on a well-chosen uniqueness key after product decision; otherwise document residual duplicate risk on client-level retried create requests that somehow replay same logical transition (**NV-4**).

### 3.6 Best-effort vs transactional semantics
- **Core mutation authoritative:** task create/update/delete must commit and return success/failure based on task persistence only.
- **Notification side effect:** **best-effort, post-persist, non-fatal** (same as 019). On error: structured `tracing::warn!` with `task_id`, `workspace_id`, `org_id`, `actor_id`, `old_assignee`, `new_assignee`, `recipient_id`, `error`; do not fail request.
- **Why:** prevents auxiliary inbox delivery from reducing task-write availability on a small VPS; acceptable eventual gap for in-app notifications.
- **Stronger option (deferred, NV-5):** same-DB transaction coupling (insert/update task + insert notification atomically) requires transaction-aware repo APIs/refactor; or transactional outbox for guaranteed later delivery — both larger than 020 MVP.

### 3.7 Concurrency / ordering
- Concurrent updates may race; `old_assignee` read before update can be stale relative to another committed update. 80/20 mitigation: operate on transition observed by the request’s own read/update result and accept rare duplicate/missed signals under races; stronger fix would use conditional update (`UPDATE ... WHERE assignee_id = $old RETURNING`) or row locking (**NV-6**). Notification may appear after task response is received by client (polling inbox).

---

## 4. DB-gated, deterministic test plan

Follow 019 contract-test style in `flora-tests` (`#[cfg(test)] mod db_*`, `#[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 020)"]`, `db_state()` returns `Ok(None)` when env unset, `run_migrations`, unique fixtures via `Uuid::now_v7()`, `Router::oneshot`, scoped assertions).

### 4.1 Harness/fixtures (reuse/extend 019 pattern)
- Seed: org, workspace, role/member users A(actor/creator), B(assignee), C(other), active sessions/JWTs, org header auth (`authed_json` helper as in `contract/notifications.rs`).
- Router: nest tasks + notifications routes (`/tasks`, `/notifications`) with shared `AppState`.
- Determinism: assert on returned task/notification IDs and filtered queries (`event_type=="Assignment"`, `target_id==task_id`, `is_read==false`); avoid global counts except scoped deltas.

### 4.2 Cases

| ID | Type | Scenario | Expected |
|---|---|---|---|
| T1 | contract, DB-gated | **Create with assignee:** A creates task assigned to B | `201`; B’s `GET /notifications` contains `Assignment` for `target_id=task.id`; A does not receive it (unless also separately relevant, not in MVP). |
| T2 | contract | **Create self-assigned:** A creates task assigned to A | `201`; no `Assignment` notification for A. |
| T3 | contract | **Create unassigned:** A creates task with no assignee | `201`; no `Assignment` notification created. |
| T4 | contract | **Update assigns (None→B):** A creates unassigned task, then PATCH sets assignee to B (via supported contract) | B receives exactly one new `Assignment` for that task on transition; task update returns success. |
| T5 | contract | **No-op update (same assignee):** task already assigned to B, PATCH changes title/description or resubmits same effective assignee | No additional `Assignment` notification for B (scoped delta/count check). |
| T6 | contract | **Reassign B→C by A:** effective assignee changes to C | C receives `Assignment`; B receives none from this reassignment; A receives none if actor. |
| T7 | contract | **Self-reassign/actor becomes assignee:** A changes assignee to A | No self-notification. |
| T8 | contract | **Unassign B→None:** if/when API supports explicit clear | No `Assignment` notification to B or others in MVP (documented). If unsupported, assert current limitation/NV instead of forcing behavior. |
| T9 | contract | **Tenant/ownership guard:** actor cannot update task in foreign org (existing behavior) | `403/404` as today; no notification side effects. |
| T10 | contract | **Read/ack path:** B marks received assignment notification read | `POST /notifications/{id}/read` `204` for B, `403` for other user, `404` unknown; unread filter behavior consistent with existing API. |
| T11 | repo/integration (optional) | **Recipient validity:** assignee not an org member when assignment attempted | Define desired behavior (reject assignment vs allow but skip notify) and assert consistently (**NV-2**). |
| T12 | integration (optional, if seam exists) | **Best-effort failure:** notification insert fails (fault injection/mock) | Task mutation still succeeds; warning logged (**NV-7**, may be manual if no seam). |

### 4.3 Determinism notes
- Use fresh UUIDs per test run, isolated org/workspace/users.
- Query notifications for the specific recipient and filter by `event_type=="Assignment"` and `target_id==<task_id>` to avoid interference.
- No sleeps; `oneshot` request/response ordering is sufficient because notification insert happens in-handler before response in proposed design (as in 019) — if made async later, tests must poll/read-after-write accordingly.

---

## 5. Risks

| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| **PATCH cannot clear assignee** (`Option` merge ambiguity) | Unassign/reassign-from-assigned semantics incorrect or impossible; transition detection misleading for clear intent | High (current code fact) | Add tri-state/explicit clear contract or dedicated endpoint; document NV; tests must reflect actual supported behavior. |
| **Duplicate notifications on retried mutations** | Noisy inbox | Medium | Transition-only emission; optional unique/dedupe guard; future request idempotency keys. |
| **Notify non-member / unauthorized assignee UUID** | Confusing or leaking signal to unintended user context | Medium (depends on validation) | Validate assignee membership/workspace access before notify (and ideally before assignment persist). |
| **Race conditions on concurrent updates** | Missed or duplicate transition signals | Low–Med on small VPS, possible under concurrency | Accept in MVP; consider conditional updates/locking later. |
| **Best-effort drop after task commit** | User misses notification | Low (DB-local insert) | Structured logs/metrics hook; outbox only if required by product. |
| **`target_url` missing from API response** | Client cannot deep-link without local inference | Medium | Store stable `target_url` in DB now; optionally extend GET response. |
| **Cross-tenant leakage if guards wrong** | Security/correctness issue | Low with explicit checks | Always use route/task `organization_id`, verify membership/org on mutation, DB-gated negative tests. |
| **Routes bypass `TaskService`** | Logic duplication/drift when service evolves | Medium | Keep helper small and colocated; refactor routes to service in follow-up with tests. |

---

## 6. NVs (needs verification / decision)

- **NV-1 (unassign contract):** how clients express “clear assignee” in `PATCH /tasks/{id}` (tri-state JSON semantics) and whether 020 must support it now.
- **NV-2 (recipient eligibility):** must assignee be an org member and/or workspace member at notification time? required validation/behavior if not.
- **NV-3 (deep-link contract):** canonical `target_url` format for tasks in current frontend(s) and whether `GET /api/v1/notifications` should expose `target_url`/`organization_id`.
- **NV-4 (idempotency hardening):** accept transition-only best-effort vs add unique constraint / `ON CONFLICT DO NOTHING` / recent-window dedupe; exact uniqueness key compatible with legitimate re-assignment later.
- **NV-5 (transactional guarantee):** product need for atomic “task state change + notification row” vs best-effort post-commit insert.
- **NV-6 (concurrency control):** need for conditional update/row locking to make transition detection race-safe under concurrent edits.
- **NV-7 (fault-injection seam):** practical way to deterministically test best-effort failure without invasive refactor (trait/mock boundary or injectable repo).
- **NV-8 (product semantics extensions):** notify previous assignee on unassign/reassign? notify watchers/subscribers? notify on `TaskCompleted`/status changes? actor attribution in UI (not stored in model today).
- **NV-9 (performance/retention):** notification volume, index adequacy, and cleanup (`delete_old`) expectations on 2 vCPU/4 GB VPS.

**Multilingual verification cues (evidence prompts):** EN: “assignment transition, self-notify suppression, idempotent side effect”; pt-BR: “transição de atribuição, evitar auto-notificação, idempotência”; es: “transición de asignación, sin autnotificación, idempotencia”; de: „Zuweisungsübergang, keine Selbstbenachrichtigung, Idempotenz“; fr: « transition d’affectation, pas d’auto-notification, idempotence »; zh-CN: 「指派状态迁移、避免自通知、幂等」; ja: 「割り当て遷移・自己通知抑止・冪等性」。

---

## 7. 80/20 recommendation summary

1. **Detect transition, not presence:** notify only when effective assignee becomes a *new* non-actor user (`old != Some(new)`).
2. **Wire in `routes/tasks.rs` (create + update)** with a tiny helper, **post-persist and best-effort**, matching 019.
3. **Persist `NotificationType::Assignment`** with `target_id=task.id` and a stable relative `target_url` (workspace/task path); keep core mutation success independent.
4. **No heavy infra:** no outbox/Valkey/worker in 020; rely on Postgres row durability + logs.
5. **DB-gated deterministic tests** for create-assign, self-assign, no-op, reassign, and read/ack; explicitly gate/mark cases blocked by the PATCH unassign contract gap (NV-1).
6. **Follow-up hardening (if cheap):** assignee membership check, explicit unassign contract, idempotent insert guard, and optional `target_url` exposure in notification API.

## 8. References (local)

- `flora-api/src/routes/tasks.rs`, `flora-api/src/routes/channels.rs` (`maybe_notify_reply`, 019), `flora-api/src/routes/notifications.rs`
- `flora-core/src/models/task.rs`, `flora-core/src/models/notification.rs`
- `flora-core/src/repositories/task_repository.rs`, `flora-core/src/repositories/notification_repository.rs`
- `flora-core/src/services/task_service.rs`, `flora-core/src/services/notification_service.rs`
- `flora-core/src/traits/repository.rs`
- `flora-core/src/migrations/sql/20240818000001_init_schema.sql`
- `flora-tests/src/contract/notifications.rs`, `flora-tests/src/integration/notifications.rs`
- `docs/research/2026-11-16-019-inapp-notification-wiring-vps.md`, `docs/research/2026-11-16-019-db-gated-tests-search-notifications.md`
- Prior context: `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`
