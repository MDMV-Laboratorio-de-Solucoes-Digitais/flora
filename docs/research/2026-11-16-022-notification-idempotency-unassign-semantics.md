# 022 — Notification idempotency/dedup hardening & assignment/unassignment semantics (VPS, Postgres-first)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (research-only; no source/schema changes)  
**Topic:** harden in-app notification correctness for `Assignment` (and applicable patterns) with minimal infra: idempotency/dedup strategies (query-before-insert vs DB-enforced uniqueness/`ON CONFLICT`/upsert, unread vs all-history scope), race-condition/concurrency behavior, schema/migration implications, and transaction-coupling trade-offs (best-effort vs in-transaction), plus assignment/unassignment notification semantics and transition rules.

**Method:** codebase cross-check + primary/authoritative web sources (PostgreSQL docs) + product/operational references (GitHub/Linear/Asana signals, outbox/upsert practice articles). Multilingual discovery pass executed (EN, pt-BR, es, de, fr, zh-CN, ja): actionable primary product internals remained mostly EN; non-EN sources were sparse/generic for this specific topic and no contradictions to cited primaries were found. Where evidence is product/community-level rather than normative spec, claims are marked **NV**.

---

## 1. Flora baseline (cycle 021, cross-checked)

| Area | Evidence (local) | Correctness implication |
|---|---|---|
| Assignment emission | `flora-api/src/routes/tasks.rs`: `maybe_notify_task_assignment` emits only when `new_assignee_id` becomes effective and `Some(assignee_id) != old_assignee_id && assignee_id != actor_id`. Create notifies on initial effective assignment; update uses tri-state PATCH semantics (`assignee_id` omitted = no change, `null` = explicit unassign, value = set) and computes `new_assignee_id = req.assignee_id.unwrap_or(old_assignee_id)`. | Transition-only for **assign/reassign-to-new-user** is implemented. **Explicit unassign (`Some -> None`) produces no notification** (function returns early when `new_assignee_id` is `None`). Self-assignment/no-op suppressed. |
| Best-effort side effect | Task create/update call notify helper and on error `tracing::warn!` without failing core mutation (`routes/tasks.rs` create/update handlers). | Core task write availability prioritized; notification may be dropped on insert failure after persist (at-most-once *attempt*, not guaranteed delivery). |
| Dedup guard (current) | `routes/tasks.rs` + `PgNotificationRepository::has_unread_for_target(user_id,event_type,target_id)` + trait in `flora-core/src/traits/repository.rs`: query-before-insert `SELECT EXISTS(... WHERE user_id=$1 AND event_type=$2 AND target_id=$3 AND is_read=false)`; on true, skip insert. | **Race-prone TOCTOU:** two concurrent requests can both observe no unread row and both insert. Scope is **unread-only** (a read notification does not suppress a new unread duplicate). No DB uniqueness constraint enforces dedup. |
| Persistence/model | `PgNotificationRepository::create` does plain `INSERT ... RETURNING ...` (`flora-core/src/repositories/notification_repository.rs`). `Notification` has `id, organization_id, user_id, event_type, target_id, target_url, is_read, created_at, updated_at` (`flora-core/src/models/notification.rs`); `NotificationType::Assignment` exists. | Rows are append-only event records by default; dedup policy must be made explicit (suppress insert, upsert/refresh, or allow duplicates) because schema does not constrain it. |
| Schema/indexes | Init migration `20240818000001_init_schema.sql`: `notifications` table, FKs, event_type check, indexes on `(organization_id)`, `(user_id)`, `(event_type)`, `(target_id)`, partial `(is_read) WHERE is_read = FALSE`, `(organization_id, created_at DESC)`. **No unique/partial-unique constraint** on `(user_id,event_type,target_id,...)`. RLS enabled (app-level scoping noted). `delete_old` exists (retention). | DB cannot currently reject duplicate assignment notifications; any hard guarantee requires schema + insert semantics change. Existing indexes help lookups but do not prevent duplicates. |
| API/read semantics | `GET /api/v1/notifications` lists all or unread (`unread_only`) for authenticated user; response omits `target_url`/`organization_id` (`routes/notifications.rs`). `mark_as_read` enforces ownership. | Product meaning of “dedup” affects inbox UX (collapse vs repeated rows) and must be consistent with read-state transitions. |

**Cross-check with prior Flora research:** `docs/research/2026-11-16-020-task-assignment-notification-vps.md`, `2026-11-16-021-task-unassign-contract-notification-semantics.md`, and `2026-11-16-021-macro-search-parity-unassign-notifications.md` already identify: unassign contract fixed in 021 (tri-state) but unassign event intentionally not emitted; unread query-before-insert dedup is best-effort/racy; DB-enforced dedup and unassign-recipient/product semantics remain NV/product decisions.

---

## 2. Problem framing

### 2.1 Correctness goals (target)

For a given logical notification intent, define whether duplicates are allowed:

- **At-most-once per intent** (idempotent create): retries/concurrent duplicates must not create extra rows for the same intent key.
- **At-most-one active/unread signal**: suppress/coalesce while an unread notification exists (current Flora intent), but allow a new one after read/clear or after a new intent epoch.
- **Append-only history**: every real transition emits a row (dedup only against accidental replay, not against legitimate repeated transitions).

The answer differs by scope (unread vs all history) and by intent key (e.g. same assignee re-assigned later is often a *new* intent).

### 2.2 Race condition / concurrency (current design)

`has_unread_for_target` (read) and `create` (write) are separate statements on separate repository calls: under concurrency/retry (double-submit, overlapping requests, future workers), two actors can interleave `SELECT EXISTS -> false` before either `INSERT` commits, producing duplicates. This is the classic TOCTOU/idempotency gap; application checks alone cannot guarantee uniqueness without serialization/atomic DB enforcement. PostgreSQL `INSERT ... ON CONFLICT` provides atomic conflict handling for arbiter constraints/indexes, and partial unique indexes constrain only rows matching a predicate (useful for “only while unread/active”). [PG-INSERT-ON-CONFLICT], [PG-PARTIAL-INDEX]

### 2.3 Transaction-coupling spectrum (no heavy infra)

| Mode | Semantics | Pros | Cons / fit for Flora VPS |
|---|---|---|---|
| **Post-persist, best-effort (current)** | Task mutation commits; notification insert attempted after; failure logged, request still succeeds. | Protects core write latency/availability; simple; matches 019/020 posture. | Can miss notifications on insert/infra failure; cannot be atomic with task state without outbox. |
| **Same-DB transaction (coupled)** | Insert/update task and notification in one transaction (commit together / rollback together). | Atomic visibility: no “assigned but never notified” due to post-commit failure; enables transactional `ON CONFLICT` dedup tied to mutation. | Requires tx-aware repo/service APIs/refactor; notification failure can fail/rollback core mutation (policy choice); longer tx contention on small VPS. |
| **Transactional outbox (deferred)** | Write intent/event row in same tx as mutation; separate worker publishes/creates inbox row. | Reliable eventual delivery + atomic intent capture. | Extra table/worker/polling/ops; heavier than 80/20 target. (Background: [OUTBOX-MILAN], [EVENTDRIVEN-OUTBOX], [OUTBOX-REVAMP] — NV for Flora need.) |

**80/20:** keep **best-effort, non-fatal** for core UX unless product requires guaranteed delivery; gain correctness primarily via **atomic DB-enforced dedup on insert** (works even when called outside task tx, as long as insert itself is atomic/conflict-safe).

---

## 3. Dedup/idempotency strategies (Postgres-first)

### 3.1 Query-before-insert (current)

- **How:** `EXISTS` unread row for `(user_id,event_type,target_id)` then `INSERT`.
- **Scope now:** unread-only, same user/type/target.
- **Guarantee:** none under concurrency; reduces common duplicates only in low contention.
- **Use when:** advisory suppression only, never as sole correctness mechanism.

### 3.2 DB-enforced uniqueness + `INSERT ... ON CONFLICT DO NOTHING`

`ON CONFLICT` specifies an alternative action on unique/exclusion constraint violation; `DO NOTHING` avoids inserting the conflicting row (idempotent insert). A *conflict target* can be column inference, `ON CONSTRAINT`, and for inference may include a partial-index predicate (`WHERE index_predicate`) matching the arbiter index. [PG-INSERT-ON-CONFLICT]

**Implication:** a plain unique index prevents duplicates for *all* time (all history) for the key; a **partial unique index** prevents duplicates only for rows satisfying its predicate (e.g. active/unread subset), which matches “at-most-one active signal” without blocking future legitimate notifications after state change. [PG-PARTIAL-INDEX], [HEROKU-PARTIAL], [DEV-RACE]

### 3.3 Upsert/refresh (`ON CONFLICT DO UPDATE`)

Instead of suppressing, update existing row (e.g. bump `updated_at`/`created_at` semantics, set `is_read=false`, refresh `target_url`) to coalesce repeated intents into one visible item. Requires careful definition (do we want “latest activity time” or preserve original?) and, for partial arbiters, the conflicting row must satisfy the inferred target/index conditions. Deterministic under concurrency but changes inbox semantics (coalescing vs preserving event history). [PG-INSERT-ON-CONFLICT]

### 3.4 Unread vs all-history scope (design choice)

| Scope | Enforcement shape | Behavior | Trade-off |
|---|---|---|---|
| **Unread/active only** | Partial unique index, e.g. `UNIQUE (user_id, event_type, target_id) WHERE is_read = false` (conceptual) | At most one unread Assignment per user+task; after read, a later legitimate transition can create a new unread row. | Matches current guard intent and avoids permanent suppression; allows historical multiplicity over time (read then reassigned again) — often desirable. |
| **All history (strict per key)** | Non-partial `UNIQUE (user_id,event_type,target_id)` (or broader key) | Only ever one row per key forever (unless deleted). | Suppresses legitimate future notifications for same key (e.g. re-assign same user again later) unless key includes an epoch/version — usually **wrong** for `Assignment` without versioning. |
| **Per-transition/epoch** | Include transition identity in key/row (e.g. assignment version, causal mutation id, or actor+timestamp bucket) + unique constraint on that identity | True idempotency for retries of the *same logical transition* while allowing distinct transitions. | Needs schema/producer changes; highest correctness, more design work. |

**Flora-specific note:** `Assignment` is naturally about transitions. “Same user, same task, unread exists” is a strong dedup candidate; “same user, same task, previously read, assigned again later” is typically a **new** notifiable event and must not be permanently blocked by an all-history key.

---

## 4. Schema/migration implications (feasibility, 80/20)

No migrations applied in this research; feasibility assessment only.

1. **Add partial unique index for unread assignment dedup (high leverage):**  
   Candidate (illustrative): `CREATE UNIQUE INDEX CONCURRENTLY ... ON notifications (user_id, event_type, target_id) WHERE is_read = false;`  
   - Enforces at-most-one unread per (user,type,target) atomically at DB level. [PG-PARTIAL-INDEX], [PG-INSERT-ON-CONFLICT]  
   - Must validate against existing data (pre-existing duplicates, if any, will block index creation) and choose online migration strategy (`CONCURRENTLY` in non-transactional migration step as required by ops/sqlx practices — **NV** for this repo’s migration runner behavior).  
   - Consider scoping predicate to `event_type = 'Assignment'` as well if semantics differ by type (partial predicate can include it, but planner/index matching and write overhead trade-offs apply — **NV**).

2. **Repository insert path change:** `PgNotificationRepository::create` currently plain `INSERT`. To use DB enforcement, switch to `INSERT ... ON CONFLICT (user_id, event_type, target_id) WHERE is_read = false DO NOTHING RETURNING ...` (or `ON CONSTRAINT ...`) and surface “inserted vs suppressed” (e.g. `Option<Notification>` / bool) so callers/metrics can distinguish. Trait signature change ripples through impl/service/tests. [PG-INSERT-ON-CONFLICT]

3. **Keep or remove app-level precheck:** once insert is conflict-safe/idempotent, `has_unread_for_target` becomes an optimization/read helper rather than correctness guard (can be removed from hot path to avoid extra round-trip, or retained for cheap skip — **NV** micro-optim).

4. **`mark_as_read` interaction:** clearing unread state “releases” the partial-unique slot, allowing a future transition to create a new unread notification — desired for unread-scope dedup, but confirm UX (users may receive another ping after reading if reassigned again later). No schema change needed beyond index.

5. **Retention/cleanup:** `delete_old` does not affect unread partial unique semantics except by removing rows; ensure cleanup policy does not delete unread rows needed for dedup unless intentional.

6. **Broader/generalized dedup:** per-event-type policies or epoch/version columns require additive schema/migration and product rules; defer unless required.

---

## 5. Assignment/unassignment semantics (product & transitions)

### 5.1 Transition matrix (recommended deterministic core)

Assuming actor `A`, previous assignee `O` (old), new assignee `N` (new), all within same authorized org/workspace context:

| Transition | Condition | Recommended 022 default | Rationale |
|---|---|---|---|
| **Assign (none -> N)** | `O=None, N=Some(u), u!=A` | Notify `N` (`Assignment`) | Direct responsibility created. |
| **Reassign (O -> N, O!=N)** | `O=Some(o), N=Some(n), n!=o, n!=A` | Notify `N` only | `N` gains ownership; notifying `O` is a separate product event (often “unassigned/reassigned away”) not covered by `NotificationType::Assignment` today. |
| **Unassign (O -> None)** | `O=Some(o), N=None` | **No notification (MVP/default)** | Recipient/meaning ambiguous (notify former assignee? creator? actor?) and no dedicated type/contract in Flora; avoids inventing semantics. Revisit if product requires it. |
| **No-op** | `O==N` | No notify | Not a transition. |
| **Self-assign/self-transition** | effective new recipient `== A` | No notify | Avoid self-noise (common pattern). |
| **Delete/soft-delete task** | task removed from active use | No assignment notify | Not an assignment transition. |

**Flora status:** core of this matrix for `N=Some` is implemented (021); unassign is representable but intentionally silent — consistent with “default no notification” until decided.

### 5.2 External/product signals (not identical systems, use as triangulation)

- **GitHub:** users are automatically subscribed/notified in contexts such as being assigned to an issue/PR (default subscription/notification reasons). Explicit, authoritative documentation on whether *unassignment/removal* generates a notification to the former assignee was not established from fetched docs in this pass (**NV**); behavior may depend on subscription state/events. [GH-NOTIFS-ABOUT], [GH-ASSIGN]
- **Linear:** users are automatically subscribed when assigned to an issue; notification settings group status/other changes but assignment-removal-specific notify behavior not explicitly documented in fetched page (**NV**). [LINEAR-NOTIFS], [LINEAR-ASSIGN]
- **Asana (community signal):** users have requested notifications when a task becomes unassigned (to alert assigner/owner of “no responsible person”), implying this is not a universal default behavior and is a deliberate product choice (**NV**, community/forum evidence, not official spec). [ASANA-UNASSIGN-FORUM]
- **Macro (from prior Flora research, NV):** `TaskAssigned` exists; no explicit unassign notification variant found in inspected artifacts; treat as unknown/product-specific. [FLORA-021-MACRO-SEARCH]

**Interpretation:** “notify on (re)assignment to the new assignee” is the common, low-risk baseline. “Notify on unassign” and “notify previous assignee on reassignment” are **optional, recipient-sensitive** events requiring explicit product semantics and possibly distinct event types/reasons.

### 5.3 Unassign event decision space (cycle 022)

| Option | Who gets notified | Pros | Cons / prerequisites |
|---|---|---|---|
| **U0: No event (status quo/default)** | nobody | Simplest, no ambiguity, no new schema/type; matches current 021 behavior. | Former assignee/assigner may miss loss of ownership; auditability limited to task history (if any). |
| **U1: Notify former assignee** | old assignee (if not actor?) | Clear handoff signal. | Can be noisy/unwanted; needs guard against self-unassign and definition when actor==old; may warrant distinct type (e.g. `Unassignment`) rather than overloading `Assignment`. |
| **U2: Notify creator/assigner/watcher** | task creator or actor-chosen stakeholder | Surfaces “task now unowned”. | Recipient derivation rules needed; multiple stakeholders possible. |
| **U3: Record state event only (no inbox ping)** | none in inbox; rely on activity/audit/timeline | Preserves history without interruption. | Requires activity model/UI Flora may not have. |

**80/20 recommendation:** keep **U0** for 022 unless a concrete stakeholder need is accepted; if added, define recipient + event type + dedup key explicitly and gate by actor/transition rules.

---

## 6. 80/20 options for cycle 022

### Option A (recommended): DB-enforced unread dedup + keep current semantics

- **DB:** add partial unique index for unread notifications on dedup key (at minimum `user_id,event_type,target_id` where `is_read=false`, refined as needed). Preflight/clean duplicates if present (**NV**).
- **Repo:** change `create` to `INSERT ... ON CONFLICT ... DO NOTHING RETURNING ...` (or equivalent) and handle “conflict/no row” as successful idempotent suppression. Optionally drop pre-insert `has_unread_for_target` from critical path.
- **Product semantics:** keep **assignment/reassignment notify new non-actor assignee only**; **no unassign notification**.
- **Tx coupling:** remain **best-effort post-persist** (no refactor to shared tx).
- **Why:** closes the main race window with small, local changes; preserves low-risk behavior and VPS simplicity.

### Option B: DB-enforced unread dedup + coalescing upsert

- Same index/arbiter as A but `ON CONFLICT DO UPDATE` to refresh unread row (and set `is_read=false`/timestamps/url as defined).
- **Why:** stronger coalescing under repeated triggers; **risk:** reorders/rewrites notification recency semantics and may surprise users; needs explicit UX/product decision.

### Option C: Transaction-coupled insert (same DB tx) + conflict-safe insert

- Refactor task mutation + notification insert into one transaction; use `ON CONFLICT DO NOTHING` for idempotency.
- **Why:** atomic “state + notification intent” (or chosen failure policy) and no TOCTOU.
- **Cost/risk:** larger API/repo/service refactor, tx lifetime/locking considerations, and decision on whether notification failure should abort task write.

### Option D: Intent-epoch idempotency (future-leaning)

- Introduce assignment/version/correlation id per effective transition and unique constraint on `(user_id,event_type,target_id,assignment_epoch_or_event_id)` (shape NV).
- **Why:** true idempotency per logical transition while permitting future reassignments to same user.
- **Cost:** schema/product/model changes beyond minimal 022.

**Feasibility ranking (effort vs correctness gain):** **A >> B ≈ A+** (if coalesce wanted) **>> C >> D**. For Flora’s stated constraints (no heavy infra, cheap VPS), **A** is the pragmatic 022 hardening step.

---

## 7. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| TOCTOU duplicates persist if only app-check kept | Noisy inbox, user distrust | DB-enforced `ON CONFLICT` on correct arbiter/predicate. |
| Wrong uniqueness scope (all-history) | Legitimate future reassign notifications suppressed | Prefer unread/active partial scope or epoch-keyed uniqueness; test reassign-after-read. |
| Partial index/predicate mismatch with query/insert | Conflicts not caught or migration/index unusable for inference | Match `ON CONFLICT` target to actual unique/partial index definition exactly. [PG-INSERT-ON-CONFLICT] |
| Existing duplicate rows block unique index creation | Migration failure | Audit/deduplicate existing data before/with migration (policy NV). |
| Best-effort post-persist drops | Missing notifications on transient DB/error | Structured warn logs/metrics; accept for MVP or move to outbox/tx coupling later. |
| Coupling in same tx | Notification path can fail core mutation or increase contention | Explicit failure policy, keep tx short, measure. |
| Unassign semantics invented implicitly | Wrong recipients/noise/product mismatch | Keep U0 until decided; if implemented, add explicit type/recipient rules and tests. |
| Tenant/recipient leakage | Security/correctness issue | Continue server-derived `organization_id`/recipient from persisted transition, membership/authorization checks, scoped tests. |
| Concurrent task updates/races | Missed or surprising transitions | Transition computed from observed old/effective values; consider conditional updates/locking only if proven necessary (**NV**). |
| API/UX mismatch | Clients expect `target_url`/deep-link or different grouping | Optionally expose stored fields; align dedup/coalesce behavior with inbox expectations (**NV**). |

---

## 8. NVs (needs verification / decision)

- **NV-1 (product):** should explicit unassign emit any notification? if yes, recipient(s) (former assignee, creator, actor, watchers) and event type/wording.
- **NV-2 (product):** on reassignment `O -> N`, notify `O`? (likely separate event, not `Assignment`).
- **NV-3 (scope):** dedup policy by event type — is unread-partial uniqueness correct for all `NotificationType`s or only `Assignment`/specific cases?
- **NV-4 (key):** canonical uniqueness key for idempotency: `(user_id,event_type,target_id)` unread-only vs include org/epoch/version/mutation id; compatibility with legitimate re-notify after read/reassign.
- **NV-5 (migration):** sqlx/migration execution constraints for `CREATE INDEX CONCURRENTLY` in this codebase and production rollout steps on Postgres 18/VPS.
- **NV-6 (data hygiene):** existing duplicate unread rows to reconcile before adding unique index.
- **NV-7 (API contract):** return/surface “suppressed duplicate” vs “created” internally/observably? expose `target_url` in list response?
- **NV-8 (tx):** require atomic task+notification commit, or is best-effort acceptable SLO?
- **NV-9 (concurrency):** need conditional `UPDATE tasks ... WHERE assignee_id IS NOT DISTINCT FROM $old` / locking to harden transition detection under races.
- **NV-10 (external truth):** authoritative GitHub/Linear behavior for unassign/removal notifications and Macro production unassign semantics (not fully established here).

---

## 9. References

### Primary / authoritative
- [PG-PARTIAL-INDEX] PostgreSQL Documentation (current/18): *11.8. Partial Indexes* — partial index predicate, unique partial indexes, planner/use constraints. https://www.postgresql.org/docs/current/indexes-partial.html
- [PG-INSERT-ON-CONFLICT] PostgreSQL Documentation (current/18): *INSERT — ON CONFLICT Clause* — arbiter inference, `conflict_target`, partial unique index inference/predicate, `DO NOTHING`/`DO UPDATE`, atomic upsert semantics and privileges/locking notes. https://www.postgresql.org/docs/current/sql-insert.html#SQL-ON-CONFLICT

### Flora local evidence
- [F-ROUTE-TASKS] `flora-api/src/routes/tasks.rs` (tri-state unassign, transition-only best-effort notify, unread dedup guard call)
- [F-REPO-NOTIF] `flora-core/src/repositories/notification_repository.rs` (plain insert, `has_unread_for_target`, read/mark/delete queries)
- [F-TRAIT-NOTIF] `flora-core/src/traits/repository.rs` (`NotificationRepository` incl. `has_unread_for_target`)
- [F-MODEL-NOTIF] `flora-core/src/models/notification.rs` (`Notification`, `NotificationType::Assignment`)
- [F-MIGRATION] `flora-core/src/migrations/sql/20240818000001_init_schema.sql` (`notifications` schema/indexes/RLS, no uniqueness)
- [F-ROUTE-NOTIFS] `flora-api/src/routes/notifications.rs` (list/read API behavior)
- [F-020] `docs/research/2026-11-16-020-task-assignment-notification-vps.md`
- [F-021-UNASSIGN] `docs/research/2026-11-16-021-task-unassign-contract-notification-semantics.md`
- [F-021-MACRO-SEARCH] `docs/research/2026-11-16-021-macro-search-parity-unassign-notifications.md`

### Product / behavior signals (triangulation, partly NV)
- [GH-NOTIFS-ABOUT] GitHub Docs: *About notifications* (default subscriptions incl. assigned to issue/PR; reasons/triage model). https://docs.github.com/en/subscriptions-and-notifications/concepts/about-notifications
- [GH-ASSIGN] GitHub Docs: *Assigning issues and pull requests to other GitHub users*. https://docs.github.com/en/issues/tracking-your-work-with-issues/assigning-issues-and-pull-requests-to-other-github-users
- [LINEAR-NOTIFS] Linear Docs: *Notifications* (auto-subscribe when assigned; grouped notification types/timing). https://linear.app/docs/notifications
- [LINEAR-ASSIGN] Linear Docs: *Assign and delegate issues*. https://linear.app/docs/assigning-issues
- [ASANA-UNASSIGN-FORUM] Asana Forum: *Notification when a task is unassigned* (feature request/community signal; NV). https://forum.asana.com/t/notification-when-a-task-is-unassigned/94349

### Practice / background (non-normative, implementation context)
- [DEV-RACE] M. Valjak, *Winning Race Conditions With PostgreSQL* (partial unique index pattern, `ON CONFLICT`) — DEV Community, 2021. https://dev.to/mistval/winning-race-conditions-with-postgresql-54gn
- [ONEUPTIME-RACE] *How to Handle Race Conditions in PostgreSQL* (advisory locks, `ON CONFLICT`/upsert patterns), 2026. https://oneuptime.com/blog/post/2026-01-25-postgresql-race-conditions/view
- [HEROKU-PARTIAL] Heroku Dev Center: *Efficient Use of PostgreSQL Indexes* (partial/unique indexes). https://devcenter.heroku.com/articles/postgresql-indexes
- [OUTBOX-MILAN] M. Jovanović, *Outbox Pattern in .NET With Postgres...* (tx coupling/perf trade-offs), 2026. https://milanjovanovic.tech/blog/outbox-pattern-postgres-workers-vs-wal
- [EVENTDRIVEN-OUTBOX] O. Dudycz, *Push-based Outbox Pattern with Postgres Logical Replication* (at-least-once/dedup context). https://event-driven.io/en/push_based_outbox_pattern_with_postgres_logical_replication/
- [OUTBOX-REVAMP] M. Dousti, *PostgreSQL + Outbox Pattern Revamped* (outbox ops/tx considerations), DEV Community. https://dev.to/msdousti/postgresql-outbox-pattern-revamped-part-1-3lai

**Citation hygiene:** product behavior claims beyond fetched official docs are treated as signals (**NV**). PostgreSQL semantics are taken from current official documentation pages cited above.
