# 019 — DB-gated, deterministic contract/integration tests: Postgres FTS search fallback & reply notifications

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`3c8eddbfd`) — **research/design only** (no source/commit changes).  
**Scope:** cycle 019 disposable **PostgreSQL 18** tests for (A) `GET /api/v1/search` message search via **Postgres FTS fallback** (indexing on insert/update/delete, tenant isolation, empty/type/limit, Meilisearch-unavailable path) and (B) **1 in-app notification** created on a core mutation (**reply-in-thread → `Reply`**) visible via `GET /api/v1/notifications`.

## 1. Cross-check: existing Flora patterns and current state

| Area | Evidence (read) | Implication for 019 tests |
|---|---|---|
| DB-gated contract style | `flora-tests/src/contract/{auth.rs,messaging.rs,files.rs,websocket.rs}`: `#[cfg(test)] mod db_* { ... }`, `#[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle N)"]`, `db_state() -> anyhow::Result<Option<(AppState, Config)>>` reads `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`, connects, runs `flora_core::migrations::run_migrations(&pool)`, returns `Ok(None)` + `eprintln!` skip when unset. Seeds use `Uuid::now_v7()` uniqueness, direct repo inserts (`PgUser/Org/Role/Membership/Workspace/SessionRepository`), `jwt::encode_token` with `FloraClaims`, `Router::oneshot` via `tower::ServiceExt`. | Reuse exact env-gating/skip semantics, migration-on-connect, unique fixtures, `oneshot` contract requests, `Config::default()` base with targeted overrides (e.g. storage in files). Keep deterministic by scoping assertions to seeded IDs/orgs/users, not global counts/order except explicitly ordered API results. |
| Harness/wiring | `AppState` (`flora-api/src/state/app_state.rs`) requires real `PgPool` + `Config`; builds repos/services, sets `search_service` only when Meilisearch enabled/init succeeds, `realtime_hub` in-process, optional Valkey/messaging. `flora-tests/src/contract/mod.rs` already registers `search`, `notifications` stubs; `src/integration.rs` registers `search`, `notifications`. | Tests must construct `AppState`/routers consistent with target behavior (may need config toggles for backend selection). Prefer contract tests through routers (`/api/v1/search`, `/api/v1/notifications`, `/channels`) to lock API behavior; use integration tests for DB invariants/idempotency hooks if needed. |
| Search (current) | `flora-api/src/routes/search.rs`: `GET /api/v1/search?q=&type=&limit=` requires auth+org membership, empty/blank `q` => `200 {query, results:[]}`, `search_service` `None` => `503 ServiceUnavailable("Search is disabled or unavailable...")`, delegates to `flora_search::SearchService` (Meilisearch, `flora_org_{org_id}`, message doc `content/channel_id/sender_id/created_at`), CSV `type` filter, `limit` default 20 capped 50, response `{query, results:[{type,id,snippet}]}`. No mutation/index wiring in `channels.rs`/`tasks.rs`. No FTS columns/indexes in migrations (`20240818000001_init_schema.sql`, `20261116000002_add_workspace_id_to_files.sql`). Config: `flora_core::config::search` (`enabled,url,api_key,index_template,...`). | **Target 019 contract differs from current impl**: Postgres FTS fallback must serve message search when Meilisearch unavailable/disabled-for-external-engine (no hard dependency). Tests should be written against the intended 019 contract and gated to the implemented backend toggle/migration (see NVs). Deterministic by avoiding external Meilisearch in default DB tests. |
| Notifications (current) | `notifications` table + indexes exist; `Notification/NotificationType::Reply` in `flora-core/src/models/notification.rs`; `PgNotificationRepository::{find_unread_by_user_id,find_by_user_id,create,mark_as_read}` in `flora-core/src/repositories/notification_repository.rs`; trait in `traits/repository.rs`. Routes: `GET /api/v1/notifications?limit&unread_only` lists authed user only, `POST /{id}/read` ownership-checked. `post_message` in `channels.rs` persists message (supports `thread_id`) and emits realtime only; no notification creation. `flora-notifications` crate (Valkey `XADD`) is unused by API. | 019 B is a mutation-side wiring gap: reply to another user’s message should create persisted `Reply` notification visible via existing GET. Tests can drive HTTP mutation path and assert via GET + optional DB read for completeness. |
| Prior research alignment | `docs/research/2026-11-16-018-search-notifications-wiring-vps.md` (Postgres-first/optional search, in-process hooks, DB truth), `...-018-macro-search-notifications-parity.md` (80/20: idempotent source-of-truth updates, tenant-scoped search, simple in-app notifications), `...-019-minimal-notification-wiring-vps.md` (choose reply-in-thread, guards, best-effort/post-persist semantics, NVs on idempotency/target_url/tx). | Test design follows chosen 80/20 slice: message FTS fallback + reply notification; avoids Meilisearch/Kafka/outbox/push requirements. |

## 2. Test topology and env-gating (disposable PostgreSQL 18)

### 2.1 Gating and runtime contract
- **Execution:** `#[tokio::test]` + `#[ignore = "requires Postgres 18; set FLORA_DATABASE__POSTGRES_URL or DATABASE_URL (cycle 019)"]` (CI/job runs `--ignored` in DB-enabled lane, matching cycles 009/012/013/015/018).
- **DB selection:** `FLORA_DATABASE__POSTGRES_URL` preferred, fallback `DATABASE_URL`; if missing return `Ok(())` after `eprintln!("skipping ...")` (no false failures in non-DB envs).
- **Disposable PG18 strategy:** prefer CI-provided ephemeral Postgres 18 service/container; tests are safe on disposable DBs because fixtures are uniquely namespaced (`Uuid::now_v7`) and never assume empty tables except within scoped IDs. Optional local helper: create/drop `flora_test_019_<uuid>` database per run when permissions allow (**NV: runner privileges**).
- **Migrations:** always `flora_core::migrations::run_migrations(&pool).await?` before use. For (A), Postgres FTS fallback tests additionally require the 019 migration that introduces message search projection/index (`tsvector` generated column/index or equivalent) — gate with capability probe or feature/config flag and skip with explicit reason if absent (**NV-1**).
- **Determinism:** no wall-clock sleeps; assert on returned IDs/fields; fixed query strings (`"deterministic-needle-<uniq>"`), bounded limits, explicit org/user scoping, `oneshot` routers (no shared global state except in-process hub which is unused here).

### 2.2 Minimal harness helpers (proposed, factor from existing db_* modules)

Add a small internal test helper module (e.g. `flora-tests/src/contract/db_harness.rs` or duplicated per-module initially, then factored) to remove repeated setup and keep 019 tests readable:

```rust
struct DbCtx { state: AppState, config: Config }

async fn db_ctx() -> anyhow::Result<Option<DbCtx>>; // env gate + connect + migrate + make_test_state
fn make_test_state(pool: PgPool, config: &Config) -> AppState; // same as auth/files/messaging (search/messaging/valkey None by default)

struct SeedBase { org_a: Organization, org_b: Organization, user_a: User, user_b: User, ws_a: Workspace, ws_b: Workspace, ch_a: Uuid, ch_b: Uuid, token_a: String, token_b: String, jti_a: String }

async fn seed_base(ctx: &DbCtx) -> anyhow::Result<SeedBase>; // two isolated orgs/workspaces/channels, members, active sessions, JWTs
async fn authed_json(app: Router, method: &str, uri: impl Into<String>, token: &str, body: Option<Value>) -> anyhow::Result<(StatusCode, Value)>;
fn api_app(ctx: &DbCtx) -> Router; // nest /api/v1/search, /api/v1/notifications, /channels (and /api/v1/tasks if needed) with_state(ctx.state.clone())
async fn insert_message_sql(state: &AppState, msg: &Message) -> anyhow::Result<()>; // deterministic direct insert for indexing/delete cases
async fn update_message_content(state: &AppState, id: Uuid, content: &str) -> anyhow::Result<()>; // SQL update
async fn soft_delete_message(state: &AppState, id: Uuid) -> anyhow::Result<()>; // SQL or repo
```

Notes:
- `make_test_state` should accept/allow `search_service: None` and, for fallback tests, a `Config` with search backend configured to Postgres fallback (e.g. `search.provider = "postgres"` or `search.enabled = true` + degraded-external behavior) once 019 implements it; **do not require Meilisearch env vars**.
- Keep helpers `anyhow::Result` and avoid `unwrap/expect` in non-test-control paths per workspace lints (test modules already use `?`/`anyhow::ensure!`).
- Reuse existing direct-SQL insert style from `messaging.rs::insert_channel` where repo enum mapping is awkward; for messages prefer repo insert when possible to exercise the same path mutation hooks should use.

## 3. (A) Postgres FTS search fallback — `GET /api/v1/search`

### 3.1 Target contract (019, Postgres fallback path)
- **Availability:** when Meilisearch is unavailable/disabled/unconfigured for external use and Postgres FTS fallback is enabled, endpoint remains usable (`200`) rather than `503` (current behavior) for supported fallback-backed queries/types; degraded/unsupported modes must return explicit, stable error/capability (**NV-2** if product chooses 503-until-impl, tests must match chosen contract).
- **Empty/type/limit:** `q` blank/whitespace => `200 {query:<original>, results:[]}`. `type` CSV filters supported fallback types (at least `message`; unknown types yield empty/no-match per impl, must be deterministic). `limit` default 20, max 50 (or current cap) — clamp enforced.
- **Index freshness:** message insert/update/delete (soft-delete) reflected in fallback search results according to implemented hook/projection model (generated `tsvector` = immediate read-after-write in same DB; trigger/outbox = after commit/hook completion — tests should perform mutations through the same app/DB connection context and query after awaited completion).
- **Tenant isolation:** results include only caller’s `organization_id`; membership required (403/forbidden path preserved).
- **Ranking/snippet:** deterministic enough to assert membership/content presence; avoid brittle rank-order assertions except simple cases (single match) or explicit `ORDER BY` contract.

### 3.2 Proposed tests (contract-level, DB-gated)

| ID | Name | Seed / mutation | Request | Assertions |
|---|---|---|---|---|
| A1 | `db_search_empty_query_returns_empty` | base seed, at least 1 indexed message in org A | `GET /api/v1/search?q=%20%20%20&type=message` as user A | `200`, `query` preserves raw param (or trimmed per contract — assert chosen behavior), `results == []` |
| A2 | `db_search_message_insert_is_indexed` | insert channel+message in org A with unique token `fts-insert-<uniq>` via repo/API mutation path | `GET /api/v1/search?q=fts-insert-<uniq>&type=message` | `200`, results contain entry with `type=="message"`, `id==message.id`, snippet/content evidence includes token (or non-empty snippet), no org B leakage |
| A3 | `db_search_message_update_reindexes` | create indexed message `"alpha-<uniq>"`, then update content to `"beta-<uniq>"` via edit path/repo+hook | search `alpha` then `beta` | old token not returned for that message (or absent), new token returned; `id` stable |
| A4 | `db_search_message_delete_removes_or_hides` | create indexed message, soft-delete/hard-delete per implemented semantics | search token | message absent from results (soft-deleted messages must not be searchable) |
| A5 | `db_search_tenant_isolation` | org A has `"sharedtoken-<uniq>"`, org B has `"sharedtoken-<uniq>"` in different messages/channels | search token as user A and as user B | each sees only own org message ID(s); cross-org ID never present |
| A6 | `db_search_type_filter_and_limit` | seed >=3 message matches and (if/when available) a task/file match sharing token, or seed only messages and request `type=task` | `type=message`, `type=task`, `type=message,task`, `limit=1`, `limit=999` | type filter respected for fallback-supported types; `len(results) <= 1` for `limit=1`; oversized limit clamped (<=50) and no error |
| A7 | `db_search_meilisearch_unavailable_uses_postgres_fallback` | configure state with `search_service: None` / external search disabled-unavailable **and** Postgres fallback enabled; seeded message | same as A2 | `200` with DB-backed message hit (proves fallback path, not 503) |
| A8 | `db_search_requires_membership` | valid token/user but request in context where org membership missing/mismatched (follow existing authz test style) | `GET /api/v1/search?q=x` | `403`/forbidden consistent with route; no data leak |

### 3.3 Integration/DB invariants (optional but valuable)

| ID | Focus | Assertion |
|---|---|---|
| AI1 | Projection/index presence | if 019 uses generated `search_tsv` column, `SELECT to_regclass(...)`/column existence check in test setup or dedicated ignored test confirms migration applied; query uses `@@` path (can be inferred behaviorally) |
| AI2 | Read-after-write consistency | insert via same pool/transaction semantics used by app, immediate subsequent search returns row (guards async hook regressions) |
| AI3 | Soft-delete predicate | DB row with `is_deleted=true` excluded even if token still in content column |

## 4. (B) In-app notification on core mutation — reply-in-thread

### 4.1 Target contract (019, single event)
- **Trigger:** `POST /api/v1/channels/{channel_id}/messages` with `thread_id = <parent_id>` creating a reply.
- **Emit when:** parent exists, not deleted, `parent.channel_id == channel_id`, `parent.organization_id == org_id`, and `parent.sender_id != auth.user_id`.
- **No emit when:** root message (`thread_id` null), self-reply, missing/invalid/cross-channel/cross-org parent, deleted parent (**NV-3**: if parent deleted but reply allowed, notification policy must be explicit).
- **Persisted row:** `NotificationType::Reply`, `organization_id = org_id`, `user_id = parent.sender_id` (recipient), `target_id = reply_message.id` (created reply), `target_url` canonical deep-link (**NV-4**), `is_read = false`.
- **Visible via API:** `GET /api/v1/notifications` (recipient token) returns the notification; unread filter includes it until read; non-recipient does not see it.

### 4.2 Proposed tests (contract + integration, DB-gated)

| ID | Name | Setup | Action | Assertions |
|---|---|---|---|---|
| B1 | `db_reply_creates_notification_for_parent_author` | seed user A (token A), user B (token B) same org/workspace/channel/session membership; A posts root message | B posts reply with `{"content":"r","thread_id":"<root_id>"}` | reply `201`; `GET /api/v1/notifications` as A `200` contains item with `event_type=="Reply"`, `target_id==reply_id`, `is_read==false`, plausible `created_at`; item absent/ not owned by B |
| B2 | `db_self_reply_does_not_create_notification` | user A posts root then replies to self | same as above by A | `201`; A’s notifications unchanged for this target/event (or count delta 0 for scoped query) |
| B3 | `db_unread_only_filter_includes_reply_then_mark_read_hides` | from B1 | `GET /notifications?unread_only=true` as A, then `POST /notifications/{id}/read`, then unread query again | unread includes reply; read returns `204`; subsequent unread excludes it; full list may still include read item depending on API semantics (assert actual contract) |
| B4 | `db_reply_notification_tenant_and_channel_guard` | parent in org/channel A; attempt reply in different valid channel/org context or crafted mismatch path (if API permits thread_id only) | mutate/query | no `Reply` notification to foreign author; no cross-org visibility in notifications list |
| B5 | `db_root_message_does_not_create_reply_notification` | post message without `thread_id` | inspect recipient notifications | no new `Reply` row for that target |
| B6 | `db_reply_notification_target_and_recipient_integrity` | B replies to A | fetch notifications as A and optionally `find_unread_by_user_id`/DB check | exactly intended recipient (`user_id==A.id`), not actor; `target_id` points to reply (not root) unless contract states otherwise — assert 019 choice; `organization_id` matches |

### 4.3 Integration/idempotency probes (design for implementation hardening)

| ID | Focus | Assertion |
|---|---|---|
| BI1 | Best-effort non-fatal side effect | if notification insert can be fault-injected (**NV-5**), message create still `201` and realtime/message readable; otherwise document as manual/resilience check, not automated in 019 |
| BI2 | Duplicate suppression | if unique guard `(user_id,event_type,target_id)` or equivalent added, repeated emission attempt for same logical reply does not create second row; if not added, mark **expected NV** and avoid asserting singleton globally except “one per single successful request path” |
| BI3 | Multi-reply sequence | two different replies to same root produce distinct notifications (distinct `target_id`s) for parent author (unless dedupe policy states otherwise) |

## 5. Concrete fixture recipes

- **Search fixtures:** `org_a/org_b`, `user_a/user_b`, `workspace_a/b`, `channel_a/b`, membership+session for each caller. Messages with high-entropy tokens: `format!("fts-{case}-{}", Uuid::now_v7().simple())` in `content`; include ASCII baseline token for deterministic parsing and, as optional extra cases, localized tokens (`café-<u>`, `mañana-<u>`, `straße-<u>`, `français-<u>`, `搜索-<u>`, `検索-<u>`) behind a locale-capability note because default `'simple'`/current fallback implementation may not stem/segment equally (**NV-6**).
- **Notification fixtures:** one root author (recipient candidate), one different replier (actor), one channel in same org/workspace; create root via `POST /channels/{id}/messages` to obtain real `root_id`, reply via same API to obtain `reply_id`; read notifications via `GET /api/v1/notifications` as recipient token.
- **Router composition:** `Router::new().nest("/api/v1/search", create_search_router()).nest("/api/v1/notifications", create_notifications_router()).nest("/channels", create_channels_router()).with_state(state.clone())` to exercise real route stacks and auth extractors.

## 6. Risks and NVs (must be resolved to make tests executable green)

| NV | Issue | Test impact |
|---|---|---|
| NV-1 | **FTS migration/schema absent today** (`tsvector`/GIN/triggers/projection). | A2–A7/AI1–AI3 require implemented Postgres FTS fallback; until then keep as `#[ignore]` with “pending 019 impl” or gate on schema capability. |
| NV-2 | **Current search behavior is 503 when `search_service` is None**. | A7 conflicts with current route; define/implement fallback contract (backend selection, degraded Meilisearch, or provider config) before enabling. |
| NV-3 | **Reply-to-deleted-parent semantics** not fully specified. | B4/B5 guard expectations depend on chosen mutation contract. |
| NV-4 | **Canonical `target_url` / deep-link format** unknown. | B1/B6 can assert non-empty/prefix/contains IDs, but exact string needs product/frontend agreement. |
| NV-5 | **Best-effort fault injection seam** unavailable in current route/repo structure. | BI1 may remain design-only/manual unless helper boundary introduced. |
| NV-6 | **Multilingual tokenization quality** (`'simple'` vs locale dicts, CJK/JP segmentation). | Locale fixtures should be non-blocking or split into capability-gated tests; baseline ASCII deterministic. |
| NV-7 | **Idempotency/dedupe** not present in `notifications` schema/repo. | BI2 depends on explicit design decision (unique constraint vs accept at-least-once). |
| NV-8 | **Indexing hook placement** (repo/service/route/outbox) for insert/update/delete. | Determines whether A2–A4 should run through HTTP mutations only or also allow direct SQL+hook invocation. |

## 7. Recommended implementation order (80/20)

1. **Harness first:** add reusable `db_ctx`/`seed_base`/`authed_json` helpers in `flora-tests` (or inline in new `db_search`/`db_notifications` modules) mirroring auth/messaging/files patterns.
2. **B before A if FTS migration not ready:** reply-notification tests (B1–B3,B5,B6) need only existing schema/APIs + mutation wiring and give immediate P0 value; keep A* ignored pending fallback implementation.
3. **A with backend contract:** land Postgres FTS migration/projection + fallback selection, then enable A1–A8.
4. **Harden:** add idempotency guard and tenant/soft-delete negative cases (A4,A5,B2,B4,BI2).

## 8. References (local)
- `flora-tests/src/contract/auth.rs`, `messaging.rs`, `files.rs`, `mod.rs`, `flora-tests/src/integration.rs`
- `flora-api/src/routes/search.rs`, `notifications.rs`, `channels.rs`, `tasks.rs`, `state/app_state.rs`
- `flora-core/src/migrations/sql/20240818000001_init_schema.sql`, `20261116000002_add_workspace_id_to_files.sql`, `flora-core/src/migrations/mod.rs`
- `flora-core/src/models/{message.rs,notification.rs}`, `flora-core/src/repositories/{message_repository.rs,notification_repository.rs}`, `flora-core/src/config/search.rs`
- `flora-search/src/lib.rs`, `flora-notifications/src/lib.rs`
- `docs/research/2026-11-16-018-macro-search-notifications-parity.md`, `2026-11-16-018-search-notifications-wiring-vps.md`, `2026-11-16-019-minimal-notification-wiring-vps.md`
