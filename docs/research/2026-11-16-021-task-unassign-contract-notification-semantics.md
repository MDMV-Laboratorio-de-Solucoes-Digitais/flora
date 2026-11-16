# 021 — Task unassign contract & notification semantics (nullable `assignee_id`, PATCH/update)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`0dcde353a`) — **research only** (no source/commit changes).  
**Scope:** deep, high-reliability design for representing **explicit unassign/clear of a nullable field (`task.assignee_id`) in JSON PATCH/update APIs** in Rust (Axum + serde + sqlx/Postgres) and the resulting **assignment/unassignment notification semantics**. Covers: distinguishing **omitted vs `null` vs value** (`Option<Option<T>>`, double-option patterns, custom deserialize, merge-patch semantics), backward compatibility/migration, DB/update behavior, and deterministic event rules (assign, reassign, unassign, self, no-op) with simple dedup guards without heavy infra.

**Cross-check (primary):** `flora-api/src/routes/tasks.rs`, `flora-core/src/models/task.rs`, `flora-core/src/repositories/task_repository.rs`, `flora-core/src/services/task_service.rs`, and prior research `docs/research/2026-11-16-020-task-assignment-notification-vps.md` (020 assignment hook/semantics). DB schema facts referenced from 020/init-migration notes: `tasks.assignee_id` nullable UUID, `notifications` append-only rows (`event_type='Assignment'`, `target_id` task id, `target_url` optional).

**Method:** codebase inspection + standards and ecosystem sources (RFC 7396 JSON Merge Patch; serde field attributes/impl behavior; `serde_with::rust::double_option`; serde issue discussions; JSON Patch contrast; API design guidance on null-vs-absent/PATCH). Multilingual evidence/terminology cues (**EN, pt-BR, es, de, fr, zh-CN, ja**) included for contract clarity and operator communication. Claims not directly evidenced in this repo/runtime are marked **NV**.

---

## 1. Flora current state (facts)

| Area | Evidence | Consequence |
|---|---|---|
| HTTP DTOs | `CreateTaskRequest { assignee_id: Option<Uuid> }`, `UpdateTaskRequest { assignee_id: Option<Uuid>, ... }` in `routes/tasks.rs`; `CreateTaskInput`/`UpdateTaskInput` in `models/task.rs` also `Option<Uuid>`. | `Option<Uuid>` collapses **absent** and **explicit `null`** into `None` on deserialize in normal serde usage (cannot express “clear”). Create-time `None` legitimately means “unassigned”; update-time `None` is ambiguous. |
| PATCH merge (route) | `update_task`: captures `old_assignee_id`, `requested_assignee_id = req.assignee_id`; builds `Task { assignee_id: requested_assignee_id.or(old_assignee_id), ... }` then `task_repo.update(...)`. Notify only if `requested_assignee_id` is `Some(new)` (`maybe_notify_task_assignment` requires non-self transition). | **Unassign impossible today:** client cannot clear an existing assignee (sending `null`/omitting preserves old). Notification fires only on explicit requested `Some` new value, so unassign generates no event (and cannot be detected as transition). |
| PATCH merge (service) | `TaskService::update_task` does `assignee_id: input.assignee_id.or(task.assignee_id)`; `assign_task` takes `Option<Uuid>` and sets it directly (can clear) but routes bypass service. | Domain/service API also ambiguous for generic update; a dedicated “set assignee (nullable)” operation exists in service shape but is not the HTTP contract. |
| Persistence | `PgTaskRepository::{create,update}` bind `assignee_id: Option<Uuid>` to nullable `assignee_id` column (`SET assignee_id = $5`, insert `$5`) via sqlx; `Task.assignee_id: Option<Uuid>`. | DB can represent assigned/unassigned; correctness depends entirely on passing the intended `Option<Uuid>` (`Some(id)` or `None` to clear) after contract resolution. |
| 020 hook (020 doc + current route impl) | `maybe_notify_task_assignment` guards `new==old || new==actor` then inserts `NotificationType::Assignment` for recipient=new, `target_id=task.id`, `target_url=None`. Best-effort, post-persist, warn-on-error. 020 recommends transition-only emission and notes unassign out-of-scope until contract fixed (NV-1). | Deterministic assign/reassign/self/no-op is partially addressed for “set to Some” cases; **unassign transition is not representable/detectable** end-to-end. No dedupe key/unique constraint. |

**Conclusion:** the blocking issue is **representation/merge semantics**, not storage. Fixing notification semantics requires first making `old -> effective_new` observable for clear.

---

## 2. Semantics to distinguish (normative vocabulary)

For a nullable, mutable field `assignee_id`:

| Client intent | JSON member state (object field) | Meaning | Target effect |
|---|---|---|---|
| **No change** | **omitted / absent** | leave as-is | keep `existing.assignee_id` |
| **Clear/unassign** | **present with `null`** (in merge-patch style) | remove/clear value | set `assignee_id = NULL` |
| **Assign/change** | **present with value** (UUID/string) | set value | set `assignee_id = <id>` |

**Critical:** omission ≠ `null` for PATCH/partial update (EN: omitted vs null; pt-BR: ausente/omitido vs nulo; es: ausente/omitido vs nulo; de: fehlend/ausgelassen vs `null`; fr: absent/omis vs `null`; zh-CN: 缺省/未提供 vs `null`; ja: 欠落/未指定 vs `null`). RFC 7396 (JSON Merge Patch, `application/merge-patch+json`) defines: members absent in patch are unchanged; members present replace target; **`null` means remove** that member (not “set to JSON null” as a storable domain value unless removal is the intent) and arrays/non-objects replace wholesale. Therefore merge-patch is a good fit for “clear means remove field” but cannot express “store an explicit null as distinct data value” when `null` is overloaded to delete (not an issue here, since unassigned == NULL in DB).

**Pitfall (widely reported):** many stacks collapse absent→`null`/default in DTOs or serializers, silently turning “no change” into “clear” or vice versa. Contract, deserialization, and merge must preserve the three states end-to-end.

---

## 3. Representation options in Rust (Axum + serde + sqlx/Postgres)

### 3.1 `Option<Option<T>>` double-option (recommended primitive)

- **Encoding:** outer = presence, inner = nullable value.
  - `None` → field absent (no change)
  - `Some(None)` → field present as `null` (clear)
  - `Some(Some(v))` → field present with value (set)
- **Serde requirements:** plain `Option<T>` cannot distinguish absent vs `null` when both map to `None`. For double option, field **must** be annotated:
  - `#[serde(default)]` so missing key deserializes to outer `None` (custom `deserialize_with` opts out of serde derive’s implicit missing-Option default behavior; explicit default restores it).
  - `#[serde(deserialize_with = "…")]` (or equivalent) that **deserializes when key is present** to `Some(Option::deserialize(...))`, yielding `Some(None)` for JSON `null` and `Some(Some(v))` for value.
  - `#[serde(skip_serializing_if = "Option::is_none")]` on responses/echo if you must omit untouched fields (request DTOs usually only deserialize).
- **Implementations:**
  - **`serde_with::rust::double_option`** (`#[serde(with = "::serde_with::rust::double_option")]` or `serde_as`-free field attrs) — well-known helper encoding exactly this contract (docs show `None`/`Some(None)`/`Some(Some(5))`).
  - **Manual helper** (common pattern from serde discussions, e.g. `deserialize_some` mapping `T -> Some(T)` over `Option<_>` deserialization; conceptually `Option::<T>::deserialize(de).map(Some)` for present keys).
- **Pros:** explicit, zero runtime infra, maps 1:1 to tri-state intent and to DB `Option<Uuid>` after merge; easy to unit-test. **Cons:** `Option<Option<T>>` is less self-documenting at type sites; must enforce attributes consistently (easy footgun if `default` omitted).

### 3.2 Explicit tri-state enum (clearer domain type)

```rust
// illustrative
enum PatchField<T> { Absent, Null, Value(T) } // or Missing/Clear/Set
```
- Custom `Deserialize`/`Serialize` to treat missing as `Absent` (needs `#[serde(default)]`/container handling) and JSON `null` as `Null`; helpers like `apply(self, current: Option<T>) -> Option<T>` or `is_noop(&current)` reduce merge bugs.
- **Pros:** self-describing, harder to misuse than nested `Option`; aligns with some API guides. **Cons:** more code/boilerplate than double-option helper; must get “missing” detection right (field-level default vs custom visitor).

### 3.3 JSON Merge Patch document (`application/merge-patch+json`, RFC 7396)

- Accept raw/partial document (e.g. `serde_json::Value`, `Json<Value>`, or typed struct deserialized from merge-patch payload) and apply merge semantics: absent = unchanged, present null = remove/clear, present value/scalar/array = replace.
- For a flat field like `assignee_id` this is equivalent in effect to tri-state handling; for nested objects it gives standard recursive merge.
- **Pros:** standards-based, interoperable; `null`-means-clear is explicit. **Cons:** requires choosing/communicating media type & behavior (arrays replace, cannot set a field to meaningful `null` distinct from delete — irrelevant here); typed Rust structs still need tri-state representation unless you merge via `Value`.

### 3.4 JSON Patch (`application/json-patch+json`, RFC 6902)

- Operation list (`add`/`remove`/`replace`/`test`/…). `remove /assignee_id` (or set-null via `replace` if null were a valid stored value) is explicit and unambiguous.
- **Pros:** precise, supports complex/array ops and optimistic `test`. **Cons:** heavier client/server contract and implementation than needed for one nullable scalar; overkill for 80/20.

### 3.5 Alternative API shapes (avoid tri-state parsing ambiguity)

- **Dedicated action/endpoint:** `PUT/PATCH /tasks/{id}/assignee` with body `{ "assignee_id": null | "<uuid>" }` (or `DELETE /tasks/{id}/assignee`) — value is always authoritative, no “omit means unchanged” confusion for that field.
- **Explicit flag:** `clear_assignee: true` alongside/ instead of `assignee_id` (must define precedence/conflict errors).
- **Wrapper:** `assignee: { "set": null | "<uuid>" }` or discriminated object.
- **Pros:** very clear wire intent, easy backward-compatible addition. **Cons:** expands surface area; generic `PATCH /tasks/{id}` still needs a story for `assignee_id` unless removed.

### 3.6 What is **not** sufficient

- `Option<Uuid>` in `Update*` DTO merged with `.or(existing)` (current) — cannot clear.
- Treating `null` and absent as equivalent by policy (some APIs do) — makes unassign indistinguishable from no-op on PATCH; conflicts with need to explicitly unassign.
- Relying only on `serde(default)` without a presence-preserving deserialize for nullable fields — default handles absence but not null-vs-absent for the nullable case.

---

## 4. Backward compatibility & migration (80/20)

**Current behavior (implicit contract):** update omits `assignee_id` or sends `null` → no change to assignee (because `None.or(existing)`); sending a UUID sets/changes it. Clients may depend on `null` meaning “ignore”.

**Migration options:**

| Option | Change | Compat impact | Notes |
|---|---|---|---|
| **A. Adopt tri-state on existing `assignee_id` (merge-patch semantics)** | `assignee_id: Option<Option<Uuid>>` (or enum) + `default` + double-option deserialize; merge: `Absent => keep`, `Null => clear`, `Value(v)=>set`. | **Behavioral change:** `{"assignee_id": null}` becomes **clear** (was previously treated as no-op/absent in effect). Omitted remains no-op. | Most correct/standard-aligned (RFC 7396) but **breaking for clients that sent `null` to mean “leave unchanged”**. Requires release notes/versioning/client update. |
| **B. Add explicit clear channel, keep legacy `assignee_id` meaning** | Keep `assignee_id: Option<Uuid>` as “if Some set, if None ignore (legacy)”; add `clear_assignee: bool` or new endpoint/action. | Backward compatible for existing payloads. | Two ways to mutate; must reject conflicting combos (`assignee_id` Some + `clear_assignee=true`). Temporary complexity. |
| **C. New versioned/media-typed contract** | e.g. accept `application/merge-patch+json` with strict merge semantics, keep old behavior for `application/json` (or `v2` route). | Compatible if old clients unchanged. | Clear separation but more maintenance; must document. |
| **D. Service-level `assign_task` endpoint as source of truth** | Expose explicit set/clear operation using `TaskService::assign_task(task_id, Option<Uuid>)` semantics. | Additive. | Bypasses tri-state in generic PATCH for this field; good incremental step. |

**80/20 recommendation:** prefer **A (tri-state / merge-patch meaning) on `PATCH`** because it matches user intent, standards, and enables deterministic unassign events; gate with explicit migration note (clients must stop sending spurious `null` to mean unchanged). If immediate compat is mandatory, ship **D or B** first, then converge to A. In all cases, **treat omitted as no-op** (never clear on omission).

---

## 5. DB/update behavior (sqlx/Postgres)

- `tasks.assignee_id` is nullable; repository already binds `Option<Uuid>` (`Some(id)` or `None`) for insert/update. **No schema change** required for unassign/assign values.
- **Resolved merge must produce final `Option<Uuid>`:**
  - absent → `existing.assignee_id`
  - clear → `None`
  - set(v) → `Some(v)`
- Use resolved value for: persisted `Task`, transition detection, and response. Avoid reusing ambiguous raw request `Option` after introducing tri-state.
- **Concurrency (NV):** read-then-write can race; transition observed may be stale vs concurrent commits. Cheap mitigations if needed later: conditional update (`UPDATE … WHERE id=$1 AND assignee_id IS NOT DISTINCT FROM $old RETURNING …`) / row lock in tx, or accept rare duplicate/missed under MVP posture (020).
- **Transactional coupling (NV):** 020 keeps notification best-effort post-persist; atomic task+notification would require tx-aware repo/API refactor or outbox — not required for 80/20.

---

## 6. Deterministic assignment/unassignment notification semantics

### 6.1 Inputs (must be known at decision point)

- `actor: Uuid` (authenticated mutator)
- `old: Option<Uuid>` (assignee before mutation, from loaded/persisted prior state)
- `new: Option<Uuid>` (**effective** assignee after resolved merge/persist)
- `task_id`, `org_id`, `workspace_id` (tenant/link context)
- operation kind (create/update) — useful for reasoning but rules below are transition-based

### 6.2 Canonical transition function (single `Assignment` event type, 80/20)

Let `recipient = new` when `new.is_some() && new != old`; else `None`. Emit **at most one** `Assignment` notification to `recipient` iff:

1. `recipient.is_some()` (effective **new assignment** occurred: unassigned→assigned or reassigned A→B),
2. `recipient != Some(actor)` (no self-notify),
3. task mutation is authorized and within same tenant/org (and not a delete); recipient eligibility holds (**NV**: org/workspace membership).

**Event classes:**

| Case | `old` | `new` | Transition | Notify (MVP) |
|---|---|---|---|---|
| No-op / unchanged | X | X (==old) | none | **none** |
| Create unassigned | — | `None` | none | **none** |
| Create assigned | — | `Some(B)` | assign | `B` if `B!=actor` |
| Create self-assigned | — | `Some(A)` actor | self | **none** |
| Assign `None→B` | `None` | `Some(B)` | assign | `B` if `B!=actor` |
| Reassign `A→B` | `Some(A)` | `Some(B)` | reassign | **`B` only** if `B!=actor` (do not notify `A` in MVP) |
| Self-reassign / actor takes it | `Some(A)` | `Some(actor)` | self/transition | **none** |
| **Unassign `A→None`** | `Some(A)` | `None` | unassign/clear | **none for `Assignment` in MVP** (no new assignee recipient; unassign-specific event/notify-old is a product extension, NV) |
| Field-irrelevant updates | same | same | none | **none** |
| Delete (soft) | any | n/a | — | **none** |

This matches 020’s “notify newly assigned non-actor user” and becomes fully correct for unassign only once `new=None` is observable as an explicit transition (not conflated with absent).

### 6.3 Simple dedup guards (no heavy infra)

- **Required (logical):** compute from **effective `old -> new` diff**; emit only on real transition to a new assignee. Prevents duplicates/noise from no-op updates and repeated payloads that do not change effective state.
- **Best-effort transport/idempotency (cheap):** post-persist insert, warn-and-continue (020 posture). Accept that client-retried *creates* may create duplicate tasks/notifications absent request-idempotency keys (core concern).
- **Optional cheap DB guard:** if schema/repo change allowed, `INSERT ... ON CONFLICT DO NOTHING` on a chosen uniqueness key (e.g. `(user_id, event_type, target_id)` for a single active signal — but note legitimate future re-assign to same user may warrant another notification; reconcile with product) or a recent-window existence check. **MVP:** rely on transition-only logic; no outbox/streams/Valkey/worker.
- **Race caveat:** concurrent writers can yield rare duplicate/missed signals; acceptable for small VPS MVP unless stronger ordering required (NV).

### 6.4 Payload/tenant

- Keep `NotificationType::Assignment`, `target_id=task.id`, `organization_id` from trusted context. Prefer stable relative `target_url` (e.g. `/workspaces/{workspace_id}/tasks/{task_id}`) when available; 020/current impl uses `None` (API also omits `target_url`) — **NV** for canonical deep-link exposure.
- Never trust client-supplied org/user for recipient binding; validate task/org membership as applicable.

---

## 7. 80/20 API & notification design (recommended)

1. **Contract (fix the root cause):** make `PATCH /tasks/{id}` `assignee_id` **tri-state** (double-option or `PatchField`) with merge-patch meaning: omitted=no change, `null`=clear, UUID=set. **Migrate compat** per §4 (announce `null` semantics change or provide additive clear path/version).
2. **Centralize merge:** resolve `effective_assignee: Option<Uuid>` once from `(existing.assignee_id, patch_state)` and pass to DB update, response construction, and event detection. Apply same rule in `TaskService::update_task` / DTOs to avoid route-service drift (routes currently bypass service).
3. **Event decision:** `if effective_assignee != existing.assignee_id { on_assign_transition(old, new, actor, ...) }` where `on_assign_transition` notifies only when `new` is `Some(uid)` and `uid!=actor` (assign/reassign). `Some(_) -> None` is a recognized **unassign transition** with **no Assignment notification in MVP** (logged/metric-able if useful). No-op when equal.
4. **Create path:** treat as transition from `None` to requested `assignee_id`.
5. **Non-fatal side effects:** keep notification insert best-effort post-persist with structured warnings (task/org/workspace/actor/old/new/recipient, error), never fail core task mutation.
6. **Validation hardening (cheap, recommended):** when assigning, verify assignee is eligible org/workspace member before persist/notify; reject or skip-notify deterministically (**NV** policy).
7. **Tests first-class for contract:** DB-gated tests must assert explicit-clear behavior once supported; until then mark blocked by NV-1 (do not fake it).

---

## 8. Deterministic test cases (proposed)

Style: DB-gated contract/integration tests (follow 019/020 harness patterns: `#[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL …"]`, fresh UUID fixtures, scoped assertions by recipient+`event_type=="Assignment"`+`target_id==task_id`, no sleeps).

| ID | Case | Setup / action | Expected (deterministic) |
|---|---|---|---|
| C1 | **Create unassigned** | `POST` task without assignee | 201, `assignee_id=null`, no `Assignment` notification. |
| C2 | **Create assigned** | `POST` task `assignee_id=B`, actor A | 201, B gets exactly 1 `Assignment` for task; A none. |
| C3 | **Create self-assigned** | `POST` assignee A by A | 201, no self-notification. |
| A1 | **Assign `null→B` via PATCH** | task unassigned; `PATCH {"assignee_id":"B"}` | 200, persisted `B`, B notified once. |
| N1 | **No-op omitted** | assigned B; `PATCH {"title":"x"}` (omit assignee) | 200, still `B`, **no** new notification (delta=0). |
| N2 | **No-op same value** | assigned B; `PATCH {"assignee_id":"B"}` | 200, **no** new notification. |
| R1 | **Reassign `B→C`** | assigned B; actor A patches to C | 200, C notified once, B none, A none. |
| S1 | **Self-assign on update** | assigned B; actor A patches to A | 200, assignee A, **no** self-notification. |
| U1 | **Explicit unassign `B→null`** *(requires tri-state/clear contract)* | assigned B; `PATCH {"assignee_id":null}` with merge-patch semantics | 200, persisted `null`, **no `Assignment` notification** in MVP, transition recognized as unassign (assert no Assignment rows for task/recipient set as applicable). |
| U2 | **Omit after unassign-capable contract** | after unassign state; `PATCH` omits assignee | 200, remains `null`, no notification. |
| G1 | **Tenant/auth guards** | cross-org/unauthorized update or notify path | 403/404 as today, no side-effect rows in foreign scope. |
| G2 | **Read/ack** | recipient reads + marks notification | list includes Assignment with correct target; mark-read ownership enforced (existing behavior). |
| D1 | **Dedup/idempotency sanity** | repeat identical effective no-op PATCH | notification count unchanged. |
| D2 *(optional, NV)* | **Retry duplicate request** | replay create at app boundary if idempotency supported | no duplicate logical assignment (or document residual risk). |

**Unit-level (fast, no DB):** pure tests for patch-merge resolver (`Absent/Null/Value × existing`) and `assignment_recipient(old,new,actor)` / `transition_kind` matrix including unassign.

---

## 9. Risks

| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| **Absent/null conflation** in DTO/merge (current bug) | Cannot unassign; wrong transitions/notifications | High (fact) | Tri-state representation + explicit merge rules + unit/contract tests (U1,N1). |
| **Backward-compat break** (`null` becomes clear) | Existing clients accidentally unassign | Medium on migration | Version/media-type strategy or additive clear flag/endpoint; changelog & client audit. |
| **Serde attribute footguns** (`default`/`deserialize_with` misuse) | `Some(None)` never produced or missing treated wrongly | Medium | Use `serde_with::rust::double_option` or well-tested helper; round-trip tests for `{}`, `{"assignee_id":null}`, `{"assignee_id":"…"}`. |
| **Double-notify / retries** | Noisy inbox | Low–Med | Transition-only emission; optional conflict-free insert guard; request idempotency later. |
| **Self/tenant leakage** | Wrong recipient or cross-tenant signal | Low–Med | Actor/recipient equality check, trusted `org_id`, membership validation, scoped tests. |
| **Concurrent updates** | Missed/duplicate transition | Low on VPS, possible | Accept MVP; conditional update/locking if needed. |
| **Best-effort insert failure after commit** | Missed notification | Low (DB-local) | Structured warn logs/metrics; outbox only if required. |
| **Route vs service drift** | Inconsistent behavior | Medium (routes bypass service today) | Share resolver/helper; refactor toward service with tests. |
| **Product ambiguity on unassign** | Should previous assignee be notified? | Med (product) | Keep MVP as no Assignment on unassign; define separate event/type if needed (NV). |

---

## 10. NVs (needs verification / decision)

- **NV-1 (contract & migration):** adopt merge-patch tri-state on `assignee_id` now (breaking `null` semantics) vs additive `clear_assignee`/dedicated endpoint/versioned media type.
- **NV-2 (recipient eligibility):** must assignee be an active org/workspace member at persist/notify time; reject assignment, allow+skip notify, or both.
- **NV-3 (deep link/API):** canonical `target_url` for tasks and whether `GET /notifications` should expose `target_url`/org.
- **NV-4 (unassign semantics):** notify previous assignee on unassign/reassign? introduce distinct event type vs reuse `Assignment`?
- **NV-5 (idempotency):** accept transition-only MVP vs unique index/`ON CONFLICT DO NOTHING` key compatible with legitimate re-assignment.
- **NV-6 (consistency):** need atomic task+notification (same tx/outbox) vs best-effort post-commit.
- **NV-7 (concurrency):** conditional `UPDATE ... WHERE assignee_id IS NOT DISTINCT FROM $old` / locking for race-safe transitions.
- **NV-8 (scope):** extend same tri-state pattern to other nullable fields and align `TaskService`/`UpdateTaskInput` with route DTOs.
- **NV-9 (operational):** notification volume/retention/cleanup on 2 vCPU/4 GB VPS.

**Multilingual verification cues:** EN “omitted vs null vs value, unassign transition, idempotent no-op”; pt-BR “campo ausente vs nulo vs valor, transição de desatribuição, idempotência/sem duplicar”; es “campo ausente vs nulo vs valor, transición de desasignación, idempotencia”; de „fehlendes Feld vs null vs Wert, Aufhebung der Zuweisung, Idempotenz/keine Duplikate“; fr « champ omis vs nul vs valeur, transition de désaffectation, idempotence/sans doublon »; zh-CN 「字段缺省 vs null vs 有值、取消指派/清空转移、幂等去重」; ja 「未指定(欠落)・null・値の区別、割り当て解除の遷移、冪等性/重複抑止」。

---

## 11. 80/20 summary

- **Root fix:** represent `assignee_id` updates as **tri-state** (`absent | null | value`) — e.g. `Option<Option<Uuid>>` with `serde(default, deserialize_with=double_option, skip_serializing_if=Option::is_none)` or an explicit `PatchField` enum; **never** merge raw `Option<Uuid>` with `.or(existing)` for PATCH.
- **Merge rule:** `Absent -> keep existing`, `Null -> clear (None)`, `Value(v) -> set Some(v)`; persist and reason over the **effective** value.
- **Notification rule:** emit `Assignment` only on **effective transition to a new non-actor assignee** (create assign, assign, reassign-to-new). **No-op and unassign** produce no `Assignment` in MVP; self never notifies. Keep best-effort, post-persist, tenant-safe, with transition-only logical dedup (no heavy infra).
- **Migration:** choose compat strategy for `{"assignee_id":null}` (correct clear vs legacy ignore) explicitly; align route, service, and models.

## 12. References

- **Standards:** RFC 7396 JSON Merge Patch (`application/merge-patch+json`, null removes member); RFC 6902 JSON Patch (contrast); RFC 5789 PATCH.
- **Rust/serde:** serde field attrs (`default`, `deserialize_with`, `skip_serializing_if`); `serde_with::rust::double_option` (distinguish missing/unset/existing); serde-rs/serde#984/#1042 discussions; common `deserialize_some`/manual double-option and `Patch` enum patterns.
- **Design practice:** null-vs-absent semantics in PATCH APIs, merge-patch null pitfalls, explicit-clear alternatives (dedicated endpoint/flag/update mask); multilingual terminology as in §10.
- **Local:** `flora-api/src/routes/tasks.rs`, `flora-core/src/models/task.rs`, `flora-core/src/repositories/task_repository.rs`, `flora-core/src/services/task_service.rs`, `docs/research/2026-11-16-020-task-assignment-notification-vps.md`.
