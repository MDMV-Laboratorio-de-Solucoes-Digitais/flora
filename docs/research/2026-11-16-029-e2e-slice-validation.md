> **Superseded by PR #238/#239 (cycle 029):** this is a pre-implementation snapshot; check committed files for current state.

# 029 — Validation & sharpening of the cycle-028 e2e slice (read-only audit)

**Date:** 2026-11-16
**Repo:** `flora` — actual baseline at audit time: branch `dev`, HEAD **`43b21a675`**, working tree clean (`git status --porcelain` empty).
**Subject:** deep validation of `docs/research/2026-11-16-028-e2e-jwt-implementation-plan.md` (feature-gated `flora-mint-session` bin, Playwright setup project, non-required Chromium CI job).
**Method:** read-only re-derivation of every load-bearing claim against the checked-out files, plus direct fetches of the primary docs (Cargo Book, Playwright, Vite) on 2026-11-16. **No source, manifest, config, seed, workflow, `.gitignore`, lockfile or test file was modified by this pass.** The only artifact is this document. Nothing was executed (no build / clippy / seed / API / Playwright / CI run) — see NV-1.

---

## 0. Verdict

**GO — conditional**, for exactly **one** smoke, on the corrected edit list in §4 and the five gates in §6.1. The 028 design is fundamentally sound: all of its load-bearing code claims re-verified true against this checkout, and all three primary-doc pillars (Cargo `required-features`, Playwright `webServer`/auth, Vite build-time env) check out verbatim.

However, this pass found **four material corrections / additions** that 028 missed or got wrong:

| # | Finding | Severity | Effect on 028 |
|---|---|---|---|
| **F1** | Baseline commit is stale: 028 says `e9ef62589`, checkout is `43b21a675`. | M | Re-anchor all line citations; substance still holds (re-verified here). |
| **F2** | `select_organization` **revokes the injected jti** and mints a new one (`auth.rs:384-407`). Playwright dependencies run once; a test retry reuses the now-revoked jti → 401. | **H** | **Set `retries: 0`** (028's `retries: CI ? 1 : 0` is wrong for this slice). Smoke is one-shot by construction. |
| **F3** | 028 §2.2/§3.5 says "write auth state under `outputDir`" but §3.4 code writes a fixed `playwright/.auth/alice.json`. | L | Internal contradiction. Pick the fixed path + gitignore (matches Playwright's own recommendation); drop the `outputDir` wording. |
| **F4** | `deny.toml:7` sets `[graph] all-features = false`; CI's `cargo deny check` therefore never sees the new optional `clap`/`anyhow`. | M | Existing gate won't break (good), but the new deps are un-audited (blind spot). They are already workspace deps with allow-listed licenses — record as NV. |

Everything else in 028 is **confirmed**. §1 gives the evidence table; §2 the primary-doc cross-check; §3 the risks; §4 the exact ordered edits; §6 the gates.

---

## 1. Re-verified baseline (exact evidence, this checkout)

### 1.1 Cargo manifest / feature mechanics — CONFIRMED

- `flora-core/Cargo.toml:23-54`: `serde_json`, `chrono`, `uuid`, `jsonwebtoken`, `figment`, `thiserror`, `tokio`, `axum` are all **regular** deps — the minter needs no new runtime dep for JWT/JSON/config. **No `clap`, no `anyhow`** (028's "dependency gap" correct).
- `flora-core/Cargo.toml:19-21`: `[lib] name = "flora_core"`. `:61-64`: `[features] default = []`, `sqlite = ["sqlx/sqlite"]`, `test = []`. No `[[bin]]`.
- **New:** `flora-core/src/bin/` **does not exist** (verified via `glob` + `find`); no `cfg(feature = ...)` in Rust source. So Step A/B are greenfield, no conflict with auto-discovery.
- Root `Cargo.toml:18` `resolver = "3"`; `:40-91` workspace lints deny `unsafe_code`, `missing_docs`, `missing_debug_implementations`, `unreachable_pub`, `unused_results`, `allow_attributes`, `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `multiple_crate_versions`; `print_stdout = "warn"`. `:87` confirms `print_stdout` is declared under `[workspace.lints.clippy]` — the sketch's `#![expect(clippy::print_stdout, reason = …)]` targets the **correct** lint namespace (I checked `rustc -W help`: no bare `print_stdout` lint exists on this toolchain; `rustc 1.99.1` locally, CI pins 1.98.0).
- Root `Cargo.toml:156` `anyhow = "1.0"`, `:221` `clap = { version = "4.5", features = ["derive", "env"] }` — both already workspace deps and already in the lock graph. Adding them as `optional = true` members introduces **no new version** (so `multiple_crate_versions = "deny"` stays clean) and **no new license** (both are in `deny.toml:20-34` allow list).

### 1.2 `flora-core/src/utils/jwt.rs` claim shape — CONFIRMED

- `:16-33` `FloraClaims`: exactly 7 fields, all `pub`: `sub`, `jti`, `organization_id`, `email`, `exp: i64`, `iat: i64`, `ver: u32` with `#[serde(default)]` on `ver`. **Struct-literal construction is mandatory** to choose `exp`.
- `:35-56` `FloraClaims::new(...)` always derives `exp = now + Duration::seconds(ttl)` — cannot back/forward-date. `:60-68` `user_id()`/`org_id()` parse with `unwrap_or_default()` (nil UUID on bad input).
- `:82-89` `encode_token` = `encode(&Header::default(), claims, EncodingKey::from_secret(...))` → HS256, `typ=JWT`; `:96-104` `decode_token` = `decode::<FloraClaims>(..., Validation::default())` → validates signature + `exp` (HS256 default alg set), does **not** interpret `ver`. Round-trip via `decode_token` is a valid, cheap correctness guard (mirrors `flora-tests/src/contract/auth.rs:504` et al., which use `encode_token(&FloraClaims::new(...), …)`).

### 1.3 Secret precedence — CONFIRMED

- `flora-core/src/config/mod.rs:43-49`: `Figment::new().merge(Serialized::defaults(Self::default())).merge(Env::prefixed("FLORA_").split("__"))` — Env merged **last**, nested keys win. (028 wrote "Figment::defaults(Self::default())"; the code is the equivalent `Figment::new().merge(Serialized::defaults(...))` — same precedence, notation in 028 is loose.)
- `flora-core/src/config/app.rs:38-65`: `AppConfig::default()` reads flat `FLORA_JWT_SECRET` at default-construction time (`:47-48`), falls back to `dev-secret-change-in-production`; `:49` `session_ttl_secs = 3600`; `:42` `env = "development"`. `:79-86` ≥32-char secret enforced only when `env == "production"`.
- Therefore `FLORA_APP__JWT_SECRET` (nested, merged last) **beats** flat `FLORA_JWT_SECRET`; `.env.example:14,20` set both to the same value by design. The minter **must** call `Config::load()`, not `AppConfig::default()`, to inherit the effective secret (028 correct).
- `dotenv = "0.15"` is declared at `flora-core/Cargo.toml:24` but no Rust file calls it; **Rust does not auto-load `.env`** — process env only. (Bun, on the frontend side, does — see §2.3.)
- `flora-core/src/config/database.rs:9,25`: field is `postgres_url`, default reads legacy `DATABASE_URL`; nested `FLORA_DATABASE__POSTGRES_URL` maps correctly (already exercised by `rust.yml:56-57`, which sets both). The minter's `Config::load()` will extract fine with the e2e job env.

### 1.4 Seed / session / FK — CONFIRMED

- `scripts/dev-smoke-session.sql:10-48`: upserts **jti** `018f0000-0000-7000-8000-000000000001`, user `2222…` (Alice), org **nil**, `expires_at = NOW() + 365 days`, `is_active = TRUE`, id `018f0000-…-0010`, `ON CONFLICT (jti) DO UPDATE ... is_active = TRUE`. Idempotent and **reactivating** — so a fresh CI run always works.
- `dev-seed.sql:12-22` Acme `1111…` (`acme-corp`); `:28-41` Alice `2222…` `alice@acme.test`; `:154-163` Alice ↔ Acme with the `Admin` role (required by `select-organization`'s membership check).
- Migration `20240818000001_init_schema.sql:27` inserts nil-UUID `System` org; `:128-141` `sessions.organization_id NOT NULL REFERENCES organizations(id)` and `jti VARCHAR(255) NOT NULL UNIQUE`; `:191` jti index. Nil-org session is **FK-valid**; jti uniqueness is why `workers: 1` is mandatory.
- DB TTL (365 d) ≫ JWT TTL (≤1 h): 028's expired-static-token diagnosis is correct.

### 1.5 Server-side authority — CONFIRMED, plus F2

- `flora-api/src/extractors/auth.rs:33-97` `verify_bearer_token`: decode → parse `sub`/`organization_id` → `session_service.validate_session(jti)` → reject unless active, unexpired, same user and same org (nil allowed). `:212-226` `require_org()` rejects nil; `list_user_orgs` (`routes/org.rs:88-119`) uses plain `AuthExtractor`, **not** `require_org` — so `GET /api/v1/organizations` is a valid nil-org drift guard.
- `flora-core/src/services/session_service.rs:80-82`: `validate_session` is a pure `find_by_jti` lookup; the active/expiry/org checks all live in the extractor above.
- **F2 (new):** `flora-api/src/routes/auth.rs:352-416` `select_organization` validates membership (`:379-382`), then **`session_repo.revoke(current.id)` at `:384`**, creates a **new** session with **`jti = Uuid::now_v7().to_string()`** (`:387`) and org scope (`:393`), and returns a rotated JWT (`:406-415`). Consequences:
  1. The injected nil-org jti is **dead after one successful selection** — do not reuse the storageState for a second select in the same run.
  2. A Playwright **retry** of the smoke reuses the same file (`storageState` is fixed in project config) → `/select-org` immediately 401s. Dependencies are set up once, not per retry. **Set `retries: 0`.**
  3. A fresh `bun run test:e2e` is safe because the setup project re-applies `dev-smoke-session.sql`, which flips `is_active = TRUE` back on.

### 1.6 Frontend state / storage — CONFIRMED

- `AuthState.svelte.ts:8` key `flora.auth.v1`; `:14-31` synchronous `localStorage.getItem` load; `:43-50` constructor sets `isAuthenticated = Boolean(token)` (so the value must exist **before** app JS runs); `:52-65` persists `{token, user, organizationId, workspaceId}`.
- `client.ts:3` `API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'`; `:78-80` Bearer header from `authState.token`; `:95-97` 401 → `lockSession()` (5-min grace then logout, `:124-135`).
- `select-org/+page.svelte:38-48` lists orgs via `GET /api/v1/organizations`; `:50-69` POST `select-organization` → `setOrganization(newToken, org)` → `goto('/tasks')`; `:91-101` redirects to `/tasks` when `organizationId` is already set → **inject `organizationId: null`**, per 028.
- No cookies are used by the auth path → `cookies: []` in storageState is correct.

### 1.7 Playwright / scripts / CI — CONFIRMED (with corrections)

- `apps/frontend/playwright.config.ts:1-6` is exactly the 6-line file 028 describes: `webServer: { command: 'npm run build && npm run preview', port: 4173 }`, `testMatch: '**/*.e2e.{ts,js}'`, **no `testDir`** (default = config dir, so today it collects only `src/routes/demo/playwright/page.svelte.e2e.ts`, which asserts an `h1`).
- `apps/frontend/tests/e2e/` contains only `tasks.spec.ts` and `files.spec.ts` — both `.spec.ts`, both stale/dead (they target UI selectors/flows not present in the current Svelte 5 app, e.g. `Add Task`, `New Task Title`, file-upload `.error-message`). With `testDir: './tests/e2e'` + `testMatch: '**/*.e2e.ts'` they stay out. **Do not broaden `testMatch` to `.spec.ts`** (028 correct).
- `apps/frontend/package.json:17` `"test:e2e": "playwright install && playwright test"`; devDeps `@playwright/test ^1.60.0` and `playwright ^1.60.0` (`:23,35`). Change the script as planned; install Chromium explicitly in CI.
- `apps/frontend/.gitignore:24-25` ends with `# Playwright` / `test-results` — **no `playwright/.auth`**. Add it.
- `.github/workflows/ci.yml:1-24`: Bun-only frontend job (check/lint/unit/build), **no e2e, no Postgres service**. `.github/workflows/rust.yml:26-37` strict fmt/clippy/deny/build/test; `:39-85` `db-tests` with `postgres:18` service + 6 gated suites. Adding the e2e job to **`rust.yml`** (028's choice) reuses the Rust toolchain block and the proven service-container shape; `ci.yml` stays untouched. Either workflow would work — this is a placement choice, not a correctness constraint. **Keep it non-required.**
- `vite.config.ts:1-5` imports `defineConfig` from `vitest/config`, `playwright()` from `@vitest/browser-playwright`, `tailwindcss`, `adapter` from `@sveltejs/adapter-node`, `sveltekit` from `@sveltejs/kit/vite`. `package.json:9` preview script is plain `vite preview`. Vite env semantics (§2.3) still apply. `svelte.config.js` uses `adapter-node` (per 028) — confirm `vite preview` actually serves the SSR app on 4173 (NV-4).

---

## 2. Primary-doc cross-check (directly fetched 2026-11-16)

For every non-obvious step, the owning source was consulted — no secondary blog.

### 2.1 Cargo — feature-gated bin & optional deps
- [Cargo Book — Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html): *"Binary targets are executable programs … A binary's source can be `src/main.rs` and/or stored in the `src/bin/` directory"*; *"Binaries can use the public API of the package's library"*; **`required-features`: "If any of the required features are not enabled, the target will be skipped. This is only relevant for `[[bin]]`, `[[bench]]`, `[[test]]`, and `[[example]]`, it has no effect on `[lib]`."** → a plain `cargo build --release` and CI's `cargo clippy --workspace --lib --bins` never compile the minter.
- [Cargo Book — Features](https://doc.rust-lang.org/cargo/reference/features.html): features are **disabled unless in `default`**; `dep:` prefix suppresses the implicit feature and is available since **Rust 1.60** (workspace is 1.98/edition 2024 — safe); feature unification is per-package and the `resolver = "3"` graph keeps `dev-mint` out of `flora-api`.
- Archived but relevant: root `Cargo.toml:18` `resolver = "3"`.

### 2.2 Playwright — webServer / auth / state shape
- [Web server](https://playwright.dev/docs/test-webserver): **`port` is deprecated → use `url`**; `url` means ready when the server returns **2xx/3xx/400/401/402/403**; `env` **defaults to inherited `process.env` + `PLAYWRIGHT_TEST=1`**; webServer accepts an **array**; `reuseExistingServer: !process.env.CI`; `timeout` default **60 000 ms** (raise to 120 000 for `cargo build`); `baseURL` makes `page.goto('/…')` resolve against it — and therefore fixes the localStorage origin. All of 028's Step C claims verified.
- [Authentication](https://playwright.dev/docs/auth): *"We recommend to create `playwright/.auth` directory and add it to your `.gitignore`"*; danger note about impersonation; *"If you don't need to keep the state between test runs, write the browser state under `testProject.outputDir`"*; **"Reusing authenticated state covers cookies, local storage, IndexedDB and passkey"**; sessionStorage is not persisted. Setup project + `use.storageState` is the recommended shared-account pattern; setup is a dependency that "will always run and authenticate before all the tests."
- [Retries](https://playwright.dev/docs/test-retries): on failure the worker process is discarded and a new one starts by **retrying the failed test**; the doc's worker diagrams show `beforeAll` re-running but say nothing about re-running `dependencies`/setup projects — combined with F2, the only safe posture for this slice is `retries: 0`.
- [BrowserContext storageState](https://playwright.dev/docs/api/class-browsercontext#browser-context-storage-state) (schema carried from 027/028): `{ cookies: [], origins: [ { origin, localStorage: [ { name, value } ] } ] }`; `origin` must be exact. `addInitScript` runs after document creation but before page scripts and has undefined ordering between multiple scripts — `storageState` (applied at context creation) is strictly better here.

### 2.3 Vite — build-time env
- [Env Variables and Modes](https://vite.dev/guide/env-and-mode) (EN; 028 additionally cross-checked es/zh-CN/ja): *"defined as global variables during dev and **statically replaced at build time**"*; *"environment variables that already exist when Vite is executed have the **highest priority** and will not be overwritten by `.env` files"*; **"Bun users … Bun automatically loads `.env` files before your script runs … can interfere with Vite's feature."** → `VITE_API_URL` must be in the **job env** at `npm run build` time; a local `apps/frontend/.env` is a real pitfall. That env is inherited by Playwright's `webServer.env` (§2.2), so setting it once on the job is sufficient.
- Vite current docs also publish 한국어/Deutsch (the 028 "no natural-language locales" note applies to Playwright, not Vite; no load-bearing divergence either way).

### 2.4 GitHub Actions — Postgres 18 service
- [Creating PostgreSQL service containers](https://docs.github.com/en/actions/using-containerized-services/creating-postgresql-service-containers): `services.postgres` with `POSTGRES_PASSWORD`, `ports: 5432:5432`, `pg_isready` health check and `localhost` networking — already implemented verbatim at `rust.yml:41-54`; reuse that block.

---

## 3. Risks / NVs (carried from 028, corrected + new)

Severity: **H** = gate before merge; **M** = watch; **L** = note. New in this pass marked ★.

- **NV-1 (H) — Nothing executed.** All design. Every §5 gate is unverified.
- **NV-2 (H) — Manifest/lint mechanics uncompiled.** `optional = true` + `workspace = true`, `required-features` under `--bins`, and `clippy::print_stdout` suppression were not compiled. Confirmed by docs but not by rustc. Fallback for Cargo inheritance rejection: literal `clap = { version = "4.5", features = ["derive", "env"], optional = true }` / `anyhow = { version = "1.0", optional = true }` (same versions as root `:156,221`).
- **★ NV-3 (H) — Retry hazard (F2).** `retries: 1` is unsafe: rotation revokes the injected jti. Set `retries: 0`. If CI insists on retries, the setup must re-seed/re-mint before each retry — out of scope.
- **NV-4 (H) — Secret precedence not runtime-verified.** Nested-vs-flat override derived from Figment merge order; gate by minting with two different `FLORA_APP__JWT_SECRET` values.
- **NV-5 (H) — Origin equality not browser-verified.** Pin `--host 127.0.0.1`; make `E2E_FRONTEND_ORIGIN`, preview `--host/--port`, `webServer.url` and `use.baseURL` the **same literal**. ★ Also confirm `vite preview` (with `adapter-node`) actually serves the SSR app on `127.0.0.1:4173`.
- **★ NV-6 (M) — Stale baseline (F1).** 028 cites `e9ef62589`; audit found `43b21a675`. Substance re-verified here, but any implementer following 028's raw line numbers without re-checking is at risk.
- **NV-7 (M) — `psql` availability / seed ordering.** `rust.yml` DB job uses a service container but never invokes `psql` as a client; `ON_ERROR_STOP=1`, cwd and `PGPASSWORD` path are unverified. Fallback: move seed+mint into a dedicated CI step and set `reuseExistingServer: true` for the api webServer.
- **NV-8 (M) — Bun `.env` preload** (Vite §2.3). Keep `VITE_API_URL` in process env; add an optional fast-fail `test -z "$(git ls-files apps/frontend/.env)"`.
- **NV-9 (M) — Profile-A rotation not executed.** The `select-organization` → `/tasks` path with the seeded Admin membership was not run (also 026:NV-3, 027). ★ Note F2: the rotated jti/session is created in the DB — verify it survives the navigation.
- **NV-10 (M) — WS with lean backend.** `ChatLayout` calls `wsClient.connect()` while `FLORA_MESSAGING__ENABLED=false`; whether it retries noisily or blocks assertions is unverified.
- **NV-11 (M) — Dead specs.** `tasks.spec.ts` / `files.spec.ts` stay dead by design; re-enabling is a separate task.
- **NV-12 (M) — Node HMAC fallback (if chosen) untested.** Must read `FLORA_APP__JWT_SECRET` → flat → default and keep the positive API assertion.
- **★ NV-13 (M) — Supply-chain blind spot (F4).** `deny.toml:7 all-features = false`: the existing `cargo deny check` never audits the new optional deps. They are already workspace deps with allow-listed licenses (MIT/Apache-2.0) and no new versions, so risk is low — but run `cargo deny check -p flora-core --features dev-mint` once in the e2e job (or accept explicitly as NV).
- **NV-14 (L) — CORS hardening.** `cors.rs:14-20` is `Any` today; tightening later needs an explicit `http://127.0.0.1:4173` allowlist.
- **NV-15 (L) — Parallelism/sharding.** `workers: 1` assumed; jti is `UNIQUE` and the account is shared. Parallel needs per-worker jti/account namespaces.
- **NV-16 (L) — `port` deprecation.** Switch to `url` per current Playwright docs; not verified against the installed 1.60 runtime (docs current at fetch time were newer than 1.60 — the `url` form has been supported well before 1.60).
- **NV-17 (L) — Multilingual coverage.** Cargo Book EN-only; Playwright no natural-language locales; Vite has more locales than 028 listed but no divergence. No pt-BR Vite locale claimed here.
- **NV-18 (L) — CI cost/plan unknown.** ~$0.04–$0.06/run estimate from public list prices; actual plan not checked.

---

## 4. Ordered, exact edit list — ONE smoke

Apply in order. Each step names the file and the exact change. **Stop and run the §5 gates between stages A and B.**

### Stage A — Rust minter (no production source change)

1. **`flora-core/Cargo.toml`** — add to `[dependencies]` (end of the "Core-specific" block, after `axum.workspace = true` at line 54):
   ```toml
   clap = { workspace = true, optional = true }
   anyhow = { workspace = true, optional = true }
   ```
   In `[features]` (after `default = []`, line 62) add:
   ```toml
   dev-mint = ["dep:clap", "dep:anyhow"]
   ```
   Append after `test = []` (line 64):
   ```toml
   [[bin]]
   name = "flora-mint-session"
   path = "src/bin/flora-mint-session.rs"
   required-features = ["dev-mint"]
   ```
   Keep `default = []`. Do **not** add any dev API route. (If `optional = true` + `workspace = true` is rejected, use the literal-version fallback in NV-2.)

2. **`flora-core/src/bin/flora-mint-session.rs`** (new directory + file) — implement per 028 §3.2 with these **mandatory sharpenings**:
   - `#![expect(clippy::print_stdout, reason = "dev CLI emits the storageState JSON consumed by the e2e harness")]` (clippy namespace confirmed §1.1); add `#[expect(clippy::missing_docs_in_private_items, reason = "...")]` **only if** rustc/clippy actually demands it — do not pre-emptively add bare `#[allow]`.
   - Literal `FloraClaims { sub: ALICE, jti: JTI, organization_id: NIL_ORG, email: EMAIL, iat: now.timestamp(), exp: now.timestamp() + ttl, ver: 1 }` — **not** `FloraClaims::new` (cannot choose `exp`).
   - `let cfg = Config::load().context(...) ?` — **never** `AppConfig::default()`; refuse when `cfg.app.env == "production"`.
   - `ttl = args.ttl_secs.unwrap_or(cfg.app.session_ttl_secs)`; convert with `i64::try_from(ttl).context("ttl_secs exceeds i64")?` (no `as` cast — `trivial_casts`/`pedantic`).
   - Round-trip `decode_token(&token, &cfg.app.jwt_secret).context("mint roundtrip failed")?` and assert `jti`/`exp`/`sub` equality, `bail!` on drift.
   - Write the **whole** storageState: `{ cookies: [], origins: [{ origin: <E2E_FRONTEND_ORIGIN>, localStorage: [{ name: "flora.auth.v1", value: JSON.stringify({token, user:{id:ALICE}, organizationId:null, workspaceId:null}) }] }] }`; `--storage-state -` prints to stdout, otherwise `create_dir_all(parent)` + `write`.
   - `--origin` / `--storage-state` / `--ttl-secs` via clap `env` (feature already enabled at root `:221`) so minter and Playwright share one source of truth.

3. **Gate:** run §5 Rust commands. Do not proceed until `cargo build -p flora-core --bin flora-mint-session --features dev-mint` is green, strict clippy on the bin is green, and a plain `cargo build --release` produces **no** `flora-mint-session` binary.

### Stage B — Frontend wiring (one smoke only)

4. **`apps/frontend/playwright.config.ts`** — replace all 6 lines with 028 §3.3, with these **corrections**:
   - `retries: 0` (**not** `CI ? 1 : 0` — F2/NV-3).
   - `workers: 1` in all environments (jti is UNIQUE and the smoke is one-shot; do not leave local `undefined`).
   - `testDir: './tests/e2e'`, `testMatch: '**/*.e2e.ts'`.
   - `use: { baseURL: FRONTEND_ORIGIN, trace: 'on-first-retry' }`, `projects: [setup, chromium(deps setup, storageState AUTH_FILE)]`.
   - `webServer: [ api{ command:'cargo run -q -p flora-api', url:`${API_ORIGIN}/health`, timeout:120_000, reuseExistingServer:!process.env.CI }, web{ command:'npm run build && npm run preview -- --host 127.0.0.1 --port 4173', url: FRONTEND_ORIGIN, timeout:120_000, reuseExistingServer:!process.env.CI } ]`.
   - Auth file path **fixed** at `playwright/.auth/alice.json` (resolve F3: drop the `outputDir` recommendation from the plan; it is an alternative, not an additional requirement).

5. **`apps/frontend/tests/e2e/seed.setup.ts`** (new) — exactly 028 §3.4: after both webServers are ready, `psql -v ON_ERROR_STOP=1 -h 127.0.0.1 -U postgres -d flora -f <repoRoot>/dev-seed.sql` then `…-f <repoRoot>/scripts/dev-smoke-session.sql`; then `cargo run -q -p flora-core --bin flora-mint-session --features dev-mint -- --storage-state <abs path>`; then the **positive drift guard**: `GET ${E2E_API_ORIGIN}/api/v1/organizations` with the minted Bearer token must be 200 and contain org `1111…`. No `storageState` on the setup project. (Keep `PGPASSWORD` defaulted from env.)

6. **`apps/frontend/tests/e2e/seed-flow.e2e.ts`** (new — the **single smoke**) — inherits `storageState` from `chromium`, so no `addInitScript`. Steps:
   1. `page.goto('/select-org')` → assert Acme (`acme-corp`) is listed. This proves `flora.auth.v1` was read on the **frontend** origin with `organizationId: null`.
   2. Click `Select` on Acme; assert navigation to `/tasks` and a `/tasks` heading. (One selection only — F2.)
   3. Optional postcondition via `page.request` using the **page's current** token (the rotated one), not the injected one; do not reuse the storageState token after selection.
   Use `crypto.randomUUID()` suffixes if any server-side mutation is added later. Do not re-enable `tasks.spec.ts` / `files.spec.ts`.

7. **`apps/frontend/.gitignore`** — under the `# Playwright` block (after line 25 `test-results`) add a line `playwright/.auth`.

8. **`apps/frontend/package.json`** — line 17 change `"test:e2e": "playwright install && playwright test"` → `"test:e2e": "playwright test"`. Leave all deps/scripts else unchanged.

### Stage C — CI (non-required)

9. **`.github/workflows/rust.yml`** — append a new job `e2e-browser` per 028 §3.6, with these options:
   - `services.postgres` `postgres:18`, health check, `ports: 5432:5432` — copy `rust.yml:41-54`.
   - `env:` exactly as 028 §3.6 (note `FLORA_APP__JWT_SECRET` ≥32 chars, `FLORA_APP__SESSION_TTL_SECS: '3600'`, `VITE_API_URL: http://127.0.0.1:3000`, `E2E_*` origins, `PGPASSWORD`). Keep messaging/search disabled.
   - Rust setup block copied verbatim from `rust.yml:10-21` (toolchain 1.98.0); `oven-sh/setup-bun@v2`; `bun install` in `apps/frontend`.
   - Build/lint: `cargo build -p flora-api` and `cargo clippy -p flora-core --bin flora-mint-session --features dev-mint -- -D warnings`.
   - ★ Add `cargo deny check -p flora-core --features dev-mint` (NV-13) — informational if it complicates the gate; do not touch the existing `cargo deny check bans licenses sources` in the `build` job.
   - `npx playwright install chromium --with-deps` in `apps/frontend`; `bun run test:e2e`.
   - `actions/upload-artifact@v4` with `if: ${{ !cancelled() }}`, `path: apps/frontend/playwright-report/`, `retention-days: 3`.
   - **Do not enable `dev-mint` in any existing job.** Leave `ci.yml` untouched. Keep the new job **non-required** for the first weeks.

10. **Docs (optional, doc-only):** `docs/dev-smoke.md:67,69` — replace the two expired static tokens with the mint command (028 §3.7). Do **not** modify `dev-seed.sql` or `scripts/dev-smoke-session.sql`.

---

## 5. Verification commands (for the implementing agent; NOT run in this pass)

```bash
# Rust / gating
cargo build -p flora-core --bin flora-mint-session --features dev-mint
cargo build --release                       # bin must NOT appear
cargo clippy -p flora-core --bin flora-mint-session --features dev-mint -- -D warnings
cargo fmt --all -- --check
cargo deny check -p flora-core --features dev-mint   # new optional deps (NV-13)

# Effective secret / claim shape (no DB)
FLORA_APP__JWT_SECRET=<32+ chars> cargo run -q -p flora-core \
  --bin flora-mint-session --features dev-mint -- --storage-state /tmp/auth.json
# decode payload: sub/jti/org/iat/exp/ver + signature verifies with the SAME secret
# repeat with a different FLORA_APP__JWT_SECRET to prove nested precedence (NV-4)

# Frontend
cd apps/frontend && bun run check && bun run lint && bun run build
VITE_API_URL=http://127.0.0.1:3000 bun run build   # grep bundle for 127.0.0.1:3000
```

---

## 6. Go / no-go

### 6.1 GO — minimal slice, conditional on five gates
Implement §4 stages A–C **only when all of these are true**:
1. `cargo build -p flora-core --bin flora-mint-session --features dev-mint` succeeds **and** plain `cargo build --release` does not produce the bin (Cargo Book §2.1).
2. Setup project's positive `GET /api/v1/organizations` returns 200 with Acme — proving secret, signature, claims, DB session and CORS align before the browser starts.
3. Injected `flora.auth.v1` is observed by `AuthState` on `http://127.0.0.1:4173` (`/select-org` lists Acme) with `organizationId: null`.
4. `VITE_API_URL` is verified in the built preview bundle as `http://127.0.0.1:3000` (Vite §2.3).
5. The job is added **non-required**, Chromium-only, **`retries: 0`**, `workers: 1`, artifacts on failure with ≤3-day retention.

### 6.2 NO-GO — explicitly excluded from this slice
- One smoke only; **no** full `workspace→channel→message→task→file` suite.
- **No** file-upload step (no mounted UI, no seeded file row; covered by DB-gated contracts).
- **No** parallel workers/shards (jti UNIQUE; account shared; F2 rotation).
- **No** Tauri, `adapter-static`, per-OS release jobs.
- **No** dev mint HTTP route, **no** `dev-mint` outside the e2e job, **no** production image enabling it.
- **No** fresh far-future static token in docs — mint-at-start only.
- **No** `retries: 1` (F2/NV-3).

---

## 7. Sources

**External (primary, directly fetched this pass):**
- Cargo: [The Cargo Book — Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) (`required-features`, `src/bin/`, binaries using the package lib, auto-discovery); [Features](https://doc.rust-lang.org/cargo/reference/features.html) (default off, `dep:` since 1.60, unification, resolver).
- Playwright: [Web server](https://playwright.dev/docs/test-webserver) (`url` vs deprecated `port`, readiness statuses, `env`, arrays, `reuseExistingServer`, `baseURL`, 60 s default); [Authentication](https://playwright.dev/docs/auth) (setup project + `storageState`, `.auth` gitignore/danger note, `outputDir` alternative, localStorage/IndexedDB coverage, sessionStorage caveat); [Retries](https://playwright.dev/docs/test-retries) (worker restart semantics); [BrowserContext storageState schema](https://playwright.dev/docs/api/class-browsercontext#browser-context-storage-state) and [addInitScript](https://playwright.dev/docs/api/class-browsercontext#browser-context-add-init-script) carried from 027/028.
- Vite: [Env Variables and Modes](https://vite.dev/guide/env-and-mode) — build-time static replacement, process-env highest priority, Bun `.env` preload note.
- GitHub Actions: [Creating PostgreSQL service containers](https://docs.github.com/en/actions/using-containerized-services/creating-postgresql-service-containers) (shape already in `rust.yml`).

**Local (repo, read-only, this checkout `43b21a675`):**
- `flora-core/Cargo.toml:19-64`; `flora-core/src/utils/jwt.rs:15-104`; `flora-core/src/config/mod.rs:33-49`; `flora-core/src/config/app.rs:5-93`; `flora-core/src/config/database.rs:7-25`; `flora-core/src/services/session_service.rs:80-82`; `flora-core/src/migrations/sql/20240818000001_init_schema.sql:27,128-141,191`; `flora-core/src/bin/` (absent).
- `flora-api/src/extractors/auth.rs:33-97,212-254`; `flora-api/src/routes/auth.rs:352-416`; `flora-api/src/routes/org.rs:80-119`; `flora-api/src/routes/mod.rs:20-31`; `flora-api/src/main.rs:26-39`; `flora-core/src/middleware/`→`flora-api/src/middleware/cors.rs:14-20` (via 028).
- `Cargo.toml:17-21,40-91,119-156,220-225`; `deny.toml:1-40`; `.env.example:8-24`; `dev-seed.sql:12-22,28-41,154-163`; `scripts/dev-smoke-session.sql:1-50`; `flora-tests/src/contract/auth.rs:504` et al.
- `apps/frontend/playwright.config.ts:1-6`; `apps/frontend/package.json:6-48`; `apps/frontend/vite.config.ts:1-47`; `apps/frontend/.gitignore:24-25`; `apps/frontend/src/lib/state/AuthState.svelte.ts:8-65,124-135`; `apps/frontend/src/lib/api/client.ts:3,73-110`; `apps/frontend/src/routes/(app)/select-org/+page.svelte:38-101`; `apps/frontend/src/routes/demo/playwright/page.svelte.e2e.ts:1-6`; `apps/frontend/tests/e2e/{tasks,files}.spec.ts`.
- `.github/workflows/rust.yml:1-85`; `.github/workflows/ci.yml:1-24`.
- Prior research: `2026-11-16-026-browser-e2e-tauri-decision.md`, `2026-11-16-027-e2e-jwt-bootstrap.md`, `2026-11-16-028-e2e-jwt-implementation-plan.md`.
