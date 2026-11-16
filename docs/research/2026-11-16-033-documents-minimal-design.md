# 033 — FASE 7 Documents: minimal design for a cheap-VPS Markdown notes feature (multilingual research)

**Date:** 2026-11-16
**Scope:** Product/technical direction for Flora cycle 033/034, FASE 7 "Documents" (`.specify/memory/constitution.md:148`, `README.md:45`, `ROADMAP.md:54`). Research-only: **no application files were modified**; the only artifact is this note.
**Question:** What is the smallest reliable versioned-Markdown-notes feature that fits the existing Rust/Postgres/Svelte-5 Seed and a 2 vCPU / 4 GB VPS, and what must be deferred?
**Method:** (1) repo-grounded audit of the existing task/message/search/frontend patterns; (2) primary-source verification (PostgreSQL 18 docs, Svelte 5 docs, CRDT library docs); (3) multilingual retrieval in EN, pt-BR, es, de, fr, zh-CN, ja. Claims carry source tags `[P…]` (primary external), `[S…]` (secondary external), `[F…]` (repo file), `[M…]` (multilingual). Items without direct verification are marked **NV**. No runtime benchmark, no DB load test, and no live API contract test were executed in this run.

---

## 0. Executive summary (the 80/20 cut)

**Build:** one `documents` table with a denormalized `organization_id`, workspace scope, optional channel scope, plain Markdown `content`, an integer `version` used as an optimistic-concurrency token, soft-delete, timestamps, and a **stored generated `tsvector` + partial GIN index copied verbatim from the existing FTS pattern**. Add a thin CRUD router under `/api/v1/documents`, one `DocumentRepository` trait + Pg implementation, one `DocumentService`, wire into `AppState`, add a `documents` branch to the existing `/api/v1/search`, and ship a **`<textarea>` + debounced PATCH** Svelte 5 editor (runes) with a 409 "changed elsewhere" banner. Zero new Rust crates; zero new frontend dependencies for P0.

**Defer:** CRDT/OT co-editing, presence/cursors, offline merge, WYSIWYG/`contenteditable`, attachments, full revision history/restore UI, import/export, @-links/backlinks, per-channel ACL, public sharing, and Meilisearch indexing.

**Why this is the line:** cycles 019–023 already established Postgres FTS as authoritative and Meilisearch as dormant `[F2][F5]`; the frontend already has all transport/store primitives `[F11]`; and Loro's own docs say to consider alternatives when "simple client–server sync is sufficient" and when bundle size matters — its WASM binary is ~970 KB gzipped `[P5]`. A version counter + HTTP 409 gives honest concurrent-edit behavior without a sync engine `[S1][S2]`.

**Cycle 033 context:** the 032 assessment already ranked "FASE 7 Documents" as the next product candidate (P1, greenfield) and noted the "FE dead search box" as a cheap adjacent gap `[F13]`. Wiring the dead search box to the extended `type=documents` search is a natural side-effect of this slice.

---

## 1. Truth anchors in the repo (verified)

| Area | Existing pattern (evidence) | Consequence for Documents |
|---|---|---|
| Multi-tenancy | Every tenant entity carries `organization_id`; comments say "every query MUST include organization_id" `[F1:2,143-144]`. Handlers take `org_id` from `AuthExtractor::require_org()` (session-authoritative, not a request header) and reject `row.organization_id != org_id` with 403 `[F4][F7]`. | Reuse exactly. Prefer pushing `organization_id` into SQL `WHERE`, not only the handler. |
| Scope | `tasks` are `workspace_id + organization_id`; `channels` add `type ∈ {Public, Private}` and carry `organization_id`; `messages` hang off `channel_id` `[F1:52-96]`. A later migration added `workspace_id` to `files` `[F2]`. | `workspace_id NOT NULL` + `channel_id NULL` is the natural doc scope. |
| Soft delete | `is_deleted BOOLEAN DEFAULT FALSE` + partial index `WHERE is_deleted = false` on messages/tasks/files; repositories expose `soft_delete`, `restore`, `purge_old` and a 90-day retention policy `[F1:162-180][F3:176-216][F3:11-33]`. | Copy; retention/purge is P1 (reuse `TaskRetentionPolicy` shape). |
| FTS | Stored generated `tsvector` with `to_tsvector('simple', …)` + `coalesce`, GIN partial index `WHERE is_deleted = false`, and a separator-normalization migration using `replace(replace(replace(x,'-',' '),'.',' '),'_',' ')` `[F2]`. Postgres docs endorse exactly this (generated column + GIN) `[P1][P3]`. | Copy the normalized expression byte-for-byte. |
| Search endpoint | `/api/v1/search?q=&type=&limit=` merges per-entity branches with `websearch_to_tsquery('simple',$q)` + `plainto_tsquery` fallback, `ts_rank_cd`, `search_tsv @@ q.tsq`, `organization_id = $1`, `is_deleted = false`, `q.tsq IS NOT NULL AND q.tsq <> ''::tsquery`, `ORDER BY rank DESC, created_at DESC, id DESC`, snippet `left(regexp_replace(content,'\s+',' ','g'),200)`, response `source: "postgres"` `[F5:60-293]`. | Add a fourth branch; do not introduce a new search service. |
| CRUD layering | Model (`flora-core/src/models/…`) → trait (`traits/repository.rs`) → Pg repo (`repositories/…`) → service (`services/…`) → route (`flora-api/src/routes/…`) → router (`routes/mod.rs`) `[F1][F3][F4][F6][F9][F14]`. | Keep crate-local; no new crate. |
| Errors | `Error` enum with `DocumentNotFound`-style 404 variants and 409 via `DuplicateEntry`/`InvalidInput`; `IntoResponse` is an exhaustive match; `is_client_error`/`is_not_found` are explicit matches `[F10]`. | Add dedicated 404 + 409 variants (touch all three sites). |
| State wiring | `AppState::new` builds every repo/service explicitly; `extractors/auth.rs` tests construct `AppState` with a struct literal `[F8][F7:272-355]`. | **Adding an `AppState` field breaks that test helper** — exact touch point below. |
| Frontend | Svelte 5 runes + `.svelte.ts` class stores, `ApiClient` with Bearer + 401→`lockSession`, `mappers.ts` for snake_case→camelCase, `(app)` route group with sidebar, no markdown/editor dependency in `package.json` `[F11]`. | P0 editor = `bind:value` textarea + `<pre>` view; no new deps. |
| Governance | Constitution Phase 7 = Documents; FASE 8 = Email; no parity/Macro surface as Seed requirement; no mandatory proprietary/heavy deps `[F13]`. | Confirms narrow scope and AGPL-compatible clean-room approach. |

**Migration-ordering note (not a bug):** `20261116000004_search_fts_separator_normalization.sql` predates `20261117000001_add_messages_search_fts.sql` and `20261118000001_add_tasks_files_search_fts.sql` by filename, but was committed later `[F2]`. It works because the normalization migration uses `DROP … IF EXISTS` + `ADD COLUMN`, and the later ones use `ADD COLUMN IF NOT EXISTS` + `CREATE INDEX IF NOT EXISTS` (no-ops on a fresh DB). **Convention to follow:** new migrations use a timestamp strictly greater than `20261118000001` and guarded DDL.

---

## 2. Minimal data model (schema sketch)

```sql
-- New file: flora-core/src/migrations/sql/20261119000001_documents.sql
-- (timestamp > all existing migrations; guarded DDL; matches [F2] convention)

CREATE TABLE IF NOT EXISTS documents (
    id              UUID PRIMARY KEY DEFAULT uuidv7(),
    organization_id UUID NOT NULL,                         -- denormalized tenant key, per [F1]
    workspace_id    UUID NOT NULL REFERENCES workspaces(id) ON DELETE CASCADE,
    channel_id      UUID REFERENCES channels(id) ON DELETE SET NULL, -- NULL => workspace-scoped
    creator_id      UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title           VARCHAR(255) NOT NULL,
    content         TEXT NOT NULL DEFAULT '',              -- Markdown source, bounded in app (see R5)
    version         INTEGER NOT NULL DEFAULT 1,            -- CAS token, NOT a history log
    is_deleted      BOOLEAN NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Verbatim normalized expression from the 023 migration [F2]; 'simple' is immutable-safe [P1]
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
-- "recently edited first" is the useful default for notes (vs created_at for tasks)
CREATE INDEX IF NOT EXISTS idx_documents_org_updated    ON documents(organization_id, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_documents_search_tsv     ON documents USING GIN (search_tsv) WHERE is_deleted = FALSE;

ALTER TABLE documents ENABLE ROW LEVEL SECURITY;  -- defense-in-depth only; no policies, per [F1:196-215]
```

**Field-by-field rationale**

| Field | Decision | Rationale / source |
|---|---|---|
| `id` | `uuidv7()` default | Every existing table uses it; gives time-ordered IDs for cursor-like sorting `[F1]`. |
| `organization_id` | denormalized, no FK | Matches messages/tasks/files; isolates queries without a join `[F1]`. |
| `workspace_id` | `NOT NULL`, FK cascade | Matches tasks; deleting a workspace removes its notes `[F1]`. |
| `channel_id` | **nullable**, FK `ON DELETE SET NULL` | Optional scope; a note outlives its channel and degrades to workspace scope. App must verify the channel belongs to `organization_id` and `workspace_id` (SQL `CHECK` cannot express this) — see R3. |
| `creator_id` | FK cascade | Matches tasks/messages `[F3]`. Flag R7 for user-deletion data loss. |
| `title` | 1–255 via `validator` | Matches `CreateTaskInput` `[F3:60-72]`. |
| `content` | `TEXT`, app cap ~200,000 chars | Markdown source of truth; cap bounds tsvector/GIN cost on 4 GB (R5). |
| `version` | `INTEGER DEFAULT 1`, incremented in the same `UPDATE` | Compare-and-swap prevents silent lost updates; integer avoids timestamp/JS-ms false conflicts `[S1][S2][S3]`. |
| `is_deleted`, timestamps | exact task pattern | Enables partial FTS index and 90-day purge parity `[F2][F3]`. |
| `search_tsv` | generated, stored, normalized | Official recommendation; derived data only, so index/column rebuild loses no source data `[P1][P2][F2]`. |

**Not in P0:** a `document_revisions` table. If history is demanded later, the cheap append-only shape is `(id, document_id, version, title, content, created_by, created_at)` written on each successful save; keep it out now because it roughly doubles storage and adds a retention/ACL surface (R8). The `version` integer is **only** a concurrency token — say so explicitly in docs to avoid a false "version history" claim.

---

## 3. FTS integration (reuse, don't reinvent)

**Migration:** the generated column above is the whole integration — no trigger, no outbox, no Meilisearch hook. A generated column is recomputed on `INSERT`/`UPDATE` automatically and survives restore `[P1][P3]`.

**Search branch** in `flora-api/src/routes/search.rs` (mirror the `tasks` branch at `[F5:178-228]`):

```rust
// inside search_pg(), after include_files:
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
               left(regexp_replace(d.content, '\s+', ' ', 'g'), 200) AS snippet,
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
    // push SearchResult { result_type: "document", id, snippet } into `merged`
}
```

Keep `source: "postgres"`, the `limit.min(50)` cap, and the deterministic merge/sort/truncate at `[F5:282-288]`. Update the `type` doc comment to `messages, tasks, files, documents`.

**Why `websearch_to_tsquery`:** it "will never raise syntax errors, which makes it possible to use raw user-supplied input for search" `[P2]`; `to_tsquery` would throw on ordinary user text. `ts_rank_cd` rewards proximity but needs positional data and can be I/O-bound; keep the existing `LIMIT` discipline `[P2]`. Do **not** use `ts_headline`: the Postgres docs carry an explicit XSS warning for its output, and Flora already uses a plain-text `regexp_replace` snippet `[F5][P2]`.

**Known semantic seam (R2):** the index normalizes `-`, `.`, `_` to spaces, but `websearch_to_tsquery('simple', $q)` treats `-` as NOT and keeps `.`/`_` inside tokens. This mismatch already exists for messages/tasks/files `[F2][F5]`. Keep parity in P0; a query-side normalization change should be a separate hardening cycle because it alters `-` semantics for all entities.

---

## 4. CRUD API shape

### 4.1 Router (`flora-api/src/routes/documents.rs`, nested in `routes/mod.rs`)

| Method | Path | Request | Response |
|---|---|---|---|
| `GET` | `/api/v1/documents?workspace_id=<uuid>&channel_id=<uuid>&limit=&offset=` | — | `200 {documents: [...]}` (or `Json<Vec<DocumentResponse>>` to copy tasks) |
| `POST` | `/api/v1/documents` | `{ workspace_id, channel_id?, title, content? }` | `201 DocumentResponse` |
| `GET` | `/api/v1/documents/{id}` | — | `200 DocumentResponse` |
| `PATCH` | `/api/v1/documents/{id}` | `{ expected_version, title?, content? }` | `200 DocumentResponse` (new `version`) or `409` |
| `DELETE` | `/api/v1/documents/{id}` | — | `204` (soft delete) |
| `POST` | `/api/v1/documents/{id}/restore` *(optional P0.5)* | — | `204` |

`DocumentResponse` fields: `id`, `organization_id`, `workspace_id`, `channel_id: Option<String>`, `creator_id`, `title`, `content`, `version: i32`, `is_deleted: bool` (optional in response), `created_at`, `updated_at` — all IDs `String`, timestamps `to_rfc3339()`, exactly like `TaskResponse` `[F4:84-128]`.

**PATCH semantics:** `content: Some("")` clears the note; omitted field = no change (no tri-state helper needed, unlike `assignee_id` `[F4:37-82]`). `expected_version` is **required** on PATCH; returning 428 for a missing token is a P1 refinement (HTTP vocabulary per `[S1][S2]`; the RFC 9110 primary text was not fetched this run — **NV-primary**).

**CAS update (the core invariant):**

```sql
UPDATE documents
   SET title = $4, content = $5, version = version + 1, updated_at = NOW()
 WHERE id = $1 AND organization_id = $2 AND version = $3 AND is_deleted = false
RETURNING id, workspace_id, channel_id, organization_id, creator_id,
          title, content, version, is_deleted, created_at, updated_at;
```

Zero rows ⇒ either gone or stale. Follow the recommended disambiguation: a follow-up `find_by_id_for_org` returns 404 if absent, otherwise 409 with `current_version` `[S2][S3]`. Never compute `new_version` in application code and write it back — increment inside the statement `[S3]`.

### 4.2 Tenancy/authorization per handler (copy `tasks.rs` `[F4:130-167,218-364]`)

1. `let org_id = auth.require_org()?` → 400 when nil org.
2. `PgMembershipRepository::find_by_user_and_organization(user_id, org_id)` → 403 if not a member.
3. For create: load `workspace` and (if present) `channel`, assert `row.organization_id == org_id` (and `channel.workspace_id == workspace_id`) → 403. **The existing tasks create handler does not validate that `req.workspace_id` belongs to `org_id`** `[F4:169-192]` — do not replicate that gap for documents (R3).
4. For get/update/delete: prefer a repository method `find_by_id_for_org(id, org_id)` that puts the tenant predicate in SQL; also keep the handler-level 403 check as defense in depth `[F4:235-239]`.
5. Channel scope is a **filter, not an ACL**: there is no channel-membership table in the Seed schema `[F1]`, so `Private` channel docs are visible to any org member. Flag as R4; do not silently claim channel-level privacy.

### 4.3 Layers and error mapping

- **Trait** `DocumentRepository` in `flora-core/src/traits/repository.rs` (add import of `Document`): `find_by_id_for_org`, `find_by_workspace_id(org_id, workspace_id, channel_id, pagination)`, `create`, `update_checked(id, org_id, expected_version, doc)`, `soft_delete`, `restore` `[F9]`.
- **Repo** `flora-core/src/repositories/document_repository.rs` + `pub mod`/`pub use` in `repositories/mod.rs` `[F14]`; explicit column lists, `?`-propagation, `Error::from_sqlx`, `fetch_optional` + `ok_or_else` for not-found, `#[expect(..., reason)]` only in tests `[F3]`.
- **Service** `flora-core/src/services/document_service.rs` + exports in `services/mod.rs` `[F14]`: `validate()` inputs, apply title/content, `tracing::info!`/`debug!` (never `println!` — `print_stdout` is warned/denied in workspace lints `[F12]`), no `unwrap`/`expect` in prod.
- **Model** `flora-core/src/models/document.rs` + `pub mod`/`pub use` in `models/mod.rs` `[F14]`: `Document`, `CreateDocumentInput`, `UpdateDocumentInput`, `#[validate(length(...))]` on title/content.
- **Errors** in `flora-core/src/error.rs` `[F10]`: add `DocumentNotFound(String)` (404 — add to all three 404 match arms) and `DocumentVersionConflict { expected: i32, current: i32 }` (409 — add to `is_client_error` and `IntoResponse`, body `{"error":"version_conflict", ...}`). `DuplicateEntry` already maps to 409 but is semantically wrong here.

---

## 5. Exact touch points (implementation checklist)

**Backend (all new/modified paths are relative to repo root):**

1. `flora-core/src/migrations/sql/20261119000001_documents.sql` — **new** (schema §2).
2. `flora-core/src/models/document.rs` — **new**; `flora-core/src/models/mod.rs` — add `pub mod` + `pub use`.
3. `flora-core/src/traits/repository.rs` — add `DocumentRepository`.
4. `flora-core/src/repositories/document_repository.rs` — **new**; `repositories/mod.rs` — register.
5. `flora-core/src/services/document_service.rs` — **new**; `services/mod.rs` — register.
6. `flora-core/src/error.rs` — 2 variants + `is_client_error` + `is_not_found` + `IntoResponse`.
7. `flora-api/src/routes/documents.rs` — **new**; `flora-api/src/routes/mod.rs` — `pub mod documents;` + `.nest("/api/v1/documents", …)`.
8. `flora-api/src/routes/search.rs` — `include_documents` branch + doc comment.
9. `flora-api/src/state/app_state.rs` — build `document_repo`/`document_service` in `AppState::new`, add `pub document_service: Arc<DocumentService>` field, include in returned `Self`.
10. `flora-api/src/extractors/auth.rs` — **compile break risk**: `make_test_app_state()` constructs `AppState` literally; add the new field `[F7:272-355]`.
11. `flora-tests/src/contract/documents.rs` (+ `contract/mod.rs`) and optionally `integration/documents.rs` — DB-gated suite following the `search.rs`/`tasks.rs` gating convention `[F15]`.

**Frontend (`apps/frontend`):**

12. `src/lib/types/models.ts` — `Document` interface (`id`, `workspaceId`, `channelId`, `title`, `content`, `version`, timestamps).
13. `src/lib/api/mappers.ts` — `ApiDocument` + `toDocument()` (snake_case→camelCase) `[F11]`.
14. `src/lib/state/DocumentStore.svelte.ts` — **new**; `$state<Document[]>` + idempotent upsert (pattern: `TaskStore`) `[F11]`.
15. `src/routes/(app)/documents/+page.svelte` — list + "New note".
16. `src/routes/(app)/documents/[id]/+page.svelte` — load doc, host editor.
17. `src/lib/components/documents/DocumentEditor.svelte` — **new** (§6).
18. `src/routes/(app)/+layout.svelte` — sidebar `FileText` item + optionally wire the dead search box to `/api/v1/search?type=documents` `[F11][F13]`.
19. `src/lib/vitest-examples/` or a colocated `.spec.ts` — dirty-state / conflict-handling unit test.

**Docs/governance:** `ROADMAP.md:54`, `OBJETIVO.md` §5, and a cycle-033 release/assessment note must record the delivered slice + NVs; no tag, release, or branch-protection change without explicit authorization `[F13]`.

---

## 6. Simple Svelte 5 editor (no CRDT, no heavy deps)

**Core widget:** a plain `<textarea bind:value={draft}>` — `bind:value` is a documented Svelte 5 binding that works on textareas `[P8]`. Runes state in `.svelte.ts` or component: `let { document, onSave } = $props(); let draft = $state(document.content); let version = $state(document.version); let status = $state<'saved'|'dirty'|'conflict'|'saving'>('saved'); let dirty = $derived(draft !== document.content);` `[P7]`.

**Save loop (no library):**
- `oninput` updates `draft` and marks `dirty`; a small local `debounce(fn, 800–1500 ms)` using `setTimeout`/`clearTimeout` triggers `PATCH`.
- On success: set `version = response.version`, `status = 'saved'`.
- On 409: set `status = 'conflict'`, keep the local draft intact, show "This note changed elsewhere — reload to see the newer version" with an explicit **Reload** button. **Never** auto-overwrite on conflict `[S1][S3]`.
- `Ctrl/Cmd+S` forces an immediate save. `aria-live="polite"` status text; `<label>` for the textarea; visible focus.

**Preview / rendering — the XSS decision:**
- P0 (zero deps): show the Markdown source in `<pre>{draft}</pre>`. Svelte text expressions are escaped automatically, so **no HTML injection path exists** `[P7]`.
- If a rendered preview is later required: parse to HTML with a small library and pass through a sanitizer before `{@html sanitized}`. Svelte's docs state plainly that `{@html}` does **not** sanitize and "Never render unsanitized content" `[P7]`; Postgres likewise warns that `ts_headline` output is not XSS-safe `[P2]`.
- Do **not** use `contenteditable`/`bind:innerHTML` for P0. Besides round-trip drift, there was a moderate Svelte SSR XSS advisory for `bind:innerText`/`bind:textContent` on `contenteditable`, patched in 5.53.5 `[P9]`; if it is ever adopted, pin `svelte >= 5.53.5`.
- Do **not** adopt Milkdown/ProseMirror/CodeMirror/MdxEditor-class editors: they contradict the debloat goal (multiple runtime deps, large bundles) `[S4][P5]`.

**Explicitly not in the UI:** formatting toolbar, drag-and-drop images, slash commands, tables/footnotes support, collaborative cursors, undo history beyond the browser's native textarea undo.

---

## 7. What to defer, and why (with primary evidence)

| Deferred | Why (evidence) | Re-entry trigger |
|---|---|---|
| **CRDT / OT co-editing** (Loro, Yjs, Automerge) | CRDTs solve concurrent, offline/P2P, no-central-authority editing `[P6][P5]`; Flora is single-node with a Postgres source of truth. Loro itself says consider alternatives for "simple client–server sync" and bundle-size-sensitive apps, citing ~970 KB gzipped WASM `[P5]`. | Measured demand for simultaneous same-doc editing; budget for sync protocol + persistence + WASM. |
| **Realtime presence / cursors / typing** | Needs a fanout contract the Seed is still finishing; prior research recommends DB commit + cursor catch-up + best-effort push, not CRDT `[F13]`. | Cycle 015 realtime slice fully closed and multi-client tests green. |
| **Offline merge / local-first** | Requires CRDT state, durable client DB, conflict UI. | Same as above; explicit product decision. |
| **Attachments / embeds** | `flora-files` already owns binary storage `[F1][F13]`; notes should reference file IDs/URLs, not duplicate storage. | Never for P0; link-only is sufficient. |
| **Rich text / WYSIWYG** | `contenteditable` XSS/round-trip risk `[P7][P9]`; heavy editor bundles `[S4]`. | Demand for tables/images; then adopt a sanitizer-first renderer. |
| **Revision history / restore UI** | Doubles storage and adds ACL/retention work; `version` already prevents lost updates. | Users ask for "who changed what / rollback". |
| **Import/export, templates, @-links/backlinks** | Macro-parity surface, explicitly out of Seed scope `[F13]`. | P2 backlog. |
| **Per-channel ACL / public sharing** | No channel-membership table exists `[F1]`; adding one is a cross-cutting auth change (R4). | Security requirement with a dedicated cycle. |
| **Meilisearch indexing of docs** | Meilisearch is dormant/non-authoritative by contract `[F2][F5][F13]`. | Re-enablement gate with determinism/rebuild evidence. |

---

## 8. 80/20 acceptance criteria (verifiable slice)

1. **Schema/migration:** fresh Postgres + `run_migrations()` yields `documents` with generated `search_tsv` and partial GIN index; `ALTER`/restore is safe `[P1]`.
2. **CRUD:** create → get → patch → list → soft-delete works end-to-end; `is_deleted = true` rows excluded from list and search.
3. **Tenancy:** org A cannot get/patch/delete org B's document (403/404); create rejects a workspace/channel from another org.
4. **Concurrency:** two PATCHes with the same `expected_version` → exactly one 200 (version increments), one 409 with `current_version`; no silent overwrite.
5. **Search:** `GET /api/v1/search?q=<term>&type=documents` returns only `organization_id = org`, `is_deleted = false` docs, `source: "postgres"`, and degrades to empty results for punctuation-only queries without a 500.
6. **Frontend:** note appears in sidebar list; typing marks dirty; debounced save updates version; stale save shows conflict banner; no `{@html}` of raw user content; `bun run check`/`lint`/`test:unit`/`build` green.
7. **Quality gates:** `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test -p flora-core -p flora-api -p flora-tests`; frontend checks per `AGENTS.md` `[F12]`. DB-gated suites stay env-gated like existing ones.
8. **No new mandatory dependency** in `Cargo.toml` or `apps/frontend/package.json` for P0.

---

## 9. Risks and NVs (name them; do not paper over)

- **R1 — i18n/CJK recall (NV, high).** Flora uses `text-search configuration 'simple'`: no stemming, no stop words, and **no CJK dictionaries**. Chinese docs index a whole sentence as one lexeme; sub-token queries miss. Evidence: PostgreSQL has no built-in CJK config `[S5][S7]`; Alibaba Cloud's docs show `to_tsvector('你好，这是一条中文测试文本')` → one token and require `zhparser`/SCWS `[M-zh]`; Japanese sources show `to_tsvector('simple','東京タワーの夜景を撮影しました')` → one token and recommend `pg_trgm`/`pg_bigm`/PGroonga/MeCab `[M-ja]`. Mitigation for P0: keep `simple` for cross-entity parity; document that CJK/morphological search is degraded; potential P1 = `pg_trgm` (bundled, but ≥3 chars and `LC_CTYPE != C`) or `pg_bigm`/PGroonga (extra extension, VPS cost). **NV:** no CJK recall benchmark in this environment.
- **R2 — index/query normalization mismatch.** Index replaces `-`/`.`/`_` with spaces; `websearch_to_tsquery` does not (and `-` means NOT) `[F2][P2]`. Hyphenated identifiers may not match as expected. Accept parity now; harden separately.
- **R3 — cross-tenant scope attachment.** Existing task create does not verify workspace ownership `[F4:169-192]`. Documents must; otherwise a member of org A could attach a note to org B's workspace by ID. Also enforce `channel.workspace_id == workspace_id`.
- **R4 — channel scope is not an ACL.** No channel-membership table `[F1]`; `Private` channel docs remain org-visible. Do not market as private-channel notes until channel ACLs exist. Search also lacks workspace/channel filtering `[F5]`.
- **R5 — storage/GIN cost on 4 GB.** `content TEXT` + stored `tsvector` + GIN increase disk and write cost; GIN maintenance and autovacuum matter on write-heavy tables `[P1][S5]`. Cap content length in `validator` (e.g. 200k chars) and keep `LIMIT ≤ 50` in search. **NV:** no load/OOM measurement done here.
- **R6 — AppState struct-literal break.** Adding a field requires updating `extractors/auth.rs` test helper or tests fail to compile `[F7]`.
- **R7 — user deletion cascades notes.** `creator_id … ON DELETE CASCADE` follows tasks but loses authored notes when a user is removed. Consider `ON DELETE SET NULL` + nullable `creator_id` in a hardening pass. **NV:** no policy defined for user offboarding.
- **R8 — "versioned" ≠ history.** The `version` integer is a CAS token only. If stakeholders expect version history/restore, that is a separate `document_revisions` feature; set expectations in cycle docs now.
- **R9 — Svelte XSS.** `{@html}` is not sanitizing `[P7]`; `contenteditable` bindings had an SSR XSS advisory `[P9]`. P0 textarea + `<pre>` avoids both; any future preview must sanitize.
- **R10 — migration timestamp discipline.** Use `> 20261118000001` and `IF EXISTS`/`IF NOT EXISTS`; understand why the older FTS migrations work out of order `[F2]`.
- **R11 — no license/dependency drift.** AGPL-3.0 clean-room; no Macro code reuse; cargo-vet/dependency-review gates remain informational/bounded by authorization `[F12][F13]`. No new deps chosen deliberately.
- **NV — runtime truth:** no live API smoke, no DB-backed test execution, no browser e2e, and no perf measurement were run for this research; all behavior above is derived from code and primary docs, not observed in this environment.

---

## 10. Multilingual evidence map

| Language | Representative sources (this run) | Finding used |
|---|---|---|
| **EN (primary)** | PostgreSQL 18 docs: `textsearch-tables`, `textsearch-controls`, `textsearch-indexes` `[P1][P2][P3]`; Svelte 5 docs `{@html}`, `bind`, runes `[P7][P8]`; Loro docs/benchmarks, Yjs README `[P5][P6]` | Generated-column FTS, `websearch_to_tsquery`, XSS warnings, CRDT cost/bundle. |
| **pt-BR** | Runstack "Full Text Search no PostgreSQL" `[M-pt1]`; Urhoba "Busca de Texto Completo" `[M-pt2]` | `portuguese` config stems but Flora's `simple` does not; generated column + GIN is the recommended path locally too. |
| **es** | wrujel "Búsqueda de texto completo en Postgres" `[M-es1]`; Kaleidos, StackPractices `[M-es2]` | Same consensus; `websearch_to_tsquery` over `to_tsquery` for user input; GIN read-fast/write-costly. |
| **de** | synyx "Volltextsuche mit PostgreSQL" `[M-de]` | `german` stemming removes stop words; mixing document/query configs fails silently → reinforces keeping `'simple'` explicit everywhere. |
| **fr** | StackHarbor "Recherche plein texte PostgreSQL" `[M-fr]` | `french` config + `ts_rank_cd`; for mixed content choose `simple` or multiple vectors — matches Flora's choice. |
| **zh-CN** | Alibaba Cloud AnalyticDB full-text docs (Zhparser/SCWS) `[M-zh]` | Default `to_tsvector` cannot segment Chinese; `zhparser` + custom config needed. Direct, concrete NV evidence. |
| **ja** | RJC `pg_trgm` vs `pg_bigm` benchmark; Zenn/ParadeDB BM25; NEC `pg_bigm`; PowerGres `textsearch_ja` `[M-ja]` | `tsvector(simple)` is effectively unusable for Japanese; `pg_trgm` needs ≥3 chars and non-`C` locale; `pg_bigm`/PGroonga/MeCab are extra extensions. |
| **Secondary (concurrency/editor)** | optimistic-locking write-ups `[S1][S2][S3]`; markdown-editor ecosystem survey `[S4]` | CAS `UPDATE … WHERE version = $n`, 409/412 vocabulary; avoid heavy editor frameworks. |

**NV-multilingual:** non-EN primary vendor docs were available for zh-CN (Alibaba Cloud) and ja (NEC/PowerGres vendors); pt-BR/es/de/fr evidence is high-quality secondary (community/consulting), not first-party Postgres translations. Conclusions do not depend on locale-specific nuance beyond the CJK tokenization fact, which is corroborated by the EN docs' lack of CJK configurations `[S5]`.

---

## 11. Sources

### Primary — external
- **[P1]** PostgreSQL 18 docs, *Tables and Indexes* (stored generated `tsvector`, `coalesce`, GIN, immutability, restore): https://www.postgresql.org/docs/current/textsearch-tables.html (fetched 2026-11-16)
- **[P2]** PostgreSQL 18 docs, *Controlling Text Search* (`websearch_to_tsquery` never raises syntax errors; `ts_rank_cd`; `ts_headline` XSS warning): https://www.postgresql.org/docs/current/textsearch-controls.html (fetched 2026-11-16)
- **[P3]** PostgreSQL 18 docs, *Preferred Index Types for Text Search* (GIN preferred; lexeme posting lists): https://www.postgresql.org/docs/current/textsearch-indexes.html
- **[P4]** PostgreSQL 18 docs, *Dictionaries* (configuations/stemming/stop words): https://www.postgresql.org/docs/current/textsearch-dictionaries.html
- **[P5]** Loro, *Introduction* ("consider alternatives" for simple client–server sync / bundle size; ~970 KB gzipped WASM) and *JS/WASM Benchmarks*: https://www.loro.dev/docs , https://www.loro.dev/docs/performance
- **[P6]** Yjs README (CRDTs for distributed/no-central-source, offline, editor bindings): https://github.com/yjs/yjs
- **[P7]** Svelte docs, `{@html}` (explicit "Never render unsanitized content"), *Basic markup* (text expressions escaped), *What are runes?*, `bind:`: https://svelte.dev/docs/svelte/@html , https://svelte.dev/docs/svelte/basic-markup , https://svelte.dev/docs/svelte/what-are-runes , https://svelte.dev/docs/svelte/bind
- **[P8]** Svelte docs, `bind:value` on textarea (Svelte 5 compatible): https://svelte.dev/docs/svelte/bind
- **[P9]** GitHub Advisory `GHSA-phwv-c562-gvmh`, Svelte `contenteditable` SSR XSS, patched 5.53.5: https://github.com/sveltejs/svelte/security/advisories/GHSA-phwv-c562-gvmh

### Secondary — external
- **[S1]** Optimistic concurrency / lost updates, version-in-body → 409: https://www.lochana.co/blog/optimistic-concurrency-lost-updates
- **[S2]** Optimistic concurrency for HTTP APIs (CAS, 409 vs 412/428, integer over timestamp): https://matheuspalma.com/blog/optimistic-concurrency-http-apis-etags-if-match-and-conflict-design
- **[S3]** Prevent lost updates with optimistic locking in SQL (increment in same `UPDATE`, zero rows = conflict): https://nalar.dev/prevent-lost-updates-with-optimistic-locking-in-sql
- **[S4]** Markdown-editor ecosystem survey (Milkdown/ProseMirror/CodeMirror and DOMPurify requirements): https://www.npmjs.com/package/svelte5-milkdown-editor , https://markdown.svelte.page/blog/preventing-markdown-xss-in-svelte
- **[S5]** Xata, *Create an advanced search engine with PostgreSQL* (no built-in CJK config; `simple` for mixed content): https://xata.io/blog/postgres-full-text-search-engine
- **[S6]** Renode, *PostgreSQL FTS: tsvector, tsquery, GIN* (generated column checklist, mixed-language `regconfig`, `EXPLAIN`): https://renode.space/blog/postgresql-full-text-search-basics
- **[S7]** rivestack, *PostgreSQL Full Text Search* (configurations per language; CJK needs `zhparser`/trigram): https://rivestack.io/blog/postgres-full-text-search

### Multilingual
- **[M-pt1]** Runstack (pt-BR): https://runstack.com.br/postgres-full-text-search
- **[M-pt2]** Urhoba (pt-BR): https://www.urhoba.net/pt/artigo/busca-de-texto-completo
- **[M-es1]** wrujel (es): https://blog.wrujel.com/es/postgres-full-text-search-without-elasticsearch-d9269e
- **[M-es2]** Kaleidos (es): https://www.kaleidos.net/blog/como-usar-busqueda-de-texto-en-postgresql
- **[M-de]** synyx (de): https://synyx.org/blog/volltextsuche-mit-postgresql-und-spring-teil-i
- **[M-fr]** StackHarbor (fr): https://stackharbor.com/fr/base-de-connaissances/pg-full-text-search-tsvector
- **[M-zh]** Alibaba Cloud AnalyticDB docs (zh-CN, `Zhparser`/SCWS, single-token example): https://help.aliyun.com/zh/analyticdb/analyticdb-for-postgresql/developer-reference/full-text-search-1
- **[M-ja]** RJC (ja, `pg_trgm` vs `pg_bigm` benchmark): https://www.r-jc.jp/news-blog/1724/ ; Zenn/ParadeDB (ja): https://zenn.dev/okamyuji/articles/paradedb-japanese-fulltext-search ; NEC `pg_bigm`: https://jpn.nec.com/postgresql/technical_info/pg_bigm_v2.html ; PowerGres `textsearch_ja`: https://powergres.sraoss.co.jp/tech/exp/04_textsearch

### Repo (internal — verified this run)
- **[F1]** `flora-core/src/migrations/sql/20240818000001_init_schema.sql`
- **[F2]** `flora-core/src/migrations/sql/{20261116000004_search_fts_separator_normalization,20261117000001_add_messages_search_fts,20261118000001_add_tasks_files_search_fts}.sql`; `20261116000002_add_workspace_id_to_files.sql`
- **[F3]** `flora-core/src/models/task.rs`, `repositories/task_repository.rs`, `services/task_service.rs`
- **[F4]** `flora-api/src/routes/tasks.rs`
- **[F5]** `flora-api/src/routes/search.rs`
- **[F6]** `flora-api/src/routes/mod.rs`
- **[F7]** `flora-api/src/extractors/auth.rs`
- **[F8]** `flora-api/src/state/app_state.rs`
- **[F9]** `flora-core/src/traits/repository.rs`
- **[F10]** `flora-core/src/error.rs`
- **[F11]** `apps/frontend/src/{lib/api/client.ts,lib/api/mappers.ts,lib/state/TaskStore.svelte.ts,lib/types/models.ts,routes/(app)/+layout.svelte,package.json}`
- **[F12]** `AGENTS.md` (repo + HQ), `.github/workflows/{rust,ci}.yml`
- **[F13]** `OBJETIVO.md`, `ROADMAP.md`, `.specify/memory/constitution.md`, `.specify/assessments/032-next-vertical-slice/research.md`, `.specify/releases/032-cors-allowlist/release.md`
- **[F14]** `flora-core/src/{lib.rs,models/mod.rs,repositories/mod.rs,services/mod.rs,migrations/mod.rs}`
- **[F15]** `flora-tests/src/{contract,integration}/**`
