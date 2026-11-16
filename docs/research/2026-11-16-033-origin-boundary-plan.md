# 033 — Origin boundary: runtime FE config + WS `Origin` check + deterministic non-browser proof

**Date:** 2026-11-16 (primary docs fetched same day).
**Repo:** `flora` (archived checkout), branch `dev`, HEAD `763b8c553281326105934eef586f5b751e012e67` (`2026-11-16 17:12:47 -0400`, working tree clean).
**Type:** research/design only — **no source, config, env, workflow, or test file was changed; no stack was started; no HTTP request was executed.** This document is the only artifact written.

**Builds on:** ADR-0002 §"Re-entry criteria" #2 [L14]; docs 030 (`2026-11-16-030-cors-allowlist-plan.md`), 031 (`…-031-cors-exact-edits.md`), 032-final (`…-032-cors-final-plan.md`, §9 WS caveat / §10 dependency), docs 016 Tauri/seed research.

**Question answered:** With cycle-032 backend CORS already merged, what is the *exact minimal* remaining edit set for (a) runtime FE API/WS URLs, (b) a WS-handshake `Origin` check reusing the same allowlist, and (c) a **deterministic, DB-free** test proving allowed/disallowed non-browser-origin behavior? What is the go/no-go, the risks/NVs, and the ordered edit sequence? Can criterion #2 be marked met?

---

## 0. Decision summary (go / no-go)

| Scope | Verdict | Reason |
|---|---|---|
| **(a) runtime FE config** `static/config.js` → `window.__FLORA_CONFIG__`, resolver `runtime > VITE_API_URL > default` | **GO** | 4 tiny FE files, no new dependency. Vite docs confirm `VITE_*` is build-time only [S5]; SvelteKit docs confirm `static/` + `%sveltekit.assets%` are the supported unaltered-asset path [S3]. `$env/dynamic/public` is Legacy (deprecated v3, removed v4) [S4] — the window global avoids that churn. |
| **(b) WS `Origin` check in `flora-api/src/routes/websocket.rs` using `app.cors_origins`** | **GO** | The handler already receives `headers: HeaderMap` and `State(state)` with `state.config.app.cors_origins` [L5][L7]. A pre-auth pure predicate + early `403` needs no new extractor, no new dependency, and no signature change. MDN: browsers send `Origin` on WS handshake and a `403` for a disliked origin is the documented CSWH defense [S9]. |
| **(c) deterministic non-browser-origin test** | **GO** | Put the `Origin` guard **before** auth. Then `disallowed Origin + no token → 403` and `allowed Origin + no token → 401` are both DB-free (no-token path returns 401 before any session lookup [L8]); a pure predicate test needs no router at all. Existing `flora-tests` WS harness supplies the router/oneshot pattern [L10]. |
| **Mark ADR-0002 re-entry #2 "met"** | **NO-GO from these edits alone** | Still required: live non-browser preflight echo + bearer `select-organization` round-trip on a seeded API (030 T8), browser regression lane green (030 T7), and an ADR/gate note recording the result (032 §10). Tauri CSP `connect-src` and real webview QA are separate gates (#3/#4). |

**One-line closure:** *Keep the cycle-032 `CorsLayer` unchanged; add a blocking `static/config.js` + pure resolver in `client.ts`; add one `origin_allowed(&HeaderMap, &[String])` predicate in `websocket.rs`, called before `extract_org_context`, rejecting any present-but-unlisted/`null` origin with `403`; prove it with two pure unit tests plus two router oneshot tests using a lazy (unconnected) `PgPool`; then mark #2 met only after the live seeded-API gate proof and ADR update.*

---

## 1. Code reality confirmed (read-only at HEAD `763b8c553`)

| ID | Fact | Evidence |
|---|---|---|
| C1 | ADR-0002 criterion #2 requires **runtime-configurable API/WS URLs** + **bearer/`select-organization` verified from a non-browser origin with explicit CORS allowlist**, no cookies, no `CORS::Any`. | `docs/adr/0002-…md:27-31` (`:29` = #2) [L14] |
| C2 | Backend CORS **is already hardened**: `cors_layer(origins: &[String])` builds `AllowOrigin::list(...)` + explicit methods/headers + 24h max-age, **no credentials**; unit tests cover allowed/denied/Tauri/wildcard/default. | `flora-api/src/middleware/cors.rs:26-55`, tests `:57-175` [L1] |
| C3 | Layer applied once at router creation from `app_state.config.app.cors_origins`, before `.with_state(app_state)`. WS routes `/ws` and `/api/v1/ws` are registered on the same router (so CORS layers over them, but CORS does not apply to WS handshakes). | `flora-api/src/routes/mod.rs:49-50,59-66` [L2] |
| C4 | `AppConfig.cors_origins: Vec<String>` exists; default = 4 browser origins (`localhost`/`127.0.0.1` × 5173/4173); `validate()` rejects empty/empty-entry/`*`/invalid `HeaderValue`, warns on loopback in production. **Tauri origins are NOT in the default.** | `flora-core/src/config/app.rs:18-19,46-51,78-132` [L3] |
| C5 | `AppState` has `pub config: Arc<Config>`; `AppConfig::validate()` is invoked at startup. | `flora-api/src/state/app_state.rs:24-28`; `flora-core/src/config/mod.rs`; `flora-api/src/main.rs` [L7] |
| C6 | `websocket_handler(ws: Result<WebSocketUpgrade,…>, State(state), headers: HeaderMap, Query(params))` → `extract_org_context()` → `verify_bearer_token()` → `ws.on_upgrade()`. **No `Origin` check anywhere.** Token resolves header-first then `?token=`. | `flora-api/src/routes/websocket.rs:105-110,116-145,150-179` [L5] |
| C7 | No-token path is DB-free: `resolve_ws_token` → `None` → `Err(UNAUTHORIZED)` at `extract_org_context` before any session lookup. | `websocket.rs:161-168`; `flora-api/src/extractors/auth.rs:33-74` [L8] |
| C8 | FE API base is **build-time only**: `API_BASE_URL = import.meta.env.VITE_API_URL \|\| 'http://localhost:3000'`. No `static/config.js`, no `window.__FLORA_CONFIG__`, no script in `app.html`. | `apps/frontend/src/lib/api/client.ts:3`; `apps/frontend/src/app.html:1-18`; `apps/frontend/static/` (only `robots.txt`) [L6][L9] |
| C9 | `buildWsUrl(base, token)` derives `ws(s)://…/api/v1/ws?token=…` from the same base; `connect()` calls it with `API_BASE_URL`. WS URL needs no second source of truth. | `apps/frontend/src/lib/api/websocket.ts:34-39,127`; unit test `tests/unit/websocket.test.ts:83-91` [L6b] |
| C10 | Existing deterministic WS router harness (DB-gated) builds `AppState` by hand, routes `/api/v1/ws` with `.with_state(state)`, and sends a complete upgrade handshake via `ServiceExt::oneshot`. `flora-tests` has `sqlx.workspace = true` (0.9.0) and `tower` util. | `flora-tests/src/contract/websocket.rs:40-96,188-209`; `flora-tests/Cargo.toml:23-40`; `Cargo.toml:150` [L10] |
| C11 | Toolchain pins: `axum 0.8.9` (+ `ws`, `multipart` in `flora-api`), `tower-http 0.7.0` (cors, trace), `tower 0.5`; FE `@sveltejs/kit ^2.63.0`, `vite ^8.0.16`, `adapter-node ^5.5.4`, `vitest ^4.1.8`. | `Cargo.toml:115,141-142`; `flora-api/Cargo.toml:37-39`; `apps/frontend/package.json:24-46` [L11] |
| C12 | CI FE lane: `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build`. `vite.config.ts` server project includes `tests/**/*.{test,spec}.{js,ts}`. | `.github/workflows/ci.yml:17-24`; `apps/frontend/vite.config.ts:36-44` [L12] |
| C13 | Workspace lints deny `unwrap_used`/`expect_used`/`panic`/`todo`/`unsafe_code`; test-only `expect_used` is scoped with `#[expect(…, reason)]`. New tests must avoid `unwrap`/`expect` (use `from_static`, `anyhow::ensure!`, `unwrap_or_default()`). | `Cargo.toml:41-88`; `AGENTS.md`; existing test style in `cors.rs:57-175`, `websocket.rs:487-559` [L13] |
| C14 | `.env.example:31-35` already documents `FLORA_APP__CORS_ORIGINS` (Figment array syntax, browser origins only). Add runtime FE keys here as comments only. | `.env.example:31-35` [L4] |
| C15 | Prior docs already flagged exactly this gap: CORS ≠ cross-origin boundary; WS needs an explicit `Origin` check; `?token=` logging needs separate assessment. | `…-032-cors-final-plan.md:559-577`; `…-030-cors-allowlist-plan.md:23,243` [L15] |

---

## 2. Primary-source findings (normative, with localizations cross-checked)

### 2.1 Vite: `VITE_*` is build-time — cannot satisfy the gate alone [S5]
Docs.rs-era and current Vite docs both state `import.meta.env` constants are "defined as global during dev and **statically replaced at build time**"; `VITE_*` values are "bundled into your source code at build time". Env precedence: process env at Vite invocation > mode files > generic files. Bun preloads `.env` into `process.env` and can interfere. Confirmed identically in **zh-CN** (`构建阶段被静态替换`) and **ja** (`ビルド時には静的に置き換えられます`) localizations, including the Bun caveat. ⇒ a runtime-injected global is required.

### 2.2 SvelteKit: supported injection points [S3][S4]
- `src/app.html` placeholders: `%sveltekit.assets%` (assets path or relative path to base), `%sveltekit.head%`, `%sveltekit.body%`.
- `static/` holds assets "served without any alteration to the name — such as `robots.txt`". ⇒ `config.js` belongs there.
- A classic `<script src="%sveltekit.assets%/config.js"></script>` in `<head>` before `%sveltekit.head%` runs before the (deferred) module bundle, so `window.__FLORA_CONFIG__` exists when `client.ts` is evaluated.
- `$env/dynamic/public` is under **Legacy**: "deprecated in SvelteKit 3 … removed in 4" [S4]. Current replacement is explicit `src/env.ts` + `$app/env/public` [S4b]. The framework-agnostic window global avoids both the deprecation and adapter-node's "production `.env` not auto-loaded" trap [S6].

### 2.3 Tauri v2: origins & CSP [S7][S8][S9]
- OS WebView (WKWebView/WebView2/WebKitGTK); IPC trust boundary is separate from HTTP CORS [S7].
- Production webview origins: `tauri://localhost` (macOS/Linux/iOS), `http://tauri.localhost` (Windows/Android default; `https://tauri.localhost` with `useHttpsScheme`; `tauri dev` uses normal `http://localhost:<port>`) [S8]. Tauri config docs show `Access-Control-Allow-Origin: http://tauri.localhost` as an actual served header [S8]. Default CSP example is `connect-src ipc: http://ipc.localhost` — **the API/WS origin must be added to `connect-src` once a Tauri build exists** [S9]. Tauri docs ship Français/Español/简体中文/日本語/German versions; the trust-boundary/CSP/header model is locale-invariant.
- No translated string enters any code path in this plan: config values are ASCII origin literals; error/log messages stay English (consistent with `flora-core/src/error.rs`).

### 2.4 tower-http 0.7.0 `AllowOrigin::list` — exact-match semantics confirmed in source [S1][S2]
`OriginInner::List(l) => origin.filter(|o| l.contains(o))` — i.e. `Vec<HeaderValue>::contains` (exact `HeaderValue` equality; **case-sensitive**, no normalization). `list()` **panics** on a literal `*` (`"Wildcard origin (`*`) cannot be passed…"`). `AppConfig::validate()` already rejects `*` before this is reached (C4), so no new panic path. **The WS predicate must use the same exact-string comparison** so CORS and WS never disagree about the same config entry.

### 2.5 Axum 0.8.9 `WebSocketUpgrade` [S10]
`FromRequestParts` yields `Result<WebSocketUpgrade, WebSocketUpgradeRejection>`; HTTP/1.1 requires `GET`; `on_upgrade()` finalizes the 101. It does **not** validate `Origin`. Because Flora's handler takes `Result<WebSocketUpgrade,…>` (C6), the handler body still runs on extraction failure — which is why an early `Origin` return works even in a oneshot harness.

### 2.6 MDN — WS `Origin` is the anti-CSWH control [S11][S12]
"All **browsers** send an `Origin` header. You can use this header for security (checking for same origin …) and send a **403 Forbidden** if you don't like what you see. This is effective against Cross Site WebSocket Hijacking (CSWH). … non-browser agents can send a faked `Origin`. Most applications reject requests without this header." MDN CORS (EN + pt-BR/es/de/fr/zh-CN/ja) confirms: `Authorization` triggers preflight, `Origin` is always sent on CORS requests, and `*` is invalid for credentialed requests — the cycle-032 design already follows this.

---

## 3. Exact minimal edits (sketches — **not applied**)

No change to `flora-api/src/middleware/cors.rs`, `flora-core/src/config/app.rs` validation, `routes/mod.rs`, or Cargo manifests. CORS backend from cycle 032 is kept as-is (C2/C3).

### E1 — `apps/frontend/static/config.js` (new file)
```js
// Deploy-time rendered; committed with safe local-dev defaults.
window.__FLORA_CONFIG__ = {
	apiBaseUrl: "http://localhost:3000",
	wsBaseUrl: null
};
```
Must be overwritten at container/deploy start (or rendered by entrypoint) with the real API origin. `%sveltekit.assets%/config.js` resolves to `/config.js` in adapter-node and to a relative path in a future static/Tauri build [S3].

### E2 — `apps/frontend/src/app.html` (one line, before `%sveltekit.head%`)
```html
<script src="%sveltekit.assets%/config.js"></script>
```
Classic (non-`async`, non-`defer`) so it is synchronous with HTML parsing and precedes the module bundle. Do not use `%sveltekit.env.*%` (that would be render-time/baked).

### E3 — `apps/frontend/src/app.d.ts` (type the global)
```ts
interface Window {
	__FLORA_CONFIG__?: { apiBaseUrl?: string; wsBaseUrl?: string | null };
}
```
(inside the existing `declare global` block). Without this, `svelte-check` fails on the new references.

### E4 — `apps/frontend/src/lib/api/client.ts` (replace line 3)
```ts
export function resolveApiBaseUrl(
	runtime: string | null | undefined,
	vite: string | undefined,
	fallback: string
): string {
	if (typeof runtime === 'string' && runtime.length > 0) return runtime;
	if (typeof vite === 'string' && vite.length > 0) return vite;
	return fallback;
}

export const API_BASE_URL = resolveApiBaseUrl(
	globalThis.__FLORA_CONFIG__?.apiBaseUrl,
	import.meta.env.VITE_API_URL,
	'http://localhost:3000'
);
```
Exporting the pure helper makes precedence trivially unit-testable (no `vi.stubEnv`/`resetModules` gymnastics). All call sites (`client.ts`, `websocket.ts`, `(auth)/+page.svelte`, `ApiClient`) keep using `API_BASE_URL` unchanged.

### E5 — `apps/frontend/src/lib/api/websocket.ts` (optional full WS override; 2 lines)
In `connect()`, change only the `buildWsUrl(...)` argument:
```ts
const url = buildWsUrl(globalThis.__FLORA_CONFIG__?.wsBaseUrl || API_BASE_URL, authState.token);
```
`buildWsUrl()` stays as-is, so existing test `tests/unit/websocket.test.ts:83-91` (which mocks `API_BASE_URL` at `:44-46`) still passes. Effective WS precedence: `wsBaseUrl` (runtime) → resolved `API_BASE_URL` (runtime > Vite > default) → `ws(s):///api/v1/ws?token=…`.

### E6 — `flora-api/src/routes/websocket.rs`: shared-allowlist origin guard
Add a pure helper near `resolve_ws_token`:
```rust
/// `None` (no Origin) is allowed for native/non-browser clients; a present
/// Origin must exactly match the configured CORS allowlist. `null` is denied
/// unless explicitly configured (validation forbids `*` but not `null`).
fn origin_allowed(headers: &HeaderMap, allowed: &[String]) -> bool {
    let Some(value) = headers.get(axum::http::header::ORIGIN) else {
        return true;
    };
    let Ok(value) = value.to_str() else {
        return false;
    };
    allowed.iter().any(|entry| entry == value)
}
```
Call it as the **first** statement of `websocket_handler`, before `extract_org_context` and before the `ws` match:
```rust
if !origin_allowed(&headers, &state.config.app.cors_origins) {
    tracing::warn!("WebSocket origin rejected");
    return StatusCode::FORBIDDEN.into_response();
}
```
`Response`, `StatusCode`, `HeaderMap`, `IntoResponse` are already imported (`:11-15`); `state.config` is `Arc<Config>` (C5). No new dependency, no signature change. Returning `403` (not 401) matches MDN's WS guidance [S11] and keeps "bad origin" distinguishable from "bad token". **Policy decision to record in the ADR:** absent `Origin` allowed (native interop); present-but-unlisted and `Origin: null` denied. Stricter alternative (deny absent) is valid but must be an explicit decision — see NV-3.

To accept Tauri, `FLORA_APP__CORS_ORIGINS` must include `tauri://localhost` and `http://tauri.localhost` (C4). One list serves both CORS and WS.

### E7 — deterministic tests (the core of (c))
**(i) Pure unit tests in `websocket.rs#[cfg(test)]`** — no router, no DB, no network:
- `origin_allowed_accepts_allowlisted_origins`: for `http://127.0.0.1:4173`, `tauri://localhost`, `http://tauri.localhost` → `true` (use `HeaderValue::from_static`, no `unwrap`).
- `origin_allowed_rejects_unlisted_and_null`: `https://evil.example`, `null` → `false`.
- `origin_allowed_accepts_missing_origin`: empty `HeaderMap` → `true`.

**(ii) Router-level oneshot tests in `flora-tests/src/contract/websocket.rs`** (DB-free): add a helper that builds `AppState` from a **lazy, unconnected** pool:
```rust
fn no_db_state(config: &Config) -> anyhow::Result<AppState> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect_lazy("postgres://unused@localhost/unused")?; // no connection attempt
    Ok(make_test_state(pool, config))
}
```
Extend `handshake_status` with an `origin: Option<&str>` parameter (add `.header("origin", …)`), then assert:
- `disallowed_origin_is_403_without_db`: `origin = "https://evil.example"`, token `None` → `FORBIDDEN` (fires before auth).
- `allowed_origin_passes_guard_then_fails_auth`: `origin = "tauri://localhost"` (after adding it to a cloned `config.app.cors_origins`), token `None` → `UNAUTHORIZED` (guard passed; no-token path is DB-free per C7).
Both are deterministic: no Postgres, no Valkey, no seed, no timing, no server. Mark them **not** `#[ignore]` — unlike the 8 existing DB-gated tests (C10).

**(iii) FE unit test in `apps/frontend/tests/unit/resolver.test.ts`** (server project): table-drive `resolveApiBaseUrl(runtime, vite, fallback)` for the 5 precedence cases, plus a `buildWsUrl('http://api:3000','t')` assertion (`ws://api:3000/api/v1/ws?token=t`) and a `wss:` case. Pure, no DOM/network.

### E8 — `.env.example` (documentation only)
Append commented keys near `:35`: `PUBLIC_API_BASE_URL` / `PUBLIC_WS_BASE_URL` (rendered into `static/config.js` at deploy), and note that Tauri origins must be added to `FLORA_APP__CORS_ORIGINS` when re-entry happens. `VITE_API_URL` stays as build-time fallback only.

---

## 4. Ordered edits & verification gates

1. **E6 + E7(i)** backend guard and pure tests → `cargo test -p flora-api websocket` (must be green, no DB).
2. **E7(ii)** router oneshot tests → `cargo test -p flora-tests contract::websocket::…` (run the two new tests **without** `FLORA_DATABASE__POSTGRES_URL`; they must pass while the 8 DB-gated ones remain `#[ignore]`).
3. **E1–E3 + E4 + E5** FE config chain → `bun run check` (global type), `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build` (CI parity, C12).
4. **E7(iii)** resolver tests included in step 3.
5. **E8** docs.
6. Full gates: `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test`; then the **execution** steps still owed: 030 T7 browser regression + 030 T8 live seeded-API non-browser preflight echo and bearer `select-organization` round-trip (curl does not enforce CORS — record both), and finally an ADR/gate note. **Do not mark #2 met before step 6.**

---

## 5. Risks / unknowns (NV)

| ID | Sev | Item |
|---|---|---|
| NV-1 | M | **Research only.** No live API, no seeded DB, no webview, no HTTP request was run. Header behavior is from docs/source [S1][S2][S11]; the live gate (030 T8) is mandatory. |
| NV-2 | M | **Missing-Origin policy.** Recommended allow-absent (native interop); MDN notes most apps reject absent `Origin` [S11]. If security prefers fail-closed, deny `None` too — but that requires every native client/curl WS probe to set `Origin` explicitly. Must be an explicit ADR decision, not an accident. |
| NV-3 | M | **Tauri origin-string ambiguity** (`tauri://localhost` vs `http(s)://tauri.localhost` vs scheme-less `tauri.localhost` vs `null`) [S8][S15]. Exact-match WS guard (C2/S2) means a wrong literal silently blocks the webview. Allow exactly what a Windows smoke proves; keep `null` denied. |
| NV-4 | M | **Default allowlist lacks Tauri origins** (C4). Until ops extends `FLORA_APP__CORS_ORIGINS`, WS from a Tauri webview gets 403. Ship the exact env value in E8; do not assume defaults suffice. |
| NV-5 | M | **`?token=` leakage.** WS token in query string can reach proxy/access logs; the Origin check does not fix this. Separate assessment owed (030 NV-6 / 032 §9), out of scope here. |
| NV-6 | M | **Reverse proxy** must forward `Origin` unchanged and support `Upgrade`; if it strips `Origin`, the allow-absent policy keeps connections working but the check is bypassed. Verify on the VPS path. |
| NV-7 | L | **Config is startup-only** — allowlist/`config.js` changes need API restart / redeploy (032 NV-16). Committed dev-default `config.js` silently used if deploy forgets to render it: add an ops check. |
| NV-8 | L | **Tauri CSP `connect-src`** must include API/WS origins or the webview blocks requests regardless of CORS/Origin checks [S9] — prerequisite for gates #3/#4, out of scope here. |
| NV-9 | L | **WebView preflight caching** (WebView2 per-origin) means an already-running client may need restart after config change [S15]. |
| NV-10 | L | **Bun `.env` precedence** can shadow `VITE_API_URL` [S5]; the runtime global wins by construction, but keep local `.env` gitignored (already true). |
| NV-11 | L | `X-Organization-ID` remains allowlisted for legacy/test compatibility; no route trusts it. Do not read the WS guard as tenancy enforcement. |

---

## 6. Sources

**Local / code (read-only at HEAD `763b8c553`):**
- [L1] `flora-api/src/middleware/cors.rs:26-55,57-175`
- [L2] `flora-api/src/routes/mod.rs:20-67` (WS `:49-50`, CORS `:59-61`)
- [L3] `flora-core/src/config/app.rs:18-19,46-51,78-132`
- [L4] `.env.example:31-35`
- [L5] `flora-api/src/routes/websocket.rs:105-110,116-145,150-179`; tests `:487-559`
- [L6] `apps/frontend/src/lib/api/client.ts:3,73-109`
- [L6b] `apps/frontend/src/lib/api/websocket.ts:34-39,127`; `tests/unit/websocket.test.ts:44-46,83-91`
- [L7] `flora-api/src/state/app_state.rs:24-28`; `flora-core/src/config/mod.rs`; `flora-api/src/main.rs`
- [L8] `flora-api/src/extractors/auth.rs:33-74`
- [L9] `apps/frontend/src/app.html:1-18`; `apps/frontend/static/robots.txt`
- [L10] `flora-tests/src/contract/websocket.rs:40-96,188-209`; `flora-tests/Cargo.toml:23-40`
- [L11] `Cargo.toml:115,141-142,150,223`; `flora-api/Cargo.toml:37-39`; `apps/frontend/package.json:24-46`; `apps/frontend/svelte.config.js:1,14`
- [L12] `.github/workflows/ci.yml:17-24`; `apps/frontend/vite.config.ts:36-44`
- [L13] `Cargo.toml:41-88`
- [L14] `docs/adr/0002-tauri-deferral-from-canonical-seed-frontend.md:27-31`
- [L15] `docs/research/2026-11-16-032-cors-final-plan.md:559-577`; `…-030-cors-allowlist-plan.md:23,243`

**Primary / external (fetched 2026-11-16):**
- [S1] [tower-http 0.7.0 — `AllowOrigin`](https://docs.rs/tower-http/0.7.0/tower_http/cors/struct.AllowOrigin.html)
- [S2] [tower-http 0.7.0 — `allow_origin.rs` source (`List` exact match; `list` panics on `*`)](https://docs.rs/tower-http/0.7.0/src/tower_http/cors/allow_origin.rs.html)
- [S3] [SvelteKit — Project structure (`static/`, `app.html`, `%sveltekit.assets%`)](https://svelte.dev/docs/kit/project-structure)
- [S4] [SvelteKit — `$env/dynamic/public` (Legacy: deprecated v3, removed v4)](https://svelte.dev/docs/kit/$env-dynamic-public)
- [S4b] [SvelteKit — Environment variables (`src/env.ts`, `$app/env/public`, `%sveltekit.env.*%`)](https://svelte.dev/docs/kit/environment-variables)
- [S5] [Vite — Env Variables and Modes (build-time replacement; Bun `.env` caveat)](https://vite.dev/guide/env-and-mode); [zh-CN](https://cn.vite.dev/guide/env-and-mode); [ja](https://ja.vite.dev/guide/env-and-mode)
- [S6] [SvelteKit — `adapter-node` (production `.env` not auto-loaded)](https://svelte.dev/docs/kit/adapter-node)
- [S7] [Tauri v2 — Security / trust boundaries](https://v2.tauri.app/security/)
- [S8] [Tauri v2 — Configuration reference (`frontendDist`, `HeaderConfig`, `http://tauri.localhost` served header)](https://v2.tauri.app/reference/config/)
- [S9] [Tauri v2 — CSP (`connect-src ipc: http://ipc.localhost`)](https://v2.tauri.app/security/csp/)
- [S10] [Axum 0.8.9 — `WebSocketUpgrade`](https://docs.rs/axum/0.8.9/axum/extract/ws/struct.WebSocketUpgrade.html)
- [S11] [MDN — Writing WebSocket servers (browsers send `Origin`; 403 for disliked origin; CSWH)](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers)
- [S12] [MDN — CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS); [pt-BR](https://developer.mozilla.org/pt-BR/docs/Web/HTTP/Guides/CORS); [es](https://developer.mozilla.org/es/docs/Web/HTTP/Guides/CORS)
- [S15] [prior research — Tauri origin / WebView2 preflight-cache field evidence](docs/research/2026-11-16-016-tauri-svelte5-packaging.md); `…-030-cors-allowlist-plan.md:75-83`
