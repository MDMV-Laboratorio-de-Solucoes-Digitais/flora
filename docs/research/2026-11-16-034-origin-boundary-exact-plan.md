# 034 — Origin boundary: exact, minimal, drop-in plan (runtime FE config + WS `Origin` check + deterministic non-browser proof)

**Date:** 2026-11-16. **Repo:** `flora` (archived checkout at `/home/luis/development/MDMV/projetos/Arquivados/flora`).
**Canonical base:** `github/dev@1be723ccadbc300af2b5adfed1915df47d4c103f` (merge PR #247, `docs/033-finalize`).
**Local HEAD:** `dev@babac3ea1287f4cdcd0ae8d3ccd9d4be26d42dc5` (merge of canonical into local). `git diff 1be723cc babac3ea` changes **only** `flora-api/src/routes/health.rs` (+48) and `flora-api/src/routes/mod.rs` (+1): other-agent `/metrics` work. **Never revert it; none of the edits below touch those two files.**
**Type:** research/design only — **no source, config, env, workflow, or test file was changed.** Only read-only HTTP GET/OPTIONS probes were issued against the running API (§3). This document is the only artifact written.
**Refreshes:** `docs/research/2026-11-16-033-origin-boundary-plan.md` (246 lines, written at old HEAD `763b8c553`). All facts below were re-read against canonical `1be723cc`; deltas are called out in §2.

---

## 0. Decision summary

| Scope | Verdict | Reason |
|---|---|---|
| **(a) runtime FE config** (`static/config.js` → `window.__FLORA_CONFIG__`, pure `resolveApiBaseUrl()` + `resolveWsBaseUrl()`, precedence runtime > `VITE_API_URL` > default, injection in `app.html`) | **GO** | 5 tiny FE files, no new dependency. Cycle-033 `client.ts` only added `ApiClient.search()`; line 3 is still the build-time constant, so the 033 edit applies cleanly (§2 D1). |
| **(b) WS `Origin` check in `websocket.rs` reusing `app.cors_origins` (exact match, absent allowed, present-unlisted + `null` denied, early 403 before auth, DB-free)** | **GO** | Handler already has `headers: HeaderMap` and `State(state)`; `StatusCode`/`IntoResponse` already imported. No new dependency, extractor, or signature change. |
| **(c) deterministic tests** (pure Rust units + Axum oneshot with `connect_lazy` + FE Vitest server project) | **GO** | Guard-before-auth gives `403` (bad origin) and `401` (no token) with no DB. `connect_lazy` pattern already exists in `flora-api/src/extractors/auth.rs:273`. CI FE lane runs only the `server` project (`.github/workflows/ci.yml:22`). |
| **(d) 030 T8 LIVE non-browser proof** | **GO with a blocker** | The API currently running on `localhost:3000` is **stale (pre-cycle-032)**: it echoes `Access-Control-Allow-Origin: *` for *every* origin, including `https://evil.example` and `null`, and emits `Allow-Methods/Headers: *`. Canonical code cannot produce this. The live proof must be re-run after a canonical rebuild/restart with `FLORA_APP__CORS_ORIGINS` exported (§3, §7). |
| **Mark ADR-0002 re-entry #2 “met” after edits + live proof** | **NO-GO as of this document; CONDITIONAL GO afterwards** | Needs: (i) all edits green under CI gates, (ii) a fresh canonical API build/restart, (iii) T8 preflight echo + bearer `select-organization` round-trip against that fresh build, (iv) browser regression lane T7 green, (v) ADR/gate note. Tauri CSP and real webview QA remain separate gates #3/#4. |

**One-line closure:** *Keep cycle-032 `CorsLayer` unchanged; add blocking `static/config.js` + two pure URL resolvers in `client.ts`; add one `origin_allowed(&HeaderMap, &[String])` predicate in `websocket.rs` called before `extract_org_context`, returning 403 for present-but-unlisted/`null` origins; prove with 3 pure + 2 oneshot Rust tests and 8 FE server-project assertions; then rebuild/restart the API and re-run T8 before marking #2 met — never against the currently running wildcard-CORS binary.*

---

## 1. Question answered

Exact minimal remaining edit set for ADR-0002 re-entry criterion #2 after cycles 032 (backend CORS) and 033 (search UI), refreshed against canonical `1be723cc`, including what is safe to execute autonomously and what needs authorization.

---

## 2. Refresh deltas vs 033 research (canonical `1be723cc`)

| ID | Delta / confirmation | Evidence |
|---|---|---|
| **D1** | Cycle 033 **did not** change the FE API base line. Canonical `apps/frontend/src/lib/api/client.ts:3` is still `export const API_BASE_URL = import.meta.env.VITE_API_URL \|\| 'http://localhost:3000';`. It added `ApiError`, `parseResponseBody`, `ApiClient.request` hardening and `ApiClient.search()` (lines 5-109). E4 below is written for this exact canonical content. | `git show 1be723cc:apps/frontend/src/lib/api/client.ts` |
| **D2** | Cycle 033 added `apps/frontend/tests/unit/searchApi.test.ts`, which imports `client.ts` and `authState` directly in the **server** Vitest project and passes. Therefore a new resolver test may import `client.ts` without DOM mocks; `AuthState.svelte.ts:11` guards with `globalThis.window !== undefined`, so Node import is safe. | `tests/unit/searchApi.test.ts:1-3`; `AuthState.svelte.ts:11,143` |
| **D3** | `websocket.test.ts:44-46` mocks `$lib/api/client` with **only** `API_BASE_URL`. If `websocket.ts` starts importing a second symbol from `client.ts`, that mock must be extended or all four existing WS tests break. E5 includes the exact one-line mock update. | `apps/frontend/tests/unit/websocket.test.ts:44-46` |
| **D4** | `vite.config.ts` server project includes `tests/**/*.{test,spec}.{js,ts}` and excludes `tests/e2e/**`; CI runs `--project server` only. New tests belong in `tests/unit/`. | `vite.config.ts:36-44`; `.github/workflows/ci.yml:22` |
| **D5** | No `verify.sh` at repo root (AGENTS.md’s caution is correct); Rust gates are `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test`, plus six DB-gated suites in `rust.yml`. | filesystem check; `AGENTS.md` |
| **D6** | `flora-tests/Cargo.toml:33` has `sqlx.workspace = true`; workspace `sqlx 0.9.0` enables `runtime-tokio` + `postgres` (`Cargo.toml:150`), so `PgPoolOptions::connect_lazy` is available with no manifest change. | `flora-tests/Cargo.toml:23-40`; `Cargo.toml:150` |
| **D7** | `flora-core/src/bin/flora-mint-session.rs` is gated by feature `dev-mint` (`flora-core/Cargo.toml:67,70-72`) and emits a Playwright storageState; the simpler curl-token path is the Node recipe in `docs/dev-smoke.md:72-90`. | `flora-core/Cargo.toml:63-72`; `docs/dev-smoke.md:60-95` |
| **D8** | 030 T8’s sample org id `11111111-1111-7111-8111-111111111111` (`…-030-cors-allowlist-plan.md:202`) is a **typo**. The seeded Acme org is `11111111-1111-1111-1111-111111111111` (`dev-seed.sql:14,82,106`; `test-flora.sh:18`). Use the latter. | `dev-seed.sql`; `test-flora.sh:18` |
| **D9** | **NEW / critical:** live API is stale — wildcard CORS for all origins and no `/llms.txt`. See §3. This invalidates using the running process as evidence of cycle-032 behavior. | read-only curl probes, §3 |
| **D10** | Unchanged facts from 033 confirmed: `cors.rs:26-55` uses `AllowOrigin::list` (exact match, no credentials); `app.rs:18-19,46-51,78-132` default origins + validation; `routes/mod.rs` (canonical) registers `/ws` and `/api/v1/ws` and applies CORS once; `websocket.rs:116-179` has no `Origin` check; no-token path returns 401 before DB (`extractors/auth.rs:33-74`). | files read at `1be723cc` |

---

## 3. LIVE evidence (read-only probes, 2026-11-16)

Commands issued (no state change): `GET /health`, `/health/ready`, `/health/live`; `OPTIONS` preflights with `Origin` + `Access-Control-Request-Method/Headers`; `GET /metrics`, `/api/v1/search?q=x`, `/llms.txt`.

| Probe | Observed | Canonical expectation |
|---|---|---|
| `GET /health*` | `200` | `200` (API is up) |
| `OPTIONS /api/v1/auth/select-organization`, `Origin: http://127.0.0.1:4173`, `ACRM: POST`, `ACRH: authorization,content-type` | `200`, **`access-control-allow-origin: *`**, `access-control-allow-methods: *`, `access-control-allow-headers: *`, `access-control-max-age: 86400` | ACAO exactly `http://127.0.0.1:4173`; ACAM an explicit method list; ACAH an explicit header list |
| Same preflight with `Origin: https://evil.example` | identical `*` echo | **no ACAO** |
| Same with `Origin: tauri://localhost`, `http://tauri.localhost`, `null` | all echo `*` | no ACAO (these are not in the default allowlist) |
| Real `GET /health` with `Origin: http://localhost:5173` | ACAO `*` | ACAO exact origin |
| `GET /metrics` | `404` | `404` on canonical (the `/metrics` route exists only in local `babac3ea`) |
| `GET /llms.txt` | `404` | `200` on canonical (`routes/health.rs::llms_txt`) |

**Interpretation:** `AllowOrigin::list` can never emit `*` (it returns the matched `HeaderValue` verbatim), `allow_methods([...])` never emits `*` (it serializes an explicit list), and `AppConfig::validate()` rejects any `*` entry before `list()` could panic. Therefore the running process was built from a **pre-032 binary** (or an otherwise different codebase). Any T8 evidence captured against it would be false-positive for the *old* wildcard policy and would actively contradict the hardened code. **This is the gating NV for marking #2 met.**

---

## 4. Exact edits (complete code, imports, file:line, test impact)

All line numbers refer to canonical `1be723cc` (identical to local working tree for every file below except `routes/health.rs`/`routes/mod.rs`, which are out of scope).

### E1 — `apps/frontend/static/config.js` (NEW file; directory exists, currently only `robots.txt`)

```js
// Deploy-time rendered. Committed values are safe local-dev defaults.
// Overwrite at container/entrypoint start with the real API origin; see .env.example (E9).
window.__FLORA_CONFIG__ = {
	apiBaseUrl: "http://localhost:3000",
	wsBaseUrl: null
};
```

- **Test impact:** none. `static/` assets are served unaltered; SvelteKit `%sveltekit.assets%/config.js` resolves to `/config.js` under `adapter-node`.

### E2 — `apps/frontend/src/app.html`, insert at line 13 (immediately before `%sveltekit.head%`)

```html
		<script src="%sveltekit.assets%/config.js"></script>
```

Classic (no `async`/`defer`) so the global exists before the deferred module bundle evaluates `client.ts`. Do **not** use `%sveltekit.env.*%` (baked at render time).

### E3 — `apps/frontend/src/app.d.ts`, insert after line 3 (`declare global {`), before `namespace App`

```ts
	/** Deploy-time runtime config injected by `/config.js` before the app bundle. */
	interface Window {
		__FLORA_CONFIG__?: {
			apiBaseUrl?: string;
			wsBaseUrl?: string | null;
		};
	}
	// Declared as a global var as well so `globalThis.__FLORA_CONFIG__` type-checks
	// in the Node-based Vitest "server" project, where `window` does not exist.
	var __FLORA_CONFIG__: Window['__FLORA_CONFIG__'];
```

Complete resulting file:

```ts
// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	/** Deploy-time runtime config injected by `/config.js` before the app bundle. */
	interface Window {
		__FLORA_CONFIG__?: {
			apiBaseUrl?: string;
			wsBaseUrl?: string | null;
		};
	}
	// Declared as a global var as well so `globalThis.__FLORA_CONFIG__` type-checks
	// in the Node-based Vitest "server" project, where `window` does not exist.
	var __FLORA_CONFIG__: Window['__FLORA_CONFIG__'];
	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
```

- **Imports:** none.
- **Test impact:** fixes `bun run check` for the new `globalThis.__FLORA_CONFIG__` references. If ESLint ever flags `no-var` (it is not enabled in `eslint.config.js`), replace the `var` with a local typed cast in `client.ts` (NV-10).

### E4 — `apps/frontend/src/lib/api/client.ts`, replace line 3 with the block below

Imports unchanged (lines 1-2 stay). New exported code, inserted at line 3:

```ts
/** Default API origin when neither runtime config nor VITE_API_URL is present. */
export const DEFAULT_API_BASE_URL = 'http://localhost:3000'; // Default local Rust backend URL

/** Shape of the deploy-time global injected by `static/config.js`. */
export type FloraRuntimeConfig = {
	apiBaseUrl?: string;
	wsBaseUrl?: string | null;
};

/**
 * Resolves the HTTP API origin: runtime global > Vite build env > default.
 * Pure so it is testable without a browser, network, or module reset.
 */
export function resolveApiBaseUrl(
	runtime: string | null | undefined,
	vite: string | undefined,
	fallback: string = DEFAULT_API_BASE_URL
): string {
	if (typeof runtime === 'string' && runtime.trim().length > 0) return runtime.trim();
	if (typeof vite === 'string' && vite.trim().length > 0) return vite.trim();
	return fallback;
}

/** Resolves the WS origin: explicit runtime override, else the resolved API origin. */
export function resolveWsBaseUrl(
	runtimeWs: string | null | undefined,
	apiBaseUrl: string
): string {
	if (typeof runtimeWs === 'string' && runtimeWs.trim().length > 0) return runtimeWs.trim();
	return apiBaseUrl;
}

export const API_BASE_URL = resolveApiBaseUrl(
	globalThis.__FLORA_CONFIG__?.apiBaseUrl,
	import.meta.env.VITE_API_URL
);
```

- **Imports:** none new. `globalThis.__FLORA_CONFIG__` is typed by E3.
- **Call sites:** all existing consumers (`ApiClient.request`, `ApiClient.search`, websocket.ts, `(auth)/+page.svelte`) keep using `API_BASE_URL` unchanged — no other FE file changes.
- **Test impact:** existing `tests/unit/searchApi.test.ts:38` still sees `API_BASE_URL === 'http://localhost:3000'` in the Node test env (global absent, `VITE_API_URL` unset) → green. New tests in E7(iii).

### E5 — `apps/frontend/src/lib/api/websocket.ts`, two edits

**(i) line 2** — add the second import from `client` (line 3 stays unchanged):

```ts
import { API_BASE_URL, resolveWsBaseUrl } from './client';
```

**(ii) line 127** — replace `const url = buildWsUrl(API_BASE_URL, authState.token);` with:

```ts
		const wsBase = resolveWsBaseUrl(globalThis.__FLORA_CONFIG__?.wsBaseUrl, API_BASE_URL);
		const url = buildWsUrl(wsBase, authState.token);
```

`buildWsUrl()` (lines 34-39) is untouched, so `tests/unit/websocket.test.ts:83-91` keeps passing. Effective WS precedence: runtime `wsBaseUrl` → resolved `API_BASE_URL` (runtime > Vite > default) → `ws(s)://…/api/v1/ws?token=…`.

**Mandatory companion edit — `apps/frontend/tests/unit/websocket.test.ts:44-46`** (otherwise all four existing WsClient tests throw on the mocked missing export):

```ts
vi.mock('$lib/api/client', () => ({
	API_BASE_URL: 'http://localhost:3000',
	resolveWsBaseUrl: (ws: string | null | undefined, api: string) =>
		typeof ws === 'string' && ws.trim().length > 0 ? ws.trim() : api
}));
```

- **Test impact:** existing WS tests stay green; new pure WS-derivation assertions can be added in E7(iii).

### E6 — `flora-api/src/routes/websocket.rs`, two production edits

**(i) Insert pure predicate after line 156** (after `resolve_ws_token`):

```rust
/// Returns `true` when the handshake carries no `Origin` (native/non-browser client)
/// or when a present `Origin` exactly matches the configured CORS allowlist.
///
/// `Origin: null` (opaque origin) is denied unless explicitly configured.
/// `AppConfig::validate()` forbids `*`, so this predicate and `cors_layer` cannot
/// disagree about any configured entry.
#[must_use]
fn origin_allowed(headers: &HeaderMap, allowed: &[String]) -> bool {
    let Some(value) = headers.get(axum::http::header::ORIGIN) else {
        return true;
    };
    let Ok(value) = value.to_str() else {
        return false;
    };
    allowed.iter().any(|entry| entry.as_str() == value)
}
```

- **Imports:** none new — `HeaderMap` is already imported at line 13; `axum::http::header::ORIGIN` is used fully qualified (consistent with `AUTHORIZATION` at line 152).

**(ii) Insert guard as the first statement of `websocket_handler`, at line 122** (before the `// Validate session authority first…` comment):

```rust
    // Reject present-but-unlisted/opaque origins before any auth or DB work.
    if !origin_allowed(&headers, &state.config.app.cors_origins) {
        tracing::warn!("WebSocket origin rejected");
        return StatusCode::FORBIDDEN.into_response();
    }
```

- **Imports:** none new — `StatusCode` (line 13) and `IntoResponse` (line 14) are already imported.
- **Why before auth:** `403` for a bad origin is distinguishable from `401` for a bad token, and both new router tests stay DB-free (no-token path returns at `websocket.rs:166-168` before `verify_bearer_token`).
- **Policy recorded:** absent `Origin` allowed (native/interop); present-but-unlisted and `null` denied. One list (`app.cors_origins`) serves both CORS and WS. Tauri origins must be added by ops via `FLORA_APP__CORS_ORIGINS` (E9) — they are not in the defaults.
- **Test impact:** 3 new pure tests (E7(i)); no existing test changes.

### E7 — deterministic tests

#### (i) Pure Rust unit tests — `flora-api/src/routes/websocket.rs`, `mod tests`, insert before line 559 (closing brace)

Uses `use super::*;` and `use axum::http::HeaderMap;` already present at lines 489-490.

```rust
    #[test]
    fn origin_allowed_accepts_allowlisted_origins() {
        let mut headers = HeaderMap::new();
        let _ = headers.insert(
            axum::http::header::ORIGIN,
            axum::http::HeaderValue::from_static("tauri://localhost"),
        );
        let allowed = vec![
            "http://127.0.0.1:4173".to_owned(),
            "tauri://localhost".to_owned(),
            "http://tauri.localhost".to_owned(),
        ];
        assert!(origin_allowed(&headers, &allowed));
    }

    #[test]
    fn origin_allowed_rejects_unlisted_and_null_origins() {
        let allowed = vec!["http://127.0.0.1:4173".to_owned()];
        for raw in ["https://evil.example", "null"] {
            let mut headers = HeaderMap::new();
            let _ = headers.insert(
                axum::http::header::ORIGIN,
                axum::http::HeaderValue::from_static(raw),
            );
            assert!(
                !origin_allowed(&headers, &allowed),
                "origin {raw} must be denied"
            );
        }
    }

    #[test]
    fn origin_allowed_accepts_missing_origin() {
        let headers = HeaderMap::new();
        let allowed = vec!["http://127.0.0.1:4173".to_owned()];
        assert!(origin_allowed(&headers, &allowed));
    }
```

No `unwrap`/`expect`/`panic`; `let _ =` satisfies `unused_results = "deny"` (Cargo.toml:45). `from_static` takes `&'static str` (array items qualify).

#### (ii) DB-free Axum oneshot tests — `flora-tests/src/contract/websocket.rs`, inside `mod db_websocket`

**Imports:** none new. Already present: `Config`, `AppState`, `websocket_handler`, `PgPool` (line 35), `Request/Body/StatusCode` (lines 5-9), `ServiceExt` (line 37), `anyhow`, `axum::Router`, `get`.

Insert `no_db_state` after `make_test_state` (line 96):

```rust
    /// Builds a fully wired AppState over an unconnected pool: no DB, seed or server.
    fn no_db_state(config: &Config) -> anyhow::Result<AppState> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy("postgresql://unused@localhost/unused")?;
        Ok(make_test_state(pool, config))
    }
```

Insert `handshake_status_with_origin` after `handshake_status` (line 209). **Do not change the existing `handshake_status` signature** — this keeps all 9 DB-gated tests untouched:

```rust
    async fn handshake_status_with_origin(
        state: AppState,
        token: Option<&str>,
        origin: Option<&str>,
    ) -> anyhow::Result<StatusCode> {
        let router = ws_router(state);
        let mut builder = Request::builder()
            .method("GET")
            .uri("/api/v1/ws")
            .header("connection", "Upgrade")
            .header("upgrade", "websocket")
            .header("sec-websocket-version", "13")
            .header("sec-websocket-key", "dGhlIHNhbXBsZSBub25jZQ==");
        if let Some(origin) = origin {
            builder = builder.header("origin", origin);
        }
        if let Some(token) = token {
            builder = builder.header("authorization", format!("Bearer {token}"));
        }
        let request = builder.body(Body::empty())?;
        let response = ServiceExt::oneshot(router, request).await?;
        Ok(response.status())
    }
```

Insert the two tests before line 413 (module close). **Not** `#[ignore]`:

```rust
    #[tokio::test]
    async fn ws_disallowed_origin_is_403_without_db() -> anyhow::Result<()> {
        let config = Config::default();
        let state = no_db_state(&config)?;
        let status =
            handshake_status_with_origin(state, None, Some("https://evil.example")).await?;
        anyhow::ensure!(status == StatusCode::FORBIDDEN, "expected 403, got {status}");
        Ok(())
    }

    #[tokio::test]
    async fn ws_allowed_origin_passes_guard_then_fails_auth_without_db() -> anyhow::Result<()> {
        let mut config = Config::default();
        config.app.cors_origins.push("tauri://localhost".to_owned());
        let state = no_db_state(&config)?;
        let status = handshake_status_with_origin(state, None, Some("tauri://localhost")).await?;
        anyhow::ensure!(
            status == StatusCode::UNAUTHORIZED,
            "expected 401, got {status}"
        );
        Ok(())
    }
```

Both are deterministic: no Postgres, no Valkey, no seed, no timer. `connect_lazy` performs no connection attempt (same pattern as `flora-api/src/extractors/auth.rs:273`). Mocked WS upgrade headers make `WebSocketUpgrade` extraction succeed, but the guard/auth returns before `on_upgrade()`.

#### (iii) FE Vitest server-project tests — `apps/frontend/tests/unit/resolver.test.ts` (NEW)

Imports from `client.ts` only (Node-safe per D2); no DOM mocks, no `vi.stubEnv`, no module reset.

```ts
import { describe, expect, it } from 'vitest';
import {
	DEFAULT_API_BASE_URL,
	resolveApiBaseUrl,
	resolveWsBaseUrl
} from '../../src/lib/api/client';

describe('resolveApiBaseUrl', () => {
	it('prefers the runtime value over Vite env and default', () => {
		expect(
			resolveApiBaseUrl('https://api.runtime', 'https://api.vite', 'https://api.default')
		).toBe('https://api.runtime');
	});

	it('uses the Vite value when the runtime value is absent', () => {
		expect(resolveApiBaseUrl(undefined, 'https://api.vite', 'https://api.default')).toBe(
			'https://api.vite'
		);
	});

	it('falls back to the default when both inputs are empty', () => {
		expect(resolveApiBaseUrl(null, undefined, DEFAULT_API_BASE_URL)).toBe(DEFAULT_API_BASE_URL);
		expect(resolveApiBaseUrl('   ', '  ', DEFAULT_API_BASE_URL)).toBe(DEFAULT_API_BASE_URL);
	});
});

describe('resolveWsBaseUrl', () => {
	it('uses an explicit runtime WS override', () => {
		expect(resolveWsBaseUrl('wss://ws.runtime', 'https://api.runtime')).toBe('wss://ws.runtime');
	});

	it('derives the WS base from the resolved API origin otherwise', () => {
		expect(resolveWsBaseUrl(null, 'https://api.runtime')).toBe('https://api.runtime');
		expect(resolveWsBaseUrl(undefined, DEFAULT_API_BASE_URL)).toBe(DEFAULT_API_BASE_URL);
		expect(resolveWsBaseUrl('', DEFAULT_API_BASE_URL)).toBe(DEFAULT_API_BASE_URL);
	});
});
```

`vite.config.ts:20` sets `requireAssertions: true` — every `it` has at least one `expect`. Kept within Prettier `printWidth: 100`, tabs, single quotes, no trailing commas.

### E8 — `.env.example`, append after line 39 (documentation only)

```ini
# --- Frontend runtime config (rendered into static/config.js at deploy) ---
# PUBLIC_API_BASE_URL=https://flora.example.com
# PUBLIC_WS_BASE_URL=wss://flora.example.com
# These are injected as window.__FLORA_CONFIG__ BEFORE the app bundle; they win over
# VITE_API_URL (build-time fallback only). Leave wsBaseUrl empty to derive from API base.
```

Keep the existing note that `FLORA_APP__CORS_ORIGINS` must also include the webview origin for a Tauri re-entry (already at `.env.example:36-39`).

---

## 5. Ordered edit list

1. **E6 + E7(i)** backend guard + pure Rust tests → `cargo test -p flora-api websocket` (no DB).
2. **E7(ii)** DB-free oneshot tests → `cargo test -p flora-tests ws_disallowed_origin_is_403_without_db` and `… ws_allowed_origin_passes_guard_then_fails_auth_without_db` (run **without** `FLORA_DATABASE__POSTGRES_URL`).
3. **E1 + E3 + E2 + E4 + E5** FE config chain, including the mandatory `websocket.test.ts` mock update.
4. **E7(iii)** new resolver test; run the FE gates (step 5).
5. **E8** docs.
6. **Full gates** (§6). Only then proceed to the execution gates (§7): rebuild/restart API from canonical, T8 live proof, T7 browser regression, ADR/gate note.

---

## 6. Verification commands

```bash
# Rust (repo root) — no DB needed for steps 1-2
cargo fmt --all -- --check
cargo test -p flora-api websocket
cargo test -p flora-tests ws_disallowed_origin_is_403_without_db
cargo test -p flora-tests ws_allowed_origin_passes_guard_then_fails_auth_without_db

# Full Rust gates (CI parity: .github/workflows/rust.yml)
cargo clippy --workspace --all-targets -- -D warnings
cargo test
# DB-gated suites remain gated by FLORA_DATABASE__POSTGRES_URL in CI.

# Frontend (cd apps/frontend) — CI parity: .github/workflows/ci.yml:17-24
bun install
bun run check                                  # svelte-check, global type
bun run lint                                   # prettier --check . && eslint .
bun run test:unit -- --run --project server
bun run build

# Optional dependency gate before any new dependency (none added here)
./scripts/supply-chain.sh 2>/dev/null || { cargo deny check && cargo audit; }
```

**Note:** no new dependency is introduced, so the supply-chain gate is informational only.

---

## 7. 030 T8 LIVE non-browser proof — exact commands and authorization boundary

### 7.1 Safe to execute autonomously (read-only, no DB write, no process change)

These only send `OPTIONS`/`GET`; they cannot mutate state. Reproduce after the canonical rebuild/restart (step 6) — the current stale binary will echo `*` everywhere (§3).

```bash
export API='http://localhost:3000'
export ALLOWED='http://127.0.0.1:4173'

# (1) Preflight MUST echo the exact allowlisted origin; ACAH must include authorization;
#     no Access-Control-Allow-Credentials; Vary: Origin present.
curl -si -X OPTIONS "$API/api/v1/auth/select-organization" \
  -H "Origin: $ALLOWED" \
  -H 'Access-Control-Request-Method: POST' \
  -H 'Access-Control-Request-Headers: authorization,content-type' \
  | grep -iE '^(HTTP/|access-control-|vary:)'
# EXPECT: 200; access-control-allow-origin: http://127.0.0.1:4173
#         ACAM explicit list; ACAH includes authorization; no ACAC; Vary: Origin

# (2) Negative controls — must print NOTHING (no ACAO line):
for O in 'https://evil.example' 'null' 'tauri://localhost' 'http://tauri.localhost'; do
  printf 'origin %s -> ' "$O"
  curl -si -X OPTIONS "$API/health" -H "Origin: $O" \
    -H 'Access-Control-Request-Method: GET' \
    | grep -i '^access-control-allow-origin:' || echo '(no ACAO = denied)'
done

# (3) WS Origin guard, DB-free (no token): disallowed origin -> 403;
#     allowed origin -> passes guard, then 401 from missing token.
for O in 'https://evil.example' 'null' "$ALLOWED"; do
  printf 'ws origin %s -> ' "$O"
  curl -si -X GET "$API/api/v1/ws" -m 5 \
    -H 'Connection: Upgrade' -H 'Upgrade: websocket' \
    -H 'Sec-WebSocket-Version: 13' \
    -H 'Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==' \
    -H "Origin: $O" | head -1
done
# EXPECT: 403, 403, 401 respectively. On the stale binary all three would be 401.
```

### 7.2 Needs authorization before execution (DB write / process change / secret handling)

| Step | Why authorization |
|---|---|
| `cargo build --release -p flora-api` + restart the service with `FLORA_APP__CORS_ORIGINS` exported | Changes the running process; the currently running binary is stale (§3). Without this, no live proof is honest. |
| Re-run `PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -d flora -f scripts/dev-smoke-session.sql` | Upserts/activates a session row (DB write); needed because a prior `select-organization` may have revoked jti `018f0000-0000-7000-8000-000000000001`. |
| `POST /api/v1/auth/select-organization` | **Mutates DB**: revokes the current session and creates a new rotated-jti session. Dev seed only, but still a write. |
| Render/overwrite `apps/frontend/static/config.js` with real origins | Deploy-time change; committed dev defaults are intentional. |
| ADR-0002 update / gate note | Governance artifact, not a code edit. |

**Token acquisition (no OIDC needed).** Primary: the Node recipe from `docs/dev-smoke.md:72-90` (read-only, uses the effective secret, no build):

```bash
export FLORA_TOKEN="$(node - <<'NODE'
const crypto = require('crypto');
const b64 = (o) => Buffer.from(JSON.stringify(o)).toString('base64url');
const secret = process.env.FLORA_APP__JWT_SECRET
  || 'change-me-to-a-random-secret-at-least-32-characters-long';
const now = Math.floor(Date.now() / 1000);
const claims = {
  sub: '22222222-2222-2222-2222-222222222222',
  jti: '018f0000-0000-7000-8000-000000000001',
  organization_id: '00000000-0000-0000-0000-000000000000',
  email: 'alice@acme.test', exp: now + 3600, iat: now, ver: 1
};
const input = `${b64({ alg: 'HS256', typ: 'JWT' })}.${b64(claims)}`;
process.stdout.write(`${input}.${crypto.createHmac('sha256', secret).update(input).digest('base64url')}\n`);
NODE
)"
```

Alternative (with authorization, requires build): `cargo run -p flora-core --features dev-mint --bin flora-mint-session -- --storage-state /tmp/flora-state.json --frontend-origin http://127.0.0.1:4173`, then `FLORA_TOKEN=$(jq -r '.origins[0].localStorage[] | select(.name=="flora.auth.v1") | .value | fromjson | .token' /tmp/flora-state.json)`. `flora-mint-session.rs:59-67` mints the same Alice/nil-org claims and refuses `production` (`flora-mint-session.rs:50-54`).

**Bearer + `select-organization` round-trip from a non-browser origin** (run only after §7.2 authorization is granted):

```bash
export ORG_ID='11111111-1111-1111-1111-111111111111'   # seeded Acme org (NOT the 030 doc typo)

# Rotate to an org-scoped session. Expect 200 + ACAO echo of $ALLOWED.
SELECT_RESP=$(curl -si -X POST "$API/api/v1/auth/select-organization" \
  -H "Authorization: Bearer $FLORA_TOKEN" \
  -H "Origin: $ALLOWED" \
  -H 'Content-Type: application/json' \
  -d "{\"organization_id\":\"$ORG_ID\"}")
printf '%s\n' "$SELECT_RESP" | grep -iE '^(HTTP/|access-control-allow-origin:|vary:)'

SCOPED=$(printf '%s\n' "$SELECT_RESP" | sed -n 's/.*"session_token":"\([^"]*\)".*/\1/p')
# Prove the scoped token works from the same non-browser origin: expect 200 + ACAO.
curl -si -H "Authorization: Bearer $SCOPED" -H "Origin: $ALLOWED" \
  "$API/api/v1/workspaces" | grep -iE '^(HTTP/|access-control-allow-origin:|vary:)'
```

**Caveat to record in evidence:** `curl` does **not** enforce CORS — it proves the server emits the right headers (and that Bearer + `select-organization` work from a non-browser client), not that a browser/webview would block a disallowed origin. T7 (real webview/browser smoke, docs 028/029 at `:4173`) remains the enforcement proof and must stay green.

---

## 8. Risks / unknowns (NV)

| ID | Sev | Item |
|---|---|---|
| **NV-1** | **H** | **Live API is stale** (§3): wildcard CORS for all origins, no `/llms.txt`. No live gate can be honest until a canonical rebuild/restart. Treat any pre-restart T8 “pass” as invalid. |
| **NV-2** | M | Missing-Origin policy is an explicit decision: allow absent (native interop, chosen here). MDN notes most applications reject absent `Origin` [S11]. Fail-closed is valid but would require every native client and curl WS probe to set `Origin`; record the choice in the ADR. |
| **NV-3** | M | `Origin` string ambiguity (`tauri://localhost` vs `http(s)://tauri.localhost` vs `null`); exact match means a wrong literal silently blocks the webview. Allow exactly what a Windows smoke proves; keep `null` denied. |
| **NV-4** | M | Default allowlist lacks Tauri origins (`app.rs:46-51`). Until ops extends `FLORA_APP__CORS_ORIGINS`, WS from a Tauri webview gets 403. |
| **NV-5** | M | `?token=` WS query token can reach proxy/access logs; the Origin check does not fix this (030 NV-6 / 032 §9). Separate assessment owed. |
| **NV-6** | M | Reverse proxy must forward `Origin` unchanged and support `Upgrade`; if it strips `Origin`, allow-absent keeps connections working while the check is bypassed. Verify on the VPS path. |
| **NV-7** | M | Secret mismatch: the Node recipe falls back to the template secret; if the running API uses another `FLORA_APP__JWT_SECRET`, minted tokens 401. Capture the effective secret from the process environment (authorization) or restart with a known dev secret. |
| **NV-8** | L | Config is startup-only; allowlist / `config.js` changes require API restart / FE redeploy. A committed dev-default `config.js` silently used if deploy forgets to render it — add an ops check. |
| **NV-9** | L | Tauri CSP `connect-src` must include API/WS origins or the webview blocks requests regardless of CORS/Origin checks (gates #3/#4, out of scope). |
| **NV-10** | L | If `var __FLORA_CONFIG__` in `app.d.ts` ever trips a lint rule, replace with a local typed cast in `client.ts`; no behavior change. |
| **NV-11** | L | Bun `.env` precedence can shadow `VITE_API_URL` [S5]; the runtime global wins by construction. Keep local `.env` gitignored. |
| **NV-12** | L | `X-Organization-ID` stays allowlisted for legacy/test compatibility (`cors.rs:33-34`); no route trusts it. The WS guard is not tenancy enforcement. |
| **NV-13** | L | No source file was changed and no test was executed in this audit; all “expected” results are source-derived. The FE/Rust verification commands in §6 are for the implementer. |

---

## 9. Can ADR-0002 #2 honestly be marked met?

**Not by these edits alone — and not today, because of NV-1.** Wording from 030 §Step 6 / 032 §9 requires all of:

1. (a) API/WS URLs runtime-configurable — satisfied by E1-E5 **once implemented and green**.
2. Explicit CORS allowlist with no `Any`/credentials — already true in canonical code (cycle 032), but **the running binary contradicts it**; must be rebuilt/restarted to be observable.
3. T8 live: preflight echo for allowed origins, no ACAO for disallowed/`null`, and Bearer `select-organization` → 200 + ACAO from a non-browser origin — **owed** against the fresh canonical build.
4. T7 browser regression lane green — **owed**; not run in this audit.

Additionally: record the absent-Origin policy decision (NV-2) and the T8 evidence in an ADR/gate note (not in a research doc). Gates #3/#4 (named owner + Linux spike) remain deferred by ADR-0002 and are unaffected by cycle 034.

**Verdict:** *conditional go* — after E1-E9 land green under §6, the API is rebuilt/restarted, §7.1 + §7.2 pass with evidence, and the ADR note is written, then criterion #2 can be marked met. Until then it stays “accepted-as-deferred”.

---

## 10. Sources

**Primary / external (fetched 2026-11-16; Vite and MDN re-fetched this cycle):**
- [S1] [tower-http 0.7.0 — `AllowOrigin`](https://docs.rs/tower-http/0.7.0/tower_http/cors/struct.AllowOrigin.html)
- [S2] [tower-http 0.7.0 — `allow_origin.rs` (`List` exact match; `list` panics on `*`)](https://docs.rs/tower-http/0.7.0/src/tower_http/cors/allow_origin.rs.html)
- [S3] [SvelteKit — Project structure (`static/`, `app.html`, `%sveltekit.assets%`)](https://svelte.dev/docs/kit/project-structure)
- [S4] [SvelteKit — `$env/dynamic/public` (Legacy: deprecated v3, removed v4)](https://svelte.dev/docs/kit/$env-dynamic-public)
- [S5] [Vite — Env Variables and Modes (build-time replacement; process env precedence; Bun `.env` caveat)](https://vite.dev/guide/env-and-mode)
- [S6] [SvelteKit — `adapter-node`](https://svelte.dev/docs/kit/adapter-node)
- [S10] [Axum 0.8.9 — `WebSocketUpgrade`](https://docs.rs/axum/0.8.9/axum/extract/ws/struct.WebSocketUpgrade.html)
- [S11] [MDN — Writing WebSocket servers (all browsers send `Origin`; send 403 for a disliked origin; CSWH)](https://developer.mozilla.org/en-US/docs/Web/API/WebSockets_API/Writing_WebSocket_servers)
- [S12] [MDN — CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS)
- [S7/S8/S9] Tauri v2 security / config / CSP (`connect-src`) — see 033 §2.3, out of scope here.

**Local (read-only at canonical `1be723cc`):**
- `flora-api/src/middleware/cors.rs:26-55` · `flora-api/src/routes/websocket.rs:116-179,487-559` · `flora-api/src/routes/mod.rs` (canonical) · `flora-api/src/extractors/auth.rs:33-74,273` · `flora-api/src/state/app_state.rs:24-28`
- `flora-core/src/config/app.rs:18-19,46-51,78-132` · `flora-core/src/bin/flora-mint-session.rs:20-28,47-103` · `flora-core/Cargo.toml:63-72`
- `flora-tests/src/contract/websocket.rs:40-96,188-209,211-413` · `flora-tests/Cargo.toml:23-40` · `Cargo.toml:41-91,115,140-150`
- `apps/frontend/src/lib/api/client.ts:1-3,73-109` · `apps/frontend/src/lib/api/websocket.ts:2-3,34-39,127` · `apps/frontend/src/app.html:1-18` · `apps/frontend/src/app.d.ts:1-13` · `apps/frontend/vite.config.ts:19-47` · `apps/frontend/tests/unit/websocket.test.ts:44-46,83-91` · `tests/unit/searchApi.test.ts:1-3` · `.github/workflows/ci.yml:17-24`
- `test-flora.sh:16-60` · `scripts/dev-smoke-session.sql:10-48` · `docs/dev-smoke.md:60-113` · `.env.example:31-39` · `dev-seed.sql:14,82,106`
- `docs/adr/0002-…-frontend.md:27-31` · `docs/research/2026-11-16-030-cors-allowlist-plan.md:178-210` · `…-032-cors-final-plan.md:465,566,575` · `2026-11-16-033-origin-boundary-plan.md`

---

## 11. Compact verdict

**GO** for (a)-(c) as specified; **GO-with-blocker** for (d).
**Files:** new `apps/frontend/static/config.js`, new `tests/unit/resolver.test.ts`; edit `app.html`, `app.d.ts`, `client.ts`, `websocket.ts`, `websocket.test.ts` (FE); `flora-api/src/routes/websocket.rs`, `flora-tests/src/contract/websocket.rs` (Rust); `.env.example` (docs). **No Cargo.toml, cors.rs, config/app.rs, routes/mod.rs changes.**
**Risky bits:** running API is stale/wildcard-CORS (NV-1); absent-Origin policy must be an explicit ADR decision (NV-2); Tauri origin literals + default allowlist (NV-3/4); `?token=` leakage (NV-5); FE mock breakage if E5’s `websocket.test.ts` update is skipped (D3).
**#2 met after edits + live proof?** **NO today** (stale binary, no live proof, T7 not run). **YES conditionally** after green gates + canonical rebuild/restart + T8 evidence + T7 + ADR note.
