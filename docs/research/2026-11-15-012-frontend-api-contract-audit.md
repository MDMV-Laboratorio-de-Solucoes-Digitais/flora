# Frontend ↔ API contract audit (input for cycle 012)

**Base:** `dev@598f08152` (cycle 011 merged, PR #214)
**Date:** 2026-11-15
**Scope:** `apps/frontend/src` (SvelteKit/Svelte 5) vs `flora-api/src/routes` + `flora-core` DTOs
**Method:** static read of every call site and handler, plus axum 0.8.9 routing semantics verified in the
local cargo registry (`path_for_nested_route`, `nest`). No runtime/DB execution, no source changes.
**Out of scope of this document:** UI redesign, feature proposals, production deploy hardening.

> **Verified baseline fact:** axum 0.8.9 `nest("/api/v1/tasks", …)` + inner `/` registers exactly
> `/api/v1/tasks` (`~/.cargo/.../axum-0.8.9/src/routing/path_router.rs:535-545`). Frontend calls
> without a trailing slash are **not** a mismatch. This was checked explicitly to avoid a false finding.

---

## 0. Executive summary

1. The 011 slice (**auth → org → workspace → task**) is functionally aligned in the happy path, but the
   session authority model is still inconsistent: the frontend never calls `POST /auth/logout`, and
   protected routes validate only the JWT, never the session row (`flora-api/src/extractors/auth.rs:170-239`).
   Logout therefore does not invalidate a token server-side.
2. `ApiClient` still drops the response body on **401**, contradicting FR-002 of spec 011
   (`apps/frontend/src/lib/api/client.ts:80-83`, test only asserts the message at
   `apps/frontend/tests/integration/api-interceptor.test.ts:13-30`).
3. Messaging is contract-broken end-to-end: response uses `sender_id`/`thread_id`, frontend reads
   `authorId`/`threadId`; request uses `content` + `fileIds` but the backend ignores unknown fields;
   channel IDs are hardcoded non-UUIDs; the WebSocket path/auth/protocol all disagree.
   These were explicit **011 non-goals**, so they are not regressions, but `/channels` and `/dms` are
   reachable from the nav and cannot work.
4. File upload/download, telemetry logging and user invite call endpoints that **do not exist**.
5. Test signal is weak: contract tests for tasks/workspace/files/messaging are `Ok(())` stubs; the
   `tests/e2e/*.spec.ts` Playwright specs never match `testMatch`; the 401 test does not assert the body.

---

## 1. Frontend call inventory

Every `ApiClient.request`, raw `fetch`, WebSocket and beacon call in `apps/frontend/src`.

| # | Call site (file:line) | Method + path | Request | Expected response (as typed in code) | Error handling |
|---|---|---|---|---|---|
| F1 | `routes/(auth)/+page.svelte:22-32` | `GET /api/v1/auth/login?redirect_uri=` (raw `fetch`) | — | `{ authorization_url, state }` (untyped `any`) | `try/catch` → `console.error` only; no `response.ok` check; no user feedback |
| F2 | `routes/(auth)/callback/+page.svelte:46-52` | `GET /api/v1/auth/callback?code&state&redirect_uri` | — | `{ session_token, user_id, next_step? }` | catch → `goto('/?error=exchange_failed')`; state check is conditional (:30-35) |
| F3 | `routes/(app)/select-org/+page.svelte:42` | `GET /api/v1/organizations` | — | `Organization[]` `{id,name,slug,role?}` | catch → `errorMessage` banner |
| F4 | `…/select-org/+page.svelte:54-62` | `POST /api/v1/auth/select-organization` | `{ organization_id }` | `{ session_token,user_id,organization_id }` | catch → banner |
| F5 | `…/select-org/+page.svelte:76-83` | `POST /api/v1/organizations` | `{ name, slug }` | `{ id,name,slug }` | catch → banner |
| F6 | `routes/(app)/tasks/+page.svelte:38` | `GET /api/v1/workspaces` | — | inline `{id,name}[]` | catch in `refresh()` → banner |
| F7 | `…/tasks/+page.svelte:50` | `GET /api/v1/tasks` | — | inline `ApiTask[]` (snake_case, no `updated_at`) | catch → banner |
| F8 | `…/tasks/+page.svelte:72-82` | `POST /api/v1/tasks` | `{ title, description?, workspace_id, assignee_id? }` | `ApiTask` | catch → banner |
| F9 | `lib/components/organization/OrganizationManager.svelte:17-20` | `POST /api/v1/organizations` | `{ name }` **only** | ignored (`void`) | catch → `logger.error`; **no UI error state** |
| F10 | `lib/components/workspace/WorkspaceCreate.svelte:15-18` | `POST /api/v1/workspaces` | `{ name }` | ignored | catch → `logger.error`; no UI error |
| F11 | `lib/components/users/UserManager.svelte:18-21` | `POST /api/v1/users/invite` | `{ email, role:"Member" }` | ignored | catch → `logger.error`; no UI error |
| F12 | `lib/components/files/FileUpload.svelte:25-29` | `POST /api/v1/files/upload` | `FormData(file)` | `FileAttachment` (camelCase) | catch → generic `"Upload failed."` |
| F13 | `lib/components/files/FilePreview.svelte:6-8` | `GET /api/v1/files/download/{id}` (anchor, no auth) | — | binary | none (browser navigation) |
| F14 | `lib/components/chat/ChatLayout.svelte:22` | `GET /api/v1/channels/{id}/messages` | — | `Message[]` (camelCase) | catch → `logger.error` only |
| F15 | `…/ChatLayout.svelte:43-49` | `POST /api/v1/channels/{id}/messages` | `{ content, fileIds }` | ignored | catch → `logger.error` only |
| F16 | `lib/components/chat/thread/ThreadReply.svelte:16-19` | `POST /api/v1/channels/{id}/messages` | `{ content, threadId }` | ignored | catch → `logger.error` only |
| F17 | `lib/api/websocket.ts:23-27` | `WS /api/v1/ws?token=<jwt>` | query token, no header | — | close → 3 s reconnect loop (:63-72) |
| F18 | `lib/api/websocket.ts:78-80` | `GET /api/v1/messages/sync?last_known_message_id=` | — | `Message[]` | catch → `logger.error` |
| F19 | `lib/utils/logger.ts:106-109` | `POST /api/v1/telemetry/logs` | `LogEntry[]` | ignored | catch swallowed |
| F20 | `lib/utils/logger.ts:40` | `navigator.sendBeacon('/api/v1/telemetry/logs')` | `LogEntry[]`, no auth | — | none (relative to **frontend** origin) |
| F21 | `routes/+layout.svelte:9-14` | invokes `wsClient.connect()` | — | — | — |

Error handling patterns actually used: `error.message` string in tasks/select-org; `logger.error` only in
settings/chat/files; silent drop in logger. No component reads `ApiError.status`, `.code` or `.field`.

---

## 2. Backend route / DTO inventory

Router assembly: `flora-api/src/routes/mod.rs:20-58`.

| Path | Method | Handler (file:line) | Success | Response shape | Error shape | Auth / org |
|---|---|---|---|---|---|---|
| `/health`, `/health/ready`, `/health/live` | GET | `routes/mod.rs:23-25` | 200 | JSON | — | none |
| `/api/v1/auth/login` | GET | `auth.rs:85-140` | 200 | `{authorization_url,state,next_step}` | domain JSON | none |
| `/api/v1/auth/callback` | GET | `auth.rs:147-265` | 200 | `{session_token,expires_in,user_id,next_step}` | domain JSON | none |
| `/api/v1/auth/logout` | POST | `auth.rs:268-281` | 204 always | — | — | optional bearer |
| `/api/v1/auth/refresh` | POST | `auth.rs:284-319` | 200 | `{session_token,expires_in,user_id}` | domain JSON | body token + active session |
| `/api/v1/auth/select-organization` | POST | `auth.rs:325-385` | 200 | `+organization_id` | domain JSON | bearer, org may be nil |
| `/api/v1/organizations` | GET | `org.rs:88-119` | 200 | `OrgListItem[]` `{id,name,slug,role,settings}` | domain JSON | bearer only |
| `/api/v1/organizations` | POST | `org.rs:124-176` | **201** | `OrgResponse` `{id,name,slug,settings,created_at}`; `slug` **required** (:28-35) | domain JSON | bearer only |
| `/api/v1/organizations/{id}` | GET | `org.rs:179-204` | 200 | `OrgResponse` | 403/404 JSON | bearer + membership |
| `/api/v1/workspaces` | GET | `workspace.rs:95-118` | 200 | `WorkspaceResponse[]` | 422/403 JSON | bearer + org + membership |
| `/api/v1/workspaces` | POST | `workspace.rs:121-154` | 201 | `WorkspaceResponse` | 422/403 JSON | same |
| `/api/v1/workspaces/{id}` | GET/PATCH/DELETE | `workspace.rs:157-280` | 200/200/204 | `WorkspaceResponse` | JSON | same (+role on PATCH/DELETE) |
| `/api/v1/workspaces/{id}/switch` | POST | `workspace.rs:286-327` | 200 | `WorkspaceSwitchResponse` | JSON | same (**unused by FE**) |
| `/api/v1/tasks` | GET | `tasks.rs:95-131` | 200 | `TaskResponse[]` | JSON | bearer + org + membership |
| `/api/v1/tasks` | POST | `tasks.rs:134-160` | **201** | `TaskResponse` | JSON | same |
| `/api/v1/tasks/{id}` | GET/PATCH/DELETE | `tasks.rs:163-255` | 200/200/204 | `TaskResponse` | JSON | org claim only (no membership recheck) |
| `/api/v1/channels` | POST | `channels.rs:139-192` | 201 | `ChannelResponse` `{id,workspace_id,name,type,created_at}` | JSON | bearer + org + membership |
| `/api/v1/channels/{id}/messages` | GET | `channels.rs:195-233` | 200 | `MessageResponse[]` | JSON | org claim + channel org |
| `/api/v1/channels/{id}/messages` | POST | `channels.rs:236-271` | **201** | `MessageResponse` | JSON | same |
| `/api/v1/channels/{id}/messages/{msg}` | PATCH/DELETE | `channels.rs:274-345` | 200/204 | `MessageResponse` | JSON | sender-only; `channel_id` **ignored** |
| `/api/v1/files` | POST | `files.rs:60-100` | 201 | `FileResponse` `{id,name,file_type,size_bytes,owner_id,created_at}` | JSON | bearer + org + membership |
| `/api/v1/files/{id}` | GET/DELETE | `files.rs:103-159` | 200/204 | `FileResponse` | JSON | org + owner on delete |
| `/api/v1/roles…` | GET/POST/… | `rbac.rs:18-21` | 200/201 | role DTOs | JSON | unused by FE |
| `/api/v1/notifications…` | GET/POST | `notifications.rs:47-48` | 200 | — | JSON | unused by FE |
| `/api/v1/search` | GET | `search.rs:53` | 200 | — | JSON | unused by FE |
| `/ws` | GET (upgrade) | `websocket.rs:100-120` | 101 | `WsServerMessage` | **empty 401** | `Authorization: Bearer` header only |

Error envelope for domain errors: `flora-core/src/error.rs:288-373` → JSON
`{error, message}` plus `field` for validation; 403 always says `"Insufficient permissions"`
(:295-298), dropping the specific reason. Extractor rejections are `(StatusCode, String)` →
**`text/plain`** (`extractors/auth.rs:33, 84, 175`); axum `Path`/`Json` rejections are also text/plain
(e.g. non-UUID path → 400, malformed/missing JSON field → 422). Rate limit returns 429 with an **empty**
body (`middleware/rate_limit.rs:183-185`).

**Auth/org semantics:** the organization comes from the JWT `organization_id` claim; `require_org()`
maps the nil org to `Error::Validation` → **422** (`extractors/auth.rs:159-168`, e.g. `tasks.rs:139-142`).
`Error::OrganizationContextRequired` (400) exists (`error.rs:140-142, 344-347`) but is not used by routes.
No route reads `X-Organization-ID`, despite the stale comment at `routes/mod.rs:28`.

---

## 3. DTO field map (contract drift)

| Entity | Backend keys (source) | Frontend type (source) | Verdict |
|---|---|---|---|
| Task | `id, workspace_id, creator_id, assignee_id, title, description, status, created_at` (`tasks.rs:50-67`) | inline `ApiTask` same snake_case minus `creator_id` (`tasks/+page.svelte:11-19`) | OK for page; `models.ts:38-47` is camelCase + `TODO/IN_PROGRESS/DONE` and is **wrong** (unused outside stores) |
| Task status | `"Todo" \| "InProgress" \| "Done" \| "Archived"` (`flora-core/src/models/task.rs:102-118`) | `'TODO' \| 'IN_PROGRESS' \| 'DONE'` (`models.ts:43`) | **Mismatch** (latent; `KanbanBoard.svelte` is not mounted) |
| Workspace | `id, organization_id, name, description, created_at, updated_at` (`workspace.rs:42-55`) | `{id,name}` (`tasks/+page.svelte:10`) | OK (extra fields ignored) |
| Organization | `id, name, slug, role, settings` (`org.rs:66-77`) | `{id,name,slug,role?}` (`select-org:10-15`) | OK |
| Message | `id, channel_id, sender_id, content, thread_id, is_edited, created_at, updated_at` (`channels.rs:82-99`) | `id, channelId, authorId, content, threadId, fileAttachments, createdAt, updatedAt` (`models.ts:27-36`) | **Broken**: `sender_id` vs `authorId`, no `file_attachments` at all |
| Channel | `id, workspace_id, name, type, created_at` (`channels.rs:66-78`) | `id, workspaceId, name, description, isPrivate, createdAt` (`models.ts:18-25`) | **Mismatch** (`type` vs `isPrivate`, no `description`) — unused today |
| File | `id, name, file_type, size_bytes, owner_id, created_at` (`files.rs:34-48`) | `id, fileName, mimeType, byteSize, rustFsUrl, uploadedAt` (`models.ts:49-56`) | **Broken**: no field name overlap, no URL field |
| Auth | `session_token, expires_in, user_id, organization_id, next_step` (`auth.rs:259-264, 379-384`) | only `session_token/user_id/organization_id`; `expires_in` never read; `user = {id}` (`callback:55`) | Partial: no profile hydration, no refresh scheduling |

---

## 4. Mismatch & gap register

Severity: **S1** blocks a feature / security, **S2** user-visible defect or silent data loss, **S3** polish.

| ID | Sev | Issue | Evidence |
|---|---|---|---|
| C1 | S1 | 401 discards the parsed body and always throws `body:null`; `code` is hardcoded `unauthorized` | `apps/frontend/src/lib/api/client.ts:80-83` (return happens before `parseResponseBody` at :85) |
| C2 | S1 | Logout is client-only; protected routes never check the session row, so a stolen/stale JWT stays valid until `exp` (3600 s) | `AuthState.svelte.ts:104-122`; `extractors/auth.rs:170-239`; `config/app.rs:49` |
| C3 | S1 | No token refresh; `expires_in` ignored; after 401 the app is fully blocked, then force-logged out after 5 min | `client.ts:80-83`; `AuthState.svelte.ts:124-135`; `routes/+layout.svelte:25-42` |
| C4 | S1 | Messaging DTOs disagree (`sender_id`→`authorId`, `thread_id`→`threadId`, no attachments); `authorId.at(0)` throws at render | `channels.rs:82-99` vs `models.ts:27-36`; `ChatLayout.svelte:71,77-79,101` |
| C5 | S1 | Chat channel IDs are hardcoded non-UUIDs (`default-channel`, `dm-channel-1`) → `Path<Uuid>` rejection 400 before the handler | `ChatLayout.svelte:13`; `dms/+page.svelte:8`; `channels/+page.svelte:7`; `channels.rs:198` |
| C6 | S1 | WS endpoint/path/auth/protocol mismatch: FE `/api/v1/ws?token=`; BE `/ws` + `Authorization` header; server never broadcasts | `websocket.ts:23-27`; `routes/mod.rs:47`; `websocket.rs:100-141,159-171` |
| C7 | S1 | `POST /api/v1/files/upload` does not exist; backend is `POST /api/v1/files` with JSON metadata, FE sends multipart | `FileUpload.svelte:25-29`; `files.rs:22-31,53-55` |
| C8 | S1 | `GET /api/v1/files/download/{id}` does not exist; FilePreview links it without auth headers | `FilePreview.svelte:6-8`; `files.rs:53-55` |
| C9 | S1 | `POST /api/v1/users/invite` does not exist (no users router) | `UserManager.svelte:18-21`; route list `routes/mod.rs:29-45` |
| C10 | S2 | `POST /organizations` from Settings omits required `slug` → axum 422 text/plain; UI shows nothing | `OrganizationManager.svelte:17-20`; `org.rs:28-35`; only `successMessage` state |
| C11 | S2 | `GET /tasks` is assignee-scoped and ignores the selected workspace; a task created unassigned never appears; 50-row limit applied **before** the org filter | `tasks.rs:113-128`; `task_repository.rs:117-137`; `tasks/+page.svelte:23,50` |
| C12 | S2 | `threadId` (FE) vs `thread_id` (BE) → serde ignores unknown field; even correctly named `thread_id` is dropped on REST create | `ThreadReply.svelte:18`; `channels.rs:50-55,265-267`; no `deny_unknown_fields` in repo |
| C13 | S2 | `fileIds` (FE) is silently ignored on message create; message/attachment relation does not exist in the model | `ChatLayout.svelte:45-48`; `channels.rs:50-55`; `models/message.rs:14-30` |
| C14 | S2 | Telemetry endpoint does not exist; `sendBeacon` uses a frontend-relative URL and cannot send auth | `logger.ts:106-109,40` |
| C15 | S2 | Settings/chat/invite/file failures only reach `logger` (which itself posts to a 404) — users see success or nothing | `OrganizationManager.svelte:23-25`; `WorkspaceCreate.svelte:21-23`; `UserManager.svelte:24-26`; `ChatLayout.svelte:53-55` |
| C16 | S2 | PATCH task with an unknown status returns 200 and keeps the old status (no error) | `tasks.rs:212-216` |
| C17 | S2 | Unknown channel `type` is silently coerced to Public | `channels.rs:170-176` |
| C18 | S2 | `create_task`/`upload_file` never verify `workspace_id` belongs to the caller's org (cross-tenant reference) | `tasks.rs:152-156`; `files.rs:77-87` |
| C19 | S2 | Message edit/delete ignore the `channel_id` path segment; any owned message in the org can be addressed through any channel URL | `channels.rs:277,317` |
| C20 | S2 | WS fallback returns a random UUID as `message_id` when persistence fails → client is told a message was saved | `websocket.rs:230-236` |
| C21 | S2 | Error shape is not uniform: domain JSON vs extractor text/plain vs empty 429; 403 message is always generic | `error.rs:288-373,295-298`; `extractors/auth.rs:175`; `rate_limit.rs:183-185` |
| C22 | S2 | `before` pagination param is accepted and ignored in message listing | `channels.rs:29-34,216-223` |
| C23 | S2 | No `/auth/me` or user DTO; shell hardcodes `User Name` / `user@flora.app` | `(app)/+layout.svelte:157-160` |
| C24 | S2 | `next_step` is always `select_or_create_organization`, so users with one org always detour through `/select-org` | `auth.rs:259-264`; `callback:56-60` |
| C25 | S2 | Rate limiter path checks never match (`/auth/login` vs actual `/api/v1/auth/login`), so login/refresh/invite-specific limits are dead; 100 req/min per IP is shared by WS retries + log flush | `rate_limit.rs:161-172`; `routes/mod.rs:49-54` |
| C26 | S3 | `ApiClient` casts non-JSON 200 bodies and 204s to `T` (`{}`), so callers can receive a string/empty object instead of an error | `client.ts:97-101` |
| C27 | S3 | `size_bytes: i64` → `u64` with `unwrap_or(0)` hides negative values as 0 | `files.rs:95,125` |
| C28 | S3 | SSE/no `+layout.server.ts`; all protected rendering is client-side, so the shell flashes before the guard redirects | `(app)/+layout.svelte:30-52` |
| C29 | S3 | `sendBeacon`/log flush has no retry and drops entries on failure; a 401 from it would globally lock the session if the endpoint existed | `logger.ts:99-115`; `client.ts:80-83` |
| C30 | S3 | `.env.example` OIDC redirect is `http://localhost:3000/auth/callback` (missing `/api/v1`), unused/misleading | `.env.example:27`; `auth.rs:102` |

---

## 5. Detailed notes on the highest-risk findings

### 5.1 401 body loss (verified)

`ApiClient.request` checks `status === 401` **before** calling `parseResponseBody`:

```ts
// apps/frontend/src/lib/api/client.ts:78-85
const response = await fetch(url, { ...options, headers });
if (response.status === 401) {
    authState.lockSession();
    throw new ApiError(401, response.statusText, url, 'Unauthorized', null, 'unauthorized');
}
const body = await parseResponseBody(response);
```

Consequences:

- Backend `SessionExpired` (`{"error":"session_expired","message":"Session has expired"}`,
  `error.rs:303-306`) and `InvalidCredentials` are indistinguishable from a missing header.
- `ApiError.code` is always `"unauthorized"`, not the backend code; `ApiError.body` is always `null`.
- The 401 branch also runs for `/api/v1/auth/callback` and `/api/v1/auth/login`, so a failed login
  triggers the global "Session Expired" overlay (`routes/+layout.svelte:32-42`) even though the user
  was never authenticated.
- Spec 011 FR-002 says "preserva status e corpo de erro (JSON/texto)" but the test only asserts
  `status/message/lock` (`api-interceptor.test.ts:13-30`), so CI cannot catch it.

### 5.2 Session authority

- `AuthState.logout()` clears `localStorage` only (`AuthState.svelte.ts:104-122`); no `POST /auth/logout`.
- `AuthExtractor` decodes and validates the JWT but never looks up `jti` (`extractors/auth.rs:170-239`).
  Only `refresh` (`auth.rs:292-301`) and `select-organization` (`auth.rs:340-346`) check `is_active`.
- Result: a token copied from `localStorage` remains usable for up to `session_ttl_secs` (default 3600,
  `config/app.rs:23-49`) after logout, revocation or user removal. The WS path has the same property.
- 011 explicitly deferred DB-authoritative session checks; the low-effort half (call logout + make the
  client clear state) is still missing and is cheap.

### 5.3 Messaging (out of 011 scope, visible in nav)

- `GET /channels/default-channel/messages` never reaches `list_messages`: `Path<Uuid>` rejects
  (`channels.rs:198`) with 400 text/plain, logged only.
- Even with a real UUID, the response keys are `sender_id`/`thread_id`; the UI reads `authorId`
  (`ChatLayout.svelte:71,77-79,121`) → `undefined.at(0)` TypeError during render.
- `Message` has no attachment relation (`flora-core/src/models/message.rs:14-30`), so
  `fileAttachments` (`ChatLayout.svelte:101`) is always undefined.
- `ThreadReply` sends `threadId`, ignored by serde; the handler also drops a correctly named
  `thread_id` (`channels.rs:265-267`). Replies always become top-level messages.
- No channel list endpoint exists, so the sidebar "Recent Channels" is hardcoded
  (`(app)/+layout.svelte:124-137`).

### 5.4 WebSocket

- FE builds `new URL('/api/v1/ws', API_BASE_URL)` (`websocket.ts:23`); BE serves `/ws`
  (`routes/mod.rs:47`) → 404/close → reconnect every 3 s forever (`websocket.ts:63-72`).
- FE puts the token in the query string (`websocket.ts:25`); BE reads only `Authorization`
  (`websocket.rs:123-141`) → even on the right path it returns 401 with an empty body.
- Protocol mismatch: server frames are `{"type":"NewMessage","payload":{...}}` (`websocket.rs:23-94`);
  client `onMessage` expects a flat `{type,id}` (`websocket.ts:89-96`). `workspace.user.removed` is
  never emitted by the backend.
- No subscription registry/broadcast exists: `Subscribe` only echoes an ack (`websocket.rs:159-171`);
  Valkey publish happens on send but nothing consumes it into socket frames.
- `GET /api/v1/messages/sync` does not exist, so REST catch-up is always a 404.

### 5.5 Files

| Frontend | Backend |
|---|---|
| `POST /api/v1/files/upload`, `FormData` | `POST /api/v1/files`, JSON `{workspace_id,name,file_type,size_bytes}` (`files.rs:22-31,53`) |
| expects `{fileName,mimeType,byteSize,rustFsUrl,uploadedAt}` | returns `{name,file_type,size_bytes,created_at,…}` (`files.rs:34-48`) |
| `GET /api/v1/files/download/{id}` | `GET /api/v1/files/{id}` (JSON metadata only) |
| client cap 50 MB (`utils/files.ts:1-4`) | server cap `max_upload_bytes` = 100 MB (`config/app.rs:29,52`) |

The backend stores metadata only; binary storage is not wired to these routes.

### 5.6 Error semantics / silent coercion

- Axum `Path<Uuid>` / `Json<T>` rejections bypass the domain envelope; the UI only ever shows
  `ApiError.message`, which becomes the raw rejection string.
- `require_org()` → `Error::Validation` → **422** for a missing org, although
  `Error::OrganizationContextRequired` → 400 exists and is semantically right.
- 403 loses the reason (`error.rs:295-298`): "Not a member", "cross-org access forbidden" and
  "only the file owner can delete it" all render as "Insufficient permissions".
- Unknown JSON fields are accepted (no `deny_unknown_fields`), which is what silently swallows
  `fileIds`/`threadId`; a typo in any request field is indistinguishable from success.
- `PATCH /tasks/{id}` invalid status → 200 with old status; `POST /channels` unknown type → Public.
- 200 non-JSON → `body as T` (`client.ts:101`), no shape validation anywhere.

---

## 6. Top 10 ranked issues + deterministic tests

Ranking = user impact, then security, then effort (S ≤ 0.5 day, M 1–2 days, L > 2 days).

| # | Issue | Area | Impact | Security | Effort | Confidence | Evidence |
|---|---|---|---|---|---|---|---|
| 1 | Logout/revocation not authoritative | BE+FE | High | **High** | M | High | C2 |
| 2 | 401 drops body; no refresh; lock scope wrong | FE | High | Medium | S | High | C1, C3 |
| 3 | Messaging DTO/ID contract broken | FE+BE | High | Low | M | High | C4, C5, C12, C13 |
| 4 | Task list ignores workspace + pre-filter limit | BE+FE | High | Low | S | High | C11 |
| 5 | Settings actions broken/silent (org slug 422, invite 404) | FE+BE | Medium | Low | S | High | C9, C10, C15 |
| 6 | WS endpoint/auth/protocol + reconnect storm | FE+BE | Medium | Medium | L | High | C6, C20, C25 |
| 7 | Files upload/download contract | FE+BE | Medium | Low | L | High | C7, C8 |
| 8 | Error envelope/status/coercion inconsistency | BE+FE | Medium | Low | M | High | C16, C17, C21, C26 |
| 9 | Tenancy/ownership checks (workspace, channel path) | BE | Low | **High** | M | High | C18, C19 |
| 10 | Contract/e2e test signal | Tests | Medium | Low | S | High | §7 |

### T1 — Logout revokes access (DB-gated, expected to **fail today**)

Add to `flora-tests/src/contract/auth.rs` next to `db_select_organization_*` (`#[ignore]`, db-tests job):
seed user+org+session via existing `seed_session` (`:472`), mint the JWT, call `POST /auth/logout`,
then `GET /api/v1/tasks` with the same Bearer token and assert `401`. Today the second call returns 200.
Also assert `POST /auth/refresh` returns 401 `session_expired` (guards the existing DB check).

### T2 — 401 preserves body and does not lock on auth endpoints (pure Vitest, CI `server` project)

Extend `apps/frontend/tests/integration/api-interceptor.test.ts`:
mock `fetch` → 401 + `{"error":"session_expired","message":"Session has expired"}`; assert
`error.body.error === 'session_expired'`, `error.code === 'session_expired'`,
`error.message === 'Session has expired'`, `authState.token` unchanged.
Second case: same 401 for `/api/v1/auth/callback` must **not** set `isSessionLocked`
(define an explicit auth-path allowlist). Deterministic, no network.

### T3 — Messaging mapping is pinned (pure TS + Rust unit)

- Frontend: add `tests/unit/message-dto.test.ts` importing a new `toMessage(dto)` mapper and asserting
  `sender_id → authorId`, `thread_id → threadId`, `file_attachments ?? [] → fileAttachments`, and that
  `authorId` is defined for a fixture (guards the `undefined.at(0)` crash).
- Rust: add `#[test]` in `flora-api/src/routes/channels.rs` serializing `MessageResponse::from(Message)`
  and asserting the exact key set (`sender_id`, `thread_id`, no `author_id`). This makes any future
  rename a compile/test failure instead of a runtime TypeError.

### T4 — Tasks are workspace-scoped (DB-gated) + response field pin

Seed two workspaces in one org and one assigned task in each; `GET /api/v1/tasks?workspace_id=<B>` must
return only B's task. Add a no-DB unit test asserting `TaskResponse` serialization includes `updated_at`
and keeps status casing `"Todo"|"InProgress"|"Done"` (or switch FE to that casing in the same change).

### T5 — Settings failures are surfaced (FE unit + Rust contract)

- FE: extract `createOrganization(client, input)`/`inviteUser(client, input)` into a small module and
  assert it rejects with the `ApiError` (status 422/404) and that the component exposes an error state;
  browser component test is available via the existing `client` vitest project.
- Rust: `POST /api/v1/organizations` with `{"name":"Acme"}` → assert **422 JSON**
  `{error:"validation_error", field:"slug"}` (today it is axum text/plain), so the FE can render a field
  error. Add `POST /api/v1/users/invite` contract test once the route exists (or delete the form).

### T6 — WebSocket connect contract (pure Vitest)

Extract `buildWsUrl(base)` and `buildWsOptions(token)` from `WsClient` and assert:
path is `/ws`, scheme `ws(s)`, URL contains **no** `token` query, options include
`Authorization: Bearer <token>`; plus assert `scheduleReconnect` uses capped backoff (fake timers).
This fails against today's `websocket.ts:23-27`.

### T7 — Files contract (Rust router test, no DB for path resolution)

Assert `POST /api/v1/files/upload` is **not** 404 and returns the agreed shape (`id`, `name`,
`file_type`, `size_bytes`) after the route/multipart decision; assert `GET /api/v1/files/{uuid}/download`
resolves. Then a FE unit test asserts `toFileAttachment(dto)` maps `name/file_type/size_bytes` to
`fileName/mimeType/byteSize` and that the download URL is built from `API_BASE_URL` (today it is relative
and unauthenticated).

### T8 — Error envelope is uniform (Rust + Vitest table test)

- Rust: missing `Authorization` on `GET /api/v1/tasks` must return `401` with JSON
  `{error:"unauthorized"}` (change `AuthExtractor::Rejection` from `(StatusCode,String)` to
  `flora_core::error::Error`); non-UUID channel path must return the JSON envelope, not text.
- FE: table-driven test over `[401 text, 422 json+field, 403 generic, 404 empty, 429 empty]` asserting
  `ApiError.status/code/field/message/body` for each; 403 must not fabricate a message that hides the
  backend text once the backend stops discarding it.

### T9 — Tenancy checks (DB-gated)

Token for org B (member), `workspace_id` owned by org A: `POST /api/v1/tasks` →
`403/422` (today 201). `POST /api/v1/files` with the same workspace → same. Then
`PATCH /api/v1/channels/<A>/messages/<msg-in-B>` → 404/403 (today 200). All reuse existing seed helpers.

### T10 — Test signal (config + DTO snapshots)

- Fix `apps/frontend/playwright.config.ts:5` `testMatch` to include `tests/e2e/**/*.spec.ts` (today
  `**/*.e2e.{ts,js}` matches only `src/routes/demo/playwright/page.svelte.e2e.ts`), then assert
  `npx playwright test --list` reports the 3 checked-in specs. Update the stale selectors in
  `tests/e2e/tasks.spec.ts:10-28` (page says `Task title`, not `New Task Title`) in the same change.
- Replace the `Ok(())` stubs in `flora-tests/src/contract/{tasks,workspace,files,messaging}.rs` with
  key-set snapshot tests (`serde_json::to_value` of each response DTO == checked-in JSON fixture), which
  run without DB and catch FE↔BE key drift.

---

## 7. Other findings (not ranked)

| Finding | Evidence | Note |
|---|---|---|
| Telemetry POST 404, `sendBeacon` wrong origin/no auth | `logger.ts:40,106-109` | Observability is a no-op; a future 401 here would lock the session |
| Login rate limit dead (path prefix), 429 empty | `rate_limit.rs:161-185` | `/auth/login` never matches `/api/v1/auth/login`; all traffic shares 100/min per IP |
| WS reconnect consumes rate limit | `websocket.ts:63-72` | 20 attempts/min per tab against the same IP bucket |
| CORS `allow_origin(Any)` + `allow_headers(Any)` | `middleware/cors.rs:15-21` | Acceptable for bearer-only dev; must be pinned for prod |
| `Id_token` signature not verified | `auth.rs:201-203,398-414` | Documented TODO; OIDC trust relies on TLS |
| `slug` derived client-side; empty for non-ASCII names | `select-org:24-36` | Create button stays disabled with no explanation |
| `GET /organizations` returns settings and defaults role to "Member" | `org.rs:100-115` | Masks a missing role row |
| Duplicate health routes | `health.rs:28-30` vs `routes/mod.rs:23-25` | Dead router, no functional impact |
| Dead UI: `KanbanBoard`, `TaskFilter`, `TaskStore` | no imports found | These encode the wrong task status casing |
| `models.ts` camelCase types are only used by stores/tests | `MessageStore.svelte.ts`, `TaskStore.svelte.ts` | Root cause of the chat mismatch; remove or map explicitly |
| LocalStorage token, no expiry check at boot | `AuthState.svelte.ts:8,49,52-65` | Token is XSS-readable; `isAuthenticated` trusts token presence |
| Client 50 MB vs server 100 MB | `utils/files.ts:1-4`; `config/app.rs:52` | Align constant or expose server limit |
| `organization_id()` falls back to nil on parse error | `utils/jwt.rs:64-68` | Silent coercion; claim is generated internally, low risk |
| `next_step` always select-org | `auth.rs:259-264` | Extra step for single-org users; also means localStorage org can be stale |

---

## 8. Deliberately out of scope for the minimal slice

These are recorded so cycle 012 does not silently expand; each maps to an explicit 011 non-goal or to a
prerequisite that is larger than the slice.

1. **Realtime delivery semantics** (subscription registry, Valkey→socket fan-out, resume from last ID,
   outbox/at-least-once). The slice only needs a correct *connect contract* (T6) and honest UI state.
2. **Real file storage / RustFS** (multipart streaming, quotas, signed URLs, virus scanning, download
   authorization). The slice can align routes/DTOs and hide the feature, not implement storage.
3. **Messaging UI beyond compile-safety** (attachments, threads, channel CRUD/list). Fixing DTO mapping
   is in scope; making `/channels` a usable product surface is not.
4. **Search, notifications, RBAC UI** — endpoints exist, frontend does not call them.
5. **Per-request DB-authoritative session validation** — 011 deferred it. This audit recommends
   promoting at least logout-call + revocation test (T1), but the full middleware rollout remains a
   separate scoped change (needs perf consideration, nil-org pre-selection flow, WS parity).
6. **OIDC JWKS signature verification, nonce, real provider e2e** — `auth.rs:201-203` TODO; NV in CI.
7. **Token refresh UX / multi-tab session sync** — requires refresh scheduling and cross-tab storage
   events; not needed to close the contract gaps.
8. **Error taxonomy redesign** (problem+json, i18n, per-field arrays) — slice only needs one envelope
   plus non-text extractor rejections.
9. **UI polish/redesign** (session overlay semantics, empty states, skeletons).
10. **Load/perf work** (pagination, indexes, N+1 in `list_user_orgs`) — noted, not scoped.

---

## 9. Verification notes / NV

- All line references are from `dev@598f08152`; no file was modified during the audit.
- No API was run: DB-backed claims (T1, T4, T9) are derived from repository SQL and handler flow, and
  should be confirmed when the db-tests job exists. Everything else is static and directly checkable.
- Axum nesting and rejection behavior were verified against the vendored `axum-0.8.9` source; the
  trailing-slash hypothesis was rejected on that basis.
- `cargo test -p flora-tests` currently proves almost nothing for these routes: only
  `contract/auth.rs` has real assertions; the rest are stubs (`grep -c "Ok(())"` per file).

### Useful re-check commands

```bash
grep -rn "ApiClient.request\|fetch(\|new WebSocket\|sendBeacon" apps/frontend/src
grep -rn "\.route(" flora-api/src/routes/*.rs
nl -ba apps/frontend/src/lib/api/client.ts | sed -n '76,102p'
nl -ba flora-api/src/routes/channels.rs | sed -n '80,136p'
grep -rn "deny_unknown_fields" flora-api/src flora-core/src   # empty => unknown fields ignored
grep -rn "find_by_jti" flora-api/src                          # only auth.rs => no per-request check
```
