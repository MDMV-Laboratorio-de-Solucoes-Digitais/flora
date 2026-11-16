# 021 DB-gated tests — task unassign/assignment semantics and search hardening/parity

**Date:** 2026-11-16  
**Scope:** Design deterministic, DB-gated contract/integration tests for cycle 021 on disposable PostgreSQL 18 (and note Meilisearch-gated feasibility) covering (A) task unassign/assignment semantics if the API supports explicit clear (`assignee_id: null`) and (B) search hardening/edge cases for multi-entity PG fallback plus task/file parity expectations in Meilisearch mode vs explicit precedence/contract decision. Research-only; no source changes.

**Cross-checks:** `flora-tests/src/contract/{search,notifications,tasks,messaging,files,auth}.rs`, 020 spec/plan/tasks (`specs/020-pg-search-tasks-files-assignment-notification/`), routes/models (`flora-api/src/routes/{search,tasks}.rs`, `flora-core/src/models/task.rs`), migrations (`flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`, `20261118000001_add_tasks_files_search_fts.sql`), search crate (`flora-search/src/lib.rs`), 021 planning (`OBJETIVO.md` §5, `ROADMAP.md` P0/021).

---

## 1. Preconditions and gating

### 1.1 Disposable PostgreSQL 18 (required for DB-gated cases)
- **Env:** `FLORA_DATABASE__POSTGRES_URL` preferred, fallback `DATABASE_URL` (matches existing `db_state()` in search/notifications/messaging/files/auth contracts).
- **Skip behavior:** if unset, log `eprintln!("skipping DB-backed ... test: ... not set")` and `return Ok(())` (no false green/false red in non-DB CI jobs).
- **Migration/state:** `PgPool::connect` + `flora_core::migrations::run_migrations(&pool).await?` per module/test setup (idempotent). Use a disposable DB (ephemeral Postgres 18 container/schema) because fixtures insert rows and notifications accumulate.
- **Isolation/determinism:** generate unique suffixes with `Uuid::now_v7().simple().to_string()` for emails/slugs/names/storage paths; avoid shared fixed IDs except nil constants. Assert on specific IDs/content tokens, not global counts, unless the query is scoped (e.g. exact task target id counts for one recipient).
- **Config:** `Config::default()` with `search_service: None`, `messaging_service: None`, `valkey: None` for PG-fallback/assignment tests (same as 019/020). `realtime_hub: flora_api::realtime::create_hub()`. File-related fixtures only if needed; search/assignment suites do not require temp storage.

### 1.2 Meilisearch-gated (optional, feasibility-dependent)
- **Not currently exercised** by contract tests; `SearchService`/Meilisearch wiring exists (`flora-search`, `state.search_service`) but mutation-time indexing and task/file parity in Meilisearch mode are **NV/incomplete** (see §4.3).
- **Feasible gate (if added):** `FLORA_MEILISEARCH__URL` (+ key/index template as config) and a reachable disposable Meilisearch; skip with explicit reason when unset/unreachable. Tests must be isolated by per-run org/index namespace (e.g. org-scoped index `index_template` including `{org_id}` already supported) and wait for indexing tasks if async.
- **Recommendation:** treat Meilisearch parity cases as `#[ignore = "requires disposable Meilisearch; parity contract pending (cycle 021)"]` until product/contract decision is made; keep PG-fallback suite as the deterministic Seed/Postgres-first gate.

### 1.3 Runtime
- `#[tokio::test]` + `#[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 021)"]` for DB-backed cases, consistent with 019/020.
- Place under existing contract modules (`flora-tests/src/contract/tasks.rs` and/or `notifications.rs` for A; `search.rs` for B) or a small `contract::assignment`/`contract::search_parity` submodule; follow `pub mod` registration in `flora-tests/src/contract/mod.rs` only if a new file is added (design note).

---

## 2. Minimal harness/helpers (reuse or extract)

Reuse the established contract-test shape (duplicated helpers are acceptable per current repo pattern; minimal extraction preferred to avoid broad refactors):

| Helper | Purpose | Source pattern |
|---|---|---|
| `db_state() -> anyhow::Result<Option<(AppState, Config)>>` | Env gate, connect, migrate, default config, `make_test_state` with repos/services wired | `contract::search`, `notifications`, `messaging`, `files`, `auth` |
| `make_test_state(pool, &config) -> AppState` | Full `AppState` with `Pg*Repository` adapters and services; `search_service: None` for fallback | all DB-gated modules |
| `seed_member` / `seed_two_members` / `seed_three_members` | User+org+role+membership+workspace+active session+JWT (`jwt::encode_token` + `FloraClaims`) | search/notifications/messaging/auth |
| `authed_json(app, method, uri, token, org_id, Option<Value>)` / `authed_get_json` | Bearer auth + `x-organization-id` + JSON body/parse | notifications/search/messaging/files |
| `insert_channel` / `insert_message` / `insert_task` / `insert_file` (raw SQL) | Deterministic fixtures bypassing route side-effects when needed (e.g. search corpus, deleted rows) | search/messages-style inserts |
| `tasks_notifications_app(state)` / `search_app(state)` | `Router::new().nest("/tasks", ...).nest("/notifications", ...)` or `/search` | notifications/search |
| `count_notifications_for(app, token, org, predicate)` or `list_and_filter` | Assert visibility via existing `GET /notifications` API (recipient-scoped) | notifications 019/020 |

**Concrete helper additions recommended for 021 (minimal, local to module):**
- `async fn patch_task(app, token, org, task_id, body: Value) -> (StatusCode, Value)` thin wrapper over `authed_json(..., "PATCH", format!("/tasks/{task_id}"), ...)`.
- `async fn create_task(app, token, org, body: Value)` returning `(StatusCode, Value)`.
- `fn assignment_notifs(arr: &[Value], target_task_id: &str) -> Vec<&Value>` filter `event_type == "Assignment" && target_id == target_task_id`.
- `async fn insert_task_with_assignee(..., assignee_id: Option<Uuid>, is_deleted: bool)` for search corpus / direct-state setup.
- Assertion helpers: `assert_no_cross_org_leak(results, foreign_ids)`, `assert_ids_present(results, expected_type, ids)` to keep cases readable.

All helpers must use `anyhow::Result`, `anyhow::ensure!` with contextual `{status} {body}` dumps (existing style) for debuggability.

---

## 3. (A) Task unassign/assignment semantics

### 3.1 Current contract reality (feasibility gate)

**API/route (`flora-api/src/routes/tasks.rs`):**
- `CreateTaskRequest { assignee_id: Option<Uuid> }` — supports assign on create, absent/null field maps to `None` in current JSON usage (serde default for Option).
- `UpdateTaskRequest { assignee_id: Option<Uuid> }` — **cannot explicitly clear**: merge is `assignee_id: requested_assignee_id.or(old_assignee_id)`, and assignment notification triggers only when request provides `Some(new)` different from old and not actor (`requested_assignee_id` guard). Sending JSON `{"assignee_id": null}` deserializes to `None` for `Option<Uuid>` in the same way as omitted in typical serde usage for this shape, so it cannot distinguish "clear" from "no change" without a contract change.
- **Conclusion:** explicit unassign via `PATCH /tasks/{id}` with `assignee_id: null` is **not supported today**. 021 must either (1) introduce an explicit-clear contract (recommended: tri-state) and test it, or (2) document/trap the current limitation with regression tests that prove clear is impossible/no-op under current contract and defer unassign-notification semantics.

**Notification behavior today (020):** best-effort `Assignment` on effective create-assign and explicit reassignment transition; guards for actor==assignee and no-op same-assignee; no dedup uniqueness constraint observed (count-based duplicate detection in tests).

### 3.2 Contract options for explicit clear (design decision required before implementation)

| Option | Shape | Pros | Risks/notes |
|---|---|---|---|
| **Tri-state Option<Option<Uuid>>** | `#[serde(default, deserialize_with = "deserialize_optional_field")] assignee_id: Option<Option<Uuid>>` where absent => outer `None` (no change), present `null` => `Some(None)` (clear), present uuid => `Some(Some(id))` (set) | Expressive, backward-compatible for clients omitting field | Requires careful serde/default semantics and frontend/client alignment; update merge logic accordingly |
| **Dedicated action/field** | e.g. `clear_assignee: bool` or `unassign: true` | Explicit, easy to read | API surface change, mutually-exclusive validation needed |
| **Sentinel** | nil UUID means unassign | Avoids serde edge cases | Leaks sentinel semantics, easy misuse; not recommended |

**Design recommendation:** tri-state `assignee_id` on PATCH (and align create semantics only if needed) as the minimal contract fix matching 021 wording; preserve "omit => unchanged" behavior. Emit/guard notifications only on **effective transitions**.

### 3.3 Concrete DB-gated cases/assertions (A)

#### A0 — Baseline regressions (current contract, must remain green)
Source-aligned with `db_task_assignment_creates_notification_and_guards` (020):
1. **Create with assignee (other member):** A creates task assigned to B -> `201`, B sees exactly 1 `Assignment` for `task_id` unread/visible via `GET /notifications`; A (actor) sees none for that target.
2. **No-op update (same assignee explicitly provided, plus unrelated field change):** PATCH with same `assignee_id` -> `200`, B still has exactly 1 `Assignment` for task (no duplicate).
3. **Reassign A-assigned-to-B -> C:** PATCH with `assignee_id=C` -> `200`, C receives `Assignment` for task; B count unchanged (no revocation event in scope).
4. **Self-assignment on create:** A creates task assigned to A -> `201`, A has no `Assignment` for that task.

#### A1 — Explicit unassign (if tri-state/clear contract implemented)
Fixtures: 3 members A(actor), B, C in same org/workspace; task created assigned to B.
1. **Unassign via `assignee_id: null`:** `PATCH {"assignee_id": null}` -> `200`, response `assignee_id` is `null`; persisted task has `assignee_id IS NULL` (verify via `GET /tasks/{id}` if available/tenant-safe in test context or via repo/readback in a service-level integration; prefer API-visible assertion).
   - **Notification expectation (define contract):** default recommendation **no Assignment notification** (assignment events notify the newly assigned user; unassign notifies nobody) — assert no new `Assignment` rows for B/C/actor for this task beyond pre-state. Alternative product choice must be specified explicitly if "unassignment notice" is desired (out of current `NotificationType` set).
2. **Unassign idempotent/no-op when already unassigned:** task with `assignee_id = NULL`, `PATCH {"assignee_id": null}` -> `200`, assignee remains null, no notifications emitted.
3. **Unassign then reassign:** B -> null -> C produces at most 1 new `Assignment` for C (on B->C effective set step or on null->C step per defined transition rules; recommend notify only when resulting assignee is `Some(new)` and differs from previous effective assignee) and none for B on clear.

#### A2 — If explicit unassign NOT supported (current contract lock-in)
Deterministic regression cases to document the limitation:
1. **`PATCH {"assignee_id": null}` treated as omitted/no-change:** task assigned to B, PATCH with JSON null -> `200` (or accepted body) but `assignee_id` remains B (current `Option<Uuid>` null ≈ absent in effect given merge/or handling) and **no duplicate notification**.
2. **`PATCH {}` without assignee:** assignee unchanged, no notification.
3. Spec/doc assertion in test name/comment: "unassign unsupported until tri-state contract (021)" — NV tracked, not a silent pass for wrong behavior.

**021 deliverable must pick A1 vs A2 based on implementation decision; research recommends implementing A1 contract and covering A1.**

#### A3 — Self/no-op/guard edges
1. **Self-reassign explicit:** task assigned to B, actor B patches `assignee_id=B` -> no `Assignment` to B (self/no-op guard).
2. **Actor assigns to self via update:** task unassigned or assigned to other, actor A patches `assignee_id=A` -> no `Assignment` notification to A (actor==new guard).
3. **Omitted assignee with other field update:** changing title/status without `assignee_id` must not emit assignment notification even if currently assigned.
4. **Create without assignee:** no `Assignment` notifications.

#### A4 — Notification emission/guards/simple-dedup visibility
1. **Recipient scoping:** only intended assignee sees `Assignment` for task via their token/session; other members in org do not see it (unless separately targeted) — assert via per-user `GET /notifications`.
2. **Simple duplicate guard (application-level):** consecutive identical effective re-requests (same transition replay scenario representable at API level: repeat PATCH setting same new assignee when already that assignee) must not increase count (`count == 1` per effective transition) — matches 020 no-op case.
3. **Best-effort non-fatal invariant (observable):** task mutation succeeds (`2xx`) even if notification path would fail; not directly injectable without fault hooks, so mark as **NV for DB-gated contract** unless a test seam is added. Document reliance on route `tracing::warn` + response success.
4. **Idempotency/dedup under retry or concurrent updates:** no DB uniqueness constraint stated — **NV/race-prone**; design follow-up stress/integration only if harness can issue concurrent PATCHes deterministically (generally non-deterministic, avoid flaky tests).

#### A5 — Authorization/tenancy invariants (regression, relevant to semantics)
- Cross-org task PATCH/GET must remain forbidden/not-found per existing behavior; assignment notification must never be created in wrong org. (Can be covered if cheap; otherwise rely on existing task route protections.)

---

## 4. (B) Search hardening/edge cases and Meilisearch parity

### 4.1 PG fallback behavior (current, to lock with deterministic tests)

Implementation (`flora-api/src/routes/search.rs`, `search_pg`):
- **Auth/tenant:** `AuthExtractor` + `require_org`, membership check, queries scoped to `organization_id = $1` and `is_deleted = false` for each entity.
- **Query normalization:** `query = params.q.trim().to_string()`; if empty -> `{query: "", results: []}` (and preserves trimmed-empty behavior as empty result set).
- **Types:** CSV split/trim/lowercased; `include_type`: omitted/None or empty list -> include all supported; matches plural/singular (`message/messages`, `task/tasks`, `file/files`); unknown-only types -> no entity included -> empty results. Supported PG entities in 020: message, task, file.
- **Limit/bounds:** `limit = params.limit.unwrap_or(20).min(50)`; per-entity SQL `LIMIT $3` and final merged `truncate(limit)`.
- **FTS parsing:** `COALESCE(websearch_to_tsquery('simple', $2), plainto_tsquery('simple', $2))`, guards `tsq IS NOT NULL AND tsq <> ''::tsquery`.
- **Ranking/ordering:** per-entity `ORDER BY rank DESC, created_at DESC, id DESC`; merged global sort by `rank desc`, then `created_at desc`, then `id desc` (`total_cmp` for f32) then truncate. **Deterministic tie-breaker present (id).**
- **Snippet:** whitespace-normalized and truncated to 200 chars (`left(regexp_replace(..., '\s+', ' ', 'g'), 200)`); message uses `content`, task uses `title + ' ' + description`, file uses `name + ' ' + file_type`.
- **Indexes/columns:** generated stored `search_tsv` with `'simple'` config and partial GIN on non-deleted rows (messages/tasks/files migrations).

### 4.2 Concrete DB-gated cases/assertions (PG fallback)

#### B1 — Type normalization and filter semantics (hardening)
1. **Singular vs plural tokens:** `type=messages` returns messages; `type=tasks` returns tasks; `type=files` returns files (extend 020 coverage explicitly for singular/plural both ways).
2. **CSV + whitespace + case:** `type=TASK, file ,messages` includes all three; ensure only matching types returned.
3. **Unknown-only and mixed-unknown:** `type=unknown` -> empty; `type=unknown,task` -> tasks only (if term matches) to lock current `include_type` OR behavior.
4. **Omitted/empty `type`:** includes all supported entities (regression/extension of 020 merged test).

#### B2 — Ordering/snippet stability
1. **Deterministic order across identical corpus:** insert fixed-known rows (or derive expectations from response) and assert exact `id` order for a query where ranks/timestamps are controlled. Prefer controlling timestamps via SQL updates after insert (if allowed) or crafting distinct `created_at`/distinctive terms; simplest deterministic approach: unique high-selectivity terms per entity so each query returns 1 result, plus a multi-entity query with known rank separation.
2. **Tie-break on id when rank/time equal:** construct rows with same lexical content/timestamps if feasible (e.g. same `created_at` via `UPDATE ... SET created_at = $ts` for test rows) and assert descending `id` order as implemented. This is the strongest determinism check; include only if DB fixture control is straightforward.
3. **Snippet normalization:** content/name with repeated whitespace/newlines -> snippet contains single spaces and is <= 200 chars; task snippet combines title+description; file snippet combines name+file_type. Assert `snippet` is non-empty and bounded, and optionally contains normalized token.
4. **Ranking non-negative/plausible:** `rank` not exposed in API; assert relevant hit present and stable ordering rather than numeric rank.

#### B3 — Limit/bounds and query edge cases
1. **Default limit:** omit `limit` with >20 matching rows -> at most 20 results.
2. **Cap at 50:** `limit=999` -> at most 50.
3. **Small limit with multi-entity merge:** `limit=1` over corpus matching multiple entities -> exactly 1 result and it is the globally top-ranked/ordered item per merge rule.
4. **`limit=0` behavior:** current clamp uses `min(50)` only; `0` would pass through (`min(0,50)=0`) leading to `LIMIT 0`/truncate 0 -> empty. Lock explicit expected behavior (empty results, 200) to avoid ambiguity.
5. **Empty/whitespace query:** `"   "` -> 200 empty `results`, `query` echoed trimmed (matches 019/020).
6. **Non-matching query:** valid term with no hits -> 200 empty.
7. **Punctuation/websearch syntax robustness:** queries like `"alpha -beta"` or quoted phrases should not 500 (parser uses `websearch_to_tsquery` fallback to `plainto`/null guards) -> 200 with well-defined (possibly empty) results. Deterministic assertion: status 200 and array response.
8. **Very long query string:** bounded behavior/no 500 (route/service should handle gracefully) — NV/perf unless trivially testable.

#### B4 — Tenant/soft-delete/entity coverage regressions
1. **Cross-org isolation:** tasks/files/messages from org B never appear for org A token (extend 020).
2. **Soft-deleted excluded:** deleted task/file/message absent (extend 020).
3. **Legacy/null workspace file visibility:** `files.workspace_id` may be null; org-scoped search may include it (documented NV/allowed in 020) — add case only if 021 contract clarifies; otherwise assert no crash and org isolation holds.

### 4.3 Meilisearch mode parity expectations and contract decision (if in scope)

**Observed state:**
- `GET /search` prefers `state.search_service` when present and returns its hits directly (no PG fallback merge). `flora-search::SearchService` supports `search`, `index_message`, `index_task`, `index_file`, and filter syntax on `type`/timestamps.
- PG fallback now covers `message|task|file` (020); Meilisearch **runtime parity** depends on: (a) `search_service` being configured, (b) tasks/files actually indexed on create/update/delete (mutation wiring not evidenced in this audit), and (c) filter/type semantics matching PG fallback.

**Feasibility:** Meilisearch-gated DB+Meili tests are feasible **only with a disposable Meilisearch instance and an indexing completion/wait strategy**; otherwise flaky/non-deterministic. Not a default CI gate today.

**Contract/precedence decision to record for 021:**
1. **Precedence:** keep current precedence (`search_service` present => Meilisearch path, else PG fallback). Do **not** merge results implicitly.
2. **Parity target (recommended):** define "minimum parity" = same auth/tenant/soft-delete guarantees, same supported `type` tokens, same response envelope `{query, results:[{type,id,snippet}]}`, and deterministic-enough ordering contract **per backend** (not necessarily identical ranking across engines). Require task/file indexing hooks if Meilisearch mode claims parity.
3. **If parity not ready:** explicitly document Meilisearch mode as **best-effort/legacy for message-first or partial indexing** and require Seed profile to run with `search_service: None` (Postgres-first) for guaranteed multi-entity behavior; PG fallback remains authoritative for 021 acceptance.

**Proposed Meilisearch-gated cases (conditional, `#[ignore]` + skip when env missing):**
- M1: configured Meilisearch + indexed task/file/message -> query returns matching entities with correct `type/id` and tenant isolation.
- M2: `type` filter parity for singular/plural/CSV/unknown-only matches PG contract (or documented differences).
- M3: deleted/foreign-org entities not returned.
- M4: precedence sanity — when service configured, response path is Meilisearch (observable only if distinguishable; otherwise treat as behavioral parity test). Mark NV unless indexing pipeline is made deterministic in-test.

---

## 5. Fixture matrix (concise)

| Area | Entities | Notes |
|---|---|---|
| Orgs/users | Org A (members A1 actor, A2, A3), Org B (member B1) | membership/roles/sessions per user; tokens per user |
| Workspaces/channels | 1+ workspace per org; channel as needed for message fixtures | channel inserts for search/message context |
| Tasks | assigned/unassigned, deleted/non-deleted, same-org/cross-org, distinctive title/description tokens | some inserted via API (to exercise notifications), some via SQL (to isolate search corpus) |
| Files | same-org/cross-org, deleted/non-deleted, optional `workspace_id` null legacy | `name`/`file_type` tokens for FTS |
| Messages | existing 019 patterns | regression only |
| Notifications | observed via `GET /notifications?limit=...` | assert by `event_type`, `target_id`, `is_read` |

**Token strategy:** include cycle marker + case marker (e.g. `unassign021`, `typenorm021`, `limit021`) in unique strings to avoid collisions in disposable DB runs with prior data.

---

## 6. Determinism, risks, and NVs

- **Deterministic:** env-gated skip, migrations, UUIDv7-unique names, scoped assertions by IDs/tokens, stable sort contract in PG fallback (rank, created_at, id).
- **Potential flakes to avoid:** do not depend on global notification ordering beyond filtering; do not assume Meilisearch indexing completes without explicit wait; avoid concurrent mutation tests unless synchronization is controlled.
- **NV / out of 021 DB-gated scope unless implemented:** explicit unassign API support (if not added), unassign notification product semantics, notification tx-coupling/idempotency uniqueness/race guarantees, Meilisearch task/file indexing parity and cross-engine ranking equality, `pg_trgm`/i18n/CJK relevance, channel ACL in search, 2vCPU/4GB perf, browser e2e/multi-node realtime.

---

## 7. Suggested implementation checklist (when coding 021)

1. Decide and implement unassign contract (tri-state recommended) + route merge/notification transition rules; document in API spec/changelog.
2. Add DB-gated tests in `flora-tests/src/contract` (prefer `tasks.rs` stubs filled or `notifications.rs`/`search.rs` extensions) with helpers above; register modules if new files.
3. Add/adjust CI DB job filters if needed to include 021 tests (existing env-gated/ignored pattern).
4. Record Meilisearch parity/precedence decision in `OBJETIVO.md`/`ROADMAP.md` or assessment notes; add conditional Meili tests only with deterministic indexing harness.
5. Keep all changes minimal and Seed/Postgres-first aligned.