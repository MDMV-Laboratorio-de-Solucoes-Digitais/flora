# 020 — DB-gated, deterministic contract/integration tests: PG FTS for tasks/files & assignment notifications

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`c5e2f4e8a`) — **research/design only** (no source/commit changes).  
**Scope:** cycle 020 disposable **PostgreSQL 18** tests for (A) `GET /api/v1/search` **Postgres FTS fallback** extended to **tasks** (`title`, `description`) and **file metadata** (`name`, `file_type`) with `type` filters, tenant isolation, soft-delete exclusion, and Meilisearch-unavailable behavior; and (B) in-app **`Assignment`** notification created on task assignment (create with assignee / update changing assignee) visible via `GET /api/v1/notifications`, with guards for self-assignment, unassign/reassign semantics, and actor/recipient correctness.

## 1. Cross-check: baseline and 020 delta

| Area | Current evidence (019/dev) | 020 implication for tests |
|---|---|---|
| Search route/contract | `flora-api/src/routes/search.rs`: `GET /api/v1/search?q=&type=&limit`, auth+org membership required, empty/blank `q` => `200 {query, results:[]}`, `limit` default 20 capped 50, CSV `type` parsed/lowercased. If `state.search_service` (Meilisearch) exists it is used; else PG fallback currently queries **messages only** (`messages.search_tsv`, `websearch_to_tsquery`→`plainto_tsquery`, `ts_rank_cd`, snippet `left(regexp_replace(content,...),200)`, `organization_id` scoped, `is_deleted=false`). Message-only `type` filter returns empty for `task`/`file`. Response: `{query, results:[{type,id,snippet}]}`. | 020 must extend PG fallback to include `task` and `file` entities while preserving contract and current message behavior. Tests should lock: backend parity path without Meilisearch (`search_service: None`), additive `type=task|file|message|csv`, stable JSON shape, empty-query short-circuit, tenant + soft-delete exclusion, deterministic ranking tie-break where needed. |
| Search schema (019) | Migration `20261117000001_add_messages_search_fts.sql`: `messages.search_tsv tsvector GENERATED ALWAYS AS (to_tsvector('simple', coalesce(content,''))) STORED` + partial GIN `idx_messages_search_tsv ... WHERE is_deleted = false`. No task/file search columns/indexes. | 020 requires additive migration(s), e.g. `tasks.search_tsv` generated from `coalesce(title,'') || ' ' || coalesce(description,'')` (`'simple'`, stored) and `files.search_tsv` generated from `coalesce(name,'') || ' ' || coalesce(file_type,'')` (`'simple'`, stored), each with partial GIN on non-deleted rows. Tests must run after migrations (`run_migrations`) and may include a capability/assertion that queries hit those entities (behavioral). |
| Tasks model/routes | `tasks` table/model has `workspace_id, organization_id, creator_id, assignee_id (nullable), title, description, status, is_deleted`. Routes: `POST /api/v1/tasks` accepts `title,description,workspace_id,assignee_id` and persists as-is (no assignment notification), `PATCH /{id}` sets `title/description/assignee_id/status` via `assignee_id: req.assignee_id.or(existing.assignee_id)` and does not emit notifications, `DELETE` soft-deletes, `GET /{id}` enforces org, `GET /` lists assignee-scoped in org. `PgTaskRepository::update` writes `assignee_id`; `PgTaskRepository::soft_delete` sets `is_deleted=true`. | 020 notification trigger points are create/update mutation paths. Critical semantics to pin in tests: **self-assignment** (assignee == actor) must not notify; **unassign** (assignee becomes `None`) must not notify; **reassign** (A→B or None→B) should notify the **new assignee** only when changed and not actor; `PATCH` semantics in current impl are “set if provided else keep existing” (cannot explicitly clear assignee without API change) — tests must match chosen 020 contract and call out NV if clearing is unsupported. |
| Files model/routes/repos | `files` has `name, file_type, size_bytes, storage_path, checksum, metadata, is_deleted, workspace_id nullable`, org-scoped queries/repos, upload/metadata endpoints, soft delete. No search projection. | Searchable file metadata in 020 should use `name` and `file_type` only (not storage path/checksum). Soft-deleted files excluded. Legacy `workspace_id IS NULL` rows, if included, must still be org-isolated and must not leak across orgs. |
| Notifications | `NotificationType::Assignment` exists; repo/API support list/read/mark-read; reply notification emitted best-effort in `channels.rs::post_message` (`maybe_notify_reply`) after persist. No task assignment hook today. | Assignment notification should follow 019 pattern: persist `Notification` with `event_type=Assignment`, `user_id=<assignee recipient>`, `organization_id=<task org>`, `target_id=<task_id>` (recommended), `target_url` optional (assert permissively unless specified), `is_read=false`; creation **best-effort/non-fatal** relative to task mutation success unless product decides otherwise (**NV**). Visible only to recipient via `GET /api/v1/notifications`. |
| Existing DB-gated harness | `flora-tests/src/contract/{search,notifications,messaging,files,auth,...}`: `mod db_*`, env gate `FLORA_DATABASE__POSTGRES_URL|DATABASE_URL`, `PgPool::connect`, `flora_core::migrations::run_migrations`, `Config::default()` (+ overrides), `make_test_state` building full `AppState`, unique `Uuid::now_v7()` fixtures, `Router::nest(...).with_state`, `tower::ServiceExt::oneshot`, `#[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle N)"]`. | 020 tests should reuse this structure verbatim. Prefer contract-level HTTP assertions for externally visible behavior; add narrow direct-SQL setup only for deterministic row state (e.g. soft-delete flags, legacy file rows) when cleaner than API flows. |

## 2. Env-gating, determinism, and disposable PG18

- **Gate:** `#[tokio::test]` + `#[ignore = "requires Postgres 18; set FLORA_DATABASE__POSTGRES_URL or DATABASE_URL (cycle 020)"]`.
- **DB URL:** prefer `FLORA_DATABASE__POSTGRES_URL`, fallback `DATABASE_URL`; if unset print `eprintln!("skipping DB-backed 020 ...")` and `return Ok(())`.
- **Lifecycle:** `PgPool::connect(&url).await?` → `flora_core::migrations::run_migrations(&pool).await?` (must apply 020 search migrations) → build `AppState`/routers.
- **Disposable DB safety:** all names/emails/slugs/tokens/content include `Uuid::now_v7()`; assertions scoped to seeded IDs/orgs/users (never global counts except bounded per-request responses).
- **Search backend path under test:** default 020 fallback tests run with `AppState.search_service = None` and DB reachable (current 019 fallback shape) to prove Meilisearch-unavailable PG path. A separate smoke can set `search_service` only if fixture available (**out of scope/NV for disposable CI**).
- **Determinism:** no sleeps; perform mutation, `await` completion, then query; for search prefer unique high-entropy tokens (`fts-task-<uniq>`, `fts-file-<uniq>`); assert `id`/`type` membership and absence/leakage rather than global rank unless a single match.
- **CI:** run in DB-enabled `db-tests` lane (`cargo test -p flora-tests -- --ignored` subset/filter as configured) on ephemeral Postgres 18 service; local runs identical with env var set.

## 3. Minimal harness helpers (reuse/factor)

Follow existing `db_*` modules; factor only if it reduces duplication without changing behavior:

```rust
// contract helper shape (inline in 020 modules or shared db_harness)
struct DbCtx { state: AppState, config: Config }

async fn db_state() -> anyhow::Result<Option<DbCtx>> {
    let Some(url) = std::env::var("FLORA_DATABASE__POSTGRES_URL")
        .ok().or_else(|| std::env::var("DATABASE_URL").ok()) else {
        eprintln!("skipping DB-backed 020 test: FLORA_DATABASE__POSTGRES_URL/DATABASE_URL not set");
        return Ok(None);
    };
    let pool = PgPool::connect(&url).await?;
    flora_core::migrations::run_migrations(&pool).await?;
    let config = Config::default(); // or override storage/search as needed
    let state = make_test_state(pool, &config); // search_service/messaging/valkey None
    Ok(Some(DbCtx { state, config }))
}

fn api_search_app(ctx: &DbCtx) -> Router {
    Router::new().nest("/search", create_search_router()).with_state(ctx.state.clone())
}
fn api_notifications_tasks_app(ctx: &DbCtx) -> Router {
    Router::new()
        .nest("/tasks", create_tasks_router())
        .nest("/notifications", create_notifications_router())
        .with_state(ctx.state.clone())
}

async fn authed_json(app: Router, method: &str, uri: String, token: &str, org_id: Uuid, body: Option<Value>)
    -> anyhow::Result<(StatusCode, Value)>; // bearer + x-organization-id + json

async fn seed_member(ctx: &DbCtx, label: &str) -> anyhow::Result<SeedUserOrgWs>; // unique user/org/role/membership/workspace/session+jwt
async fn seed_second_member_in_org(ctx: &DbCtx, org: &Organization, label: &str) -> anyhow::Result<SeedUser>; // same org, own session/jwt
```

Additional concrete helpers useful for 020:

```rust
async fn insert_task_row(state: &AppState, task: &Task) -> anyhow::Result<Uuid>; // direct SQL insert for controlled search fixtures
async fn soft_delete_task_sql(state: &AppState, task_id: Uuid) -> anyhow::Result<()>;
async fn insert_file_row(state: &AppState, org_id: Uuid, owner_id: Uuid, workspace_id: Option<Uuid>, name: &str, file_type: &str, deleted: bool) -> anyhow::Result<Uuid>;
async fn list_notifications(app: Router, token: &str, org_id: Uuid, qs: &str) -> anyhow::Result<(StatusCode, Value)>;
fn find_notif<'a>(arr: &'a [Value], event: &str, target_id: &str) -> Option<&'a Value>;
```

Notes:
- Prefer API calls for task create/update assignment tests to exercise real mutation hooks; use direct SQL inserts only for search fixture setup/soft-delete control (and to isolate search behavior from task-route side effects).
- Keep `x-organization-id` header consistent with existing search/files contracts where required; notifications list is user-scoped but include org header if router/extractor stack expects it in composed apps (match existing 019 notification tests).

## 4. (A) PG FTS search fallback — tasks & file metadata

### 4.1 Target contract (020)

- **Endpoint/shape unchanged:** `GET /api/v1/search?q=&type=&limit` → `200 { "query": <original>, "results": [ { "type": "message"|"task"|"file", "id": "<uuid>", "snippet": "<text>" } ] }`.
- **PG fallback scope:** when Meilisearch unavailable (`search_service: None`), query Postgres over supported entities: `messages` (existing), `tasks` (`title`, `description`), `files` (`name`, `file_type` metadata).
- **Type filtering:** `type` is CSV, case-insensitive tokens; include only requested supported types. `message/messages` → messages; `task/tasks` → tasks; `file/files` → files. Empty/absent `type` => search all fallback-supported types. Unknown-only types => empty results.
- **Tenant isolation:** every branch constrained by `organization_id = caller org`; membership required (existing 403 path preserved).
- **Soft-delete exclusion:** `messages.is_deleted = false`, `tasks.is_deleted = false`, `files.is_deleted = false`.
- **Query safety/parsing:** trim empty => no results; use `websearch_to_tsquery('simple', $q)` with fallback `plainto_tsquery('simple', $q)` (or equivalent safe behavior) consistently per entity.
- **Snippets:** deterministic, bounded excerpts/sources (e.g. message content snippet; task title or title+description excerpt; file `name` and/or `file_type` string) — assert non-empty/contains token rather than exact formatting except stable cases.
- **Ordering/limit:** per-entity or merged results bounded by requested limit (capped 50); deterministic tiebreak recommended (`rank DESC, created_at DESC, id DESC`) for reproducibility.

### 4.2 Fixtures (search)

**SeedBase-S:** two orgs to prove isolation (`orgA`, `orgB`), each with member+active session/JWT, workspace; optional channels not required for task/file rows.

| Fixture | Org | Entity | Values (unique token = `tok = "s020-<kind>-<uuid>"`) | Flags |
|---|---|---|---|---|
| `msg_a` | A | message | content contains `tok_msg_a`, channel in orgA | `is_deleted=false` |
| `task_a_title` | A | task | title contains `tok_task_title`, description plain | not deleted |
| `task_a_desc` | A | task | title plain, description contains `tok_task_desc` | not deleted |
| `task_a_deleted` | A | task | title/description contains `tok_task_del` | `is_deleted=true` |
| `file_a_name` | A | file | `name="report-<tok_file_name>.pdf"`, `file_type="application/pdf"` | not deleted |
| `file_a_type` | A | file | `name="asset.bin"`, `file_type="image/<tok_file_type>"` or searchable mime token | not deleted |
| `file_a_deleted` | A | file | name contains `tok_file_del` | `is_deleted=true` |
| `task_b`, `file_b`, `msg_b` | B | task/file/message | same token families as A counterparts | not deleted |

Implementation notes: tasks/files can be inserted via direct SQL for precise control (`search_tsv` is generated/stored by migration). Ensure required FKs satisfied (`workspace_id`, `creator_id/owner_id` in org context or valid refs as schema requires).

### 4.3 Cases and assertions (search)

| ID | Case | Request / setup | Assertions |
|---|---|---|---|
| S1 | PG fallback happy path (tasks) | `GET /search?q=<tok_task_title>&type=task` as A, `search_service=None` | `200`; `query` echoes; `results` contains `{type:"task", id:task_a_title}` and not deleted/cross-org IDs; snippet non-empty |
| S2 | Task description indexed | `q=<tok_task_desc>&type=task` | hit `task_a_desc` |
| S3 | PG fallback happy path (files: name) | `q=<tok_file_name>&type=file` | hit `file_a_name` with `type:"file"` |
| S4 | File metadata indexed (`file_type`) | `q=<tok_file_type>&type=file` | hit `file_a_type` |
| S5 | Message regression | `q=<tok_msg_a>&type=message` | existing 019 behavior preserved (message returned), no task/file when filtered to message |
| S6 | Default/merged types include new entities | `q=<token>` with no `type` (or `type=message,task,file`) using distinct tokens per call or a combined token strategy | each entity retrievable when requested/eligible; when multiple types match, returned set contains only allowed types and respects `limit` cap (assert `len <= limit`) |
| S7 | Type filtering correctness | `q=<tok_task_title>&type=message` and `q=<tok_file_name>&type=task` | empty results (no cross-type leakage) |
| S8 | Soft-delete exclusion | search `tok_task_del` and `tok_file_del` (and optionally deleted message) | `200` with empty `results` |
| S9 | Tenant isolation | as A search B-only tokens (`task_b/file_b/msg_b`); as B search A-only tokens | no cross-org IDs in any response |
| S10 | Empty/blank query | `q=%20%20%20` (and/or empty) | `200`, `results: []`, no DB errors |
| S11 | Limit/clamp behavior | seed >=3 matching tasks/files/messages as applicable; `limit=1`, `limit=999` | `<=1` for 1; clamped to max (<=50) for oversized; `200` |
| S12 | Meilisearch-unavailable path | construct app with `search_service: None` (default test state), run S1/S3 | proves PG fallback serves tasks/files without external search; no 503 solely due to missing Meilisearch |
| S13 | Auth/membership guard regression | request without valid auth or non-member org context as per existing route behavior | `401/403` as applicable, no results leakage (can be lightweight/one case if already well-covered; include if cheap) |

**Suggested file targets:** extend `flora-tests/src/contract/search.rs` with `db_search_*_tasks_files_*` cases (or add `mod db_search_tasks_files`), keeping 019 message tests intact.

## 5. (B) Assignment notifications

### 5.1 Target contract (020)

- **Trigger events:**
  - `POST /api/v1/tasks` with `assignee_id = X` where `X != actor_user_id` and `X` is a valid recipient in scope → create `Assignment` notification for `X`.
  - `PATCH /api/v1/tasks/{id}` when effective assignee changes to `X` (`previous_assignee != X`) and `X != actor_user_id` → create `Assignment` notification for `X`.
- **Guards / non-events:**
  - **Self-assignment:** `assignee_id == actor` on create or change-to-self → no notification.
  - **Unassign:** assignee becomes `None` / removed → no notification.
  - **No-op:** update that does not change assignee (same value, or omitted in semantics that preserve existing) → no new notification.
  - **Cross-org/task isolation:** cannot assign/notify across unauthorized org context; task must belong to caller org (existing route checks).
- **Persisted notification:** `event_type="Assignment"` (API string), `user_id=<recipient assignee>`, `organization_id=<task org>`, `target_id=<task id>` (recommended), `is_read=false`.
- **Visibility:** recipient sees it via `GET /api/v1/notifications` (and `unread_only=true` until read); actor/non-recipient must not see recipient-owned item as their own notification.
- **Side-effect semantics:** best-effort/non-fatal preferred (reply pattern) so task mutation is not rolled back by notification insert failure (**NV**: transactional coupling/idempotency).

### 5.2 Fixtures (assignment)

**SeedBase-N:** one org, one workspace, at least three users in same org with active sessions/JWTs:
- `actor` (performs create/update)
- `assignee_b` (recipient)
- `assignee_c` (for reassign target / isolation checks)

All in same `organization_id`; create a task context where needed (workspace belongs to org).

### 5.3 Cases and assertions (assignment)

| ID | Case | Action(s) | Assertions |
|---|---|---|---|
| N1 | Create with assignee notifies recipient | `POST /tasks` as `actor` with `assignee_id=assignee_b.id` | task `201`; `GET /notifications` as `assignee_b` contains `event_type=="Assignment"` for `target_id==task.id`, `is_read==false`; absent for `actor` |
| N2 | Create self-assigned does not notify | `POST /tasks` as `actor` with `assignee_id=actor.id` | `201`; `actor` notifications have no Assignment for that task |
| N3 | Create unassigned does not notify | `POST /tasks` without `assignee_id`/null | `201`; no new Assignment for involved users |
| N4 | Update changes assignee notifies new assignee only | create task assigned to `assignee_b` (or unassigned, depending on setup) then `PATCH /tasks/{id}` as `actor` setting `assignee_id=assignee_c.id` | `200`; `assignee_c` receives Assignment for task; `assignee_b` does not receive a new Assignment for this change event; `actor` does not receive |
| N5 | Reassign semantics are deterministic | from assigned-to-B, patch to C; (optional second patch B? if API allows) verify exactly intended recipient(s) | only current transition target notified (no duplicate spurious notify to previous unless specified) |
| N6 | Update self-assign/no-op does not notify | patch `assignee_id` to `actor` or repeat same assignee | no new Assignment notification emitted for no-op/self-target cases |
| N7 | Unassign does not notify | if 020 API supports clearing assignee, patch to null/unassigned | task updated, no Assignment notification. **If current PATCH cannot clear (keeps existing when omitted/null-mapped), mark N7 as contract-dependent/NV and instead assert “no notification when assignee unchanged.”** |
| N8 | Recipient correctness and isolation | N1/N4 flows | notification `user_id` implied by visibility only to recipient token; cannot read others’ notifications via list; `target_id` matches task |
| N9 | Non-member/foreign org cannot trigger/read | attempt task mutation/read in wrong org context or by non-member | `401/403/404` per existing behavior; no cross-org Assignment leakage |
| N10 | Task mutation succeeds even if notification is best-effort (design) | normal path | task persisted/returned; notification eventually/after await visible. (Fault-injection for failure path is **NV** unless seam exists.) |

**Suggested file target:** `flora-tests/src/contract/notifications.rs` (extend `db_notifications` or add `db_assignment_notifications`) and/or `flora-tests/src/integration/tasks.rs` for end-to-end assignment flow; prefer contract module colocated with existing notification DB tests for consistency.

## 6. Implementation hooks/tests dependency (NVs to resolve for green execution)

| NV | Why it matters | Test impact |
|---|---|---|
| NV-020-1 | Additive migrations for task/file `search_tsv` + GIN indexes not present yet. | S1–S4,S6–S9,S11,S12 depend on schema; keep tests `#[ignore]` pending impl/migration or gate on column/index presence if desired. |
| NV-020-2 | `search.rs` PG fallback currently messages-only. | S1–S4,S6,S7,S12 require route/query extension and merged/type dispatch. |
| NV-020-3 | Task create/update paths emit no Assignment notification today. | N1,N4,N5,N8 depend on hook in `flora-api/src/routes/tasks.rs` (and/or service) using `PgNotificationRepository`/core model. |
| NV-020-4 | `PATCH /tasks` cannot explicitly clear `assignee_id` with current `or(existing.assignee_id)` behavior. | N7 (unassign notify suppression on explicit clear) may need API/DTO semantics change; document and test actual 020 contract. |
| NV-020-5 | `target_url` format for Assignment unspecified. | Assert presence/absence permissively or match agreed convention (e.g. contains task id/path) once defined. |
| NV-020-6 | Idempotency/dedup and tx-coupling (best-effort vs outbox) unspecified. | BI-like duplicate/retry cases remain NV unless unique constraint/guard is added. |
| NV-020-7 | File search source-of-truth limited to `name`/`file_type`; no content indexing. | Keep S3/S4 assertions on metadata fields only. |
| NV-020-8 | Channel-level ACL in search and broader notification delivery (push/realtime/workers) out of 020 scope. | Do not add assertions beyond org/member scoping and in-app list visibility. |

## 7. Recommended execution order (80/20)

1. **Harness alignment:** add 020 modules/helpers mirroring `db_search`/`db_notifications` (`db_state`, `make_test_state`, `authed_json`, seeded multi-user org fixtures).
2. **Search fixtures + behavioral tests:** land migration + PG fallback extension, then enable S1–S12 (messages regression + tenant/soft-delete/type/limit/empty/unavailable-path).
3. **Assignment mutation hook + tests:** implement create/update assignment emission with self/no-op/unassign guards, then enable N1–N6,N8–N10 (and N7 per finalized clear semantics).
4. **Stabilize in CI:** run ignored DB-gated suite on disposable Postgres 18, keep non-DB CI green, record NVs (relevance/i18n/CJK, ACL, dedup/tx-coupling, quota/operability follow-ups) in cycle artifacts.

## 8. References (local)

- `OBJETIVO.md`, `ROADMAP.md`
- 019: `specs/019-postgres-search-reply-notification/spec.md`, `.specify/assessments/019-next-vertical-slice/decision.md`, `flora-core/src/migrations/sql/20261117000001_add_messages_search_fts.sql`
- Search: `flora-api/src/routes/search.rs`, `flora-search/src/lib.rs`, `flora-core/src/config/search.rs`
- Tasks: `flora-api/src/routes/tasks.rs`, `flora-core/src/models/task.rs`, `flora-core/src/repositories/task_repository.rs`, `flora-core/src/migrations/sql/20240818000001_init_schema.sql`
- Files: `flora-api/src/routes/files.rs`, `flora-core/src/models/file.rs`, `flora-core/src/repositories/file_repository.rs`, `flora-core/src/migrations/sql/20261116000002_add_workspace_id_to_files.sql`
- Notifications: `flora-api/src/routes/notifications.rs`, `flora-api/src/routes/channels.rs` (`maybe_notify_reply` pattern), `flora-core/src/models/notification.rs`, `flora-core/src/repositories/notification_repository.rs`
- DB-gated patterns: `flora-tests/src/contract/search.rs`, `flora-tests/src/contract/notifications.rs`, `flora-tests/src/contract/tasks.rs`, `flora-tests/src/contract/files.rs`, `flora-tests/src/contract/messaging.rs`, `flora-tests/src/contract/auth.rs`, `flora-tests/src/contract/mod.rs`, `flora-tests/src/integration.rs`
- Prior research: `docs/research/2026-11-16-019-db-gated-tests-search-notifications.md`, `docs/research/2026-11-16-019-postgres-fts-search-vps.md`, `docs/research/2026-11-16-019-inapp-notification-wiring-vps.md`, `docs/research/2026-11-16-018-search-notifications-wiring-vps.md`
