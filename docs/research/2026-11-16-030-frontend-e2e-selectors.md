# 030 — Selector and flow audit for the canonical Svelte 5 frontend e2e smoke

**Date:** 2026-11-16
**Repo:** `flora` (archived portfolio repo), workspace root `/home/luis/development/MDMV/projetos/Arquivados/flora`
**Scope:** read-only audit of `apps/frontend` — stable/semantic selectors and the exact user flow for ONE browser smoke: `setup → select-org → workspace/channel → message → task`. Also classifies what is mounted/functional vs decorative, what is assertable today, and where `data-testid` is needed.
**Method:** direct file reads with line citations (no source, config, manifest, seed, workflow, or test file modified), plus primary-source cross-check of the Playwright docs (Best Practices, Locators, Assertions) fetched 2026-11-16. Nothing was executed: no build, no Playwright run, no browser session, no DB/API start — see NVs.
**Relationship to prior work:** cycles 026–029 settled *auth bootstrap* (feature-gated `flora-mint-session`, Playwright setup project, `storageState`, `retries: 0`, `workers: 1`, Postgres 18 service, non-required CI job). This document deliberately does **not** re-derive that; it answers the adjacent question: **what exactly do we click, fill and assert once the browser is up?** It assumes the 029 Stage-B setup is in place.

---

## 0. Verdict

**The whole five-step flow is implementable with today's DOM — but only with qualified selectors, and only if the harness (backend + seeded DB + injected auth) exists. There are zero `data-testid` attributes in the frontend today.**

| Step | Mounted & functional? | Selector quality today | Needs test-id? |
|---|---|---|---|
| Setup (harness, not UI) | Yes — but outside the browser | n/a (config/`storageState`) | No |
| select-org | Yes — full REST flow | Good (`heading`, `label`, row-scoped button) | **Recommended** (repeated "Select") |
| workspace / channel | Yes — full REST flow | Good (`label`, native `<select>`, button text `# name`) | Optional |
| message | Yes — REST send + WS receive | Weak (bare `<div>`s, no roles) | **Yes** for robust message assertions |
| task | Yes — full REST flow | Good (`label`, `heading`, button text) | Optional (cheap to add) |

**Hard blockers before a browser assertion can pass:** (1) Playwright currently collects only `**/*.e2e.{ts,js}` (§2.1); (2) the frontend preview starts no backend, and every step calls `http://localhost:3000` (§2.2); (3) no auth is injected, so `enforceAuth` would redirect to `/` (§2.3).

---

## 1. Baseline (this checkout)

- `apps/frontend/package.json`: Svelte `^5.56.1`, `@playwright/test ^1.60.0` (`:23`), `test:e2e = playwright install && playwright test` (`:17`).
- `apps/frontend/playwright.config.ts:1-6`: 6 lines. `webServer: { command: 'npm run build && npm run preview', port: 4173 }` (`:4`), `testMatch: '**/*.e2e.{ts,js}'` (`:5`). No `testDir`, `baseURL`, `use`, `projects`, or `storageState`.
- `vite.config.ts:19-47`: Vitest `client` project includes only `src/**/*.svelte.{test,spec}.{js,ts}` (`:31`); `server` project excludes `tests/e2e/**` (`:42`).
- Work tree clean at audit time (`git status --short` empty).

---

## 2. Preconditions (must be true before step 1 of the smoke)

### 2.1 Test discovery mismatch — the existing e2e suite is dead

`testMatch: '**/*.e2e.{ts,js}'` (`playwright.config.ts:5`) collects **only** `src/routes/demo/playwright/page.svelte.e2e.ts:1-6` (asserts an `h1`). Both files under `tests/e2e/` are `.spec.ts` and are collected by **nothing**:

- Vitest `server` explicitly excludes `tests/e2e/**` (`vite.config.ts:42`).
- Vitest `client` only includes `src/**/*.svelte.{test,spec}.{js,ts}` (`:31`), so `tests/**/*.spec.ts` is out.

Consequences:
1. `tests/e2e/tasks.spec.ts:1-37` and `tests/e2e/files.spec.ts:1-44` never run. Do not "fix" them by broadening `testMatch` to `.spec.ts` — they target a UI that no longer exists (§5).
2. The new smoke must be named `*.e2e.ts` and live under the collected root, or the 029 config (`testDir: './tests/e2e'`, `testMatch: '**/*.e2e.ts'`) must be applied first. Per 029 §4 step 4, both changes are part of the planned config edit.

### 2.2 No backend in the current webServer

`playwright.config.ts:4` starts only `npm run build && npm run preview` on port 4173. Every REST call in the flow goes through `ApiClient.request` → `API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'` (`src/lib/api/client.ts:3`). With no API process, `/select-org` fails at `loadOrganizations` (`select-org/+page.svelte:42`) and renders the error banner (`:112-118`) instead of the org list.

All required endpoints **are** mounted in `flora-api/src/routes/mod.rs:19-38`:
- `GET /api/v1/organizations` (`org.rs`), `POST /api/v1/auth/select-organization` (`auth.rs`)
- `GET /api/v1/workspaces` (`workspace.rs:84-91`, handler `:95`)
- `GET/POST /api/v1/channels` and `GET/POST /api/v1/channels/{id}/messages` (`channels.rs:163-172`, handlers `:174`, `:215`, `:274`, `:328`)
- `GET/POST /api/v1/tasks` (`tasks.rs:106-110`, handlers `:131`, `:170`)

`GET /api/v1/tasks` is hard-wired assignee-scoped to the authenticated user (`tasks.rs:139,149-157`), not a query parameter. With seeded Alice (`2222…`), the three seed tasks (assigned to Bob/Carol — `dev-seed.sql:268-294`) are **not** in her list; a task created with "Assign to me" checked will be. That makes the task assertion deterministic.

> The `X-Organization-ID` header mentioned in route comments (`mod.rs:30`, `health.rs:69-76`) is documentation drift: no Rust handler or the frontend client reads it (repo-wide grep returns only those comment lines). Org scope comes from the JWT `organization_id` claim via `AuthExtractor` — cf. 027 §1 and 029 §1.5.

### 2.3 Auth bootstrap is a harness step, not a UI step

`AuthState` reads `localStorage['flora.auth.v1']` synchronously in its constructor (`src/lib/state/AuthState.svelte.ts:8,14-31,43-50`), and `enforceAuth` in the `(app)` layout redirects to `/` when there is no token and to `/select-org` when there is no org (`src/routes/(app)/+layout.svelte:33-43`). Therefore the token must exist **before app JS runs** (Playwright `storageState` preferred; `addInitScript` fallback) on the **frontend** origin (`http://127.0.0.1:4173`), with shape:

```json
{"token":"<minted-jwt>","user":{"id":"22222222-2222-2222-2222-222222222222"},
 "organizationId":null,"workspaceId":null}
```

`organizationId: null` is required to actually exercise select-org; a non-null value makes `select-org/+page.svelte:91-101` redirect straight to `/tasks`. See 027 §3–§4 and 029 §1.6/§4 for minting, jti/session coupling, the `retries: 0` rotation hazard, and the secret-alignment guard.

**"Setup" in this smoke therefore means the Playwright setup project (seed DB + mint JWT + write storageState).** The visible *First Seed Journey* UI is not automatable hermetically — see §3.1.

---

## 3. Mounted & functional vs decorative

### 3.1 Login / setup screen — `src/routes/(auth)/+page.svelte`

| Element | Lines | Status |
|---|---|---|
| "Start First Seed Journey" → `isSetup` toggle | `:15,38-40,91-93` | Functional *toggle* (client state only). |
| "First Seed Journey" form: Workspace Name / Admin Email | `:45-65` | **Decorative.** Values bound (`:53-54,57-63`) but `handleLogin` (`:19-36`) never reads them. |
| "Sprout Workspace" / "Sign In" | `:67,87` | **Decorative / not hermetic.** Both call `handleLogin` → fetch `${API_BASE_URL}/api/v1/auth/login` then `location.assign(data.authorization_url)` (`:21-32`): a real external OIDC redirect. |
| Email / Password inputs | `:79,83` | **Decorative.** No `bind:value`, no handler. |
| Real session creation | `(auth)/callback/+page.svelte:8-65` | Functional but IdP-dependent; `next_step === 'select_or_create_organization'` → `/select-org` (`:56-60`). Do not drive in e2e. |

→ The smoke must **inject** auth (§2.3), not click through this screen.

### 3.2 App shell — `src/routes/(app)/+layout.svelte`

| Element | Lines | Status |
|---|---|---|
| Auth guard + WS lifecycle | `:33-60`, `:62-83` | Functional. Invisible to selectors but controls redirects and reconnects. |
| Sidebar nav `Channels` / `DMs` / `Tasks` | `:123-146` (`href` at `:126,134,142`) | Functional links. `Button href` renders `<a>` (`ui/button/button.svelte:65-77`) → `getByRole('link', …)`. |
| Sidebar collapse button | `:105-116` | Functional (local state); no accessible name — icon-only. |
| Sidebar "Channels" section text | `:150-161` (text `:157-159`) | **Decorative placeholder** ("Open Channels to browse workspace channels."). Not a channel list. |
| User identity "User Name" / "user@flora.app" | `:181-182` | **Decorative** — hardcoded, not the real `authState.user`. |
| Settings link / Log out | `:193-207` | Functional. Dropdown must be opened first. |
| Header search input | `:220-227` | **Decorative** — no `bind`, no handler. |
| AI panel toggle + panel | `:231-243`, `:254-286` | Toggle functional; panel content and "Ask AI…" input (`:282`) **decorative**. |
| `<main>` renders route children | `:250-252` | The smoke's content area. |

### 3.3 Route pages and components

| Surface | Lines | Status |
|---|---|---|
| select-org page (list/select/create) | `select-org/+page.svelte:38-101,104-162` | Fully functional. |
| channels page (workspaces, channels, create) | `channels/+page.svelte:24-106,111-204` | Fully functional. |
| ChatLayout (load/send messages, threads) | `chat/ChatLayout.svelte:32-94,97-230` | Fully functional; REST send is the assertion anchor, WS is best-effort (§7 NV-3). |
| ThreadReply | `chat/thread/ThreadReply.svelte`, mounted at `ChatLayout.svelte:181` | Mounted but hover-only (`opacity-0 group-hover:opacity-100`, `:180`) — out of scope. |
| tasks page (workspaces, create, list) | `tasks/+page.svelte:37-110,113-198` | Fully functional. |
| DMs page | `dms/+page.svelte:1-9` | Mounts `ChatLayout` with hardcoded `channelId="dm-channel-1"` — out of scope; likely API 404. |
| Settings page | `settings/+page.svelte:1-19` | Mounts `OrganizationManager`, `WorkspaceCreate`, `UserManager` — out of scope. |
| **FileUpload / FilePreview** | `files/FileUpload.svelte`, `files/FilePreview.svelte` | **Dead code — not imported by any route** (repo-wide grep of `$lib/components` imports). |
| **KanbanBoard / TaskFilter** | `tasks/KanbanBoard.svelte`, `tasks/TaskFilter.svelte` | **Dead code — not imported anywhere.** The live tasks UI is the inline table at `tasks/+page.svelte:137-197`. |

This is the decisive reason `files.spec.ts` is not just stale but unfixable as written: it waits for `input[type="file"]` on `/channels` (`files.spec.ts:5-8`), and no such element can ever render.

---

## 4. The smoke, step by step — selectors and assertions

All snippets are **proposed research artifacts**, not applied code. Locator choice follows Playwright's documented priority: `getByRole` → `getByLabel` → `getByText` → `getByTestId` → CSS last ([Locators](https://playwright.dev/docs/locators)).

### Step 0 — Setup (outside the browser)

Per 029 §4 stages A/B: setup project seeds `dev-seed.sql` + `scripts/dev-smoke-session.sql`, mints a pre-selection JWT (nil org, jti `018f0000-0000-7000-8000-000000000001`, sub Alice `2222…`), writes `playwright/.auth/alice.json` on origin `http://127.0.0.1:4173`, and guards with `GET /api/v1/organizations` → 200 containing `acme-corp`. `retries: 0`, `workers: 1`.

### Step 1 — select-org

Route `/select-org`. Guard at `select-org/+page.svelte:91-101` requires token + no org (§2.3). `loadOrganizations` (`:38-48`) renders rows at `:128-140`.

| Action | Best selector today | Why / caveat |
|---|---|---|
| Land on page | `page.goto('/select-org')` | `baseURL` must be set (029). |
| Assert page | `expect(page.getByRole('heading', { name: 'Select organization' })).toBeVisible()` | `h1` at `:106`. |
| Wait for data | `expect(page.getByText('acme-corp')).toBeVisible()` | Slug rendered at `:133`; seed slug `acme-corp` (`dev-seed.sql:12-22`). Avoids racing `Loading...` (`:123`). |
| Select org | Scope first, then click: `const row = page.locator('[data-testid="org-row-acme"]'); await row.getByRole('button', { name: 'Select' }).click()` | **Strict-mode hazard:** every org emits an identical `<button>Select</button>` (`:136-138`). Seed has one org today, so a bare `getByRole('button', {name:'Select'})` passes now but breaks when a second org is added. Without a test-id, the resilient fallback is `page.getByText('acme-corp').locator('..').getByRole('button', {name:'Select'})` — DOM-coupled, hence the test-id recommendation in §6. |
| Assert navigation | `await expect(page).toHaveURL(/\/tasks$/)` | `selectOrganization` calls `goto('/tasks')` at `:63`. Auto-retrying `toHaveURL` ([Assertions](https://playwright.dev/docs/test-assertions)). |
| Optional POST check | `page.waitForResponse(r => r.url().includes('/api/v1/auth/select-organization') && r.ok())` | Confirms the real contract; combine with the URL assertion, don't replace it. |

Do **not** click "Create and select" (`:155-160`): it mutates the seed org set and needs a unique slug.

### Step 2 — workspace / channel

Navigate via the shell link (exercises the layout) or `page.goto('/channels')`. Link: `page.getByRole('link', { name: 'Channels' })` from `(app)/+layout.svelte:123-130`; sidebar is open by default (`isSidebarOpen = true`, `:28`), so the visible text "Channels" is present at `:129`.

| Action | Best selector today | Why / caveat |
|---|---|---|
| Assert page | `getByRole('heading', { name: 'Channels' })` | `h1` at `channels/+page.svelte:114`. |
| Assert workspace loaded | `const ws = page.getByLabel('Workspace')` | `<Label for="workspace">` at `:120` + `<select id="workspace">` at `:121-135`. `Input`/`Label` primitives pass `id` through (`ui/input/input.svelte:37-47`); use `toHaveValue` with the UUID from `authState.workspaceId` or `expect(ws.locator('option:checked')).toHaveText('Main Workspace')`. Seed: `Main Workspace` = `8888…`, `Design Workspace` = `9999…` (`dev-seed.sql:192-202`). |
| Wait for channels | `expect(page.getByRole('button', { name: '# general' })).toBeVisible()` | Channel buttons at `:174-185`, text `# {channel.name}` (`:183`). Seed channels: `general`, `random`, `engineering`, `design-general` (`dev-seed.sql:207-229`). Guard text `No channels yet.` at `:171` means "loaded but empty". |
| Select channel | `await page.getByRole('button', { name: '# general' }).click()` | Sets `selectedChannelId` (`:181`). Auto-select of `channels[0]` at `:43-45` may already have chosen it, but an explicit click removes order dependence. |
| Assert selection | `expect(page.getByRole('heading', { name: '# general' })).toBeVisible()` | Header at `:193-197` switches from `No channel selected` (`:195`). Heading role avoids confusion with the same-text button. |
| Optional create channel | `getByLabel('New channel')` (`:150-158`), `getByRole('button', {name:'Create'})` (`:159-164`) | Works, but adds DB mutation; skip for the smoke (use a `crypto.randomUUID()` suffix if added). |

`loadWorkspaces` calls `authState.setWorkspace(...)` (`:30`), so the subsequent tasks page inherits the workspace deterministically.

### Step 3 — message

`ChatLayout` is mounted at `channels/+page.svelte:199` with the selected channel id. Its REST path is the stable assertion anchor: `GET /api/v1/channels/{id}/messages?limit=100` (`:42-44`) → `setMessages(toReversed())` (`:46`) → render at `:117-186`.

| Action | Best selector today | Why / caveat |
|---|---|---|
| Assert chat loaded | `expect(page.getByText('Welcome to Acme Corporation!', { exact: false })).toBeVisible()` | Seeded message content `dev-seed.sql:237-242`; rendered content at `ChatLayout.svelte:146-150` with `[AI]` stripped (`:149`). Confirms GET + mapper + store + render in one assertion. |
| Assert composer ready | `const composer = page.getByPlaceholder('Write a message…')` (`:206-215`) | Placeholder is `Select a channel first` until `channelId` is set (`:208`) and the textarea is `disabled` without a channel (`:215`). `toBeEnabled()` here is a clean proxy for "channel selected". |
| Send | `await composer.fill(unique); await page.getByRole('button', { name: 'Send' }).click()` | Or press Enter unshifted (`onkeydown` at `:210-214`). Button at `:218-226`. `unique` should include `crypto.randomUUID()` to avoid collisions on re-runs. |
| Assert send | `expect(page.getByText(unique, { exact: true })).toBeVisible()` | `sendMessage` POSTs (`:69-75`) then `messageStore.upsertMessage` (`:77`) → re-render. Auto-retrying `toBeVisible` handles the round-trip. |
| Optional network check | `page.waitForResponse(r => r.url().includes('/messages') && r.request().method() === 'POST' && r.ok())` | Stronger evidence that the UI wrote, not just optimistic render. |
| **Avoid** | `page.locator('.group.flex.flex-col…')` or nth-child on the message list | Message rows are anonymous `<div>`s (`:118`) with no role, label or test-id — direct CSS is brittle. §6 recommends test-ids. |

Notes: the sender label is `User {authorId.slice(0,8)}` (`:128-130`), not a human name — don't assert on it. The `[AI]` badge (`:137-143`) and thread replies (`:152-178`) are out of scope. WS reconnect chatter (`websocket.ts:108-205`) may appear in console; it is not required for the REST assertion (§7 NV-3).

### Step 4 — task

Navigate via `page.getByRole('link', { name: 'Tasks' })` from `(app)/+layout.svelte:139-146` (href `:142`). Guards at `tasks/+page.svelte:100-110`.

| Action | Best selector today | Why / caveat |
|---|---|---|
| Assert page | `getByRole('heading', { name: 'Tasks', exact: true })` | `h1` at `:116`. Note an `h2` "Tasks (assignee-scoped in org)" also exists (`:176`) — use `exact` or role+level. |
| Assert workspace | `page.getByLabel('Workspace')` | `<Label for="workspace">` + `<select id="workspace">` at `:141-156`. Should be `Main Workspace` inherited from step 2 (`:39-46`). |
| Fill title | `getByLabel('Title')` → `tasks/+page.svelte:159-160` | `Input id="title"`, label `for="title"` → label locator works. |
| Fill description | `getByLabel('Description (optional)')` → `:163-164` | `Input id="desc"`. |
| Assignee checkbox | `getByRole('checkbox', { name: /Assign to me/ })` → `:166-169` | Wrapped in `<label>` so the accessible name is the full sentence. Checked by default (`shouldAssignToMe = true`, `:26`) — leave it: without it the task won't appear in Alice's assignee-scoped list (`tasks.rs:149-157`). |
| Create | `getByRole('button', { name: 'Create task' })` → `:170-172` | POST at `:79-82`. |
| Assert success | `getByText('Task created: ' + uniqueTitle)` → successMessage at `:83`, rendered `:131-135` | Deterministic, unique per run. |
| Assert list | `getByRole('listitem').filter({ hasText: uniqueTitle })` → `<ul>`/`<li>` at `:182-194` | `filter({hasText})` is the documented way to pick one item from a list ([Locators — Filtering](https://playwright.dev/docs/locators#filtering-locators)). Optional: assert `status · id` text at `:186-188`. |
| Refresh (optional) | `getByRole('button', { name: 'Refresh' })` → `:121` | Re-fetches both lists (`:53-64`); proves persistence, not just optimistic state. |

The stale spec's `New Task Title`, `Assignee ID (optional)`, `Add Task`, status `<select>`, `User ID` filter and `No tasks found.` (`tests/e2e/tasks.spec.ts:10-31`) do **not** exist anywhere in this page.

### Step 4b — Assertions that are *not* safe today

- No `data-testid` anywhere (repo-wide grep of `src` and `tests`: zero hits). `getByTestId(...)` only works after §6 is applied (or `testIdAttribute` is configured to an existing attribute — there is no suitable one).
- No `role`/`aria-label` on message rows, org rows, channel rows, the sidebar collapse button, or the AI toggle. Role locators exist only for real `<button>`/`<a>`/`<h1>`/`<label>`+control pairs.
- `getByRole('button', { name: 'Select' })` is not unique once >1 org exists (§4 step 1).
- Hardcoded identity `User Name` / `user@flora.app` in the shell (`+layout.svelte:181-182`) must not be asserted as the logged-in user.

---

## 5. Dead / misleading e2e specs — disposition

| File | Claims | Reality | Disposition |
|---|---|---|---|
| `tests/e2e/tasks.spec.ts:1-37` | `New Task Title`, `Assignee ID (optional)`, `Add Task`, status select `DONE`, `User ID` filter, `No tasks found.` | Live page uses `Title`/`Description (optional)`/`Create task`, a checkbox, no status control, no filter (`tasks/+page.svelte:141-194`). | Delete or rewrite as the step-4 smoke. Not collected today (§2.1). |
| `tests/e2e/files.spec.ts:1-44` | `input[type=file]` on `/channels`, `.error-message` | `FileUpload` is never mounted (§3.3); `/channels` has no file input; no `.error-message` class. `page.route('/api/v1/files/upload', …)` also uses a relative glob while the client calls an absolute `http://localhost:3000/...` URL (`client.ts:3,75`). | Delete. File upload is a DB-gated contract matter (029 §6.2 explicitly excludes it). |
| `src/routes/demo/playwright/page.svelte.e2e.ts:1-6` | `h1` on `/demo/playwright` | Page is exactly `<h1>Playwright e2e test demo</h1>` (`demo/playwright/+page.svelte:1`). Valid but trivial. | Harmless placeholder; leave or remove once the real smoke lands. |

---

## 6. Test-id gaps — exact, minimal, not applied

Priority order: add only where a role/label locator is ambiguous or the rendered node is semantically anonymous.

| # | File:line | Current markup | Proposed attribute | Enables |
|---|---|---|---|---|
| T1 | `select-org/+page.svelte:129` | `<div class="flex items-center justify-between rounded-md border p-3">` | `data-testid="org-row-{org.id}"` | Row-scoped "Select" click; removes strict-mode hazard. |
| T2 | `select-org/+page.svelte:136` | `<Button disabled={isSubmitting} onclick={…}>Select</Button>` | `data-testid="org-select-{org.id}"` | Direct click if rows stay unlabelled. (T1 **or** T2; T1 preferred.) |
| T3 | `chat/ChatLayout.svelte:118` | `<div class="group flex flex-col px-4 py-2 …">` | `data-testid="message-row-{message.id}"` | Count/scope messages without CSS chains. |
| T4 | `chat/ChatLayout.svelte:146` | `<div class="font-sans text-sm …">{message.content…}</div>` | `data-testid="message-content-{message.id}"` | Exact text assertion, `toHaveText`, independent of layout wrappers. |
| T5 | `chat/ChatLayout.svelte:206` | `<textarea bind:value={newMessage} …>` | `data-testid="message-composer"` | Fallback if the Unicode ellipsis in the placeholder (`Write a message…`, `:208`) proves fragile in tooling. |
| T6 | `channels/+page.svelte:175` | `<button type="button" …># {channel.name}</button>` | `data-testid="channel-{channel.id}"` | Stable channel selection beyond text matching. |
| T7 | `tasks/+page.svelte:184` | `<li class="rounded-md border p-3">` | `data-testid="task-row-{task.id}"` | Deterministic list assertion. |

**Not needed** (already semantic): `getByRole('heading')` for all `h1`/`h2`; `getByLabel('Workspace'|'Title'|'Description (optional)'|'New channel')` via `<label for>`; `getByRole('button', {name})` for Create/Select/Send/Refresh/Create task; `getByRole('link', {name})` for sidebar nav; `getByRole('checkbox', {name:/Assign to me/})`.

Playwright's own guidance: test ids are "the most resilient way of testing as even if your text or role of the attribute changes, the test will still pass", but "testing by test ids is not user facing" — keep user-facing locators where text/role is the contract ([Locators — Locate by test id](https://playwright.dev/docs/locators#locate-by-test-id)). Default attribute is `data-testid`; a custom one can be set with `use.testIdAttribute` if the project prefers `data-pw`.

---

## 7. Playwright best-practice cross-check (primary docs, fetched 2026-11-16)

| Guidance | Primary source | Application to this smoke |
|---|---|---|
| Test user-visible behavior; avoid CSS classes/implementation detail | [Best Practices — Testing philosophy / Prefer user-facing attributes](https://playwright.dev/docs/best-practices#prefer-user-facing-attributes-to-xpath-or-css-selectors) | Use headings, labels, button text; never `button.buttonIcon…` style selectors. |
| Prioritize `getByRole()`, then label/text/test id; CSS/XPath last | [Locators — Quick Guide](https://playwright.dev/docs/locators#quick-guide), [Locate by role](https://playwright.dev/docs/locators#locate-by-role), [CSS/XPath](https://playwright.dev/docs/locators#locate-by-css-or-xpath) | Locator tables in §4 follow this order. |
| Locators are strict: >1 match throws | [Locators — Strictness](https://playwright.dev/docs/locators#strictness) | Root cause of the "Select" hazard (§4 step 1). `first()`/`nth()` are explicitly discouraged by the docs; fix uniqueness instead. |
| Chain and filter to scope | [Best Practices — Chaining and filtering](https://playwright.dev/docs/best-practices#use-chaining-and-filtering), [Locators — Filtering](https://playwright.dev/docs/locators#filtering-locators) | `getByRole('listitem').filter({hasText: uniqueTitle})`; row-scoped "Select". |
| Web-first, auto-retrying assertions; never `expect(await …isVisible())` | [Best Practices — Web first assertions](https://playwright.dev/docs/best-practices#use-web-first-assertions), [Assertions — Auto-retrying](https://playwright.dev/docs/test-assertions#auto-retrying-assertions) | Every assertion in §4 is `await expect(...).toBeVisible()/toHaveURL()/toHaveValue()`. |
| `toHaveURL` is an auto-retrying page assertion; default expect timeout 5 s | [Assertions](https://playwright.dev/docs/test-assertions) | Use `await expect(page).toHaveURL(/\/tasks$/)` after select-org instead of `waitForTimeout`. |
| `expect.poll` / `toPass` for complex retry; soft assertions for multi-check | [Assertions — expect.poll](https://playwright.dev/docs/test-assertions#expectpoll), [Best Practices — Soft assertions](https://playwright.dev/docs/best-practices#use-soft-assertions) | Useful for the setup project's API drift guard and optional postconditions. |
| Isolation: each test own storage; reuse via setup project | [Best Practices — Make tests as isolated as possible](https://playwright.dev/docs/best-practices#make-tests-as-isolated-as-possible), [Authentication](https://playwright.dev/docs/auth) | Matches 027/029's setup-project + `storageState` design. One smoke = one test, no shared UI state. |
| Avoid third-party dependencies; stub with `page.route` | [Best Practices — Avoid testing third-party dependencies](https://playwright.dev/docs/best-practices#avoid-testing-third-party-dependencies) | Never drive the external OIDC page (§3.1); assert the seeded REST contract instead. |
| Codegen picks role/text/test-id locators automatically | [Best Practices — Generate locators](https://playwright.dev/docs/best-practices#generate-locators) | After the §6 ids land, `npx playwright codegen http://127.0.0.1:4173/select-org` can regenerate the skeleton locators. |
| Trace on first retry; Chromium-only install on CI | [Best Practices — Configure debugging / Run tests on CI](https://playwright.dev/docs/best-practices#configure-debugging) | Consistent with 029 §4 (`trace: 'on-first-retry'`, `npx playwright install chromium --with-deps`). Note 029 sets `retries: 0`, so traces would need an explicit opt-in to be produced. |

---

## 8. Proposed smoke skeleton (illustrative; NOT written to disk)

```ts
// tests/e2e/seed-flow.e2e.ts  (proposed — inherits storageState from the `chromium` project)
import { test, expect } from '@playwright/test';

test('setup → select-org → workspace/channel → message → task', async ({ page }) => {
  // 1. select-org
  await page.goto('/select-org');
  await expect(page.getByRole('heading', { name: 'Select organization' })).toBeVisible();
  await expect(page.getByText('acme-corp')).toBeVisible();          // proves injected auth read
  await page.getByRole('button', { name: 'Select' }).click();        // unique with 1 seed org;
                                                                    // scope via T1/T2 when >1
  await expect(page).toHaveURL(/\/tasks$/);

  // 2. workspace / channel
  await page.getByRole('link', { name: 'Channels' }).click();
  await expect(page.getByRole('heading', { name: 'Channels' })).toBeVisible();
  await expect(page.getByLabel('Workspace')).toHaveValue(/88888888/);
  await page.getByRole('button', { name: '# general' }).click();
  await expect(page.getByRole('heading', { name: '# general' })).toBeVisible();

  // 3. message
  await expect(page.getByText('Welcome to Acme Corporation!', { exact: false })).toBeVisible();
  const content = `e2e smoke ${crypto.randomUUID()}`;
  const composer = page.getByPlaceholder('Write a message…');
  await expect(composer).toBeEnabled();
  await composer.fill(content);
  await page.getByRole('button', { name: 'Send' }).click();
  await expect(page.getByText(content, { exact: true })).toBeVisible();

  // 4. task
  await page.getByRole('link', { name: 'Tasks' }).click();
  const title = `e2e task ${crypto.randomUUID()}`;
  await page.getByLabel('Title').fill(title);
  await page.getByLabel('Description (optional)').fill('created by browser smoke');
  await expect(page.getByRole('checkbox', { name: /Assign to me/ })).toBeChecked();
  await page.getByRole('button', { name: 'Create task' }).click();
  await expect(page.getByText(`Task created: ${title}`)).toBeVisible();
  await expect(page.getByRole('listitem').filter({ hasText: title })).toBeVisible();
});
```

Design decisions embedded above: no `waitForTimeout`; every async state covered by an auto-retrying assertion; unique strings via `crypto.randomUUID()`; no UI mutation of org/channel names; checkbox deliberately left checked for assignee scoping; the "Select" line is the only one that needs T1/T2 to become future-proof.

---

## 9. NVs / open questions

- **NV-1 (H) — Nothing executed.** No build, seed, API, preview or Playwright run. Every selector is derived from source; verify with one live run + `npx playwright codegen`.
- **NV-2 (H) — Backend + seed + JWT bootstrap assumed from 026–029.** This doc does not re-validate the minter, secret precedence, session/jti coupling or CI placement. If 029 Stage A/B is not applied, none of the browser steps pass.
- **NV-3 (M) — WS noise.** `ChatLayout.svelte:48,49` calls `wsClient.connect()` and `setActiveChannel` even when messaging is lean/disabled; `websocket.ts:108-205` retries with backoff up to 8 attempts. Expected to be console noise only, but not runtime-verified (same as 029 NV-10). If it interferes, validate `page.routeWebSocket('**/api/v1/ws', …)` against the installed Playwright 1.60 API before relying on it.
- **NV-4 (M) — Selector uniqueness.** Seed has exactly one org (`acme-corp`, `dev-seed.sql:12-22`), so a bare "Select" passes today. Add T1/T2 if the seed or a test adds orgs.
- **NV-5 (M) — Channel ordering.** `loadChannels` auto-selects `channels[0]` (`channels/+page.svelte:43-45`); the smoke clicks `# general` explicitly, so ordering does not matter, but the API's ordering is not asserted.
- **NV-6 (M) — Label locators.** `getByLabel` depends on `Label for=` + `Input id=` pass-through (`ui/input/input.svelte:37-47`). Confirm in the rendered DOM; if the Svelte primitive drops `id` in some build, fall back to `page.locator('#title')` (a stable id contract, still better than CSS chains).
- **NV-7 (L) — Unicode placeholder.** `Write a message…` uses an ellipsis character (`ChatLayout.svelte:208`); copy/paste in source must preserve it. T5 removes this fragility.
- **NV-8 (L) — `data-slot="button"`** is emitted by the Button primitive (`ui/button/button.svelte:68,81`) but is a library implementation detail; do not build selectors on it (Best Practices: avoid implementation details).
- **NV-9 (L) — `files.spec.ts` route glob** uses a relative URL; even if FileUpload were mounted, `page.route('/api/v1/files/upload', …)` may not match the absolute `API_BASE_URL` request. Moot while the component is dead.

---

## 10. Sources

**Local (read-only, this checkout):**
- `apps/frontend/playwright.config.ts:1-6`; `apps/frontend/package.json:17,23,35`; `apps/frontend/vite.config.ts:19-47`; `apps/frontend/.gitignore:24-25`.
- `apps/frontend/src/routes/+layout.svelte:1-76`; `(auth)/+page.svelte:15-95`; `(auth)/callback/+page.svelte:8-65`; `(app)/+layout.svelte:28-252`; `(app)/select-org/+page.svelte:38-162`; `(app)/channels/+page.svelte:24-199`; `(app)/tasks/+page.svelte:21-198`; `(app)/dms/+page.svelte:1-9`; `(app)/settings/+page.svelte:1-19`; `demo/playwright/+page.svelte:1`.
- `apps/frontend/src/lib/components/chat/ChatLayout.svelte:1-230`; `ui/input/input.svelte:37-47`; `ui/button/button.svelte:65-89`.
- `apps/frontend/src/lib/state/AuthState.svelte.ts:8-143`; `MessageStore.svelte.ts:1-42`; `api/client.ts:3,73-110`; `api/mappers.ts:22-43`; `api/websocket.ts:34-39,108-205,207-215`.
- `apps/frontend/tests/e2e/tasks.spec.ts:1-37`; `tests/e2e/files.spec.ts:1-44`; `src/routes/demo/playwright/page.svelte.e2e.ts:1-6`; `tests/integration/api-interceptor.test.ts:1-95`.
- `flora-api/src/routes/mod.rs:19-38`; `routes/tasks.rs:106-110,131-167`; `routes/channels.rs:163-172`; `routes/workspace.rs:84-95`.
- `dev-seed.sql:12-22,192-202,207-229,234-263,268-294`.
- Prior research: `docs/research/2026-11-16-026-browser-e2e-tauri-decision.md`, `2026-11-16-027-e2e-jwt-bootstrap.md`, `2026-11-16-028-e2e-jwt-implementation-plan.md`, `2026-11-16-029-e2e-slice-validation.md`.

**External (primary, fetched 2026-11-16):**
- Playwright: [Best Practices](https://playwright.dev/docs/best-practices) (testing philosophy, locators, chaining/filtering, web-first assertions, debugging, tooling, CI, lint, soft assertions); [Locators](https://playwright.dev/docs/locators) (quick guide, role/label/text/test-id/CSS, filtering, operators, strictness); [Assertions](https://playwright.dev/docs/test-assertions) (auto-retrying vs non-retrying, `toHaveURL`, `expect.poll`, `toPass`).
- Playwright (via 027/029, not re-fetched here): [Authentication](https://playwright.dev/docs/auth), [Web server](https://playwright.dev/docs/test-webserver), [BrowserContext.addInitScript](https://playwright.dev/docs/api/class-browsercontext#browser-context-add-init-script).
