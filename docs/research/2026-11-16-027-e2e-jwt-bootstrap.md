# 027 — Deterministic Flora session-JWT bootstrap for browser e2e (no live OIDC)

**Date:** 2026-11-16  
**Repo:** `flora` @ `dev` — **research/design only** (no source, config, Cargo, seed, workflow or test changes).  
**Scope:** How to mint a *valid* Flora session JWT deterministically for the canonical Svelte 5 frontend e2e (`apps/frontend`) without a live IdP, reusing `flora_core::utils::jwt::encode_token` + `FloraClaims`; how to inject it through Playwright (`addInitScript` / `storageState`); how to keep the mint path out of production builds; and how to refresh the expired static token in `docs/dev-smoke.md`.  
**Parent / direct demand:** closes **NV-2** of `docs/research/2026-11-16-026-browser-e2e-tauri-decision.md:217` ("The pre-signed JWT in `docs/dev-smoke.md:67,69` is expired … a fresh-mint helper and secret alignment with `FLORA_APP__JWT_SECRET` must be built and verified"). Complements the proposed e2e slice in `026:97-121`.

**Method / multilingual cross-check.** Read-only Flora cross-check of the requested files (`flora-core/src/utils/jwt.rs`, `dev-seed.sql`, `docs/dev-smoke.md`, `apps/frontend/src/lib/api/*`, `apps/frontend/tests/e2e/*`, `apps/frontend/playwright.config.ts`) plus the auth extractor, config loader, auth routes, session service, seed-session SQL and Cargo manifests. External primary sources only: Playwright docs (EN; auth, `BrowserContext`), Cargo Book (features), Node.js `crypto`, `jsonwebtoken` 11 API docs. **Multilingual:** the load-bearing browser-storage fact (localStorage is origin-scoped, survives reloads/sessions, unlike sessionStorage) was cross-checked on the localized **MDN** pages for **pt-BR, es, de, fr, zh-CN and ja** — all agree with en-US, no divergence. Playwright's own docs publish programming-language variants (Node/Python/Java/.NET) but **not** natural-language localizations at the probed `/pt-br|/es|/de|/fr|/zh-cn|/ja/docs/auth` paths (all HTTP 404); CI/GHA localization was already cross-checked in `026:64` (pt-BR/ja/zh-CN). No secondary blog was used for a load-bearing claim.

---

## 0. Decision summary

| Question | Answer (this pass) |
|---|---|
| Can we just mint a JWT with `encode_token` and inject it? | **Yes — but a JWT alone is not sufficient.** The API also looks up the JWT's `jti` in `sessions` and checks user/org/active/expiry (`flora-api/src/extractors/auth.rs:21-97`). Minting must be paired with the seeded DB session (or a second seeded session row). |
| Smallest reliable mint path? | **Feature-gated Rust helper that reuses `encode_token`** (single source of truth for claims/signature), reading the effective secret via `Config::load()`. A 20-line Node `createHmac` helper is the zero-Rust-change fallback. |
| How to inject? | Prefer a Playwright **setup project writing `storageState`** to a gitignored `playwright/.auth/` file; `page.addInitScript` is the simpler fallback. Both write `localStorage['flora.auth.v1']` on the **frontend origin** (`127.0.0.1:4173`), never a cookie. |
| How to keep it out of prod? | `[[bin]]` with `required-features = ["dev-mint"]` + `default = []`, an `env != "production"` runtime guard, and enabling the feature **only** in the e2e job. No dev HTTP route in `flora-api`. |
| What about the static token in `dev-smoke.md`? | It is **expired now** (exp `1794787200` = 2026-11-16T00:00:00Z; verified against 2026-11-16T13:41Z). Replace static reuse with **mint-at-test-start** (`exp = now + ttl`, `iat = now`); never hand-edit `exp` or publish a new far-future constant. |
| Status | **Proposed, not implemented or executed.** All artifacts below are designs; see §5 NVs. |

---

## 1. Flora state — read-only cross-check

| Area | Evidence in this checkout | Implication for e2e minting |
|---|---|---|
| Mint primitive | `flora-core/src/utils/jwt.rs:15-33` `FloraClaims { sub, jti, organization_id, email, exp, iat, ver }` (all `pub`, `ver` defaults to 1 via `#[serde(default)]`); `:71-89` `encode_token(claims, secret)` uses `Header::default()` + `EncodingKey::from_secret`; `:91-104` `decode_token` uses `Validation::default()`. | A helper can build `FloraClaims` **literally** to choose `exp`/`jti`/`org`, then call `encode_token`. `FloraClaims::new` (`:35-56`) always derives `exp`/`iat` from `Utc::now()` — it **cannot** produce a chosen expiry, so it is not enough by itself. |
| Secret source | `flora-core/src/config/app.rs:20-23,47-48`: default `jwt_secret` comes from `std::env::var("FLORA_JWT_SECRET")` (fallback `dev-secret-change-in-production`); validation `:79-86` enforces ≥32 chars **only** when `env == "production"`. `flora-core/src/config/mod.rs:46-52`: `Figment::new().merge(Serialized::defaults(Self::default())).merge(Env::prefixed("FLORA_").split("__"))`. `.env.example:13-21` defines both `FLORA_JWT_SECRET` and the nested `FLORA_APP__JWT_SECRET` with the same value. | **Nested key wins** (merged last): if the two differ, the API validates with `FLORA_APP__JWT_SECRET`. The helper must call `Config::load()` — not `AppConfig::default()` — to get the *effective* secret, otherwise signatures silently diverge → generic 401. |
| `.env` loading | `dotenv = "0.15"` is declared in `flora-core/Cargo.toml:24` but **no source file calls it** (repo-wide grep: no `dotenv` in `*.rs`); `Config::load()` reads process env only. | Both the API and the mint helper must receive the secret through the **process environment** (shell/CI/`docker compose`), not by relying on an auto-loaded `.env`. |
| Server-side session authority | `flora-api/src/extractors/auth.rs:21-97`: decode JWT → parse `sub`/`organization_id` → `session_service.validate_session(&claims.jti)` → reject unless `is_active`, `expires_at > now`, `user_id == sub`, `organization_id == claim`. `:213-226` `require_org()` rejects nil org. `flora-core/src/services/session_service.rs:80-81` delegates to `find_by_jti`. | **JWT ≠ session.** A minted token must reference a live `sessions` row with matching fields. This is the single most important e2e correctness constraint. |
| Production mint flow | `flora-api/src/routes/auth.rs:261-295` (OIDC callback): create session, `jti = Uuid::now_v7()`, org = nil, `FloraClaims::new`, `encode_token`. `:314-350` refresh re-signs with same `jti`. `:352-416` `select-organization` revokes old session, creates a new org-scoped row with a **rotated `jti`**, and returns a new `session_token`. | Two valid mint profiles: **(A) pre-selection** (nil org, fixed seeded jti, then drive UI through `select-organization`) — recommended; **(B) org-scoped** directly (needs a matching org-scoped session row). |
| Seed + fixed session | `dev-seed.sql:12-22` Acme org `1111…`; `:28-73` Alice `2222…` / Bob `3333…` / Carol `4444…`; `:192-202` workspaces `8888…`/`9999…`; `:318-320` **no sessions seeded**. `scripts/dev-smoke-session.sql:10-48` upserts one pre-selection session: jti `018f0000-0000-7000-8000-000000000001`, user Alice, **nil org**, `expires_at = NOW() + INTERVAL '365 days'`, `is_active = true`, `ON CONFLICT (jti)`. | DB session is long-lived; the JWT is short-lived. The current failure is purely JWT `exp`, not the session row. Profile A needs no new SQL. |
| Frontend storage | `apps/frontend/src/lib/state/AuthState.svelte.ts:8` `STORAGE_KEY = 'flora.auth.v1'`; `:14-31` load, `:33-50` constructor sets `isAuthenticated = Boolean(stored.token)`, `:52-65` persist JSON `{token, user, organizationId, workspaceId}`. `src/lib/api/client.ts:3` `API_BASE_URL = VITE_API_URL || 'http://localhost:3000'`; `:73-108` adds `Authorization: Bearer <token>`, locks session on 401. **No cookies.** | Inject JSON into `localStorage` on the **preview/frontend origin**; the API origin is irrelevant for storage (the token travels in the header). |
| UI consumption of the token | `apps/frontend/src/routes/(app)/select-org/+page.svelte:50-69`: POST `/api/v1/auth/select-organization` → `authState.setOrganization(response.session_token, ...)` → `goto('/tasks')`; `:91-101` redirects home if unauthenticated or already has `organizationId`. | For profile A, inject `organizationId: null` so the smoke actually exercises the rotation; injecting a non-null org skips the select step. |
| Playwright wiring | `apps/frontend/playwright.config.ts:1-6`: only `webServer` (`npm run build && npm run preview`, port 4173) and `testMatch: '**/*.e2e.{ts,js}'`; no `baseURL`, `use`, `projects`, `storageState`. `tests/e2e/tasks.spec.ts`, `files.spec.ts` are `.spec.ts` → **not collected**. `package.json:15-17` `test:e2e = playwright install && playwright test`; `@playwright/test ^1.60.0` (`:23,35`). | There is currently **no auth injection at all**; setup project/storageState must be added. See `026:29-31,97-121` for the wider config/stale-selector analysis. |
| Existing mint pattern in tests | `flora-tests/src/contract/auth.rs:501-513` seeds a session for `old_jti` then `encode_token(&FloraClaims::new(...), &config.app.jwt_secret)`; same pattern in `messaging.rs:172-180`, `websocket.rs:176-184`, `files.rs:222-230`, etc. | The "seed session + mint with config secret" pattern is already idiomatic in this repo — the e2e helper should mirror it. |
| Cargo surface | Root `Cargo.toml:18-35` workspace members; `:41-92` strict lints (`print_stdout = "warn"`, `allow_attributes = "deny"`, `unwrap/expect/panic = "deny"`); `:101-113` profiles; `:223-224` `clap` (with `env`) and `figment` available as workspace deps. `flora-core/Cargo.toml:43` `jsonwebtoken 11` with `rust_crypto`; `:61-64` features `default = []`, `sqlite`, `test`. `flora-api/Cargo.toml:19-25` single bin `flora-api`; `:70-71` `default = []`. No `src/bin` anywhere; no `cfg(feature = ...)` in Flora Rust source. | A new bin + feature is a small, consistent change; strict lints require `#[expect(..., reason = "...")]` for any stdout printing. |
| Expired token (verified) | `docs/dev-smoke.md:64-69` two pre-signed tokens. Decoded payload: `{"sub":"2222…","jti":"018f0000-…0001","organization_id":"0000…0000","email":"alice@acme.test","exp":1794787200,"iat":1763251200,"ver":1}`; header `{"alg":"HS256","typ":"JWT"}`. `date -u -d @1794787200` → **2026-11-16T00:00:00Z**; `iat` → 2025-11-16T00:00:00Z; now → 2026-11-16T13:41Z. | Token has been expired ~13h41m at research time. Signature differs per secret (two variants documented), so "which token" is a second failure mode. |

---

## 2. External primary-source findings

### 2.1 Playwright ([Authentication](https://playwright.dev/docs/auth), [BrowserContext](https://playwright.dev/docs/api/class-browsercontext#browser-context-add-init-script))

1. Isolation is per **browser context** (localStorage/sessionStorage/cookies); Playwright prefers "start from scratch" over cleanup. A setup project saves state; tests reuse it via `use.storageState` ([Auth — Core concepts, Basic](https://playwright.dev/docs/auth#basic-shared-account-in-all-tests)).
2. The state file "may contain sensitive cookies and headers that could be used to impersonate" — create `playwright/.auth` and **gitignore it**; if it should not persist between runs, write it under `testProject.outputDir` (auto-cleaned) ([Auth — danger note](https://playwright.dev/docs/auth#core-concepts)).
3. "Reusing authenticated state covers cookies, **local storage**, IndexedDB and passkey … based authentication" ([Auth — Session storage](https://playwright.dev/docs/auth#session-storage)). For values not persisted automatically (sessionStorage), the doc's own recipe is `context.addInitScript(storage => { ... window.sessionStorage.setItem(...) }, data)`.
4. `addInitScript`: "evaluated **after the document was created but before any of its scripts were run**"; accepts a serializable `arg`; runs on page creation/navigation and on attached frames. Order among multiple init scripts is **not defined** ([BrowserContext.addInitScript](https://playwright.dev/docs/api/class-browsercontext#browser-context-add-init-script)). This timing is exactly what Flora needs: `AuthState` reads `localStorage` in its constructor at module load.
5. `APIRequestContext` can authenticate and then `request.storageState({ path })` ([Auth — Authenticate with API request](https://playwright.dev/docs/auth#authenticate-with-api-request)); `setStorageState` (v1.59) clears then sets cookies/localStorage/IndexedDB ([BrowserContext.setStorageState](https://playwright.dev/docs/api/class-browsercontext#browser-context-set-storage-state)). For parallel workers that mutate server state, Playwright recommends one account per `parallelIndex` ([Auth — Moderate](https://playwright.dev/docs/auth#moderate-one-account-per-parallel-worker)).

### 2.2 Cargo features ([The Cargo Book — Features](https://doc.rust-lang.org/cargo/reference/features.html))

- Features are defined in `[features]`; by default all are disabled unless `default = [...]` is set. Enable with `--features`, disable defaults with `--no-default-features`. Optional dependencies are gated via `dep:`.
- **"The `required-features` field can be used to disable specific Cargo targets if a feature is not enabled."** This is the mechanism for a dev-only `[[bin]]`: keep `default = []`, put the helper behind `required-features = ["dev-mint"]`, and normal `cargo build`/`cargo build --release` never compiles it.
- Feature **unification** means a feature enabled elsewhere can leak into a shared dependency; with `resolver = 3` (root `Cargo.toml:18`) keep the gated code confined to the bin target and never enable `dev-mint` in the API/production job. Features should be additive.

### 2.3 Node.js `node:crypto` ([createHmac](https://nodejs.org/api/crypto.html#cryptocreatehmacalgorithm-key-options))

`createHmac('sha256', secret).update(data).digest('base64url')` is the documented HMAC primitive; the same page documents `crypto.randomUUID()` and `randomUUIDv7()`. This makes a zero-Rust-change JS minter feasible with the tools already in `apps/frontend` (Node 22 types, `tsx`/Vitest/Playwright runtime) — but it duplicates Flora's claim shape and therefore needs a round-trip assertion.

### 2.4 `jsonwebtoken` 11 ([Header](https://docs.rs/jsonwebtoken/11.0.0/jsonwebtoken/struct.Header.html))

"The `alg` defaults to HS256 and `typ` is automatically set to `JWT`"; `Header::default()` "Returns a JWT header using the default Algorithm, HS256". This matches the locally decoded header of the checked-in token, so a JS minter must emit exactly `{"alg":"HS256","typ":"JWT"}` and base64url-encode header/payload/signature segments.

### 2.5 Multilingual cross-check — MDN `Window.localStorage` ([en-US](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage), [pt-BR](https://developer.mozilla.org/pt-BR/docs/Web/API/Window/localStorage), [es](https://developer.mozilla.org/es/docs/Web/API/Window/localStorage), [de](https://developer.mozilla.org/de/docs/Web/API/Window/localStorage), [fr](https://developer.mozilla.org/fr/docs/Web/API/Window/localStorage), [zh-CN](https://developer.mozilla.org/zh-CN/docs/Web/API/Window/localStorage), [ja](https://developer.mozilla.org/ja/docs/Web/API/Window/localStorage))

All seven locales state the same load-bearing semantics (verbatim fragments): en "saved across browser sessions … no expiration time"; pt-BR "não expiram"; es "entre las diferentes sesiones de navegación … no tienen fecha de expiración"; de "bleiben über Browsersitzungen hinweg erhalten … nicht ablaufen"; fr "conservées entre les sessions … n'ont pas de date d'expiration"; zh "存储的数据将保存在浏览器会话中 … 可以长期保留"; ja "ブラウザーのセッションを跨いで保存されます … 期限がない". Combined with `sessionStorage` being cleared when the page session ends, this justifies: put the token in `localStorage` (as Flora does), and let Playwright `storageState`/`addInitScript` seed it. No language-specific caveat changes the design.

---

## 3. The core constraint: a minted JWT must be backed by a `sessions` row

`verify_bearer_token` (`flora-api/src/extractors/auth.rs:33-97`) is the authority. For a token to work, **all** of the following must hold:

1. signature verifies with the API's effective `jwt_secret`;
2. `exp` is in the future (`Validation::default()` requires the `exp` claim — never omit it);
3. `sub` and `organization_id` parse as UUIDs;
4. a row exists in `sessions` with `jti == claims.jti`, `is_active = true`, `expires_at > now`, `user_id == sub`, `organization_id == claim`.

This yields two legitimate mint profiles:

| Profile | Claims | DB prerequisite | Use |
|---|---|---|---|
| **A — pre-selection (recommended)** | `sub = 2222…`, `jti = 018f0000-…-0001`, `organization_id = nil`, `email = alice@acme.test`, `exp = now + ttl` | `scripts/dev-smoke-session.sql` already provides the matching active nil-org row (jti unique, upsert-safe). | Inject `organizationId: null`, let the e2e drive `select-organization` in the UI, which rotates to an org-scoped session/JWT (`routes/auth.rs:352-416`). Covers the real contract. |
| **B — org-scoped directly** | same `sub`/`email`, **`jti` = chosen value**, `organization_id = 1111…`, `exp = now + ttl` | **Must insert a second `sessions` row** with that `jti`, Alice, org `1111…`, `is_active = true`, future `expires_at` (illustrative SQL sketch below). Without it: 401. | Faster setup when the test deliberately skips the org-selection step; higher maintenance (two session fixtures). |

Illustrative row for profile B (design only — not executed, and not needed for profile A):

```sql
-- INSERT INTO sessions (id, user_id, organization_id, jti, expires_at,
--                       last_activity_at, is_active, created_at, updated_at)
-- VALUES (gen_random_uuid(),
--         '22222222-2222-2222-2222-222222222222',
--         '11111111-1111-1111-1111-111111111111',
--         <chosen-jti>,
--         NOW() + INTERVAL '1 day', NOW(), TRUE, NOW(), NOW())
-- ON CONFLICT (jti) DO UPDATE SET organization_id = EXCLUDED.organization_id,
--   is_active = TRUE, expires_at = EXCLUDED.expires_at, updated_at = NOW();
```

Prefer profile A: it reuses the checked-in bootstrap SQL unchanged and exercises the rotation that `026:120` wants the smoke to cover.

---

## 4. Minimal 80/20 slice — **proposed, not implemented**

### 4.1 Feature-gated Rust minter (recommended; reuses `encode_token`)

Add a single dev-only bin **without touching production code paths**:

```toml
# flora-core/Cargo.toml (proposed)
[features]
default = []
dev-mint = []                      # gates only the bin target

[[bin]]
name = "flora-mint-session"
path = "src/bin/flora-mint-session.rs"
required-features = ["dev-mint"]   # Cargo Book §2.2: target skipped otherwise
```

The bin (sketch; strict lints need `#[expect(clippy::print_stdout, reason = "dev CLI emits token for e2e harness")]`):

```rust
// flora-core/src/bin/flora-mint-session.rs (proposed)
let cfg = flora_core::config::Config::load()?;          // effective secret, nested key wins
let now = chrono::Utc::now();
let claims = flora_core::utils::jwt::FloraClaims {
    sub: args.sub,                 // 2222… (Alice)
    jti: args.jti,                 // 018f0000-…-0001 for profile A
    organization_id: args.org,     // Uuid::nil() for profile A
    email: args.email,             // alice@acme.test
    iat: now.timestamp(),
    exp: now.timestamp() + i64::from(args.ttl_secs),   // chosen expiry; new() cannot do this
    ver: 1,
};
let token = flora_core::utils::jwt::encode_token(&claims, &cfg.app.jwt_secret);
```

- CLI surface via already-available `clap` (workspace `Cargo.toml:223`, with `env`): `--sub`, `--jti`, `--org`, `--email`, `--ttl-secs` (default `cfg.app.session_ttl_secs`), plus a `--storage-state <path>` mode that also writes the Playwright JSON (§4.3), so the harness needs no JWT know-how.
- Optional companion in `flora-core/src/utils/jwt.rs`: `FloraClaims::new_with_times(user_id, jti, org_id, email, iat, exp)` (a source change — out of scope here). The struct-literal approach above needs **no** production edit because all fields are `pub` (`jwt.rs:16-33`).
- `--help` should state: dev/seed only; never enable `dev-mint` in a production image.

Invocation in the e2e job only:

```bash
cargo run -q -p flora-core --bin flora-mint-session --features dev-mint -- \
  --sub 22222222-2222-2222-2222-222222222222 \
  --jti 018f0000-0000-7000-8000-000000000001 \
  --org 00000000-0000-0000-0000-000000000000 \
  --email alice@acme.test --ttl-secs 3600 --storage-state playwright/.auth/alice.json
```

**Single-source-of-truth check (cheap, high value):** add one no-DB unit test in the gated bin's module that runs `decode_token(&minted, &cfg.app.jwt_secret)` and asserts `sub/jti/org/exp` round-trip — mirrors `jwt.rs:110-130` and catches claim/secret drift.

### 4.2 Zero-Rust-change fallback (when a Rust toolchain in the e2e worker is undesirable)

A ~15-line Node helper in `apps/frontend/tests/e2e/` using `createHmac('sha256', secret).update(base64url(header) + '.' + base64url(payload)).digest('base64url')`, with header `{"alg":"HS256","typ":"JWT"}` and payload exactly the six `FloraClaims` fields. It must read the **same effective secret** from `process.env.FLORA_APP__JWT_SECRET` (fall back to `FLORA_JWT_SECRET`, then to the documented dev default) and `exp = Math.floor(Date.now()/1000) + ttl`. Trade-off: duplicates claim shape → require a **positive API assertion** (e.g. `GET /api/v1/organizations` returns 200 in setup) as the drift detector. This is the truest "80/20" if the goal is to keep Cargo entirely untouched; §4.1 is the truest "reuse `encode_token`".

### 4.3 Injection: `storageState` (preferred) or `addInitScript` (fallback)

`AuthState` reads the key synchronously in its constructor (`AuthState.svelte.ts:33-50`), so the value must exist **before** app JS runs. Shape (`:1-8,52-65`):

```json
{"token":"<minted-jwt>","user":{"id":"22222222-2222-2222-2222-222222222222"},
 "organizationId":null,"workspaceId":null}
```

**Option 1 — setup project + `storageState`** (Playwright §2.1, items 1–3):

```ts
// tests/e2e/auth.setup.ts (proposed)
const authFile = 'playwright/.auth/alice.json';
setup('mint dev session', async ({ request }) => {
  const token = /* §4.1 CLI output or §4.2 helper */;
  writeFileSync(authFile, JSON.stringify({
    cookies: [],
    origins: [{
      // CRITICAL: the FRONTEND origin, not the API origin — localStorage is origin-scoped.
      origin: 'http://127.0.0.1:4173',
      localStorage: [{ name: 'flora.auth.v1', value: JSON.stringify(stored) }],
    }],
  }));
  const res = await request.get('http://127.0.0.1:3000/api/v1/organizations', {
    headers: { Authorization: `Bearer ${token}` },
  });
  expect(res.ok()).toBeTruthy(); // drift/secret guard (§4.2)
});
```

Config: `projects: [{ name: 'setup', testMatch: /.*\.setup\.ts/ }, { name: 'chromium', use: { ...devices['Desktop Chrome'], storageState: authFile, baseURL: 'http://127.0.0.1:4173' }, dependencies: ['setup'] }]`, plus `VITE_API_URL` baked into the preview build (`026:222`). **Gitignore `playwright/.auth`** (Playwright danger note).

**Option 2 — `addInitScript`** (simplest, no file):

```ts
await page.addInitScript(({ key, value }) => {
  window.localStorage.setItem(key, value);
}, { key: 'flora.auth.v1', value: JSON.stringify(stored) });
```

Per `BrowserContext.addInitScript` this runs before app scripts (§2.1, item 4); it is idempotent across navigations. Keep one init script for this key (multiple init-script order is undefined). `page.addInitScript` must be installed **before** `page.goto('/')`.

Either way, **no cookie fixture**: Flora sends the token via `ApiClient` header (`client.ts:73-80`), so `cookies: []` is correct.

### 4.4 Secret alignment (the most common silent failure)

- Compute the secret exactly as the API does: `Config::load()` (§1). If `FLORA_APP__JWT_SECRET` is set it overrides the default seeded from `FLORA_JWT_SECRET` (Figment merge order, `config/mod.rs:46-52`).
- Mismatch symptom: every request returns 401 "Invalid or expired session" (`extractors/auth.rs:37-43`) and `AuthState.lockSession()` fires (`client.ts:95-97`) — indistinguishable from an expired token without decoding.
- Both secrets in `.env.example:14,20` currently share one value; keep them equal, and pass the chosen value into both the API process and the minter (shell/CI env — remember `.env` is not auto-loaded, §1).
- Prefer the ≥32-char `.env.example` value over the legacy `dev-secret-change-in-production` (`app.rs:48`; production validation at `:79-86`).

### 4.5 Keeping the helper out of production builds

1. `required-features = ["dev-mint"]` on the `[[bin]]` + `default = []` → target absent from `cargo build`/`cargo build --release` (Cargo Book §2.2).
2. Runtime guard in the bin: if `cfg.app.env == "production"` (or the secret is the known dev default), exit with an error before printing anything.
3. Enable `--features dev-mint` **only** in the e2e CI job; leave `.github/workflows/rust.yml` release/production targets unchanged. Note root `Cargo.toml:41-92` lints and that local `cargo clippy --all-targets` would compile the bin — keep it lint-clean with reasoned `#[expect(...)]`.
4. Do **not** add a dev-only HTTP mint route to `flora-api`: it would widen the production attack surface and require an env/capability gate to be safe. If ever needed, it must be `#[cfg(feature = "dev-mint")]` **and** behind `env != "production"`, and is explicitly out of scope for this slice.
5. Treat emitted tokens/`storageState` as impersonation material: gitignored, written under `outputDir` or `playwright/.auth`, short retention (026:181 artifact guidance), never committed.

### 4.6 Refreshing the expired `docs/dev-smoke.md` token safely

- **Diagnosis:** exp `1794787200` = 2026-11-16T00:00:00Z, iat `1763251200` = 2025-11-16T00:00:00Z (decoded + `date -u`, §1). At research time (2026-11-16T13:41Z) it is ~13h41m expired. The DB session from `scripts/dev-smoke-session.sql` is still valid (`NOW() + 365 days` when loaded), so re-seeding the DB alone will **not** fix it.
- **Safe refresh procedure:**
  1. Re-mint at test start with the **same claims** (`sub`, `jti`, nil org, `email`, `ver = 1`) but `iat = now`, `exp = now + ttl` (default `session_ttl_secs = 3600`). Keep JWT TTL ≤ the DB session TTL.
  2. Use the helper from §4.1/§4.2 with the secret the API actually loaded (§4.4). Never hand-edit the `exp` integer of the existing token — the HMAC signature covers the payload and would break.
  3. Verify once in setup with `decode_token` (Rust test) or a 200 from an authenticated API call (JS path) before running browser assertions.
- **Docs hygiene:** replace the two static token strings in `docs/dev-smoke.md:67,69` with the mint command (keeping them as clearly-labelled *illustrative/expired* examples is acceptable, but do not publish a new far-future constant — that recreates the same expiry rot in ~1 year). If a static fallback must remain for the legacy secret, label which secret it belongs to and state its expiry explicitly.
- **Do not** change `dev-seed.sql` or `scripts/dev-smoke-session.sql` for this fix; they are already correct and idempotent.

---

## 5. NVs — not verified / open for this pass

- **NV-1:** Nothing was executed or changed; §4 artifacts are designs. No cargo build, migration, seed load, API start, or Playwright run was attempted.
- **NV-2:** The minter's exact CLI surface, `required-features` gating, and lint-cleanliness under root `Cargo.toml:41-92` were not compiled. Validate with `cargo build -p flora-core --bin flora-mint-session --features dev-mint` and a normal `cargo build --release` (bin must be absent).
- **NV-3:** `storageState` origin (`http://127.0.0.1:4173`) is inferred from `playwright.config.ts:4` (`preview` port 4173) and localStorage origin-scoping; not verified in a real browser. If `baseURL`/preview host/port change, the origin must change with it.
- **NV-4:** The runtime guard (`env == "production"`) and the dev-default-secret guard were not implemented; exact env surface depends on how `FLORA_ENV`/`FLORA_APP__ENV` are supplied in CI.
- **NV-5:** Figment precedence (nested `FLORA_APP__JWT_SECRET` overriding the `FLORA_JWT_SECRET`-seeded default) is derived from merge order in `config/mod.rs:46-52`, not from a runtime decode. Confirm by minting with two different values and observing which one validates.
- **NV-6:** `.env` is not auto-loaded (no `dotenv` call in Rust source); how the local dev shell/CI actually exports the secret was not traced. Both API and minter must see the same process env.
- **NV-7:** Profile B's extra `sessions` row (§3) is an untested SQL sketch; the `sessions` unique constraint on `jti` is assumed from `dev-smoke-session.sql:38` `ON CONFLICT (jti)`. Profile A avoids this NV.
- **NV-8:** Profile A depends on `select-organization` succeeding with the seeded membership (`dev-seed.sql:154-163`) and on the UI flow (`select-org/+page.svelte:50-69`); runtime rotation was not executed here (also `026:NV-3`).
- **NV-9:** The Node `createHmac` fallback (§4.2) was not run; base64url segment encoding and `jsonwebtoken` `Validation::default()` details (e.g. `iat` handling) were not cross-checked against a live API. The positive API assertion is mandatory if this path is chosen.
- **NV-10:** Playwright natural-language docs were unavailable at probed paths (404); the multilingual cross-check therefore used localized MDN for the storage semantics and the localized GHA pages already cited in `026:64`. Re-check if Playwright adds locale paths.
- **NV-11:** Cross-origin 4173→3000 works today only because `cors_layer()` is permissive (`026:NV-6`, `flora-api/src/middleware/cors.rs:14-20`); tightening CORS later may require explicit `http://127.0.0.1:4173` allowlisting, and `VITE_API_URL` is build-time (`026:NV-7`).
- **NV-12:** Cost, worker isolation, retries/trace, and CI placement of the e2e job remain as in `026:181,214-228`; a single disposable Postgres 18 + `workers: 1` is assumed. If parallelized, per-worker jti/account namespaces are required (jti is a unique key).

---

## 6. Sources

**External (primary):**
- Playwright: [Authentication](https://playwright.dev/docs/auth) ([core concepts](https://playwright.dev/docs/auth#core-concepts), [basic/shared account](https://playwright.dev/docs/auth#basic-shared-account-in-all-tests), [API request](https://playwright.dev/docs/auth#authenticate-with-api-request), [session storage](https://playwright.dev/docs/auth#session-storage)), [BrowserContext.addInitScript](https://playwright.dev/docs/api/class-browsercontext#browser-context-add-init-script), [BrowserContext.setStorageState](https://playwright.dev/docs/api/class-browsercontext#browser-context-set-storage-state).
- Cargo: [The Cargo Book — Features](https://doc.rust-lang.org/cargo/reference/features.html) (default features, `required-features`, feature unification).
- Node.js: [crypto.createHmac](https://nodejs.org/api/crypto.html#cryptocreatehmacalgorithm-key-options).
- `jsonwebtoken` 11.0.0: [Header](https://docs.rs/jsonwebtoken/11.0.0/jsonwebtoken/struct.Header.html) (HS256/typ default).
- MDN `Window.localStorage`: [en-US](https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage), [pt-BR](https://developer.mozilla.org/pt-BR/docs/Web/API/Window/localStorage), [es](https://developer.mozilla.org/es/docs/Web/API/Window/localStorage), [de](https://developer.mozilla.org/de/docs/Web/API/Window/localStorage), [fr](https://developer.mozilla.org/fr/docs/Web/API/Window/localStorage), [zh-CN](https://developer.mozilla.org/zh-CN/docs/Web/API/Window/localStorage), [ja](https://developer.mozilla.org/ja/docs/Web/API/Window/localStorage).

**Local (repo, read-only):**
- `flora-core/src/utils/jwt.rs:15-104,106-159`; `flora-core/src/utils/mod.rs:1-4`; `flora-core/src/config/app.rs:20-23,38-65,73-93`; `flora-core/src/config/mod.rs:36-52`; `flora-core/Cargo.toml:23-64`.
- `flora-api/src/extractors/auth.rs:10-97,149-254`; `flora-api/src/routes/auth.rs:261-416`; `flora-api/Cargo.toml:19-25,70-71`; `flora-core/src/services/session_service.rs:80-81`.
- `Cargo.toml:17-35,41-92,101-113,220-224`; `.env.example:8-21`; `dev-seed.sql:12-22,28-73,79-187,192-229,268-320`; `scripts/dev-smoke-session.sql:1-50`; `docs/dev-smoke.md:3-5,31-39,47-88`.
- `apps/frontend/src/lib/state/AuthState.svelte.ts:1-65,67-97,104-143`; `apps/frontend/src/lib/api/client.ts:3,73-110`; `apps/frontend/src/routes/(app)/select-org/+page.svelte:38-69,91-101`; `apps/frontend/playwright.config.ts:1-6`; `apps/frontend/tests/e2e/tasks.spec.ts`, `apps/frontend/tests/e2e/files.spec.ts`; `apps/frontend/package.json:6-17,19-48`.
- `flora-tests/src/contract/auth.rs:501-534` (and the same mint pattern in `messaging.rs`, `websocket.rs`, `files.rs`, `notifications.rs`, `search.rs`).
- Prior research: `docs/research/2026-11-16-026-browser-e2e-tauri-decision.md:23-40,97-121,214-228` (especially NV-2, which this document answers).
