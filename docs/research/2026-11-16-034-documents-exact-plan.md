# 034 — FASE 7 Documents: exact implementation plan (cycle 035 backlog)

**Date:** 2026-11-16
**Canonical baseline:** `github/dev@1be723ccadbc300af2b5adfed1915df47d4c103f` (merge of PR #247). Local `dev@babac3ea1` differs **only** in `flora-api/src/routes/health.rs` and `flora-api/src/routes/mod.rs` (other agent's `/metrics` work). **Never revert those two files** — rebase the two Documents edits onto the local version of `routes/mod.rs`.
**Refresh of:** `docs/research/2026-11-16-033-documents-minimal-design.md` (356 lines). Research-only: **no application files were modified** by this note.
**Question:** What exactly must be written, in what order, for a thin versioned-Markdown Documents slice to land CI-green in one cycle — and where did 033's assumptions drift from the canonical tree?
**Method:** (1) line-by-line re-read of the canonical implementation anchors; (2) primary-source re-verification (PostgreSQL 18 docs, Svelte 5 docs, PG vendor docs); (3) multilingual cross-check EN/pt-BR/es/de/fr/zh-CN/ja. Tags: `[P…]` primary external, `[S…]` secondary external, `[F…]` repo, `[M…]` multilingual. **NV** = not verified in this environment (no DB run, no live API smoke, no browser e2e, no load/CJK benchmark, no compile executed for the proposed files).

---

## 0. Verdict (up front)

**GO for one CI-green cycle, conditional.** The 80/20 cut fits: one table + generated FTS column, 5 CRUD handlers + 1 search branch, one Svelte 5 editor, zero new dependencies. **No AppState field** (§4, trap T1) removes 7 struct-literal edits. The two honest seams — CJK recall under `simple` and channel scope not being an ACL — are documented, not fixed in this slice.

**Size:** 26 required files (15 backend/test/CI + 11 frontend) + 2 optional polish + 2 governance docs. Estimated 1,400–2,000 new/changed LOC; ~1 focused cycle including two CI rounds. Split only if revision history, per-channel ACL, CJK config, or attachments enter scope (§10).

### What changed since 033 (drift corrections)

| 033 said | Canonical reality (verified) | Consequence |
|---|---|---|
| "Adding an `AppState` field breaks `extractors/auth.rs` helper" | **7** struct literals: `flora-api/src/extractors/auth.rs:275` and `flora-tests/src/contract/{auth:64,files:96,messaging:66,notifications:69,search:63,websocket:65}.rs` `[F1][F2]` | Prefer **no new field** (§4 T1). |
| "FE dead search box is a cheap adjacent gap" | Cycle 033 **already wired it**: `SearchBox.svelte` → `ApiClient.search` → `/api/v1/search`, with `resultTypeLabel()` `[F11]` | Only add `document` label + copy; do not rebuild. |
| — | `flora-core/src/services/draft_service.rs` exists; in-memory auto-save for messages/tasks `[F12]` | Unrelated to Documents. Do not conflate or reuse. |
| "add DB-gated suite" | CI runs **explicit filters** for 6 modules only; no `contract::documents::db_` step `[F13]` | Must add a CI step or the suite never runs. |
| migration `20261119000001_documents.sql` | Still the next free version (`> 20261118000001`) `[F3]` | Keep the name; verify no parallel claim before merge. |
| — | FE has no markdown dep; `svelte ^5.56.1` (> 5.53.5 advisory); runes forced for all non-`node_modules` files `[F11][F14]` | P0 textarea + `<pre>`; no sanitizer dependency. |

---

## 1. Truth anchors (canonical, verified)

| Area | Evidence | Decision |
|---|---|---|
| Tenancy | `organization_id` denormalized on tasks/files/messages/channels; "every query MUST include organization_id" `[F4]`; `AuthExtractor::require_org()` is session-authoritative, not a request header `[F1]` | `organization_id NOT NULL` (no FK), pushed into SQL `WHERE`. |
| Scope | tasks = `workspace_id + organization_id`; channels add `type ∈ {Public,Private}` + `organization_id`; files gained `workspace_id` `[F4][F3]` | `workspace_id NOT NULL` FK, `channel_id NULL` FK. |
| Soft delete | `is_deleted BOOLEAN DEFAULT FALSE` + partial index `WHERE is_deleted = false` `[F4][F3]` | Copy verbatim; `purge_old` is P1. |
| FTS | Stored generated `tsvector`, `to_tsvector('simple', …)`, `coalesce`, separator normalization `replace(replace(replace(x,'-',' '),'.',' '),'_',' ')`, partial GIN `[F3]`; PG18 confirms the pattern `[P1][P2][P3]` | Copy the normalized expression byte-for-byte. |
| Search | `websearch_to_tsquery('simple',$2)` with `plainto_tsquery` fallback via `COALESCE`, `ts_rank_cd`, `@@`, `q.tsq IS NOT NULL AND q.tsq <> ''::tsquery`, org + is_deleted filters, deterministic sort/truncate, `limit.min(50)`, `source: "postgres"` `[F5]` | Add a fourth branch; no new search service. |
| Layering | model → trait → Pg repo → service → route → router, all crate-local `[F6][F7][F8][F9][F10]` | No new crate. |
| Errors | `Error` enum; `IntoResponse` is a true exhaustive `match &self`; `is_client_error`/`is_not_found` are `matches!` `[F2:error.rs]` | Three separate edit sites; see T2. |
| Handler auth | Every handler: `require_org()` → membership → entity check; tasks **does not** verify `req.workspace_id` belongs to `org_id` `[F7:169-192]` | Documents must add the missing check (§5). |
| Router wiring | Routes construct repos from `state.db_pool` (tasks.rs, search.rs) and **ignore** `state.task_service` `[F5][F7]` | Strong argument for no AppState field. |
| FE | runes `.svelte.ts` class stores, `ApiClient.request` generic + 401→`lockSession`, `toChannel`/`toMessage` mappers, UI primitives via bits-ui, `lucide-svelte` `[F11][F14]` | Reuse; add methods, not a new transport. |

---

## 2. Exact migration SQL

**New file:** `flora-core/src/migrations/sql/20261119000001_documents.sql`
Timestamp `20261119000001 > 20261118000001` (latest existing) `[F3]`. All DDL guarded. The generated column is **inline** in `CREATE TABLE`, so no `ALTER … ADD COLUMN` is needed; it is IMMUTABLE because the 2-arg `to_tsvector('simple', …)` is required for generated/expression indexes `[P1]` and `replace`/`coalesce`/`||` are immutable.

```sql
-- 035: FASE 7 Documents — one versioned Markdown note table with authoritative PG FTS.
-- Timestamp > 20261118000001; guarded DDL per 023 convention [F3].
-- The generated column is derived data only: dropping/rebuilding loses no source data [P1].

CREATE TABLE IF NOT EXISTS documents (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    organization_id UUID NOT NULL,                                  -- denormalized tenant key [F4]
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    channel_id      UUID REFERENCES channels(id) ON DELETE SET NULL, -- NULL => workspace-scoped
    creator_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title           VARCHAR(255) NOT NULL,
    content         TEXT NOT NULL DEFAULT '',                       -- Markdown source; app-capped
    version         INTEGER NOT NULL DEFAULT 1 CHECK (version > 0), -- CAS token, NOT a history log
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Verbatim 023 separator-normalization expression [F3]; 'simple' keeps the column IMMUTABLE [P1]
    search_tsv tsvector GENERATED ALWAYS AS (
        to_tsvector(
            'simple',
            replace(replace(replace(coalesce(title, ''), '-', ' '), '.', ' '), '_', ' ')
            || ' '
            || replace(replace(replace(coalesce(content, ''), '-', ' '), '.', ' '), '_', ' ')
        )
    ) STORED
);

CREATE INDEX IF NOT EXISTS idx_documents_organization_id ON documents(organization_id);
CREATE INDEX IF NOT EXISTS idx_documents_workspace_id   ON documents(workspace_id);
CREATE INDEX IF NOT EXISTS idx_documents_channel_id     ON documents(channel_id) WHERE channel_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_documents_creator_id     ON documents(creator_id);
CREATE INDEX IF NOT EXISTS idx_documents_is_deleted     ON documents(is_deleted) WHERE is_deleted = FALSE;
CREATE INDEX IF NOT EXISTS idx_documents_org_updated    ON documents(organization_id, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_documents_search_tsv     ON documents USING GIN (search_tsv) WHERE is_deleted = FALSE;

-- Defense-in-depth only; no policies, matching every other tenant table [F4].
ALTER TABLE documents ENABLE ROW LEVEL SECURITY;
```

**Idempotency note:** sqlx tracks versions, so this runs once. If a partial apply ever requires re-running, first `DROP INDEX IF EXISTS idx_documents_search_tsv; ALTER TABLE documents DROP COLUMN IF EXISTS search_tsv;` (the 023 pattern) — not needed for a fresh DB. `IF EXISTS`/`IF NOT EXISTS` guards are mandatory per task.

**Deferred, not in this migration:** `document_revisions (id, document_id, version, title, content, created_by, created_at)` append-only on every successful save. The `version` integer is **only** a CAS token; cycle docs must say so to avoid a false "version history" claim (R8).

---

## 3. Exact Rust shapes

### 3.1 Model — `flora-core/src/models/document.rs` (new)
```rust
//! Document model — a versioned Markdown note scoped to a workspace.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Document {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub workspace_id: Uuid,
    pub channel_id: Option<Uuid>,
    pub creator_id: Uuid,
    pub title: String,
    pub content: String,
    pub version: i32,              // CAS token only
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Document {
    #[must_use]
    pub fn new(organization_id: Uuid, workspace_id: Uuid, channel_id: Option<Uuid>,
               creator_id: Uuid, title: &str) -> Self {
        let now = Utc::now();
        Self { id: Uuid::now_v7(), organization_id, workspace_id, channel_id, creator_id,
               title: title.to_owned(), content: String::new(), version: 1,
               is_deleted: false, created_at: now, updated_at: now }
    }
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CreateDocumentInput {
    #[validate(length(min = 1, max = 255, message = "title must be 1-255 characters"))]
    pub title: String,
    #[validate(length(max = 200_000, message = "content must be at most 200000 characters"))]
    pub content: Option<String>,
    pub workspace_id: Uuid,
    pub channel_id: Option<Uuid>,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct UpdateDocumentInput {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    #[validate(length(max = 200_000))]
    pub content: Option<String>,
    pub expected_version: i32,     // required by serde on PATCH
}
```
`#[validate(length(...))]` on `Option<String>` is the repo's established pattern (`CreateTaskInput.description`) `[F6]`. `FromRow` maps only the explicit column lists below (never select `search_tsv`).

### 3.2 Barrel edits
- `flora-core/src/models/mod.rs`: add `pub mod document;` + `pub use document::{CreateDocumentInput, Document, UpdateDocumentInput};`
- `flora-core/src/repositories/mod.rs`: add `pub mod document_repository;` + `pub use document_repository::PgDocumentRepository;`
- `flora-core/src/services/mod.rs`: add `mod document_service;` + `pub use document_service::DocumentService;`

### 3.3 Trait — `flora-core/src/traits/repository.rs`
Add `Document` to the `use crate::models::{...}` list (lines 9–12) and append:
```rust
/// Repository trait for Document operations.
#[async_trait]
pub trait DocumentRepository: Send + Sync + std::fmt::Debug {
    /// Find a non-deleted document by ID within an organization (tenant predicate in SQL).
    async fn find_by_id_for_org(&self, id: Uuid, organization_id: Uuid)
        -> crate::Result<Option<Document>>;
    /// List workspace documents; `channel_id = None` means no channel filter.
    async fn find_by_workspace_id(&self, organization_id: Uuid, workspace_id: Uuid,
        channel_id: Option<Uuid>, pagination: Pagination) -> crate::Result<Page<Document>>;
    async fn create(&self, document: Document) -> crate::Result<Document>;
    /// CAS update. `Ok(None)` => missing OR stale version (disambiguate in the service).
    async fn update_checked(&self, id: Uuid, organization_id: Uuid, expected_version: i32,
        title: Option<&str>, content: Option<&str>) -> crate::Result<Option<Document>>;
    async fn soft_delete(&self, id: Uuid, organization_id: Uuid) -> crate::Result<()>;
    async fn restore(&self, id: Uuid, organization_id: Uuid) -> crate::Result<()>; // optional P0.5
}
```

### 3.4 Repository — `flora-core/src/repositories/document_repository.rs` (new)
`#[derive(Debug)] pub struct PgDocumentRepository { pool: PgPool }` with `pub const fn new(pool: PgPool) -> Self` (mirror `PgTaskRepository` `[F8]`). Use `#[async_trait]`, explicit 11-column lists, `.map_err(Error::from_sqlx)?` / `?`, `fetch_optional` + `ok_or_else`, no `unwrap`/`expect` in prod. Core SQL:

```sql
-- find_by_id_for_org
SELECT id, organization_id, workspace_id, channel_id, creator_id, title, content,
       version, is_deleted, created_at, updated_at
FROM documents
WHERE id = $1 AND organization_id = $2 AND is_deleted = false

-- find_by_workspace_id (channel optional)
... WHERE organization_id = $1 AND workspace_id = $2 AND is_deleted = false
      AND ($4::uuid IS NULL OR channel_id = $4)
ORDER BY updated_at DESC, id DESC LIMIT $5 OFFSET $6
-- bind offset as i64::try_from(pagination.offset).unwrap_or(0) (never .unwrap())

-- create
INSERT INTO documents (id, organization_id, workspace_id, channel_id, creator_id,
                       title, content, version)
VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
RETURNING <11 columns>

-- update_checked (the CAS invariant; increment in SQL, never write a computed version)
UPDATE documents
   SET title   = COALESCE($4, title),
       content = COALESCE($5, content),
       version = version + 1,
       updated_at = NOW()
 WHERE id = $1 AND organization_id = $2 AND version = $3 AND is_deleted = false
RETURNING <11 columns>

-- soft_delete / restore: UPDATE ... SET is_deleted = true|false, updated_at = NOW()
-- WHERE id = $1 AND organization_id = $2; rows_affected()==0 => Error::DocumentNotFound
```
`content: Some("")` clears; `None` leaves the column unchanged — no tri-state deserializer needed (unlike `assignee_id` `[F7]`).

### 3.5 Service — `flora-core/src/services/document_service.rs` (new)
```rust
#[derive(Debug, Clone)]
pub struct DocumentService {
    document_repo: Arc<dyn DocumentRepository + Send + Sync>,
}
impl DocumentService {
    #[must_use] pub fn new(document_repo: Arc<dyn DocumentRepository + Send + Sync>) -> Self { ... }

    pub async fn create_document(&self, organization_id: Uuid, creator_id: Uuid,
        input: CreateDocumentInput) -> Result<Document>;            // validate() then repo.create
    pub async fn list_documents(&self, organization_id: Uuid, workspace_id: Uuid,
        channel_id: Option<Uuid>, pagination: Pagination) -> Result<Page<Document>>;
    pub async fn get_document(&self, id: Uuid, organization_id: Uuid) -> Result<Document>;
    pub async fn update_document(&self, id: Uuid, organization_id: Uuid,
        input: UpdateDocumentInput) -> Result<Document>;
    pub async fn delete_document(&self, id: Uuid, organization_id: Uuid) -> Result<()>;
    // optional: pub async fn restore_document(...)
}
```
`update_document`: `input.validate()` → `update_checked(id, org, input.expected_version, input.title.as_deref(), input.content.as_deref())`; on `None`, call `find_by_id_for_org(id, org)` → if `Some(current)`, return `Error::DocumentVersionConflict { expected: input.expected_version, current: current.version }`; if `None`, return `Error::DocumentNotFound(id.to_string())`. Use `tracing::info!/debug!` only (never `println!` — `print_stdout` is warned/denied `[F10]`).

### 3.6 Errors — `flora-core/src/error.rs` (three edit sites)
Add after `FileNotFound(String)` (line 57):
```rust
    /// The requested document does not exist.
    #[error("document not found: {0}")]
    DocumentNotFound(String),

    /// The document changed after the client read it (optimistic-concurrency conflict).
    #[error("version conflict: expected {expected}, current {current}")]
    DocumentVersionConflict { expected: i32, current: i32 },
```
1. `is_not_found()` (lines 221–233): add `| Self::DocumentNotFound(_)`.
2. `is_client_error()` (lines 200–215): add `| Self::DocumentVersionConflict { .. }` (409 = client error). **Do not** add `DocumentNotFound` here — all existing 404 variants are deliberately absent; keep parity.
3. `IntoResponse` (exhaustive `match &self`, lines 290–371): add `| Self::DocumentNotFound(_)` to the 404 arm (311–323), and a dedicated 409 arm:
```rust
Self::DocumentVersionConflict { expected, current } => (
    axum::http::StatusCode::CONFLICT,
    serde_json::json!({ "error": "version_conflict", "message": self.to_string(),
                        "expected": expected, "current": current }),
),
```
(Adding to the existing `DuplicateEntry | InvalidInput` arm instead is legal but loses `current`.)

### 3.7 Router — `flora-api/src/routes/documents.rs` (new)
```rust
pub struct CreateDocumentRequest { pub workspace_id: Uuid, pub channel_id: Option<Uuid>,
                                   pub title: String, pub content: Option<String> }
pub struct UpdateDocumentRequest { pub expected_version: i32, pub title: Option<String>,
                                   pub content: Option<String> }
#[derive(Debug, Deserialize)]
pub struct DocumentListQuery { pub workspace_id: Uuid, pub channel_id: Option<Uuid>,
                               pub limit: Option<u32>, pub offset: Option<u32> }
#[derive(Debug, Serialize)]
pub struct DocumentResponse { /* all IDs String, timestamps to_rfc3339(), version i32,
                                  channel_id Option<String>, is_deleted bool */ }
impl From<Document> for DocumentResponse { ... }

pub fn create_documents_router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_documents).post(create_document))
        .route("/{id}", get(get_document).patch(update_document).delete(delete_document))
        .route("/{id}/restore", post(restore_document)) // optional P0.5
}
```
Handlers mirror `tasks.rs`: `State(state): State<AppState>`, `auth: AuthExtractor`, `auth.require_org().map_err(|(_s,m)| Error::Validation{field:"organization_id"..., message:m})?`, membership check via `PgMembershipRepository::find_by_user_and_organization(...).ok_or_else(|| Error::Forbidden("Not a member".to_owned()))?`, then construct `PgDocumentRepository`/`DocumentService` from `(*state.db_pool).clone()`. `POST` → `201`, PATCH → `200` or `409`, DELETE → `204`.

### 3.8 Router registration — `flora-api/src/routes/mod.rs`
Add `pub mod documents;` (alphabetical: after `channels`, before `files`) and `.nest("/api/v1/documents", documents::create_documents_router())` beside tasks/files. **Canonical file differs from local dev** — apply the two-line edit to the working-tree file, preserving `/metrics`.

### 3.9 Search branch — `flora-api/src/routes/search.rs`
Add after `include_files` (line 130) and a new branch before the final sort (line 282):
```rust
let include_documents = include_type(types, &["document", "documents"]);
if include_documents {
    let rows = sqlx::query_as::<_, PgSearchRow>(
        r"
        WITH q AS (
            SELECT COALESCE(
                websearch_to_tsquery('simple', $2),
                plainto_tsquery('simple', $2)
            ) AS tsq
        )
        SELECT d.id,
               left(regexp_replace(
                   coalesce(d.title, '') || ' ' || coalesce(d.content, ''),
                   '\s+', ' ', 'g'), 200) AS snippet,
               ts_rank_cd(d.search_tsv, q.tsq)::real AS rank,
               d.created_at
        FROM documents d, q
        WHERE d.organization_id = $1
          AND d.is_deleted = false
          AND q.tsq IS NOT NULL
          AND q.tsq <> ''::tsquery
          AND d.search_tsv @@ q.tsq
        ORDER BY rank DESC, d.created_at DESC, d.id DESC
        LIMIT $3
        ",
    )
    .bind(org_id).bind(query).bind(limit_i64)
    .fetch_all(state.db_pool.as_ref()).await?;
    for row in rows {
        merged.push((SearchResult {
            result_type: "document".to_string(),
            id: row.id.to_string(),
            snippet: row.snippet.unwrap_or_default(),
        }, row.rank, row.created_at, row.id));
    }
}
```
Update four doc comments: line 1 (`messages, tasks, files, documents`), line 24 (`type filter … messages, tasks, files, documents`), line 34 (`message, task, file, document`), line 60 (`Search messages, tasks, files, and documents`). Keep `#[expect(clippy::too_many_lines, …)]`, `source: "postgres"`, `limit.min(50)`, merge/sort/truncate untouched. Do **not** use `ts_headline` (PG18 XSS warning) `[P2]`.

---

## 4. Compile traps (and how to avoid them)

| # | Trap | Site | Fix |
|---|---|---|---|
| **T1** | Adding an `AppState` field breaks **7** struct-literal test helpers (033 only knew one) | `flora-api/src/extractors/auth.rs:275`; `flora-tests/src/contract/{auth:64,files:96,messaging:66,notifications:69,search:63,websocket:65}.rs` `[F1][F2]` | **Recommended: add no field.** Build `PgDocumentRepository`/`DocumentService` from `state.db_pool` in the router — exactly how `tasks.rs`/`search.rs` work today `[F5][F7]`. If a `document_service` field is mandated, all 7 literals must be updated in the same commit. |
| **T2** | `IntoResponse` is an **exhaustive** `match &self`; new variants without arms = hard compile error. `is_client_error`/`is_not_found` are `matches!` → no compile error, **silent** 404/409 misclassification | `error.rs:200,220,290` | Edit all three sites (§3.6). Add `DocumentNotFound` to 404 arm + `is_not_found`; add `DocumentVersionConflict` to a 409 arm + `is_client_error`. |
| **T3** | Generated column must be IMMUTABLE; a 1-arg `to_tsvector(...)` or a volatile expression fails at migration time | migration | Use 2-arg `to_tsvector('simple', …)` with `replace`/`coalesce`/`||` only `[P1]`. |
| **T4** | Punctuation-only query can yield an empty `tsquery` and a 500 if the guards are dropped | `search.rs` branch | Copy `q.tsq IS NOT NULL AND q.tsq <> ''::tsquery` verbatim. |
| **T5** | Cross-org get/patch/delete via `find_by_id` + handler compare replicates the tasks 403-after-fetch pattern and leaks existence | new router | Use `find_by_id_for_org` (tenant predicate in SQL) → `DocumentNotFound` (404). |
| **T6** | Cross-tenant **create**: tasks ignores whether `req.workspace_id` belongs to `org_id` | `tasks.rs:169-192` `[F7]` | Add `assert_scope()` in documents create (§5). |
| **T7** | TS `interface Document` shadows the DOM global when imported into `.svelte` files | FE | Name it `DocumentModel`. |
| **T8** | `.svelte.spec.ts` lands in the **client** (browser/Playwright) Vitest project; CI runs `test:unit -- --run --project server` only `[F11][F14]` | FE tests | Put editor logic in a plain `.ts` helper and test it with `*.spec.ts` in the server project; `requireAssertions: true` — every test needs an assertion. |
| **T9** | Rewriting canonical `routes/mod.rs` from local `dev` would delete other-agent `/metrics` work | `routes/mod.rs` | Two-line surgical edit only; never checkout/overwrite. |
| **T10** | `#[expect(clippy::too_many_lines)]` on `search_pg` already exists; adding a branch is fine, but a duplicated `result_type` or forgotten merge entry silently drops results | `search.rs:115-289` | Push every row into `merged` with `result_type: "document"`; leave sort/truncate intact. |

---

## 5. Authorization model (org + workspace; channel is a filter, not an ACL)

1. `let org_id = auth.require_org()?` — org comes from the **verified server-side session**, not `x-organization-id` `[F1]`.
2. Membership: `PgMembershipRepository::find_by_user_and_organization(auth.user_id, org_id)` → `403` when `None` (every handler).
3. **Create scope assertion (the tasks gap, T6):**
```rust
async fn assert_scope(state: &AppState, org_id: Uuid, workspace_id: Uuid,
                      channel_id: Option<Uuid>) -> Result<()> {
    let ws = PgWorkspaceRepository::new((*state.db_pool).clone())
        .find_by_id(workspace_id).await?
        .ok_or_else(|| Error::WorkspaceNotFound(workspace_id.to_string()))?;
    if ws.organization_id != org_id {
        return Err(Error::Forbidden("cross-organization workspace".to_owned()));
    }
    if let Some(channel_id) = channel_id {
        let ch = PgChannelRepository::new((*state.db_pool).clone())
            .find_by_id(channel_id).await?
            .ok_or_else(|| Error::ChannelNotFound(channel_id.to_string()))?;
        if ch.organization_id != org_id || ch.workspace_id != workspace_id {
            return Err(Error::Forbidden("cross-scope channel".to_owned()));
        }
    }
    Ok(())
}
```
4. Get/patch/delete: `find_by_id_for_org(id, org_id)` → 404 if absent; tenant predicate lives in SQL, not only in the handler.
5. **Channel is a filter, not an ACL.** There is no channel-membership table in the Seed schema `[F4]`, so `Private` channel notes stay visible to any org member. There is also no workspace/channel filter on `/api/v1/search` `[F5]`. Say so in release notes (R4); do not market channel privacy.

---

## 6. Frontend touch points (Svelte 5, runes, zero new deps)

| File | Change |
|---|---|
| `src/lib/types/models.ts` | Add `export interface DocumentModel { id; organizationId; workspaceId; channelId: UUID \| null; title; content; version: number; createdAt; updatedAt }` (T7). |
| `src/lib/api/mappers.ts` | Add `ApiDocument` (snake_case) + `toDocument(api): DocumentModel` mirroring `toMessage` `[F11]`. |
| `src/lib/api/client.ts` | Add `documents` methods over `ApiClient.request`: `list({workspaceId, channelId?, limit?, offset?})`, `create`, `get`, `patch({expectedVersion, title?, content?})`, `delete`, `restore`. 409 surfaces as `ApiError.status === 409` with `code === "version_conflict"` — no transport change needed `[F11]`. |
| `src/lib/state/DocumentStore.svelte.ts` (new) | `export class DocumentStore { documents = $state<DocumentModel[]>([]) … }` + idempotent upsert/remove, mirroring `TaskStore` `[F11]`. |
| `src/lib/components/documents/editorState.ts` (+ `.spec.ts`) (new) | Pure helpers: `isDirty(draft, saved)`, `nextStatusOnSave/OnConflict`, `debounce(fn, ms)`. Server-project unit-testable (T8). |
| `src/lib/components/documents/DocumentEditor.svelte` (new) | `let { document, onSave } = $props(); let draft = $state(document.content); let version = $state(document.version); let status = $state<'saved'\|'dirty'\|'saving'\|'conflict'>('saved'); let dirty = $derived(draft !== document.content);` `<textarea bind:value={draft} oninput=… onkeydown=…>` + local 800–1500 ms debounce + Ctrl/Cmd+S immediate save. On 200: `version = res.version; status='saved'`. On 409: keep draft, `status='conflict'`, show "This note changed elsewhere — reload to see the newer version" with explicit **Reload** button; **never** auto-overwrite `[S1][S3]`. `aria-live="polite"`, `<label>`, visible focus. Preview = `<pre>{draft}</pre>` (Svelte escapes text; no `{@html}`) `[P4]`. |
| `src/routes/(app)/documents/+page.svelte` (new) | List + "New note" (POST with workspace from `authState`/org context). |
| `src/routes/(app)/documents/[id]/+page.svelte` (new) | `page.params.id` → `get()` → host editor. |
| `src/routes/(app)/+layout.svelte` | Import `FileText` from `lucide-svelte`; add sidebar `Button href="/documents"` beside Tasks (lines 139–146). |
| `src/lib/utils/search.ts` | Add `document: 'Document'` to `RESULT_LABELS`. |
| `src/lib/components/search/SearchBox.svelte` (optional) | Update `aria-label`/placeholder to include documents; no `type` change needed (omitted type includes all branches). |

**Explicitly not in the UI:** formatting toolbar, WYSIWYG/`contenteditable`, drag-and-drop images, collaborative cursors, undo history beyond native textarea. `{@html}` and raw Markdown rendering stay out (Svelte docs: "Never render unsanitized content") `[P4]`.

---

## 7. Tests and CI wiring

**Backend contract** — new `flora-tests/src/contract/documents.rs` + `pub mod documents;` in `contract/mod.rs`. Follow `contract/search.rs` exactly: `#[cfg(test)] mod db_documents { … }`, local `make_test_state(pool, config)` (if Option B was chosen, update it too), `db_state()` gated on `FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL` returning `Ok(None)` when unset, seeded user/org/role/membership/workspace + JWT, `tower::ServiceExt::oneshot`, tests prefixed `db_` with `#[ignore = "requires Postgres; set FLORA_DATABASE__POSTGRES_URL (cycle 035)"]` `[F2][F13]`.

Minimum cases: (1) create→get→patch→list→soft-delete; deleted rows absent from list/search; (2) two PATCHes with same `expected_version` → exactly one 200 (version +1) and one 409 with `current`; (3) org A cannot get/patch/delete org B's doc (404); (4) create with org B's workspace → 403 (T6); (5) channel from another workspace → 403; (6) `/search?q=…&type=documents` → only own org, non-deleted, `source: "postgres"`; (7) punctuation-only query → 200 empty, no 500.

**Mandatory CI edit** — add to `.github/workflows/rust.yml` `db-tests` job (after the files step, line 83):
```yaml
      - name: Run DB-backed documents contract tests
        run: cargo test -p flora-tests contract::documents::db_ -- --ignored --test-threads=1
```
Without this step the suite is silently skipped in CI `[F13]`.

**Unit (no DB):** `Document::new` defaults, validator bounds (title 1–255, content ≤200k), `Error`→status mapping for the two new variants, and FE `editorState` helpers. `requireAssertions: true` in Vitest `[F14]`.

**Quality gates:** `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test`; `cargo deny check bans licenses sources`; DB suites as above; FE `bun install && bun run check && bun run lint && bun run test:unit -- --run --project server && bun run build` `[F10][F11][F13]`.

---

## 8. Risks / NVs

- **R1 — CJK recall (NV, high).** `simple` lowercases and does no stemming/segmentation `[P3]`; PG has no built-in CJK config. Chinese/Japanese notes become one lexeme, so sub-token queries miss. Mitigation P0: keep `simple` for cross-entity parity and document degraded CJK search. P1 options need extra extensions (`zhparser`/SCWS; `pg_trgm` bundled but ≥3 chars and `LC_CTYPE != C`; `pg_bigm`/PGroonga/MeCab) — VPS cost. **NV:** no local CJK benchmark, no extension installed.
- **R2 — index/query normalization mismatch.** Index replaces `-`/`.`/`_` with spaces; `websearch_to_tsquery` treats `-` as NOT and keeps `.`/`_` in tokens `[P2][F3][F5]`. Pre-existing across all entities; keep parity, harden separately.
- **R3 — cross-tenant scope attachment.** Fixed by `assert_scope()` (§5); this is the main correctness delta vs tasks.
- **R4 — channel ≠ ACL.** No membership table; `Private` docs remain org-visible; search has no workspace/channel filter.
- **R5 — GIN cost on 4 GB.** Stored `tsvector` + GIN add disk/write cost; cap content at 200k in `validator`, keep search `LIMIT ≤ 50` `[P1]`. **NV:** no load/OOM measurement.
- **R6 — "versioned" ≠ history.** CAS token only; `document_revisions` deferred (R8 in 033).
- **R7 — user deletion cascades notes** (`creator_id … ON DELETE CASCADE`, matching tasks). Consider `SET NULL` in a hardening pass. **NV:** no offboarding policy.
- **R8 — Svelte XSS.** `{@html}` not sanitizing; `contenteditable` bindings had advisory GHSA-phwv-c562-gvmh patched in 5.53.5; FE pins `^5.56.1` `[P4][P5]`. P0 textarea + `<pre>` avoids both.
- **R9 — canonical/local drift.** Preserve `/metrics`; rebase, don't revert (T9).
- **R10 — license/deps.** AGPL clean-room; no new Cargo/npm dependency; cargo-vet/dependency-review remain informational and authorization-gated `[F10][F13]`.
- **NV — runtime truth:** no DB-backed test execution, no live API smoke, no browser e2e, no compile of the proposed files, no perf measurement in this research run.

---

## 9. Effort / size / go-no-go

| Dimension | Honest estimate |
|---|---|
| Files | **26 required** (15 backend/test/CI + 11 frontend) + 2 optional (SearchBox copy, OBJETIVO §5) + 2 governance (ROADMAP cycle line, cycle-034 assessment note) = **30** |
| LOC | ~700–900 Rust (migration/model/trait/repo/service/errors/router/search) + ~300–450 Rust contract tests + ~400–600 TS/Svelte |
| New dependencies | **0** Cargo, **0** npm |
| CI rounds | 2–3 (fmt/clippy first, then FE check/lint/build, then DB-gated suites) |
| Single-cycle fit | **GO** if scope is locked to §2–§7 and Option A (no AppState field) is used |

**Go conditions (all mandatory):** no new deps; no AppState field; `assert_scope()` in create; all three error sites edited; CI contract step added; canonical/local drift preserved; deferred list locked.

**Split / no-go triggers (→ cycle 036):** revision history or restore UI; per-channel ACL or channel-membership table; CJK config/extension; WYSIWYG or attachments; Meilisearch indexing; any new mandatory dependency. Any one of these doubles the surface and breaks the one-cycle budget.

**Acceptance criteria:** fresh DB migration yields `documents` with generated `search_tsv` + partial GIN; CRUD + CAS conflict (one 200 / one 409) end-to-end; cross-org create/get/patch/delete blocked; `type=documents` search filtered by org and `is_deleted`, `source: "postgres"`, punctuation-only safe; FE dirty/conflict flow works with no `{@html}`; all quality gates green.

---

## 10. Multilingual cross-check — FTS tokenization limits (`simple` vs CJK)

| Lang | Finding used | Evidence | NV label |
|---|---|---|---|
| **EN (primary)** | PG18: stored generated `tsvector` + GIN is the recommended pattern; 2-arg `to_tsvector` required for index stability; `websearch_to_tsquery` "will never raise syntax errors", `-`→NOT, other punctuation ignored; `ts_rank_cd` can be I/O-bound; `ts_headline` output is **not** XSS-safe; simple dictionary only lowercases (no stemming) | `[P1][P2][P3][P6]` | **NV-none** for doc facts (fetched 2026-11-16); no local benchmark. |
| **pt-BR** | `portuguese` config stems; Flora's `simple` does not; generated column + GIN is the locally recommended path | `[M-pt1][M-pt2]` | **NV-secondary** (community/consulting, not first-party PG translation). |
| **es** | Same consensus; prefer `websearch_to_tsquery` over `to_tsquery` for user input; GIN read-fast/write-costly | `[M-es1][M-es2]` | **NV-secondary**. |
| **de** | `german` stemming removes stop words; mixing document/query configs fails silently → keep `'simple'` explicit everywhere | `[M-de]` | **NV-secondary**. |
| **fr** | `french` config + `ts_rank_cd`; for mixed-language content choose `simple` or multiple vectors — matches Flora's choice | `[M-fr]` | **NV-secondary**. |
| **zh-CN** | First-party cloud vendor: "PostgreSQL's default text parser splits input on spaces and punctuation… Chinese text has no spaces between words, so the default parser treats an entire sentence as a single token and cannot match individual words"; `zhparser` (SCWS) + custom config required | `[M-zh1][M-zh2]` | **NV-vendor** (first-party vendor doc, not local); extension not installed, no local recall test. |
| **ja** | `to_tsvector('simple','東京タワーの夜景を撮影しました')` → one lexeme `'東京タワーの夜景を撮影しました':1`; PG has no Japanese dictionary; `pg_trgm` needs ≥3 chars and `LC_CTYPE != C`; `pg_bigm` (2-gram), PGroonga/MeCab are extra extensions | `[M-ja1][M-ja2][M-ja3][M-ja4]` | **NV-vendor/engineering**; **NV-conflict**: older SRA OSS says pg_trgm cannot handle multibyte, PG18 docs + RJC say it works under `C.UTF-8` — treat the "≥3 chars + locale" constraint as the safe claim. |

**Conclusion:** the CJK single-token fact is corroborated by both EN primary docs (no CJK config) and first-party zh/ja vendor docs. It does **not** change the P0 design (keep `simple` for parity); it changes the documented expectation and the P1 extension decision.

---

## 11. Sources

### Primary — external
- **[P1]** PostgreSQL 18, *12.2 Tables and Indexes* (stored generated column + `coalesce` + GIN; 2-arg `to_tsvector` immutability/restore): https://www.postgresql.org/docs/18/textsearch-tables.html (fetched 2026-11-16)
- **[P2]** PostgreSQL 18, *12.3 Controlling Text Search* (`websearch_to_tsquery` never raises syntax errors; `-`→NOT; `ts_rank_cd`; `ts_headline` XSS warning): https://www.postgresql.org/docs/18/textsearch-controls.html (fetched 2026-11-16)
- **[P3]** PostgreSQL 18, *12.6 Dictionaries* (simple dictionary lowercases only; no CJK dictionary): https://www.postgresql.org/docs/18/textsearch-dictionaries.html (fetched 2026-11-16)
- **[P4]** Svelte docs, `{@html}` ("Never render unsanitized content"): https://svelte.dev/docs/svelte/@html (fetched 2026-11-16)
- **[P5]** Svelte docs, *What are runes?* (`$state`, `$derived`, `$props`, `$effect` are language keywords, no import): https://svelte.dev/docs/svelte/what-are-runes (fetched 2026-11-16)
- **[P6]** PostgreSQL 18, *F.35 pg_trgm* (3 consecutive chars, ignores non-word chars, trusted): https://www.postgresql.org/docs/18/pgtrgm.html (via search 2026-11-16)
- **[P7]** GitHub Advisory `GHSA-phwv-c562-gvmh`, Svelte `contenteditable` SSR XSS, patched 5.53.5: https://github.com/sveltejs/svelte/security/advisories/GHSA-phwv-c562-gvmh

### Secondary — external
- **[S1]** Optimistic concurrency / lost updates, version-in-body → 409: https://www.lochana.co/blog/optimistic-concurrency-lost-updates
- **[S2]** Optimistic concurrency for HTTP APIs (CAS, 409 vs 412/428, integer over timestamp): https://matheuspalma.com/blog/optimistic-concurrency-http-apis-etags-if-match-and-conflict-design
- **[S3]** Prevent lost updates with optimistic locking in SQL (increment in same `UPDATE`, zero rows = conflict): https://nalar.dev/prevent-lost-updates-with-optimistic-locking-in-sql

### Multilingual
- **[M-pt1]** Runstack (pt-BR): https://runstack.com.br/postgres-full-text-search
- **[M-pt2]** Urhoba (pt-BR): https://www.urhoba.net/pt/artigo/busca-de-texto-completo
- **[M-es1]** wrujel (es): https://blog.wrujel.com/es/postgres-full-text-search-without-elasticsearch-d9269e
- **[M-es2]** Kaleidos (es): https://www.kaleidos.net/blog/como-usar-busqueda-de-texto-en-postgresql
- **[M-de]** synyx (de): https://synyx.org/blog/volltextsuche-mit-postgresql-und-spring-teil-i
- **[M-fr]** StackHarbor (fr): https://stackharbor.com/fr/base-de-connaissances/pg-full-text-search-tsvector
- **[M-zh1]** Alibaba Cloud AnalyticDB (zh-CN/en, "entire sentence as a single token", zhparser): https://help.aliyun.com/en/analyticdb/analyticdb-for-postgresql/user-guide/use-zhparser-to-perform-chinese-word-segmentation (updated 2026-01-20)
- **[M-zh2]** Alibaba Cloud developer community (zh-CN, default parser splits on spaces, zhparser/SCWS): https://developer.aliyun.com/article/1515103
- **[M-ja1]** RJC (ja, `tsvector(simple)` single-token example; `pg_trgm` ≥3 chars + non-`C` locale; `pg_bigm`): https://www.r-jc.jp/news-blog/1724/ (2026-10-01)
- **[M-ja2]** NEC (ja, `pg_bigm` for 2-char Japanese): https://jpn.nec.com/postgresql/technical_info/pg_bigm_v2.html
- **[M-ja3]** Zenn / ParadeDB (ja, no Japanese dictionary; tsvector unusable): https://zenn.dev/okamyuji/articles/paradedb-japanese-fulltext-search (2026-02-07)
- **[M-ja4]** SRA OSS (ja, pg_bigm vs pg_trgm): https://www.sraoss.co.jp/tech-blog/pgsql/pg_bigm

### Repo (canonical `1be723cca`, verified this run)
- **[F1]** `flora-api/src/extractors/auth.rs` (require_org 218–226; test literal 275–355)
- **[F2]** `flora-core/src/error.rs`; `flora-tests/src/contract/*.rs` (6 `make_test_state` literals)
- **[F3]** `flora-core/src/migrations/sql/{20240818000001_init_schema,20261116000004_search_fts_separator_normalization,20261117000001_add_messages_search_fts,20261118000001_add_tasks_files_search_fts}.sql`; `flora-core/src/migrations/mod.rs`
- **[F4]** `flora-core/src/models/task.rs`; init schema (tenant/RLS/index conventions)
- **[F5]** `flora-api/src/routes/search.rs` (all 293 lines)
- **[F6]** `flora-core/src/models/{task.rs,mod.rs}`; `flora-core/src/traits/repository.rs`
- **[F7]** `flora-api/src/routes/tasks.rs` (1–364)
- **[F8]** `flora-core/src/repositories/{task_repository.rs,mod.rs}`
- **[F9]** `flora-core/src/services/{task_service.rs,mod.rs,draft_service.rs}`
- **[F10]** `flora-core/src/error.rs`; `flora-core/src/lib.rs`; `AGENTS.md`; `.github/workflows/rust.yml`
- **[F11]** `apps/frontend/src/{lib/types/models.ts,lib/api/{client,mappers}.ts,lib/state/TaskStore.svelte.ts,lib/components/search/SearchBox.svelte,routes/(app)/+layout.svelte,package.json}`
- **[F12]** `flora-core/src/services/draft_service.rs`
- **[F13]** `.github/workflows/rust.yml` (6 db-gated suites); `ROADMAP.md:55`; `.specify/assessments/032-next-vertical-slice/research.md`
- **[F14]** `apps/frontend/{vite.config.ts,tsconfig.json,svelte.config.js}`; `src/lib/utils/search.ts`; `src/lib/vitest-examples/*`
