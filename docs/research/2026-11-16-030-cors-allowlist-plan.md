# 030 — CORS allowlist & runtime API/WS URLs (ADR-0002 re-entry criterion #2)

**Date:** 2026-11-16 (primary-source fetches same day).
**Repo:** `flora` (archived checkout), branch `dev`. **Type:** research/design only — **no source, config, workflow, or env file was changed; no stack was started; no HTTP request was executed.** This document is the only artifact written.
**Builds on:** ADR-0002 §"Re-entry criteria" #2 [L14]; docs 026:NV-6/NV-7, 027:NV-11, 028:§0/R7, 029:§0-B7/B7 [L15].
**Question:** What is the minimal, code-verifiable change that replaces `CorsLayer` `Any` with an explicit allowlist and makes the browser's API/WS URLs runtime-configurable, without breaking the Seed browser flow, and what must be proven before ADR-0002 criterion #2 can be marked met?

---

## 0. Decision & evidence summary

| Item | Answer (this pass) |
|---|---|
| Verdict | **80/20 plan is feasible; do not defer the config work.** Backend allowlist is a ~15-line change reusing the already-declared-but-ignored `app.cors_origins`. Frontend runtime URL is a small resolver + one injected global. Full re-entry #2 still needs a **separate execution step** (non-browser origin preflight + `select-organization` run) that this research-only pass does not perform. |
| Bearer ≠ credentialed | The `Authorization: Bearer …` header is a **non-safelisted request header**, so it triggers preflight and needs `Access-Control-Allow-Headers: authorization`. It is **not** a CORS "credentialed request" (that means cookies/TLS certs/auth entries via `credentials: "include"`). **Do not call `allow_credentials(true)`** [S3][S4]. Keeping it off also avoids tower-http's panic when credentials are combined with wildcards [S3]. |
| Allowlist mechanism | `AllowOrigin::list(vec![HeaderValue…])` (or passing `Vec<HeaderValue>` to `.allow_origin`). **It panics if any entry is `*`** → validate in `AppConfig::validate()` and return a config error, never a panic [S2][S3]. |
| Existing gap | `AppConfig.cors_origins` exists (`flora-core/src/config/app.rs:19`, default `http://localhost:5173`) but `cors_layer()` takes no arguments and uses `Any` for origin/method/header [L1][L3]. |
| Exact backend key | **`FLORA_APP__CORS_ORIGINS`** (maps to `app.cors_origins` via `Env::prefixed("FLORA_").split("__")`) [L3][L4]. Figment parses arrays with **TOML-like bracket syntax**, *not* a bare comma-separated string — the field's doc comment ("comma-separated") is wrong and must be corrected [L4][S13]. |
| Frontend URL today | `API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000'` — **baked at build time**; not runtime-configurable; Bun preloads `.env` and can shadow it [L6][S12]. |
| Runtime URL mechanism | Prefer a **framework-agnostic `window.__FLORA_CONFIG__` global** loaded from `static/config.js` before the app bundle; resolver precedence `runtime > VITE_API_URL > default`. WS URL is **derived from the same base** by the existing `buildWsUrl()` — no second source of truth needed [L6][L7]. Avoid `$env/dynamic/public`: deprecated in SvelteKit 3, removed in 4 [S14]. |
| Browser origins to allow | Dev `http://localhost:5173`, `http://127.0.0.1:5173`; preview `http://localhost:4173`, `http://127.0.0.1:4173` (CI uses 127.0.0.1) [L15]. `localhost` and `127.0.0.1` are distinct origins. |
| Future Tauri origins | Production webview origin: **`tauri://localhost`** (macOS/Linux/iOS) and **`http://tauri.localhost`** on Windows/Android by default (`useHttpsScheme: true` → `https://tauri.localhost`); `tauri dev` uses the normal `http://localhost:<port>` dev-server origin [S7][S8][S9][S11]. A scheme-less `tauri.localhost` is also reported by WebView2 field evidence [S10] — record, do not blindly allow (NV-7). |
| WS caveat | Browsers do **not** apply CORS to WebSocket; they send `Origin` but ignore `Access-Control-Allow-Origin`. Flora's WS handler does not check `Origin`, and accepts the token via `?token=` because JS cannot set WS headers [L7][L12]. CORS allowlisting does **not** secure WS (NV-6). |
| Seed flow preservation | Keep the OIDC redirect allowlist (`FLORA_OIDC__ALLOWED_REDIRECT_URIS`, currently default empty, with `app.base_url` callback always allowed) aligned with the new frontend origin; CORS and redirect allowlists are separate controls [L10][L11]. |
| Deferral | Legitimate only with written triggers (§5); the permissive `Any` remains a logged security NV until then, and must never be treated as "CORS verified". |

**One-line closure:** *Wire `FLORA_APP__CORS_ORIGINS` (Figment array syntax) into a `CorsLayer` with explicit origins/methods/headers and no credentials, inject `PUBLIC_API_BASE_URL` at runtime via a window global while deriving the WS URL from it, then prove the gate with a curl preflight that echoes `Access-Control-Allow-Origin` for `127.0.0.1:4173` and the Tauri origins plus a bearer `select-organization` call.*

---

## 1. Flora cross-check (read-only, code-grounded)

| ID | Fact | Evidence |
|---|---|---|
| F1 | `cors_layer() -> CorsLayer` ignores config entirely: `allow_origin(Any)`, `allow_methods(Any)`, `allow_headers(Any)`, `max_age(24h)`. | `flora-api/src/middleware/cors.rs:14-20` [L1] |
| F2 | Router applies it with no argument: `.layer(crate::middleware::cors::cors_layer())`. Layer order is `.layer(rate_limit).layer(Extension).layer(cors).layer(trace)` → CORS is **outside** rate limiting, so `OPTIONS` preflights are answered before the limiter. | `flora-api/src/routes/mod.rs:20-64` [L2] |
| F3 | `AppState` carries `config: Arc<Config>`, so `create_router` can read `app_state.config.app.cors_origins` before moving state. | `flora-api/src/state/app_state.rs:24-28` [L16] |
| F4 | `AppConfig` already has `cors_origins: Vec<String>` with default `["http://localhost:5173"]`; `validate()` checks only `base_url`, JWT length, session TTL — **no CORS validation**. | `flora-core/src/config/app.rs:19,46,73-93` [L3] |
| F5 | Config loading is `Serialized::defaults(Self::default())` then `Env::prefixed("FLORA_").split("__")`. Figment features are `env,toml,yaml` — **no `test`**, so `figment::Jail` is unavailable without a dev-dep feature bump. | `flora-core/src/config/mod.rs:43-49`; `Cargo.toml:222` [L4][L13] |
| F6 | `.env.example` documents `FLORA_APP__BASE_URL` but has **no `CORS_ORIGINS` key**. | `.env.example:8-21` [L5] |
| F7 | Frontend API base is build-time `VITE_API_URL`; every request adds `Authorization: Bearer <token>` from `localStorage['flora.auth.v1']`; 401 → `lockSession()`. No cookies. | `apps/frontend/src/lib/api/client.ts:3,73-108` [L6] |
| F8 | `buildWsUrl()` derives `ws(s)://` from the API base and appends `?token=`. A separate `PUBLIC_WS_BASE_URL` can be an optional override, not a requirement. | `apps/frontend/src/lib/api/websocket.ts:34-39,127` [L7] |
| F9 | Login builds `redirect_uri` from `location.origin` (frontend origin) and calls `${API_BASE_URL}/api/v1/auth/login`; API validates it against `{base_url}/api/v1/auth/callback` + `oidc.allowed_redirect_uris`. | `(auth)/+page.svelte:21-24`; `flora-api/src/routes/auth.rs:101-115`; `flora-core/src/config/oidc.rs:20-24,41` [L8][L10][L11] |
| F10 | `app.html` has no runtime-config injection point; `static/` exists (`robots.txt`). | `apps/frontend/src/app.html:1-18`; `apps/frontend/static/robots.txt` [L9] |
| F11 | WS serves both `/ws` and `/api/v1/ws`; auth prefers `Authorization` header, falls back to `?token=`; no `Origin` check. | `flora-api/src/routes/mod.rs:49-50`; `flora-api/src/routes/websocket.rs:116-156` [L2][L12] |
| F12 | Workspace pins `tower-http 0.7.0` with `features = ["cors","trace"]`; docs consulted are 0.7.1 (same 0.7.x builder API). | `Cargo.toml:141`; `flora-api/Cargo.toml:39` [L13] |
| F13 | No route reads `X-Organization-ID` despite stale comments; test helpers still send it. Include it in `allow_headers` for compatibility, not as an auth mechanism. | `flora-api/src/routes/mod.rs:30`; `flora-tests/src/contract/search.rs:254`; prior audit [L2][L17] |
| F14 | Prior research already flagged this exact gap and the build-time URL trap as NVs; none were fixed. | docs 026:NV-6, 027:NV-11, 028:§0/414, 029:22/B7 [L15] |

---

## 2. Primary-source findings

### 2.1 tower-http `CorsLayer` (0.7.1 docs + source) [S1][S2][S3]

- `CorsLayer::new()` sends **no** CORS headers; "you need to set at least an allowed origin".
- `allow_origin` accepts a single `HeaderValue`, an array/`Vec<HeaderValue>` (→ `AllowOrigin::list`), `Any`, or a sync/async predicate. `AllowOrigin::list` **panics on a wildcard entry** [S2].
- `allow_methods`, `allow_headers`, `expose_headers` accept arrays; `Any` is available for each; **repeated calls override previous ones**.
- `max_age` is opt-in; the browser caps the value internally. Keep the existing 24h.
- `vary` is derived automatically: with an origin list, `Vary: Origin` is added; with `Any` it is not. Do not override it [S3].
- On `OPTIONS`, the layer answers the preflight itself (method/headers/max-age/allow-origin) and does **not** call the inner service [S3].
- `ensure_usable_cors_rules()` **panics** if `allow_credentials(true)` is combined with `Any` for origin, headers, methods, or exposed headers [S3]. Explicit lists are future-proof if cookies are ever introduced.
- The layer is `Clone + Send + Sync`, so building it once at router construction from `AppState` is fine.

### 2.2 MDN CORS — credentialed vs bearer [S4]

- Cross-origin `fetch()` is blocked by same-origin policy unless the response carries the right CORS headers; the browser sends `Origin` on every CORS request (value may be `null`).
- `POST`/`PUT`/custom headers (including `Authorization`) are **preflighted** with `OPTIONS` + `Access-Control-Request-Method` / `-Headers`; the server must answer with `Access-Control-Allow-Origin`, `-Methods`, `-Headers` (and `-Credentials` only for credentialed requests).
- Credentialed requests (`credentials: "include"`, cookies, HTTP auth, TLS certs) **must not** use `*` for `-Allow-Origin`, `-Allow-Headers`, `-Allow-Methods`, or `-Expose-Headers`; an explicit origin is required. When the allowlist is dynamic, include `Vary: Origin`.
- "If the request is one that triggers a preflight due to the presence of the `Authorization` header … you won't be able to work around the limitation" (no simple-request bypass). Therefore the explicit `authorization` entry in `allow_headers` is mandatory.
- Cross-origin cookie behavior is additionally subject to third-party-cookie policy — another reason to stay bearer-only.

**Consequence for Flora:** keep `credentials` at its default (`same-origin`), do not set `allow_credentials(true)`, send the JWT in `Authorization`, and rely on the explicit origin list. This satisfies "no reliance on cookies" in ADR-0002 #2.

### 2.3 Tauri v2 origins and CSP [S5][S6][S7][S8][S9][S10][S11]

- Tauri relies on the **OS WebView** (WKWebView / WebView2 / WebKitGTK); the IPC boundary is separate from HTTP CORS [S5].
- Production frontend origin is a custom scheme: **`tauri://localhost`** on macOS/Linux/iOS; on **Windows/Android** v2 serves from **`http://tauri.localhost`** by default, or `https://tauri.localhost` with `app.windows[].useHttpsScheme = true`. `tauri dev` uses the ordinary dev-server `http://localhost:<port>` origin [S7][S8][S11].
- A Tauri maintainer confirms the webview **cannot bypass the ACAO check**: if the server will not allow the origin, the workaround is Tauri's HTTP plugin or making the request from Rust [S9]. For Flora the server-side allowlist is the minimal fix.
- Field evidence (self-hosted Tauri apps) shows the failure mode precisely: preflight returns **no `Access-Control-Allow-Origin`** for the Tauri origin, WebView discards the response, JS sees only "Failed to fetch"/opaque error; adding `tauri://localhost` **and** the Windows origin fixes it; **WebView2 caches preflight per origin**, so an already-running client must be restarted after a server config change [S10].
- Separate control: the webview CSP must permit the API origin via `connect-src` (default example is `connect-src ipc: http://ipc.localhost`); CORS alone is insufficient once a Tauri build exists [S6][S7].
- On macOS, `WKAppBoundDomains` must list `localhost` (and any registrable domain) for localhost calls/isolation [S7].
- Caution: some reports use a **scheme-less `tauri.localhost`** as the `Origin` value; `http::HeaderValue` will accept it, but allowing it is a judgement call based on a Windows smoke (NV-7).

### 2.4 Runtime-configurable browser URLs [S12][S14][S15]

- Vite: `VITE_*` values are "global during dev and **statically replaced at build time**"; they are bundled into client source; process env at Vite invocation has highest priority; Bun preloads `.env` into `process.env` and can interfere [S12].
- SvelteKit's `$env/dynamic/public` is **legacy**: "deprecated in SvelteKit 3 … removed in 4"; the replacement is explicit `src/env.ts` + `$app/env/public` [S14]. Flora pins `@sveltejs/kit ^2.63.0`; using a soon-to-be-removed module creates migration debt for a gate that should be durable.
- `adapter-node` does not load `.env` in production (`node --env-file=.env build` is needed) — another reason not to depend on framework env files for this [S15].
- A framework-agnostic runtime global (`window.__FLORA_CONFIG__`) loaded by a tiny `static/config.js` works identically in the browser, in `adapter-node`, and in a future static/Tauri build, and keeps `API_BASE_URL` a synchronous constant so no call sites change [L6][L7][L9].

---

## 3. Minimal 80/20 plan (implementation outline — **not applied**)

### Step 1 — Backend: wire the allowlist

Change `flora-api/src/middleware/cors.rs` to accept origins and build explicit policies (sketch, not applied):

```rust
use std::time::Duration;
use http::{HeaderName, HeaderValue, Method, header};
use tower_http::cors::{AllowOrigin, CorsLayer};

pub fn cors_layer(origins: &[String]) -> CorsLayer {
    let origins: Vec<HeaderValue> = origins.iter()
        .filter_map(|o| HeaderValue::from_str(o).ok())
        .collect();
    let org_header = HeaderName::from_static("x-organization-id");
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            Method::GET, Method::POST, Method::PUT, Method::PATCH,
            Method::DELETE, Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            org_header,
        ])
        .max_age(Duration::from_hours(24))
    // no allow_credentials, no expose_headers(Any)
}
```

- `flora-api/src/routes/mod.rs:59`: `cors_layer(&app_state.config.app.cors_origins)` (clone/borrow before `.with_state(app_state)`).
- `flora-core/src/config/app.rs`: default `cors_origins` becomes the four browser origins (dev + preview, both host forms); fix the "comma-separated" comment to describe Figment array syntax.
- `AppConfig::validate()`: reject empty list; reject any entry equal to `*` or containing `*`; reject empty strings; **warn** in production if any `localhost`/`127.0.0.1` entry is present (prevents dev origins leaking into prod). Return `Error::Configuration` — never panic (NV-2).

### Step 2 — Backend: exact config/env keys

| Key | Layer | Value / format | Purpose |
|---|---|---|---|
| `FLORA_APP__CORS_ORIGINS` | backend (Figment → `app.cors_origins`) | TOML-style array of strings, e.g. `'["http://localhost:5173","http://127.0.0.1:5173","http://localhost:4173","http://127.0.0.1:4173","tauri://localhost","http://tauri.localhost"]'` | Explicit ACAO allowlist. Tauri entries can ship in the default (they are first-party constants) or be added only at Tauri re-entry — pick one and document it. |
| `FLORA_APP__BASE_URL` | backend (existing) | `http://localhost:3000` | API's own public URL; reused for OIDC callback. No new backend key needed. |
| `FLORA_OIDC__ALLOWED_REDIRECT_URIS` | backend (existing) | array, e.g. `'["http://127.0.0.1:4173/callback"]'` | Keep the browser login flow working when the frontend origin changes; separate from CORS. |

Figment caveat: `FLORA_APP__CORS_ORIGINS='["http://a","http://b"]'` (single quotes outside, double quotes inside) is the verified syntax; a bare `a,b` string does **not** deserialize into `Vec<String>` [S13]. Add a config-parse test (NV-3).

### Step 3 — Frontend: runtime API/WS URLs

1. Add `apps/frontend/static/config.js` (generated at deploy/container start, or committed with safe defaults for local dev):
   ```js
   window.__FLORA_CONFIG__ = {
     apiBaseUrl: "http://localhost:3000",
     wsBaseUrl: null
   };
   ```
2. Add a blocking `<script src="%sveltekit.assets%/config.js"></script>` in `apps/frontend/src/app.html` **before** `%sveltekit.head%` so the global exists when `client.ts` is evaluated.
3. In `apps/frontend/src/lib/api/client.ts`, resolve with precedence:
   `globalThis.__FLORA_CONFIG__?.apiBaseUrl` → `import.meta.env.VITE_API_URL` → `'http://localhost:3000'`; keep the `API_BASE_URL` export so `client.ts`, `websocket.ts`, and `(auth)/+page.svelte` are unchanged.
4. In `websocket.ts`, `buildWsUrl()` already derives the WS URL from the base; optionally honour `__FLORA_CONFIG__.wsBaseUrl` as a full-base override, leaving the default path `/api/v1/ws` and `?token=` intact.
5. Deploy-time injection: set `PUBLIC_API_BASE_URL` / `PUBLIC_WS_BASE_URL` in the container environment and have the entrypoint render `config.js` (or use a `hooks.server.ts` response-rewrite for adapter-node). **Do not rely on `VITE_API_URL` alone — it is build-time and does not satisfy the gate.** Keep any local `apps/frontend/.env` out of git (Bun/Vite priority trap).

Exact frontend keys: **`PUBLIC_API_BASE_URL`** (required at runtime), **`PUBLIC_WS_BASE_URL`** (optional override), **`VITE_API_URL`** (build-time fallback only).

### Step 4 — `.env.example` additions (exact text)

```dotenv
# CORS: exact browser/webview origins (Figment array syntax; no "*").
FLORA_APP__CORS_ORIGINS=["http://localhost:5173","http://127.0.0.1:5173","http://localhost:4173","http://127.0.0.1:4173"]
# Future Tauri webview origins (add with the desktop build):
#   "tauri://localhost" (macOS/Linux/iOS), "http://tauri.localhost" (Windows/Android)
# Frontend runtime config is rendered into apps/frontend/static/config.js at deploy time:
# PUBLIC_API_BASE_URL=https://api.flora.example
# PUBLIC_WS_BASE_URL=wss://api.flora.example
```

### Step 5 — Tests (exact assertions and commands)

| # | Test | Where | Assertion |
|---|---|---|---|
| T1 | CORS unit: allowed preflight | `flora-api/src/middleware/cors.rs` `#[cfg(test)]`, `tower::ServiceExt::oneshot` (or router oneshot as in `flora-tests`) | `OPTIONS /api/v1/auth/select-organization` with `Origin: http://127.0.0.1:4173`, `Access-Control-Request-Method: POST`, `Access-Control-Request-Headers: authorization,content-type` → ACAO == that origin; ACAM contains `POST`; ACAH contains `authorization`; **no** `Access-Control-Allow-Credentials`; `Vary: Origin` present. |
| T2 | CORS unit: denied preflight | same | `Origin: https://evil.example` → **no** ACAO header. |
| T3 | CORS unit: Tauri origins | same | `tauri://localhost` and `http://tauri.localhost` echo in ACAO; `Origin: null` does not. |
| T4 | Config validation | `flora-core` unit | `cors_origins = ["*"]` or empty → `validate()` is `Err`; valid list → `Ok`. No panic path. |
| T5 | Config parse | `flora-core` (add `figment` `test` feature in dev-deps, or a CI env probe) | `Config::load()` with the Step-2 env value yields the exact expected `Vec<String>` (NV-3). |
| T6 | Frontend resolver | `apps/frontend` vitest `server` project | runtime global wins over `VITE_API_URL` and default; WS URL derivation yields `ws://…/api/v1/ws?token=…` (and `wss:` for https); optional `wsBaseUrl` override honoured. |
| T7 | Existing browser lane | Playwright 4173→3000 smoke (docs 028/029) | Still green with the new allowlist — this is the regression guard for Seed browser flows. |
| T8 | **Non-browser gate proof (execution, not this pass)** | curl + seeded DB | See commands below; assert header presence/absence explicitly. |

Commands for T8 (run with a live seeded API, tokens redacted in evidence):

```bash
# Preflight must echo an allowlisted origin:
curl -si -X OPTIONS "$API/api/v1/auth/select-organization" \
  -H 'Origin: http://127.0.0.1:4173' \
  -H 'Access-Control-Request-Method: POST' \
  -H 'Access-Control-Request-Headers: authorization,content-type' \
  | grep -i '^access-control-allow-origin:'

# Same for each Tauri origin (negative control must print nothing):
for O in 'tauri://localhost' 'http://tauri.localhost' 'https://evil.example'; do
  curl -si -X OPTIONS "$API/health" -H "Origin: $O" \
    -H 'Access-Control-Request-Method: GET' | grep -i '^access-control-allow-origin:' || true
done

# Bearer + select-organization from a non-browser origin (reuse the 028/029 fresh-JWT recipe):
curl -si -X POST "$API/api/v1/auth/select-organization" \
  -H "Authorization: Bearer $SEED_JWT" -H 'Origin: http://127.0.0.1:4173' \
  -H 'Content-Type: application/json' -d '{"organization_id":"11111111-1111-7111-8111-111111111111"}' \
  | grep -Ei '^(HTTP/|access-control-allow-origin:|vary:)'   # expect 200 + echoed ACAO
```

**Important:** `curl` does not *enforce* CORS. T8 proves the server sends the right headers; it is not a substitute for T7 (real webview/browser). Record both.

### Step 6 — Gate wording after execution

ADR-0002 #2 is met only when: (a) API/WS URLs are runtime-configurable (T5/T6), (b) T1–T4 prove an explicit allowlist with no `Any` and no credentials, (c) T8 shows a bearer `select-organization` round-trip from a non-browser origin with explicit ACAO, and (d) T7 stays green. Log the result in a new ADR or an update to ADR-0002 — not in this research doc.

---

## 4. What is deliberately out of scope (80/20 boundary)

- No `tauri://` build, no `src-tauri/`, no `adapter-static` split (ADR-0002 still stands).
- No cookie/BFF auth migration; no change to JWT/session authority.
- No WS-level `Origin` enforcement, no reverse-proxy CORS config, no TLS work (NV-6/NV-8).
- No runtime reload of the allowlist; config is read at startup (`CorsLayer` is built once). The `MaxAge::dynamic` / `async_predicate` hooks exist [S1][S3] but are unnecessary for the gate.

---

## 5. Deferral option and triggers

Deferring is **acceptable** only if all of these hold:

1. No non-browser client (Tauri/mobile/third-party) is scheduled or requesting API access in the next planning window; ADR-0002 remains Accepted-as-deferred.
2. The browser lanes (thin smoke + six DB-gated suites) remain green and no cross-origin topology change is planned.
3. A short ADR/research note records the permissive `CorsLayer` as an **open security NV** with an owner and a review date, and explicitly states that criterion #2 is **not** met.
4. No production deployment exposes the API directly to untrusted web origins (otherwise the `Any` policy is an active risk, not a documented deferral).

**Triggers that force implementation now:** a non-browser client must authenticate; the frontend origin changes at runtime; a security review flags `ACAO: *`; or any attempt to add cookies/credentialed requests (tower-http would panic with the current `Any` + credentials combination [S3]).

---

## 6. Risks / unknowns (NV)

- **NV-1 (M):** Research only. No live API, no preflight, no JWT, no Tauri webview was run. All header behaviour is from docs/source [S1][S3][S4] and field reports [S10]; T8 is mandatory at execution.
- **NV-2 (H):** `AllowOrigin::list` **panics** on `*` [S2], and `ensure_usable_cors_rules` panics on credentials + wildcard [S3]. Without `AppConfig::validate()` checks, a bad env value becomes a startup panic (workspace denies `panic`/`unwrap`). Validate first.
- **NV-3 (M):** Figment array syntax is non-obvious; a bare comma-separated string fails to deserialize into `Vec<String>` [S13]. The current field comment is misleading. T5 must pin the exact env format; `figment::Jail` requires adding the `test` feature (absent today).
- **NV-4 (M):** Tauri origin string ambiguity — `tauri://localhost` vs `http://tauri.localhost` vs `https://tauri.localhost` vs scheme-less `tauri.localhost` [S7][S10][S11]. Allow exactly what a Windows smoke proves; `Origin: null` should remain denied (opaque origins can otherwise slip through).
- **NV-5 (M):** WebView2/WKWebView cache preflight results per origin; an already-running desktop client can keep failing after the server is fixed [S10]. Document "restart client".
- **NV-6 (H):** CORS does not protect WebSocket; Flora's WS handler does not check `Origin` and accepts `?token=` [L7][L12]. If Tauri re-entry proceeds, add an explicit WS origin check and assess token-in-query logging separately.
- **NV-7 (M):** Tauri CSP `connect-src` must include the runtime API origin, otherwise the webview blocks the request even with a correct CORS allowlist [S6][S7]. Out of scope until a Tauri build exists, but a known prerequisite.
- **NV-8 (M):** Reverse proxy/CDN may strip or cache `Vary: Origin` incorrectly; tower-http adds it automatically for lists [S3], but verify in the VPS deployment path.
- **NV-9 (M):** Frontend build-time env pitfall remains: Bun preloads `.env`, and `VITE_API_URL` is baked at build [S12][L15]. The runtime global must take precedence; keep local `.env` gitignored.
- **NV-10 (L):** `$env/dynamic/public` would be a shorter implementation but is deprecated/removed in SvelteKit 3/4 [S14]; the window-global approach avoids that churn.
- **NV-11 (L):** Allowing `localhost`/`127.0.0.1` in a production profile weakens the allowlist to "any local process". Gate dev defaults by `app.env` and warn at validation.
- **NV-12 (L):** `X-Organization-ID` is in `allow_headers` only for legacy/test compatibility; no route trusts it [L17]. Do not interpret its presence as tenancy enforcement.

---

## 7. Sources

**Local / code (read-only this pass):**
- [L1] `flora-api/src/middleware/cors.rs:1-20`
- [L2] `flora-api/src/routes/mod.rs:20-64`
- [L3] `flora-core/src/config/app.rs:7-36,38-64,73-93`
- [L4] `flora-core/src/config/mod.rs:33-49`; `Cargo.toml:222`
- [L5] `.env.example:8-21`
- [L6] `apps/frontend/src/lib/api/client.ts:3,73-108`
- [L7] `apps/frontend/src/lib/api/websocket.ts:34-39,127`
- [L8] `apps/frontend/src/routes/(auth)/+page.svelte:21-24`
- [L9] `apps/frontend/src/app.html:1-18`; `apps/frontend/static/robots.txt`
- [L10] `flora-api/src/routes/auth.rs:101-115`
- [L11] `flora-core/src/config/oidc.rs:20-24,41`
- [L12] `flora-api/src/routes/websocket.rs:116-156`
- [L13] `Cargo.toml:141`; `flora-api/Cargo.toml:39`
- [L14] `docs/adr/0002-tauri-deferral-from-canonical-seed-frontend.md:27-31`
- [L15] `docs/research/2026-11-16-{026-browser-e2e-tauri-decision,027-e2e-jwt-bootstrap,028-e2e-jwt-implementation-plan,029-smoke-alt-port-runbook}.md`
- [L16] `flora-api/src/state/app_state.rs:24-28`
- [L17] `docs/research/2026-11-15-012-frontend-api-contract-audit.md:112`; `flora-tests/src/contract/search.rs:254`

**Primary / external (fetched 2026-11-16):**
- [S1] [docs.rs — `tower_http::cors::CorsLayer` (0.7.1)](https://docs.rs/tower-http/latest/tower_http/cors/struct.CorsLayer.html)
- [S2] [docs.rs — `tower_http::cors::AllowOrigin` (0.7.1; `list` panics on wildcard)](https://docs.rs/tower-http/latest/tower_http/cors/struct.AllowOrigin.html)
- [S3] [docs.rs source — `tower_http/cors/mod.rs` 0.7.1 (`call` preflight path, `update_vary_header`, `ensure_usable_cors_rules`)](https://docs.rs/tower-http/0.7.1/src/tower_http/cors/mod.rs.html)
- [S4] [MDN — Cross-Origin Resource Sharing (CORS)](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS) (EN; pt-BR/es/de/fr/zh-CN/ja localizations follow the same normative Fetch text)
- [S5] [Tauri v2 — Security overview (trust boundaries, OS WebView)](https://v2.tauri.app/security/)
- [S6] [Tauri v2 — Content Security Policy (`connect-src ipc: http://ipc.localhost`)](https://v2.tauri.app/security/csp/)
- [S7] [Tauri v2 — Configuration reference (`WebviewUrl`, `useHttpsScheme`, `WKAppBoundDomains`)](https://v2.tauri.app/reference/config)
- [S8] [Tauri v2 — Migration from v1 ("New origin URL on Windows")](https://v2.tauri.app/start/migrate/from-tauri-1)
- [S9] [tauri-apps/tauri issue #11260 — maintainer: webviews cannot bypass ACAO](https://github.com/tauri-apps/tauri/issues/11260) (cross-check; not normative spec)
- [S10] [block/buzz issues #3490 / #2872 — empirical Windows `tauri.localhost` preflight evidence and WebView2 cache](https://github.com/block/buzz/issues/3490) (cross-check; not normative spec)
- [S11] [Tauri `packages/api/src/webview.ts` — `tauri://localhost` / `tauri.localhost` production origin note](https://github.com/tauri-apps/tauri/blob/dev/packages/api/src/webview.ts)
- [S12] [Vite — Env Variables and Modes (build-time replacement, Bun `.env` caveat)](https://vite.dev/guide/env-and-mode)
- [S13] [Figment — `Env` provider (TOML-like arrays; `NUMBERS=[1,2,3]`) and issue #104](https://docs.rs/figment/latest/figment/providers/struct.Env.html)
- [S14] [SvelteKit — `$env/dynamic/public` (legacy/deprecated) and Environment variables (`$app/env/public`)](https://svelte.dev/docs/kit/environment-variables)
- [S15] [SvelteKit — `adapter-node` (production `.env` not auto-loaded)](https://svelte.dev/docs/kit/adapter-node)
