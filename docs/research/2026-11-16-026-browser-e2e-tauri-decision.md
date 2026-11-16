# 026 — Minimal deterministic browser e2e (workspace → channel → message → task → file) and the Tauri deferral decision

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`0517a3a50`) — **research/design only** (no source, config, or workflow changes).  
**Scope:** A minimal, deterministic Playwright browser smoke for the canonical Svelte 5 frontend (`apps/frontend`) against a local Axum API without a live IdP; test isolation, seeded data, auth/session injection, GitHub Actions cost/stability; when browser e2e pays off versus the existing DB-gated contracts; and an explicit keep-Tauri-out-of-Seed decision with re-entry criteria.  
**Direct demand:** `OBJETIVO.md:93` lists as P2 candidate for cycle 026: *"e2e browser (`workspace→canal→mensagem→tarefa→arquivo`), decisão Tauri canônica, load/OOM/restore em 2vCPU/4GB"*. This document answers the first two items.

**Method / multilingual cross-check.** Primary sources were read first: Playwright docs (EN; the same pages are published for Python/Java/.NET), SvelteKit/Svelte CLI docs (EN) and GitHub Actions docs (EN + directly checked localized **pt-BR** and **ja** service-container pages and **zh-CN** page; EN/ja **Tauri v2 SvelteKit** guide). **es/de/fr** Tauri+SvelteKit evidence is carried from the prior cycle-016 research (`docs/research/2026-11-16-016-tauri-svelte5-packaging.md:6,58-60`); those regional articles were not re-fetched here (NV-10). No secondary blog was used for a load-bearing claim.

---

## 0. Decision summary

| Question | Answer (this pass) |
|---|---|
| Browser e2e worth it for Seed? | **Yes, but one thin smoke — not a suite.** Cover the cross-route wiring that DB-gated contracts cannot see: session bootstrap → workspace select → channel → message → task. Keep it single-browser (Chromium), single-worker on CI, trace-on-first-retry, disposable Postgres 18 service. |
| File step? | **Deferred within the e2e slice.** No file UI is mounted in any canonical route and no file rows are seeded; file upload/download correctness stays with DB-gated Rust contracts (`flora-tests/src/contract/files.rs`). Re-enter when an upload control is actually wired into a route. |
| Tauri in canonical frontend for Seed? | **Deferred — explicit decision below.** Canonical Seed target remains the browser SvelteKit app. Tauri requires a static-SPA build-mode split, a separate Rust crate, per-OS CI/signing, and a non-cookie auth path; it adds no Seed validation value and multiplies CI cost. Re-entry criteria are explicit (§5). |
| Status of the proposed slice? | **Proposed, not implemented or executed.** Everything not present in the checkout is marked NV. |

---

## 1. Flora state — read-only cross-check

| Area | Evidence in this checkout | Implication |
|---|---|---|
| Demand | `OBJETIVO.md:9` (Svelte 5 UI, "Tauri em fases posteriores"), `:93` (P2 e2e + Tauri decision); `ROADMAP.md:4,10` ("Tauri futuro") | Browser e2e and Tauri deferral are both in-scope product intents, not invented tasks. |
| e2e scripts | `apps/frontend/package.json:16-17`: `test` = unit + `test:e2e`; `test:e2e` = `playwright install && playwright test`. `@playwright/test` and `playwright` `^1.60.0` at `:23,35`. | Browsers are installed unpinned-by-OS (`install`, not `install --with-deps`) and **all** browsers are downloaded, contrary to Playwright's CI guidance to install only what is used (§2.1). |
| Playwright config | `apps/frontend/playwright.config.ts:1-6`: only `webServer` (`npm run build && npm run preview`, port 4173) and `testMatch: '**/*.e2e.{ts,js}'`. No `baseURL`, `projects`, `use`, `retries`, `workers`, `trace`, `reporter`, `globalTimeout`, `reuseExistingServer`. | Only `src/routes/demo/playwright/page.svelte.e2e.ts:3-6` (asserts an `h1`) is actually collected. The API on :3000 is not started by Playwright. Config lacks every stability knob Playwright recommends on CI. |
| Orphaned specs | `apps/frontend/tests/e2e/tasks.spec.ts` and `files.spec.ts` are `.spec.ts`, so they **do not match** `testMatch`. Their selectors are also stale: tasks spec uses `New Task Title`, `Assignee ID (optional)`, `Add Task`, `No tasks found.`, while the real page (`src/routes/(app)/tasks/+page.svelte:160,164,170-172,180`) uses `Task title`, `Description`, `Create task`, `No tasks returned for current assignee/org.`. Files spec targets `input[type="file"]` on `/channels`, but `FileUpload.svelte:59` is not imported by any route. | Two dead tests give a false impression of e2e coverage; enabling `testMatch` for `.spec.ts` without fixing them would produce red CI. |
| Unit/browser tests | `apps/frontend/vite.config.ts:28-48`: Vitest `client` project uses `@vitest/browser-playwright` (headless Chromium) for `src/**/*.svelte.{test,spec}.ts`; `server` project for unit tests and explicitly excludes `tests/e2e/**`. CI runs only `--project server` (`.github/workflows/ci.yml:24-25`). | A cheaper browser-component lane already exists but is not gating CI; useful as an incremental signal before full e2e. |
| CI | `.github/workflows/ci.yml:1-26` runs `check`, `lint`, `test:unit -- --run --project server`, `build` (Bun). It never runs `test:e2e`. `.github/workflows/rust.yml:26-71` has a separate `db-tests` job with `postgres:18` service and runs ignored DB contract tests per module (auth/messaging/websocket/search/notifications/**files**). | **DB-gated contracts are already a CI lane.** Browser e2e would be additive, not a replacement; the file contract is already covered there. |
| Auth/session model | `src/lib/state/AuthState.svelte.ts:8,14-31,52-65`: token lives in `localStorage` key `flora.auth.v1` as JSON `{token, user, organizationId, workspaceId}`. `src/lib/api/client.ts:3,73-80`: `API_BASE_URL = import.meta.env.VITE_API_URL \|\| 'http://localhost:3000'` and `Authorization: Bearer <token>`. No cookies. | Session injection for Playwright is `addInitScript`/`storageState` on `localStorage`, **not** a cookie fixture. Build-time `VITE_API_URL` must point at the API the browser can reach. |
| IdP-free bootstrap | `docs/dev-smoke.md:47-88` + `scripts/dev-smoke-session.sql:10-48`: `dev-seed.sql` then a fixed pre-selection session (jti `018f0000-0000-7000-8000-000000000001`), `POST /api/v1/auth/select-organization` rotates it into an org-scoped `session_token`. No live OIDC needed. | The bootstrap needed by e2e already exists as a documented dev path. |
| Expired fixed token | `docs/dev-smoke.md:67,69` pre-signed JWTs carry `exp=1794787200` = **2026-11-16T00:00:00Z** and `iat=1763251200` = 2025-11-16T00:00:00Z (verified with `date -u`). Today is 2026-11-16 13:04Z. | The documented token is expired **now**. CI cannot reuse it; a fresh-JWT mint step (matching `FLORA_APP__JWT_SECRET`) is a hard prerequisite (NV-2). |
| Seed data | `dev-seed.sql:192-300`: org Acme `1111…`, users Alice `2222…`/Bob/Carol, workspaces `8888…` (Main), `9999…` (Design), channels `aaaa…` general / `bbbb…` random / `cccc…` engineering / `dddd…` design, messages, tasks, notifications. `INSERT INTO files` count = **0**. | Workspace/channel/message/task fixtures exist with stable IDs; file fixture does not. |
| Known list semantics | `docs/dev-smoke.md:88`: `GET /api/v1/tasks` is assignee-scoped; a created unassigned task may not appear in the list. `tasks/+page.svelte:26,66-86` mitigates with "Assign to me" (default on). | Assert via the create response / API postcondition, not list-after-create alone. |
| File UI | `src/lib/components/files/FileUpload.svelte:59` exists but has no importer; `apps/frontend/tests/e2e/files.spec.ts` is dead; CORS is `Any` (`flora-api/src/middleware/cors.rs:14-20`), so cross-origin 4173→3000 works today. | File step is genuinely blocked in the UI; CORS is not the blocker (but permissive CORS is a separate hardening topic). |
| Tauri asset | `apps/web/package.json:2` = `@coparse/web` (SolidJS); `apps/web/tauri/src-tauri/tauri.conf.json:3,7` = product `macro`, `frontendDist: ../../dist`. `apps/frontend/svelte.config.js:5-14` = `@sveltejs/adapter-node`. | The existing Tauri shell belongs to the **non-canonical** app and cannot be reused for the Svelte 5 target; the canonical app is SSR-capable `adapter-node`, which Tauri packaging explicitly does not support. |

---

## 2. External primary-source findings

### 2.1 Playwright (primary: [Authentication](https://playwright.dev/docs/auth), [Isolation](https://playwright.dev/docs/browser-contexts), [Web server](https://playwright.dev/docs/test-webserver), [Configuration](https://playwright.dev/docs/test-configuration), [CI](https://playwright.dev/docs/ci), [Best practices](https://playwright.dev/docs/best-practices), [API testing](https://playwright.dev/docs/api-testing), [Global setup/teardown](https://playwright.dev/docs/test-global-setup-teardown))

1. **Isolation is a context, not a cleanup script.** Each test gets its own BrowserContext (localStorage/sessionStorage/cookies). Playwright explicitly prefers "start from scratch" over cleanup-in-between, which is what makes parallel/sharded runs safe ([Isolation](https://playwright.dev/docs/browser-contexts)).
2. **Auth reuse without UI login.** Recommended patterns: a `setup` project writing `storageState` to `playwright/.auth` (gitignored, because it contains impersonation material), or API authentication via `APIRequestContext` then `storageState`. Session-storage values can be injected with `context.addInitScript(...)` because Playwright does not persist `sessionStorage` ([Authentication](https://playwright.dev/docs/auth)). For Flora, `localStorage['flora.auth.v1']` is the equivalent injectable key.
3. **Multiple roles / workers.** If tests mutate shared server state, Playwright recommends one account per parallel worker keyed by `test.info().parallelIndex`; with `workers: 1` on CI a single seeded account is sufficient ([Authentication](https://playwright.dev/docs/auth#moderate-one-account-per-parallel-worker)).
4. **`webServer` supports an array** of processes (frontend + backend) with `url`, `reuseExistingServer: !process.env.CI`, `timeout`, `gracefulShutdown`; `baseURL` enables relative `page.goto('/tasks')` ([Web server](https://playwright.dev/docs/test-webserver)).
5. **CI stability defaults.** `workers: process.env.CI ? 1 : undefined`, `retries: process.env.CI ? 2 : 0`, `forbidOnly: !!process.env.CI`, `globalTimeout: 60*60*1000`, `trace: 'on-first-retry'`, upload `playwright-report/` with `if: ${{ !cancelled() }}` ([Configuration](https://playwright.dev/docs/configuration), [CI](https://playwright.dev/docs/ci)). Playwright advises **against** `trace: 'on'` (performance heavy) and against browser-binary caching (restore time ≈ download time, OS deps are not cacheable). Use Linux on CI (cheaper) and install only Chromium with `npx playwright install chromium --with-deps` ([Best practices](https://playwright.dev/docs/best-practices)).
6. **DB and third parties.** "If working with a database then make sure you control the data"; for third-party services use `route.fulfill` rather than hitting them ([Best practices](https://playwright.dev/docs/best-practices)). `APIRequestContext` is the sanctioned way to prepare server-side preconditions and validate postconditions from a browser test ([API testing](https://playwright.dev/docs/api-testing)) — this is exactly how the assignee-scoped task nuance can be asserted honestly.
7. **Setup/teardown choice.** Project dependencies (`setup db` project + `dependencies`/`teardown`) are recommended over `globalSetup` because they appear in the HTML report, support traces and fixtures ([Global setup/teardown](https://playwright.dev/docs/test-global-setup-teardown)).
8. **Cost levers.** Sharding and `--only-changed=<base>` (a heuristic; full suite must still run) are documented ways to shorten PR feedback ([CI](https://playwright.dev/docs/ci#fail-fast), [Best practices](https://playwright.dev/docs/best-practices)).

### 2.2 SvelteKit / Svelte CLI

- The current SvelteKit docs ([nav index](https://svelte.dev/docs/kit)) **no longer contain a standalone "Testing" page**. Testing is delivered through the `sv` CLI: [`sv add playwright`](https://svelte.dev/docs/cli/playwright) adds scripts, a Playwright config, `.gitignore` entries and a demo test; the SvelteKit [Integrations](https://svelte.dev/docs/kit/integrations) page lists `vitest` (unit) and `playwright` (e2e) add-ons. Flora's `apps/frontend` matches this shape (demo e2e test + config), so the deviation is in **wiring**, not in tooling choice.
- SvelteKit [Auth best practices](https://svelte.dev/docs/kit/auth) contrast DB-backed sessions (immediately revocable, DB query per request) with JWTs (lower latency, not immediately revocable). Flora uses bearer JWTs in `localStorage` plus a DB `sessions` row for `select-organization` rotation, so e2e must drive the **real** bearer flow rather than assume cookie semantics.
- Flora's `adapter-node` (`svelte.config.js:5`) is fine for a browser-targeted preview server; Tauri would need a different adapter (§2.4).

### 2.3 GitHub Actions

- **Postgres service container:** official [Creating PostgreSQL service containers](https://docs.github.com/en/actions/using-containerized-services/creating-postgresql-service-containers) shows `services.postgres` with `image: postgres`, `POSTGRES_PASSWORD`, health options `--health-cmd pg_isready ...`, and either label-based networking (job container) or `ports: - 5432:5432` + `localhost` (job on runner). Localized copies were checked in [pt-BR](https://docs.github.com/pt/actions/using-containerized-services/creating-postgresql-service-containers), [ja](https://docs.github.com/ja/actions/using-containerized-services/creating-postgresql-service-containers) and [zh-CN](https://docs.github.com/cn/actions/using-containerized-services/creating-postgresql-service-containers): same `services`/`ports`/health-check semantics. Flora's `rust.yml:27-42` already implements this pattern with `postgres:18` and health checks; reuse it.
- **Playwright GHA recipe:** [CI docs](https://playwright.dev/docs/ci#github-actions) show checkout → setup-node → install deps → `npx playwright install --with-deps` → `npx playwright test` → `actions/upload-artifact` with `if: ${{ !cancelled() }}`. (Flora uses Bun; `oven-sh/setup-bun@v2` is already in `ci.yml:12-15`.)
- **Cost:** [GitHub Actions billing](https://docs.github.com/en/billing/managing-billing-for-github-actions/about-billing-for-github-actions) (2026 model) measures included compute in **USD**, not minutes: Free plan $12/month, Pro/Team $18, Enterprise Cloud $300; standard **Linux 2-core x64 = $0.006/min**, Windows $0.010/min, **macOS $0.062/min**. Standard hosted runners are **free for public repos**; artifacts 500 MB (Free) to 50 GB (Enterprise), cache 10 GB/repo; Playwright traces/reports are artifacts and accrue GB-hours, so keep retention short. A 6–10 min Chromium job on Linux costs ~$0.04–$0.06 per run (≈$2.4–$3.6 for 60 PR runs) against $12 included on Free — affordable. A macOS Tauri job would be ~10× the per-minute rate: avoid for Seed.

### 2.4 Tauri (primary: [Tauri v2 — SvelteKit frontend guide](https://v2.tauri.app/start/frontend/sveltekit/))

The official guide's checklist is unambiguous:

- "Use SSG and SPA via `static-adapter`. **Tauri doesn't support server-based solutions.**"
- Prefer SPA mode (no prerendering): during SSG build, `load` functions cannot access Tauri APIs; SPA runs them in the webview.
- `tauri.conf.json` must set `frontendDist: "../build"`; SvelteKit must use `@sveltejs/adapter-static` with `fallback: 'index.html'`; root `+layout.ts` must set `export const ssr = false`.
- The [ja translation](https://v2.tauri.app/ja/start/frontend/sveltekit/) confirms the same four items (「Tauri はサーバー・ベースの方式には対応していません」, `fallback: 'index.html'`, `ssr = false`); the site also publishes fr/es/zh/de variants. The doc is pinned to SvelteKit 2.20.4 / Svelte 5.25.8 while Flora uses 2.63 / 5.56 (NV-10).

### 2.5 Multilingual observation

Across EN/pt-BR/ja/zh-CN no divergence was found on the load-bearing facts (context isolation; localStorage/session injection; Postgres service health checks and port mapping; adapter-static/SPA requirement). The practical difference is terminology only (e.g. ja 「サービスコンテナ」, zh 服务容器), so no language-specific caveat changes the design.

---

## 3. When browser e2e is worth it vs DB-gated contracts

| Concern | DB-gated contract (`flora-tests`, already in `rust.yml:db-tests`) | Browser e2e (proposed) | Verdict for Seed |
|---|---|---|---|
| Auth/tenancy, DTO shapes, search/notification/file semantics, idempotency | Strong: direct router/`oneshot`, unique UUIDv7 fixtures, deterministic, fast, no browser (pattern documented in `docs/research/2026-11-16-019-db-gated-tests-search-notifications.md:7-16`) | Weak/redundant: same assertions through a slower UI | **Keep contracts primary.** |
| Cross-route wiring (session stored → `/select-org` → `/channels` workspace chosen → message sent → `/tasks` created), client router redirects, localStorage persistence, build-time `VITE_API_URL`, CORS, actual rendered feedback | Cannot see it | **The unique value of e2e** | **Add one smoke.** |
| File upload/download | Already covered by ignored `contract::files::db_` in CI | Blocked: no mounted upload UI (§1) | **Defer e2e file step.** |
| Flakiness / cost | Low / ~seconds | Higher: browser binaries, startup, WS reconnect, cross-origin | Bound with workers=1, retries=1, trace-on-retry, globalTimeout, Chromium-only. |
| Diagnostics | Logs only | Trace viewer (DOM snapshots, network, timeline) | Worth it for the few failures contracts cannot explain. |

**Rule of thumb adopted:** a browser test is justified only when the assertion depends on browser-rendered state or on the client's wiring across routes; otherwise it belongs in the Rust contract lane. This matches Playwright's own "test user-visible behavior" and "control the data" guidance (§2.1) and avoids duplicating the already-wired DB lane.

---

## 4. Concrete minimal slice — **proposed, not implemented**

### 4.1 Scope and shape

One spec, `apps/frontend/tests/e2e/seed-flow.e2e.ts`, with four steps plus a deferred fifth:

1. **Bootstrap (setup project / `beforeAll`):** fresh JWT + pre-selection session; optionally call `POST /api/v1/auth/select-organization` via `request` to obtain an org-scoped `session_token`.
2. **Session injection:** `page.addInitScript` sets `localStorage['flora.auth.v1']` = `{ token, user: { id: '2222…' }, organizationId: null|'1111…', workspaceId: '8888…' }` before app code runs (Flora reads it in the `AuthState` constructor). Alternatively write a `storageState` file from a setup project; **do not commit** it.
3. **Flow:** `/select-org` → select Acme (assert redirect to `/tasks`); `/channels` → pick Main workspace → create unique channel `e2e-<suffix>` → assert visible; in chat send unique token `e2e-msg-<suffix>` → assert visible; `/tasks` → create unique title `e2e-task-<suffix>` with "Assign to me" → assert success banner and/or API postcondition.
4. **Postconditions:** `request.get('/api/v1/channels/{id}/messages')` contains the message token; `request.get('/api/v1/tasks')` contains the task (guard the assignee-scoped nuance from §1).
5. **File step — DEFERRED** (see §4.5).

Uniqueness (`<suffix>` = `crypto.randomUUID()` or `Date.now()` + worker index) comes from Playwright's DB-control guidance and the repo's UUIDv7 fixture convention (§2.1, §3).

### 4.2 Isolation and seeded data

- **One disposable Postgres 18 per job** (GitHub job VM is fresh) — reuse the `postgres:18` service pattern from `rust.yml:27-42`; apply `dev-seed.sql` + `scripts/dev-smoke-session.sql` after the API has applied migrations at startup. No per-test DB needed if `workers: 1` and the job is disposable; if parallelism is later enabled, give each worker its own org/workspace/session namespace as Playwright recommends (§2.1.3).
- **Unique app rows only:** created channels/messages/tasks carry the run suffix; seed rows keep their fixed IDs. Never assert on global counts or order.
- **Lean backend:** `FLORA_SEARCH__ENABLED=false`, `FLORA_MESSAGING__ENABLED=false` (Valkey/Meilisearch profile-gated per `docs/dev-smoke.md:17-19`). Note the chat UI calls `wsClient.connect()`; confirm it degrades gracefully without Valkey before relying on message visibility (NV-8).

### 4.3 Auth/session injection (no live IdP)

- Reuse `scripts/dev-smoke-session.sql` (upsert, safe to re-run) and **mint a fresh JWT** at test time with `FLORA_APP__JWT_SECRET`, claims `sub=2222…`, `jti=018f0000-0000-7000-8000-000000000001`, `organization_id=0000…0000`, `email=alice@acme.test`, `exp` = now + 1h. The hard-coded token in `docs/dev-smoke.md:67,69` is expired (§1) and must not be used. A tiny Node/Hono-free HMAC helper (or a Rust test binary using `flora_core::auth::jwt`) is the missing piece (NV-2).
- Prefer driving `select-organization` through the actual UI in step 3 so the smoke covers the session rotation contract; use the API route only to prepare state if a faster setup is needed.
- Keep the auth file in the Playwright output dir (auto-cleaned) and gitignore `playwright/.auth`, per Playwright's warning about impersonation material (§2.1.2).

### 4.4 Illustrative config / CI (sketch only — validate before adopting)

```ts
// apps/frontend/playwright.config.ts (proposed shape)
export default defineConfig({
  testDir: './tests/e2e',
  testMatch: '**/*.e2e.ts',
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: process.env.CI ? 1 : undefined,
  globalTimeout: 10 * 60 * 1000,
  reporter: [['html', { open: 'never' }], ['junit', { outputFile: 'test-results/e2e.xml' }]],
  use: { baseURL: 'http://127.0.0.1:4173', trace: 'on-first-retry' },
  projects: [{ name: 'chromium', use: { ...devices['Desktop Chrome'] } }],
  webServer: [
    { name: 'api', command: 'cargo run -p flora-api', url: 'http://127.0.0.1:3000/health',
      timeout: 120_000, reuseExistingServer: !process.env.CI },
    { name: 'web', command: 'npm run preview', url: 'http://127.0.0.1:4173',
      reuseExistingServer: !process.env.CI },
  ],
});
```

```yaml
# proposed extra job (research only)
e2e:
  runs-on: ubuntu-latest
  services:
    postgres:
      image: postgres:18          # matches rust.yml + PG18 uuidv7() requirement
      env: { POSTGRES_USER: postgres, POSTGRES_PASSWORD: postgres, POSTGRES_DB: flora }
      ports: ['5432:5432']
      options: >-
        --health-cmd "pg_isready -U postgres -d flora"
        --health-interval 10s --health-timeout 5s --health-retries 10
  env:
    FLORA_DATABASE__POSTGRES_URL: postgresql://postgres:postgres@localhost:5432/flora
    FLORA_APP__JWT_SECRET: change-me-to-a-random-secret-at-least-32-characters-long
    FLORA_SEARCH__ENABLED: 'false'
    FLORA_MESSAGING__ENABLED: 'false'
    VITE_API_URL: http://127.0.0.1:3000   # baked at build time by import.meta.env
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@1.98.0
    - uses: oven-sh/setup-bun@v2
    - run: bun install
      working-directory: apps/frontend
    - run: cargo build -p flora-api
    - run: npx playwright install chromium --with-deps
      working-directory: apps/frontend
    - run: psql ... -f dev-seed.sql && psql ... -f scripts/dev-smoke-session.sql   # after API migrations
    - run: bun run test:e2e
      working-directory: apps/frontend
    - uses: actions/upload-artifact@v4
      if: ${{ !cancelled() }}
      with: { name: playwright-report, path: apps/frontend/playwright-report/, retention-days: 3 }
```

**Cost/stability estimate:** ≈6–10 min × $0.006/min ≈ **$0.04–$0.06/run** on Linux (free if the repo is public; ~$2–4/month at 60 PR runs against $12 Free allowance). Keep the job **non-blocking** for the first weeks, then promote to required once green (NV-11). Do not add Firefox/WebKit or macOS jobs in Seed; that is where cost and webview/OS drift explode.

### 4.5 File step — documented deferral (inside the slice)

- **Why deferred:** `FileUpload.svelte` has no importer; no `files` seed rows; `files.spec.ts` is dead and would fail on `/channels`; file upload/download is already gated in `rust.yml` via `contract::files::db_`. A browser assertion would test nothing the contract lane does not already test better.
- **Re-entry trigger:** an upload control is mounted in a canonical route **and** a happy-path upload + download assertion can be expressed against a known seeded file. Then add one step: `setInputFiles` → assert server-side file row via `request` (mapped IDs) → download link visible.
- Until then, treat "workspace→channel→message→task→**file**" as fully covered at API/DB level and partially covered (first four steps) in browser (NV-5).

---

## 5. Tauri deferral decision — explicit

**Decision (2026-11-16): keep Tauri out of the canonical Seed frontend. Do not create `src-tauri/` under `apps/frontend`; do not rewire `svelte.config.js` to `adapter-static`; do not add per-OS release jobs. Record this as the "decisão Tauri canônica" answer for P2.**

| Factor | Cost / benefit if adopted now | Why deferral wins for Seed |
|---|---|---|
| Build architecture | Requires `adapter-static` + `fallback: 'index.html'` + root `ssr = false` + `frontendDist: build` (official Tauri guide, §2.4). Canonical app is `adapter-node` (`svelte.config.js:5`); a dual build-mode split must be designed and tested. | Seed's verification goal is the **browser** flow on a cheap VPS; a second build target adds risk without new Seed evidence. Prior research already calls this "an architecture prerequisite, not a packaging tweak" (`016-tauri…md:41`). |
| Auth | Webviews have different cookie/origin behavior; Flora relies on bearer JWT + DB `sessions` rotation and would need a non-cookie-first contract plus CORS allowlist for `tauri://` origins (`016-tauri…md:14,50-51`). | Current bearer/localStorage flow is already testable in a normal browser; packaging would force re-validation with no user-facing Seed benefit. |
| Existing asset | `apps/web/tauri` is the SolidJS `@coparse/web` shell (`apps/web/package.json:2`, `tauri.conf.json:3,7`) — not reusable for Svelte 5. | Pretending reuse exists would be false; real cost is a fresh crate. |
| CI cost | Per-OS matrix; macOS 3–4 core runners cost $0.062/min vs Linux $0.006/min (10×, §2.3), plus signing/notarization/updater keys. | Directly conflicts with "VPS barata / enxuta" and with keeping CI cheap; a Linux-only browser smoke is the right cost point. |
| Product timing | `OBJETIVO.md:9` and `ROADMAP.md:4,10` say Tauri is a **later phase**; cycle-016 research sequenced desktop only after web-core hardening (Phase 0 → 1 at `016-tauri…md:30-37`). | Deferral is consistent with stated roadmap, not a reversal. |

**Re-entry criteria (all must hold before reopening):**

1. Browser e2e smoke (§4) and DB-gated contracts are green and stable for a defined window.
2. API base URL and WS URL are runtime-configurable and the bearer/`select-organization` contract is verified from a non-browser origin (CORS allowlist explicit; no reliance on cookies).
3. A concrete user demand for desktop/mobile exists, with an owner for per-OS signing, updater keys and webview QA.
4. A time-boxed Linux desktop spike demonstrates one core flow against a self-hosted VPS; mobile remains a separate decision (`016-tauri…md:37,53`).

**Interim (zero-cost) actions:** keep `apps/web/tauri` explicitly labelled non-canonical; keep `adapter-node` in `apps/frontend`; keep browser e2e as the only e2e lane. No code change is required by this decision.

---

## 6. NVs — not verified / open for this pass

- **NV-1:** Nothing here was executed; no test, workflow, config or seed file was changed. All §4 artifacts are designs.
- **NV-2:** The pre-signed JWT in `docs/dev-smoke.md:67,69` is expired as of 2026-11-16T00:00:00Z; a fresh-mint helper and secret alignment with `FLORA_APP__JWT_SECRET` must be built and verified.
- **NV-3:** Playwright config/testMatch mismatch and stale selectors (§1) were confirmed by reading files, but no test run was attempted; exact runtime failures are inferred, not observed.
- **NV-4:** `--health-cmd`, `postgres:18-alpine` vs `postgres:18`, migration-then-seed ordering, and the API readiness URL (`/health` path is illustrative) need verification against a real runner.
- **NV-5:** File step deferred; `FileUpload.svelte` mounting and file fixture design are future work.
- **NV-6:** Cross-origin 4173→3000 works today only because `cors_layer()` uses `Any` (`flora-api/src/middleware/cors.rs:14-20`); tightening CORS later may require explicit `http://127.0.0.1:4173` allowlisting. Security hardening of permissive CORS is out of scope here.
- **NV-7:** `VITE_API_URL` is baked at build time (`client.ts:3`); the preview build must be produced with the CI env set, otherwise the browser calls `localhost:3000` implicitly. Not verified in a build.
- **NV-8:** `ChatLayout` calls `wsClient.connect()` on channel load with messaging disabled in the lean profile; whether it retries noisily or blocks assertions is unverified.
- **NV-9:** Tauri refactor effort, OS signing/updater keys, webview compatibility matrix, and reuse of any `@coparse/web` code are unmeasured; re-entry criteria are qualitative.
- **NV-10:** es/de/fr multilingual sources were inherited from `016-tauri…md:6,58-60` rather than re-fetched; Tauri's guide is pinned to older SvelteKit/Svelte versions than this checkout — revalidate before implementation.
- **NV-11:** Cost figures are GitHub list prices for 2026 (§2.3); the repo's actual plan, included minutes and public/private status were not checked.
- **NV-12:** If e2e is ever parallelized/sharded, per-worker data isolation (accounts/workspaces) must be implemented; the proposed single-worker disposable-job design avoids it only for now.

---

## 7. Sources

**External (primary):**
- Playwright: [Authentication](https://playwright.dev/docs/auth), [Isolation](https://playwright.dev/docs/browser-contexts), [Web server](https://playwright.dev/docs/test-webserver), [Configuration](https://playwright.dev/docs/test-configuration), [Continuous Integration](https://playwright.dev/docs/ci), [Best practices](https://playwright.dev/docs/best-practices), [API testing](https://playwright.dev/docs/api-testing), [Global setup and teardown](https://playwright.dev/docs/test-global-setup-teardown).
- Svelte/SvelteKit: [SvelteKit docs index](https://svelte.dev/docs/kit), [Integrations](https://svelte.dev/docs/kit/integrations), [Auth](https://svelte.dev/docs/kit/auth), [sv add playwright](https://svelte.dev/docs/cli/playwright), [adapter-static](https://svelte.dev/docs/kit/adapter-static), [single-page apps](https://svelte.dev/docs/kit/single-page-apps).
- GitHub Actions: [PostgreSQL service containers](https://docs.github.com/en/actions/using-containerized-services/creating-postgresql-service-containers) ([pt-BR](https://docs.github.com/pt/actions/using-containerized-services/creating-postgresql-service-containers), [ja](https://docs.github.com/ja/actions/using-containerized-services/creating-postgresql-service-containers), [zh-CN](https://docs.github.com/cn/actions/using-containerized-services/creating-postgresql-service-containers)), [About billing for GitHub Actions](https://docs.github.com/en/billing/managing-billing-for-github-actions/about-billing-for-github-actions).
- Tauri v2: [SvelteKit frontend guide](https://v2.tauri.app/start/frontend/sveltekit/) ([ja](https://v2.tauri.app/ja/start/frontend/sveltekit/)).

**Local (repo, read-only):**
- `OBJETIVO.md:9,93`; `ROADMAP.md:4,10`.
- `.github/workflows/ci.yml:1-26`; `.github/workflows/rust.yml:23-...` (incl. `db-tests` job).
- `apps/frontend/package.json:6-17,23,35`; `apps/frontend/playwright.config.ts:1-6`; `apps/frontend/vite.config.ts:28-48`; `apps/frontend/svelte.config.js:5-14`. `.github/workflows/rust.yml:26-71`.
- `apps/frontend/tests/e2e/{tasks,files}.spec.ts`; `apps/frontend/src/routes/demo/playwright/page.svelte.e2e.ts:3-6`.
- `apps/frontend/src/lib/state/AuthState.svelte.ts:8-65`; `apps/frontend/src/lib/api/client.ts:3,73-108`; `apps/frontend/src/lib/components/files/FileUpload.svelte:59`.
- `apps/frontend/src/routes/(app)/select-org/+page.svelte:38-69,91-101`; `.../channels/+page.svelte:24-46,96-106`; `.../tasks/+page.svelte:26,66-86,100-110`.
- `docs/dev-smoke.md:17-19,47-88`; `scripts/dev-smoke-session.sql:10-48`; `dev-seed.sql:192-300`; `flora-api/src/middleware/cors.rs:14-20`.
- `apps/web/package.json:2`; `apps/web/tauri/src-tauri/tauri.conf.json:3,7`.
- Prior research: `docs/research/2026-11-16-016-tauri-svelte5-packaging.md`, `docs/research/2026-11-16-019-db-gated-tests-search-notifications.md`, `docs/research/2026-11-15-012-frontend-api-contract-audit.md:259-357`.
