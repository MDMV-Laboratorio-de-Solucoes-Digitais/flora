# 028 — Exact implementation plan: dev-only `flora-mint-session`, Playwright `storageState` injection, and the Postgres 18 e2e CI job

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` (`e9ef62589`) — **research/design only. No source, manifest, config, seed, workflow, .gitignore or test file was modified by this pass.** The only artifact produced is this document.  
**Parent chain:** `026-browser-e2e-tauri-decision.md` (thin browser slice; NV-2 "fresh-mint helper") → `027-e2e-jwt-bootstrap.md` (mint path, claims, `jti` authority, secret precedence; NV-2…NV-12) → **this document** (turns 027's sketches into an exact, buildable, gate-by-gate plan and states the go/no-go).  
**Scope:** (1) a feature-gated `flora-mint-session` binary in `flora-core` (`required-features = ["dev-mint"]`, not default) that uses `dev-seed.sql` IDs and emits a Playwright `storageState` JSON; (2) the exact `playwright.config.ts` / setup-project / spec changes to inject it on the **frontend** origin; (3) the exact CI job shape (Postgres 18 service + API + frontend preview); (4) failure-mode table, risks/NVs and go/no-go for the minimal slice. Explicitly **out of scope**: Tauri, a dev HTTP mint route in `flora-api`, parallel/sharded e2e, the file-upload step, and turning the existing dead `.spec.ts` files back on.

**Method / multilingual cross-check.** Read-only audit of the checked-out files listed in §1 and §7, plus direct fetches (this pass) of the primary sources in §2. Multilingual verification was done where the docs publish locales: **Vite** `Env Variables and Modes` in EN / **es** / **zh-CN** / **ja** — all four state the load-bearing facts identically (static replacement at build time; process env has highest priority over `.env`; Bun auto-loads `.env` and can interfere); no **pt-BR** Vite locale exists (both `vite.dev/pt-br/...` and the probed path 404), so pt-BR coverage is carried from the MDN/GHA cross-checks in 026/027, not invented here. Playwright publishes programming-language variants but no natural-language locales (as established in 027:NV-10); Cargo Book is EN-only. GHA service-container semantics in pt-BR/ja/zh-CN were already cross-checked in `026:64`. No secondary blog was used for a load-bearing claim. Nothing in this document was executed.

---

## 0. Decision summary

| Question | Answer (this pass) |
|---|---|
| Is the mint design from 027 implementable as stated? | **Yes, with three hardening corrections**: (a) `clap`/`anyhow` are *not* dependencies of `flora-core` today and must be added as **optional** deps behind `dep:` so the production library surface is unchanged; (b) the minter should produce the **whole `storageState` file itself**, not just a token, so the harness has zero JWT knowledge; (c) `webServer.port` is **deprecated** — use `url`, and pin the preview host to `127.0.0.1` so the storage origin is deterministic. |
| Can the helper be kept out of production builds? | **Yes.** `default = []` + `required-features = ["dev-mint"]` on `[[bin]]` (Cargo Book: "if any of the required features are not enabled, the target will be skipped"), plus a runtime `env == "production"` refusal. Enable `--features dev-mint` **only** in the e2e job. No API route. |
| What is the e2e auth contract? | Mint a fresh JWT for the seeded **nil-org pre-selection** session (Alice `2222…`, jti `018f0000-…-0001`, org nil, `exp = now + ttl`) and inject `localStorage['flora.auth.v1']` on **`http://127.0.0.1:4173`** (the *frontend* origin) with `organizationId: null`; drive `select-organization` in the UI. The DB row from `scripts/dev-smoke-session.sql` already exists and is idempotent. |
| Does the seed row work with the schema? | **Yes — verified.** `sessions.organization_id` is `NOT NULL REFERENCES organizations(id)` (`20240818000001_init_schema.sql:128-141`) and migration line 27 inserts the nil-UUID `System` org, so the nil-org session row is FK-valid. jti is `UNIQUE` (`:132`) — one row per jti, which is why `workers: 1` is mandatory. |
| CI job shape? | One Linux job: `postgres:18` service with health check; Rust 1.98.0 + Bun; build API; install **Chromium only**; Playwright `webServer` starts API (`/health`) and preview (`/health`-ready app on 4173); a **setup project** seeds the DB, mints, writes the auth file and asserts a 200 from `GET /api/v1/organizations`; then one spec runs with `storageState`. Artifact on failure, short retention, non-blocking at first. |
| Go / no-go? | **GO for the minimal slice**, conditional on the five gates in §6.1. **NO-GO** for the full suite, file step, parallel workers, Tauri, and any production mint route. |
| Status | **Proposed, not implemented or executed.** All code/config below is a design sketch to be verified by the commands in §3.4. |

---

## 1. Read-only audit — exact baseline (new findings vs 027 marked ★)

| Area | Evidence in this checkout | Consequence for the plan |
|---|---|---|
| Mint primitive | `flora-core/src/utils/jwt.rs:16-33` `FloraClaims` with 7 **all-`pub`** fields (`ver` has `#[serde(default)]`); `:82-89` `encode_token(claims, secret)` = `encode(&Header::default(), …)`; `:96-104` `decode_token` uses `Validation::default()`; `:35-56` `new()` always derives `exp`/`iat` from `Utc::now()`. | Struct-literal construction is the **only** way to choose `exp` without editing production code. Round-trip via `decode_token` is a free correctness guard. |
| ★ Dependency gap | `flora-core/Cargo.toml:23-54` has `serde_json`, `chrono`, `uuid`, `jsonwebtoken`, `figment`, `thiserror` — but **no `clap` and no `anyhow`**. Root `Cargo.toml:156,221` declares them as *workspace* deps; no crate declares `clap` at all. | 027's "clap available as workspace dep" needs an actual member entry. Add both as **`optional = true`** in `flora-core` and gate them with `dev-mint = ["dep:clap", "dep:anyhow"]` so the lib/prod dependency graph is unchanged. |
| Features / bins today | `flora-core/Cargo.toml:61-64` `default = []`, `sqlite`, `test`; `[lib]` at `:19-21`. Repo-wide: **no `src/bin/`**, **no `cfg(feature = …)` in Rust source**. | A new `[[bin]]` + one feature is a small, consistent change; root lints (`:40-91`) require `#[expect(..., reason = "...")]`, never bare `#[allow]`. |
| Secret precedence | `flora-core/src/config/app.rs:47-48` default `jwt_secret` from `FLORA_JWT_SECRET` else `dev-secret-change-in-production`; validation `:79-86` enforces ≥32 chars **only** for production. `config/mod.rs:43-49`: `Figment::defaults(Self::default()).merge(Env::prefixed("FLORA_").split("__"))` — nested key merged last. `.env.example:14,20` both set the same value. | Minter must call **`Config::load()`**, not `AppConfig::default()`, or signatures diverge → generic 401. `.env` is **not** auto-loaded by Rust (`dotenv` declared at `flora-core/Cargo.toml:24` but unused in `*.rs`); process env only. |
| Server-side authority | `flora-api/src/extractors/auth.rs:33-97`: decode → parse `sub`/`organization_id` → `validate_session(jti)` → reject unless active, unexpired, same user **and** same org (nil allowed). `:213-226` `require_org()` rejects nil for scoped routes. | JWT alone is insufficient; the seeded session row is load-bearing. `GET /api/v1/organizations` uses `AuthExtractor` (not `require_org`), so it is a valid **drift guard** for a nil-org token (`flora-api/src/routes/org.rs:80-119`). |
| ★ Nil-org FK | Migration `20240818000001_init_schema.sql:27` inserts `organizations('00000000-…','System','system')`; `:128-141` `sessions.organization_id NOT NULL REFERENCES organizations(id)`, `jti VARCHAR(255) NOT NULL UNIQUE`; `:191` jti index. | Profile-A session SQL is FK-valid and idempotent. Do **not** invent a second org-scoped session row for the minimal slice. |
| Seed / session | `dev-seed.sql:12-22` Acme `1111…`; `:28-73` Alice `2222…`, Bob `3333…`, Carol `4444…`; `:154-163` Alice=Admin; `:192-202` workspaces `8888…`/`9999…`; `:318-320` **no sessions**. `scripts/dev-smoke-session.sql:10-48` upserts jti `018f0000-0000-7000-8000-000000000001`, Alice, **nil org**, `NOW() + 365 days`, active. | Reuse unchanged. DB TTL (365 d) ≫ JWT TTL (≤1 h); the expired static token is purely a JWT `exp` problem. |
| ★ API readiness | `flora-api/src/main.rs:27-35`: `Config::load()` → `validate()` → `create_pool()` → `run_migrations()` → bind listener. `flora-api/src/routes/mod.rs:23-25` mounts `/health`, `/health/ready`, `/health/live`. | Playwright `webServer.url: http://127.0.0.1:3000/health` is **real**, and a 200 there means migrations completed — so the setup project can safely seed afterwards. |
| ★ CORS | `flora-api/src/middleware/cors.rs:14-20` `allow_origin(Any)`, `allow_methods(Any)`, `allow_headers(Any)`; default `cors_origins` is `http://localhost:5173` but the middleware ignores it. | 4173→3000 preflighted `Authorization` fetch works today. Hardening later requires explicit allowlisting (risk R7). |
| Frontend storage | `apps/frontend/src/lib/state/AuthState.svelte.ts:8` key `flora.auth.v1`; `:14-31` synchronous load; `:43-50` constructor sets `isAuthenticated = Boolean(token)`; `:52-65` persists `{token,user,organizationId,workspaceId}`. `client.ts:3` `import.meta.env.VITE_API_URL \|\| 'http://localhost:3000'`; `:78-80` Bearer header; `:95-97` 401 → `lockSession()`. | Value must exist **before app JS runs**; storageState (applied at context creation) is stricter than `addInitScript`. No cookies → `cookies: []`. |
| Select-org UI | `apps/frontend/src/routes/(app)/select-org/+page.svelte:38-48` lists orgs; `:50-69` POST `select-organization` → `setOrganization(newToken, org)` → `goto('/tasks')`; `:91-101` redirects away when `organizationId` already set. | Inject `organizationId: null`, otherwise the rotation path is skipped. |
| ★ Vite build-time env | `apps/frontend/vite.config.ts` uses `@sveltejs/kit/vite` + `@sveltejs/adapter-node` (`svelte.config.js:5-14`). Vite docs (EN/es/zh/ja, §2.4): `VITE_*` values are **statically replaced at build time**; process env beats `.env`; **Bun preloads `.env` into `process.env`**. | `VITE_API_URL` must be in the **job env** at `npm run build` time; never rely on a checked-in `.env` (gitignored). Bun's `.env` behaviour is a local-only pitfall (R6). |
| ★ `webServer` shape | `apps/frontend/playwright.config.ts:1-6` only `{ command: 'npm run build && npm run preview', port: 4173 }` + `testMatch: '**/*.e2e.{ts,js}'`. Playwright docs: **`port` is deprecated → use `url`**; `env` defaults to inherited `process.env`; `webServer` supports an array; `reuseExistingServer` should be `!process.env.CI`. | Replace `port` with `url`, add the API server, add `use.baseURL`, `projects`, `dependencies`. Pin preview to `--host 127.0.0.1` so the origin literal is exact. |
| ★ Test discovery | `find` shows one collected e2e file: `apps/frontend/src/routes/demo/playwright/page.svelte.e2e.ts`. `apps/frontend/tests/e2e/{tasks,files}.spec.ts` are `.spec.ts` → **not collected** (and stale per 026:30). | For the slice set `testDir: './tests/e2e'` and keep `testMatch: '**/*.e2e.ts'`; the demo test and the two dead specs stay out. Do **not** broaden `testMatch` to `.spec.ts`. |
| ★ Gitignore gap | `apps/frontend/.gitignore` ends with `# Playwright\n test-results` — **no `playwright/.auth`** (root `.gitignore` has none either). | Add `playwright/.auth` (belt) and write the auth file under the project `outputDir` (braces) — Playwright explicitly recommends `outputDir` for auto-cleanup. |
| Existing mint idiom | `flora-tests/src/contract/auth.rs:501-513` seeds a session then `encode_token(&FloraClaims::new(…), &config.app.jwt_secret)`; same pattern in `messaging.rs`, `websocket.rs`, `files.rs`. | The helper mirrors an established repo pattern; only the chosen-`exp` literal differs. |
| Expired token | `docs/dev-smoke.md:64-70` two static tokens, `exp=1794787200` = 2026-11-16T00:00:00Z (expired ~13h at 027 research time), one per secret. | Replace with mint-at-start; never publish a new far-future constant. |
| CI today | `.github/workflows/rust.yml:26-37` fmt/strict clippy/cargo-deny/build/test; `:39-85` `db-tests` with `postgres:18` service + 6 ignored suites. `.github/workflows/ci.yml:1-24` Bun frontend check/lint/unit/build — **no e2e**. | Add a **new job**, leave both existing workflows' gates untouched. Feature-gated bin is skipped by `cargo clippy --workspace --lib --bins` unless `dev-mint` is enabled — lint it explicitly in the e2e job. |

---

## 2. External primary sources (directly verified this pass)

### 2.1 Cargo — binaries, `required-features`, optional deps ([Cargo Book: Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html), [Features](https://doc.rust-lang.org/cargo/reference/features.html))
- "Binary targets are executable programs … A binary's source can be `src/main.rs` and/or stored in the `src/bin/` directory." "Binaries can use the public API of the package's library." → the gated bin can `use flora_core::…` without duplicating claim logic.
- `required-features`: "specifies which features the target needs in order to be built. **If any of the required features are not enabled, the target will be skipped.** This is only relevant for `[[bin]]`, `[[bench]]`, `[[test]]` and `[[example]]`, it has no effect on `[lib]`." → normal `cargo build`/`--release` and CI's `--lib --bins` never compile the minter.
- Features are additive and disabled unless in `default`; optional dependency syntax `dep:clap` keeps the feature's effect confined to dependency activation; with `resolver = "3"` (root `Cargo.toml:18`) feature unification cannot silently enable the bin in the API.

### 2.2 Playwright — auth, storage, config ([Authentication](https://playwright.dev/docs/auth), [Web server](https://playwright.dev/docs/test-webserver), [BrowserContext](https://playwright.dev/docs/api/class-browsercontext#browser-context-storage-state), [addInitScript](https://playwright.dev/docs/api/class-browsercontext#browser-context-add-init-script))
1. Setup project → save auth state → `use.storageState` in dependent projects is the **recommended** pattern for tests without server-side state; with `workers: 1` and a disposable DB, Flora fits it.
2. Danger note: the state file "may contain sensitive cookies and headers that could be used to impersonate … strongly discourage checking them into … repositories." Recommended: `playwright/.auth` **gitignored**; if state need not persist between runs, write under `testProject.outputDir` (auto-cleaned). → do both.
3. Reusing state covers cookies, **localStorage**, IndexedDB and passkeys; `sessionStorage` is *not* persisted by Playwright (only relevant if Flora ever moves off localStorage).
4. Exact `storageState` object schema (verified in the API ref): `{ cookies: [], origins: [ { origin: string, localStorage: [ { name: string, value: string } ] } ] }`. `origin` must be the exact frontend origin.
5. `addInitScript` runs "after the document was created but before any of its scripts were run"; order among multiple init scripts is **not defined**. `storageState` avoids this entirely by seeding at context creation → prefer it.
6. `webServer`: `url` (status 2xx/3xx/400/401/402/403 = ready) is current; **`port` is deprecated**; `env` inherits `process.env` + `PLAYWRIGHT_TEST=1`; arrays launch multiple servers; `reuseExistingServer: !process.env.CI`; `timeout` default 60 s (raise to 120 s for `cargo build`).
7. `baseURL` makes `page.goto('/…')` resolve against it — and therefore fixes the origin that localStorage is scoped to.

### 2.3 SvelteKit / Svelte CLI ([sv add playwright](https://svelte.dev/docs/cli/playwright), [Integrations](https://svelte.dev/docs/kit/integrations), [Auth](https://svelte.dev/docs/kit/auth))
- `sv add playwright` produces scripts + config + `.gitignore` entries + a demo test; Flora's `apps/frontend` matches that shape, so the gap is wiring, not tooling. SvelteKit's auth guidance distinguishes DB-backed sessions from JWTs; Flora's bearer+DB-`sessions` rotation must be driven as a real flow, not assumed to be cookie-based. `adapter-node` is fine for a browser preview.

### 2.4 Vite env & modes ([EN](https://vite.dev/guide/env-and-mode) · [es](https://es.vite.dev/guide/env-and-mode) · [zh-CN](https://cn.vite.dev/guide/env-and-mode) · [ja](https://ja.vite.dev/guide/env-and-mode))
- EN: "defined as global variables during dev and **statically replaced at build time**"; "environment variables that already exist when Vite is executed have the **highest priority** and will not be overwritten by `.env` files"; "**Bun users** … Bun automatically loads `.env` files before your script runs … can interfere with Vite's feature."
- es: "se reemplazan estáticamente en el momento de la compilación"; "las variables de entorno que ya existen cuando se ejecuta Vite tienen la mayor prioridad"; same Bun note.
- zh-CN: "在构建阶段被静态替换"; "Vite 执行时已经存在的环境变量有最高的优先级"; same Bun note.
- ja: "ビルド時には…静的に置き換えられます"; "Vite の実行時に既に存在している環境変数は最も優先度が高く"; same Bun note.
- **No divergence.** No pt-BR Vite locale (404). Consequence: set `VITE_API_URL` in the job/`webServer.env`, and ensure no local `apps/frontend/.env` shadows it.

### 2.5 GitHub Actions ([PostgreSQL service containers](https://docs.github.com/en/actions/using-containerized-services/creating-postgresql-service-containers) — pt-BR/ja/zh-CN cross-check carried from `026:64`; [GHA billing](https://docs.github.com/en/billing/managing-billing-for-github-actions/about-billing-for-github-actions))
- `services.postgres` with `POSTGRES_PASSWORD`, `ports: ['5432:5432']`, `--health-cmd pg_isready …` and `localhost` networking is the documented pattern and is already implemented in `rust.yml:41-54`.
- Linux 2-core ≈ $0.006/min (2026 list price); a 6–10 min Chromium job ≈ $0.04–$0.06/run. Ship non-blocking first.

---

## 3. Exact implementation plan (ordered, file-by-file)

### 3.1 Step A — `flora-core/Cargo.toml` (manifest only; no production source edit)

```toml
[dependencies]
# ... existing ...
# NEW: dev-only CLI deps; optional so the lib/prod graph is unchanged.
clap = { workspace = true, optional = true }
anyhow = { workspace = true, optional = true }

[features]
default = []                                   # unchanged — NOT default
dev-mint = ["dep:clap", "dep:anyhow"]          # NEW: gates deps + bin
sqlite = ["sqlx/sqlite"]
test = []

[[bin]]                                        # NEW
name = "flora-mint-session"
path = "src/bin/flora-mint-session.rs"
required-features = ["dev-mint"]
```

Notes: `serde_json`, `chrono`, `uuid`, `jsonwebtoken`, `figment` are already regular deps, so the bin needs no other additions. Do **not** add a `flora-api` route. If `optional = true` with `workspace = true` is rejected by the installed Cargo (NV-2), fall back to a literal `clap = { version = "4.5", features = ["derive", "env"], optional = true }` (same version as root `Cargo.toml:221`, so `multiple_crate_versions` stays clean).

### 3.2 Step B — `flora-core/src/bin/flora-mint-session.rs` (new, dev-only)

Sketch; every `?`/`bail!` is there to satisfy `unwrap_used`/`expect_used`/`panic` = deny and `unused_results` = deny (`Cargo.toml:78-87`).

```rust
//! Dev-only: mint a seeded Flora pre-selection JWT and emit a Playwright
//! storageState file. Never built without the `dev-mint` feature.
#![expect(
    clippy::print_stdout,
    reason = "dev CLI emits the storageState JSON consumed by the e2e harness"
)]

use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use chrono::Utc;
use clap::Parser;
use flora_core::config::Config;
use flora_core::utils::jwt::{decode_token, encode_token, FloraClaims};

// Fixed IDs from dev-seed.sql + scripts/dev-smoke-session.sql (profile A).
const ALICE: &str = "22222222-2222-2222-2222-222222222222";
const JTI: &str = "018f0000-0000-7000-8000-000000000001";
const NIL_ORG: &str = "00000000-0000-0000-0000-000000000000";
const EMAIL: &str = "alice@acme.test";
const KEY: &str = "flora.auth.v1";

#[derive(Debug, Parser)]
#[command(name = "flora-mint-session", about = "DEV ONLY — mint a seeded e2e session")]
struct Args {
    /// Destination storageState JSON; "-" writes to stdout.
    #[arg(long, default_value = "-", env = "E2E_STORAGE_STATE")]
    storage_state: String,
    /// Token TTL seconds (default: effective FLORA_APP__SESSION_TTL_SECS / 3600).
    #[arg(long, env = "E2E_TTL_SECS")]
    ttl_secs: Option<u64>,
    /// Must equal the Playwright baseURL origin.
    #[arg(long, env = "E2E_FRONTEND_ORIGIN", default_value = "http://127.0.0.1:4173")]
    origin: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let cfg = Config::load().context("load effective Flora config")?;   // nested key wins
    if cfg.app.env == "production" {
        bail!("refusing to mint a session: config env is production");
    }
    let ttl = args.ttl_secs.unwrap_or(cfg.app.session_ttl_secs);
    let now = Utc::now();
    let claims = FloraClaims {
        sub: ALICE.to_owned(),
        jti: JTI.to_owned(),
        organization_id: NIL_ORG.to_owned(),
        email: EMAIL.to_owned(),
        iat: now.timestamp(),
        exp: now.timestamp() + i64::try_from(ttl).context("ttl_secs exceeds i64")?,
        ver: 1,
    };
    let token = encode_token(&claims, &cfg.app.jwt_secret);
    let roundtrip = decode_token(&token, &cfg.app.jwt_secret).context("mint roundtrip failed")?;
    if roundtrip.jti != claims.jti || roundtrip.exp != claims.exp || roundtrip.sub != claims.sub {
        bail!("minted claims drifted from the requested claims");
    }

    let stored = serde_json::json!({
        "token": token,
        "user": { "id": ALICE },
        "organizationId": null,   // profile A: exercise select-organization
        "workspaceId": null
    });
    let state = serde_json::json!({
        "cookies": [],
        "origins": [{
            "origin": args.origin,
            "localStorage": [{ "name": KEY, "value": stored.to_string() }]
        }]
    });
    let rendered = serde_json::to_string_pretty(&state).context("serialize storageState")?;
    if args.storage_state == "-" {
        println!("{rendered}");
    } else {
        let path = PathBuf::from(&args.storage_state);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).context("create auth dir")?;
        }
        std::fs::write(&path, format!("{rendered}\n")).context("write storageState")?;
    }
    Ok(())
}
```

Design decisions to preserve:
- **Literal claims**, not `FloraClaims::new` (which cannot choose `exp`) — `jwt.rs:35-56`.
- **Whole storageState file**, not just a token: the Playwright setup project only has to run one command and point `use.storageState` at the file.
- **`--origin` from `E2E_FRONTEND_ORIGIN`** (clap `env` feature already enabled at root `:221`) so the minter and `playwright.config.ts` share one source of truth.
- **Round-trip decode** is the cheap "single-source-of-truth" test from 027:158 without a `#[cfg(test)]` module (the bin is gated, so CI never runs its tests by default). Keep JWT TTL ≤ DB session TTL; default comes from the effective config.
- `--help` text must say "DEV ONLY — never enable `dev-mint` in a production image".
- If the legacy dev default secret is in use, the setup project's positive API assertion (Step E) is the drift detector; no need to hard-code a secret value in Rust.

### 3.3 Step C — `apps/frontend/playwright.config.ts` (replace the 6-line file)

```ts
import { defineConfig, devices } from '@playwright/test';

const FRONTEND_ORIGIN = process.env.E2E_FRONTEND_ORIGIN ?? 'http://127.0.0.1:4173';
const API_ORIGIN = process.env.E2E_API_ORIGIN ?? 'http://127.0.0.1:3000';
const AUTH_FILE = 'playwright/.auth/alice.json';

export default defineConfig({
  testDir: './tests/e2e',          // excludes demo e2e + the two dead .spec.ts files
  testMatch: '**/*.e2e.ts',
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: process.env.CI ? 1 : undefined,   // jti is UNIQUE — no parallel workers
  globalTimeout: 10 * 60 * 1000,
  reporter: [['html', { open: 'never' }], ['list']],
  use: { baseURL: FRONTEND_ORIGIN, trace: 'on-first-retry' },
  projects: [
    { name: 'setup', testMatch: /.*\.setup\.ts/ },
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'], storageState: AUTH_FILE },
      dependencies: ['setup'],
    },
  ],
  webServer: [
    {
      name: 'api',
      command: 'cargo run -q -p flora-api',
      url: `${API_ORIGIN}/health`,          // 200 ⇒ migrations applied (main.rs:27-35)
      timeout: 120_000,
      reuseExistingServer: !process.env.CI,
    },
    {
      name: 'web',
      // Pin host so localStorage origin is exactly FRONTEND_ORIGIN.
      command: 'npm run build && npm run preview -- --host 127.0.0.1 --port 4173',
      url: FRONTEND_ORIGIN,
      timeout: 120_000,
      reuseExistingServer: !process.env.CI,
    },
  ],
});
```

Notes:
- `VITE_API_URL` is inherited by `webServer.env` from `process.env`, so the **build** inside `command` bakes the CI value into the bundle (Vite §2.4). If the frontend project's own scripts ever use Bun in CI, keep `VITE_API_URL` in the job env (process env beats `.env`).
- `url` replaces the deprecated `port`; the API `/health` path is real (`routes/mod.rs:23`). If a future API change removes it, switch to `/health/live` — but `/health` currently also reports capabilities, useful for debugging.
- Do **not** set `storageState` on the `setup` project (the file does not exist yet).
- If CI ever starts the API itself in an earlier step, set that entry's `reuseExistingServer: true` in CI or Playwright will refuse to start a second server on the same `url`. This plan lets Playwright own both servers.

### 3.4 Step D — `apps/frontend/tests/e2e/seed.setup.ts` (new setup project; runs first)

```ts
import { execFileSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { expect, test as setup } from '@playwright/test';

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, '../../../');          // tests/e2e -> apps/frontend -> apps -> root
const authFile = path.join(here, '../../playwright/.auth/alice.json');
const env = { ...process.env, PGPASSWORD: process.env.PGPASSWORD ?? 'postgres' };

setup('seed + mint + verify dev session', async ({ request }) => {
  // 1) DB preconditions — API webServer has already applied migrations.
  for (const sql of ['dev-seed.sql', 'scripts/dev-smoke-session.sql']) {
    execFileSync('psql', ['-v', 'ON_ERROR_STOP=1', '-h', '127.0.0.1', '-U', 'postgres',
      '-d', 'flora', '-f', path.join(repoRoot, sql)], { env, stdio: 'inherit' });
  }

  // 2) Fresh JWT + whole storageState, minted with the API's effective secret.
  execFileSync('cargo', ['run', '-q', '-p', 'flora-core', '--bin', 'flora-mint-session',
    '--features', 'dev-mint', '--', '--storage-state', authFile],
    { cwd: repoRoot, env, stdio: 'inherit' });

  // 3) Drift/secret guard: a nil-org token must still list Alice's orgs.
  const state = JSON.parse(await (await import('node:fs/promises')).readFile(authFile, 'utf-8'));
  const token = JSON.parse(state.origins[0].localStorage[0].value).token as string;
  const res = await request.get(`${process.env.E2E_API_ORIGIN ?? 'http://127.0.0.1:3000'}/api/v1/organizations`, {
    headers: { Authorization: `Bearer ${token}` },
  });
  expect(res.ok(), `auth guard failed: ${res.status()} ${await res.text()}`).toBeTruthy();
  const orgs = await res.json();
  expect(orgs).toEqual(expect.arrayContaining([
    expect.objectContaining({ id: '11111111-1111-1111-1111-111111111111' }),
  ]));
});
```

Why this shape:
- Runs only after both `webServer`s are ready; migrations are guaranteed by `main.rs`.
- `psql -v ON_ERROR_STOP=1` makes seed failures loud (both SQL files are idempotent upserts).
- The **positive API assertion** is the mandatory drift detector: a wrong secret, an expired token, a missing/mismatched session row, or a stale claim shape all collapse to a non-2xx here, before any browser test runs. `GET /api/v1/organizations` is nil-org-safe (`org.rs:88-119` uses `AuthExtractor`, not `require_org`).
- If `psql` is unavailable on the runner (NV-6), move steps 1–2 into a dedicated CI step that starts the API itself and set `reuseExistingServer: true` for the api entry.

### 3.5 Step E — spec + gitignore + package script changes

1. **New `apps/frontend/tests/e2e/seed-flow.e2e.ts`** (the one smoke; full flow is specified in `026:101-107`, not re-derived here). It inherits `use.storageState` from the `chromium` project, so no `addInitScript` is needed. First assertion should be `GET /select-org` shows Acme (proves the injected token was read by `AuthState`); then select Acme → `/tasks`; unique suffixes via `crypto.randomUUID()`; assert via API postconditions where the UI is ambiguous (`026:37`).
2. **`apps/frontend/.gitignore`**: add `playwright/.auth` under the existing `# Playwright` block (which currently has only `test-results`). Belt-and-braces even though the file should live in `outputDir`.
3. **`apps/frontend/package.json:17`**: change `"test:e2e": "playwright install && playwright test"` → `"test:e2e": "playwright test"`, and install **only Chromium with OS deps** in CI (`npx playwright install chromium --with-deps`). This follows Playwright CI/best-practice guidance from `026:51` and avoids downloading all browsers. Local devs run the install once.
4. **Do not** change `testMatch` to accept `.spec.ts`: `tasks.spec.ts` and `files.spec.ts` are stale/dead (`026:30`) and would produce red CI. Delete or rewrite them in a separate change.

**Fallback (only if the Rust toolchain in the e2e worker is undesirable):** a ~15-line Node HMAC minter in the setup project (027:162). It must read `FLORA_APP__JWT_SECRET` → `FLORA_JWT_SECRET` → dev default, emit exactly `{"alg":"HS256","typ":"JWT"}` and the six `FloraClaims` fields, and **must** keep Step D's positive API assertion — it is the only drift detector when claim shape is duplicated. This path is a deliberate deviation from "reuse `encode_token`".

### 3.6 Step F — CI job shape (new job; append to `.github/workflows/rust.yml`)

```yaml
  e2e-browser:
    runs-on: ubuntu-latest   # Linux 2-core ≈ $0.006/min (026:66)
    services:
      postgres:
        image: postgres:18
        env:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: flora
        ports: ['5432:5432']
        options: >-
          --health-cmd "pg_isready -U postgres -d flora"
          --health-interval 10s --health-timeout 5s --health-retries 10
    env:
      FLORA_DATABASE__POSTGRES_URL: postgresql://postgres:postgres@localhost:5432/flora
      FLORA_APP__JWT_SECRET: change-me-to-a-random-secret-at-least-32-characters-long
      FLORA_APP__SESSION_TTL_SECS: '3600'
      FLORA_SEARCH__ENABLED: 'false'
      FLORA_MESSAGING__ENABLED: 'false'
      VITE_API_URL: http://127.0.0.1:3000       # baked into the preview build
      E2E_API_ORIGIN: http://127.0.0.1:3000
      E2E_FRONTEND_ORIGIN: http://127.0.0.1:4173
      PGPASSWORD: postgres
    steps:
      - uses: actions/checkout@v4
      - name: Set up Rust 1.98.0
        run: |  # reuse the rust.yml:10-21 block verbatim (manual rustup install)
          set -euo pipefail
          rustup toolchain install 1.98.0 --profile minimal --no-self-update
          rustup default 1.98.0
          rustup component add rustfmt clippy
      - uses: oven-sh/setup-bun@v2
        with: { bun-version: latest }
      - run: bun install
        working-directory: apps/frontend
      - name: Build API + lint the dev-gated bin
        run: |
          cargo build -p flora-api
          cargo clippy -p flora-core --bin flora-mint-session --features dev-mint -- -D warnings
      - name: Install Chromium only
        run: npx playwright install chromium --with-deps
        working-directory: apps/frontend
      - name: Browser e2e
        run: bun run test:e2e
        working-directory: apps/frontend
      - uses: actions/upload-artifact@v4
        if: ${{ !cancelled() }}
        with:
          name: playwright-report
          path: apps/frontend/playwright-report/
          retention-days: 3
```

Job rules:
- **Do not** enable `dev-mint` in any existing job; `rust.yml:29` strict clippy and `:35` build must stay exactly as-is. The bin is compiled/linted only in this job.
- Keep the job **non-required** for the first weeks; promote once green/stable (`026:181`). Do not add Firefox/WebKit or macOS.
- Optional fast-fail addition: after checkout, assert `git ls-files apps/frontend/.env` is empty — env files are gitignored, so a checked-in one would be a mistake.

### 3.7 Step G — refresh `docs/dev-smoke.md` (doc-only)

Replace the two static token strings at `docs/dev-smoke.md:67,69` with the mint command:
```bash
cargo run -q -p flora-core --bin flora-mint-session --features dev-mint -- \
  --storage-state /tmp/flora-e2e.json
```
Keep them only as clearly-labelled *expired illustration* if needed, and state which secret each belongs to. Never hand-edit the `exp` integer (HMAC covers the payload). Do **not** modify `dev-seed.sql` or `scripts/dev-smoke-session.sql`.

### 3.8 Step H — verification commands (to run by the implementing agent; NOT run in this pass)

```bash
# manifest/gating gates
cargo build -p flora-core --bin flora-mint-session --features dev-mint   # must succeed
cargo build --release                                                   # bin must be absent
cargo clippy -p flora-core --bin flora-mint-session --features dev-mint -- -D warnings
cargo fmt --all -- --check

# effective-secret / claim gates (no DB needed)
FLORA_APP__JWT_SECRET=<32+ chars> cargo run -q -p flora-core \
  --bin flora-mint-session --features dev-mint -- --storage-state /tmp/auth.json
# decode payload; assert sub/jti/org/exp/iat/ver and that the signature verifies with the same secret

# frontend gates
cd apps/frontend && bun run check && bun run lint && bun run build
VITE_API_URL=http://127.0.0.1:3000 bun run build   # grep bundle for 127.0.0.1:3000
```

---

## 4. Failure-mode → symptom → fix (use this before debugging)

| Symptom | Most likely cause | Check / fix |
|---|---|---|
| Every request 401 "Invalid or expired session"; `AuthState.lockSession()` fires | Secret mismatch (nested `FLORA_APP__JWT_SECRET` beat the flat one) or expired `exp` | Decode the JWT; compare `Config::load()` secret on both API and minter (027 §4.4); re-mint (never hand-edit `exp`). |
| 401 with a valid signature and future `exp` | `sessions` row missing/mismatched (jti, user, org, active, expiry) | Re-run `scripts/dev-smoke-session.sql`; confirm `jti`, nil org and Alice via SQL. |
| 400 "Organization context required" | Injected `organizationId` non-null or wrong claims profile | Inject `organizationId: null` and let the UI rotate the session. |
| `/select-org` immediately redirects to `/tasks` / shows no orgs | Stale `storageState` with a non-null org, or token not read at all | Regenerate auth file; confirm `flora.auth.v1` exists on the **frontend** origin; check `AuthState` constructor timing. |
| localStorage key present but app sees no token | Wrong origin (`localhost` vs `127.0.0.1`, or port) in `storageState.origins[0].origin` | Make `E2E_FRONTEND_ORIGIN`, preview `--host`, `webServer.url` and `use.baseURL` the same literal. |
| Browser calls `localhost:3000` while CI API is on `127.0.0.1:3000` | `VITE_API_URL` missing at **build** time; Vite baked the fallback | Set `VITE_API_URL` in job env; rebuild; grep bundle (§2.4). |
| CORS preflight failure on `Authorization` | CORS later tightened | Allow `http://127.0.0.1:4173` explicitly; today `cors_layer()` is `Any` (`cors.rs:14-20`). |
| `psql: command not found` / seed fails | Runner/client tooling or path issue | Verify runner has postgresql-client; use `ON_ERROR_STOP=1`; fallback in §3.4. |
| Playwright refuses to start API port already in use | `reuseExistingServer: false` in CI with an externally started API | Let Playwright own the server, or set `reuseExistingServer: true` (§3.3 note). |
| CI clippy suddenly compiles the minter | Someone enabled `dev-mint` outside the e2e job (feature unification, `resolver = "3"`) | Keep the feature out of all production/release invocations; re-check `cargo build --release`. |

---

## 5. Risks / NVs (severity: H = gate before merge, M = watch, L = note)

- **NV-1 (H) — Nothing executed.** All sketches are designs. No build, clippy, migration, seed load, API start, Playwright run or CI run was attempted. Every §3.8 command is a gate.
- **NV-2 (H) — Manifest mechanics uncompiled.** `optional = true` with `workspace = true` for `clap`/`anyhow`, `required-features` gating under `--bins`, and lint cleanliness of the new bin were not compiled. Fallback in §3.1 if Cargo rejects the inheritance.
- **NV-3 (H) — Secret precedence not runtime-verified.** Figment nested-key override is derived from merge order (`config/mod.rs:43-49`), not from two-secret minting. Gate: mint with two different `FLORA_APP__JWT_SECRET` values and observe which validates.
- **NV-4 (H) — Origin equality not browser-verified.** `http://127.0.0.1:4173` is inferred from the preview port + `addInitScript` semantics. Vite preview's default host binding was not probed; pin `--host 127.0.0.1` and assert in a real browser.
- **NV-5 (M) — `psql` availability / seed ordering on the runner.** `rust.yml` DB job uses a service container but never invokes `psql` as a client; `ON_ERROR_STOP=1` path/cwd unverified. Fallback in §3.4.
- **NV-6 (M) — Bun `.env` preload (Vite §2.4).** CI has no checked-in `.env`, but a developer's local `apps/frontend/.env` can shadow `VITE_API_URL` because Bun loads it into `process.env` before Vite runs. Keep the value in the process env; document the pitfall.
- **NV-7 (M) — Profile-A rotation not executed.** `select-organization` with the seeded Admin membership (`dev-seed.sql:154-163`) and the UI flow were not run (also `026:NV-3`).
- **NV-8 (M) — WS with lean backend.** `ChatLayout` calls `wsClient.connect()` while `FLORA_MESSAGING__ENABLED=false`; whether it retries noisily or blocks message assertions is unverified (`026:NV-8`).
- **NV-9 (M) — Dead/stale specs.** `tests/e2e/tasks.spec.ts` / `files.spec.ts` remain dead by design; `testDir` keeps them out. Re-enabling them is a separate task.
- **NV-10 (M) — Node HMAC fallback (if chosen) untested.** base64url segments and `jsonwebtoken` `Validation::default()` details were not checked against a live API; the positive API assertion is mandatory.
- **NV-11 (L) — CORS hardening.** Permissive `Any` CORS is a pre-existing security topic, out of scope; tightening later needs an explicit preview-origin allowlist (`026:NV-6`).
- **NV-12 (L) — Parallelism/sharding.** `workers: 1` is assumed; jti is `UNIQUE` and the account is shared. Parallel runs need per-worker jti/account namespaces (`026:NV-12`, `027:NV-12`).
- **NV-13 (L) — Multilingual coverage gaps.** Vite has no pt-BR locale (404); Playwright has no natural-language locales; Cargo Book is EN-only. pt-BR/ja/zh-CN GHA and MDN storage facts are carried from 026/027, not re-fetched here.
- **NV-14 (M) — CI cost/plan unknown.** ~$0.04–$0.06/run estimate uses public list prices; the repo's actual plan and public/private status were not checked (`026:NV-11`).
- **NV-15 (L) — `port` deprecation.** Current config uses `port: 4173`; switching to `url` is required by current Playwright docs but not verified against the installed 1.60 runtime.

---

## 6. Go / no-go

### 6.1 GO — minimal slice, conditional on five gates

Implement the slice in §3.1–§3.7 **only when all of these are true**:
1. `cargo build -p flora-core --bin flora-mint-session --features dev-mint` succeeds **and** a plain `cargo build --release` does not produce the bin (Cargo Book §2.1).
2. The setup project's positive `GET /api/v1/organizations` assertion returns 200 with Acme — proving secret, signature, claims, DB session and CORS all align before the browser starts.
3. The injected `flora.auth.v1` is observed by `AuthState` on `http://127.0.0.1:4173` (assert the `/select-org` page lists Acme) with `organizationId: null`.
4. `VITE_API_URL` is verified in the built preview bundle as `http://127.0.0.1:3000` (Vite §2.4).
5. The job is added **non-required**, Chromium-only, `workers: 1`, with artifacts on failure and ≤3-day retention.

### 6.2 NO-GO — explicitly excluded from this slice

- **No** full `workspace→channel→message→task→file` suite in the first PR: one smoke only (`026:0,183-187`).
- **No** file-upload step: no mounted UI and no seeded file row; covered by DB-gated contracts (`026:38,89`).
- **No** parallel workers/shards: jti is a unique key and the account is shared.
- **No** Tauri, no `adapter-static`, no per-OS release jobs (`026:5`).
- **No** dev mint HTTP route in `flora-api`, no `dev-mint` outside the e2e job, no production image that enables the feature.
- **No** fresh far-future static token in `docs/dev-smoke.md`; mint-at-start only.

---

## 7. Sources

**External (primary, verified this pass):**
- Cargo: [The Cargo Book — Cargo Targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html) (`required-features`, `src/bin/`, binaries using the package lib), [Features](https://doc.rust-lang.org/cargo/reference/features.html) (default off, `dep:`, unification).
- Playwright: [Authentication](https://playwright.dev/docs/auth) ([core concepts / danger note](https://playwright.dev/docs/auth#core-concepts), [basic/setup project](https://playwright.dev/docs/auth#basic-shared-account-in-all-tests), [outputDir note](https://playwright.dev/docs/auth#session-storage)), [Web server](https://playwright.dev/docs/test-webserver) (`url` vs deprecated `port`, `env`, arrays, `reuseExistingServer`), [BrowserContext storageState schema](https://playwright.dev/docs/api/class-browsercontext#browser-context-storage-state), [addInitScript](https://playwright.dev/docs/api/class-browsercontext#browser-context-add-init-script).
- Svelte/SvelteKit: [sv add playwright](https://svelte.dev/docs/cli/playwright), [Integrations](https://svelte.dev/docs/kit/integrations), [Auth](https://svelte.dev/docs/kit/auth).
- Vite: [Env Variables and Modes](https://vite.dev/guide/env-and-mode) — [es](https://es.vite.dev/guide/env-and-mode), [zh-CN](https://cn.vite.dev/guide/env-and-mode), [ja](https://ja.vite.dev/guide/env-and-mode); Bun `.env` note.
- GitHub Actions: [Creating PostgreSQL service containers](https://docs.github.com/en/actions/using-containerized-services/creating-postgresql-service-containers) (localized cross-check carried from `026:64`), [GHA billing](https://docs.github.com/en/billing/managing-billing-for-github-actions/about-billing-for-github-actions).
- `jsonwebtoken` 11 / `Header::default()` HS256+`typ=JWT`: carried from `027:64-66` (cites [docs.rs](https://docs.rs/jsonwebtoken/11.0.0/jsonwebtoken/struct.Header.html)); not re-fetched.
- MDN `localStorage` origin-scoping in en/pt-BR/es/de/fr/zh-CN/ja: carried from `027:68-70`; not re-fetched.

**Local (repo, read-only):**
- `flora-core/src/utils/jwt.rs:15-104,106-159`; `flora-core/src/config/app.rs:20-23,38-65,73-93`; `flora-core/src/config/mod.rs:33-64`; `flora-core/Cargo.toml:19-64`; `flora-core/src/error.rs:1-20`; `flora-core/src/lib.rs:13-32`; `flora-core/src/migrations/mod.rs:1-24`; `flora-core/src/migrations/sql/20240818000001_init_schema.sql:27,128-141,191`.
- `flora-api/src/extractors/auth.rs:21-97,212-254`; `flora-api/src/routes/auth.rs:261-416` (per 027); `flora-api/src/routes/org.rs:80-119`; `flora-api/src/routes/mod.rs:23-25`; `flora-api/src/routes/health.rs:1-70`; `flora-api/src/middleware/cors.rs:14-20`; `flora-api/src/main.rs:10-35`.
- `Cargo.toml:17-33,40-91,119-156,220-225`; `.env.example:8-24`; `dev-seed.sql:12-22,28-73,154-163,192-202,318-320`; `scripts/dev-smoke-session.sql:1-50`; `docs/dev-smoke.md:47-90`.
- `apps/frontend/playwright.config.ts:1-6`; `apps/frontend/package.json:6-48`; `apps/frontend/vite.config.ts:1-48`; `apps/frontend/svelte.config.js:1-14`; `apps/frontend/.gitignore` (`# Playwright` block); `apps/frontend/src/lib/state/AuthState.svelte.ts:1-65`; `apps/frontend/src/lib/api/client.ts:3,73-110`; `apps/frontend/src/routes/(app)/select-org/+page.svelte:38-101`; `apps/frontend/tests/e2e/tasks.spec.ts`, `apps/frontend/tests/e2e/files.spec.ts`, `apps/frontend/src/routes/demo/playwright/page.svelte.e2e.ts`.
- `flora-tests/src/contract/auth.rs:501-520`; `.github/workflows/rust.yml:1-85`; `.github/workflows/ci.yml:1-24`; `git status` clean on `dev` @ `e9ef62589`.
- Prior research: `docs/research/2026-11-16-026-browser-e2e-tauri-decision.md` (§0, §4, §6, NV-2/3/6/8/11/12) and `docs/research/2026-11-16-027-e2e-jwt-bootstrap.md` (§0, §3, §4, §5).
