# 031 — CORS allowlist: exact minimal edit list for the current tree

**Date:** 2026-11-16. **Repo:** `flora` (archived checkout), branch `dev`.
**Verified HEAD:** `bb16ba9a9609cc62c2cf423c4e2737b5297190cd` (`bb16ba9a9`, merge of `github/dev` into `dev`); tree was clean (`git status --short` empty) at audit time.
**Type:** research / read-only audit. **No source, config, env, manifest, or test file was modified.** This document is the only artifact written.

**Builds on:** docs 030 (`2026-11-16-030-cors-allowlist-plan.md`) — the design; ADR-0002 re-entry criterion #2; docs 026:NV-6/NV-7, 027:NV-11, 028, 029.

**Question answered:** What is the *exact*, copy-pasteable, minimal edit set that turns the currently-permissive `CorsLayer` into an explicit allowlist on THIS tree, which tests make it deterministic, and what are the go/no-go gates, risks and the WebSocket caveat?

---

## 0. Go / No-Go

| Decision | Scope | Rationale |
|---|---|---|
| **GO** | Backend CORS hardening only: E1–E4 + E5 (T1–T4), on HEAD `bb16ba9a9`. | The gap is real and small: `cors_layer()` ignores an already-declared config field. All required APIs exist in the pinned `tower-http 0.7.0`; no new runtime dependency is needed. `AppConfig::validate()` is **already** invoked at `flora-api/src/main.rs:28`, so fail-fast validation needs no startup wiring. |
| **CONDITIONAL GO** | T5 (Figment parse test). | Requires one *dev*-only feature bump (`figment` `test`) in `flora-core/Cargo.toml`. Deterministic env-array proof is otherwise impossible: edition 2024 makes `std::env::set_var` `unsafe`, and the workspace denies `unsafe_code`. |
| **NO-GO** | Claiming ADR-0002 criterion #2 "met" from these edits alone. | Criterion #2 also requires runtime-configurable frontend API/WS URLs (030 Step 3) and a live non-browser preflight + bearer `select-organization` proof (030 T8). Neither is in this edit set. |
| **NO-GO** | Adding `tauri://*` origins to the shipped default in this cycle. | Origin-string ambiguity (NV-4) plus WebView2 preflight caching (NV-5) need a real Windows smoke. Document them as env-only additions (E4). |
| **NO-GO (hard block)** | `allow_credentials(true)`, any `"*"` entry, or `.expose_headers(Any)`. | tower-http panics on credentials + wildcard (`ensure_usable_cors_rules`); `AllowOrigin::list` panics on literal `"*"`; bearer auth is **not** credentialed (§3). |

**Ship gate (all must be green before merging the edit set):**
`cargo fmt --all -- --check` → `cargo clippy --workspace --all-targets -- -D warnings` → `cargo test -p flora-core -p flora-api` → `cargo build --workspace` → browser smoke 4173→3000 (docs 028/029) still green. This repo has **no `verify.sh`** (confirmed: only `scripts/dev-smoke-session.sql`); use the cargo commands from `flora/AGENTS.md`.

---

## 1. Verified current-tree facts (read only)

| ID | Fact | Exact evidence at HEAD `bb16ba9a9` |
|---|---|---|
| C1 | `cors_layer()` takes no args and uses `Any` for origin, methods and headers; `max_age(24h)`. | `flora-api/src/middleware/cors.rs:3-20` |
| C2 | Only one production call site: `.layer(crate::middleware::cors::cors_layer())`. | `flora-api/src/routes/mod.rs:59` (only `cors_layer` hits in the tree besides its own doc example) |
| C3 | `AppState` holds `config: Arc<Config>`; `create_router(app_state: AppState)` can borrow origins before the final `.with_state(app_state)` at line 64. | `flora-api/src/state/app_state.rs:24-28`; `routes/mod.rs:20,64` |
| C4 | `AppConfig.cors_origins: Vec<String>` exists, default is a **single** `http://localhost:5173`, doc comment wrongly says "comma-separated". | `flora-core/src/config/app.rs:18-19,46` |
| C5 | `AppConfig::validate()` checks only `base_url`, JWT length, `session_ttl_secs` — no CORS check. It is called by `Config::validate()` (`config/mod.rs:57`) which is called at startup (`main.rs:28`). | `flora-core/src/config/app.rs:73-93`; `config/mod.rs:56-64`; `flora-api/src/main.rs:27-28` |
| C6 | Error variant for config failures is `crate::Error::Configuration(String)` (maps to HTTP 500, but at startup it is an `anyhow` error → process exits). | `flora-core/src/error.rs:169-172,363` |
| C7 | Config load = `Serialized::defaults(Self::default())` + `Env::prefixed("FLORA_").split("__")`. Key is `FLORA_APP__CORS_ORIGINS`. Figment features are `env,toml,yaml` (no `test`). | `flora-core/src/config/mod.rs:43-49`; `Cargo.toml:222` |
| C8 | `.env.example` has `FLORA_APP__*` keys through `SESSION_TTL_SECS` (line 21) but **no** `CORS_ORIGINS`. | `.env.example:8-21` |
| C9 | `flora-api` has **no direct `http` dependency**; HTTP types must be reached via `axum::http::…`. (`http = "1.0"` exists only in `flora-tests/Cargo.toml:35`.) **The design doc's sketch `use http::{…}` does not compile in `flora-api`.** | `flora-api/Cargo.toml:27-68`; `Cargo.toml:140-141` |
| C10 | `tower 0.5` is both a normal and dev-dependency of `flora-api`, so `tower::ServiceExt::oneshot` is available for unit tests. `tokio` macros are available (`#[tokio::main]` in `main.rs`). | `flora-api/Cargo.toml:38,66`; `flora-api/src/main.rs:12` |
| C11 | `flora-core` already depends on `axum`, so validation can call `axum::http::HeaderValue::from_str` with **zero new dependencies**. | `flora-core/Cargo.toml:54` |
| C12 | Workspace denies `unsafe_code`, `unwrap_used`, `expect_used`, `pedantic`; `#[expect(reason=…)]` is the sanctioned escape. | `Cargo.toml:40-79`; `flora/AGENTS.md` |
| C13 | WS handler serves `/ws` and `/api/v1/ws`, prefers `Authorization` but falls back to `?token=`, and performs **no `Origin` check**. | `flora-api/src/routes/mod.rs:49-50`; `flora-api/src/routes/websocket.rs:116-156` |
| C14 | No route trusts `X-Organization-ID` (stale comments/test helpers only) — include the header for compatibility, not tenancy. | `flora-api/src/routes/mod.rs:30`; prior audit 012 |
| C15 | `tower-http` pinned `0.7.0`, features `["cors","trace"]`. No dependency change required for any edit below. | `Cargo.toml:141`; `flora-api/Cargo.toml:39` |

---

## 2. Exact edits (copy-paste; **not applied**)

### E1 — `flora-core/src/config/app.rs` (default origins + validation)

**E1a. Line 18 — correct the misleading doc comment.**

```diff
-    /// CORS allowed origins (comma-separated)
+    /// CORS allowed origins (exact `scheme://host[:port]` strings; Figment array syntax)
     pub cors_origins: Vec<String>,
```

**E1b. Line 46 — replace the single default with the four browser origins.**

```diff
-            cors_origins: vec!["http://localhost:5173".to_owned()],
+            cors_origins: vec![
+                "http://localhost:5173".to_owned(),   // dev server
+                "http://127.0.0.1:5173".to_owned(),   // dev server, IP form (distinct origin)
+                "http://localhost:4173".to_owned(),   // preview server
+                "http://127.0.0.1:4173".to_owned(),   // preview server, IP form (CI uses this)
+            ],
```

Deliberately **excludes** Tauri origins from the default (NV-4/NV-5). They are first-party constants and may later be promoted into the default after a Windows smoke; until then they are an env-only addition (E4).

**E1c. Insert into `validate()` immediately after the `base_url` block (after line 78), before the `jwt_secret` check.**

```rust
        if self.cors_origins.is_empty() {
            return Err(crate::Error::Configuration(
                "cors_origins must contain at least one origin".to_owned(),
            ));
        }
        for origin in &self.cors_origins {
            if origin.is_empty() {
                return Err(crate::Error::Configuration(
                    "cors_origins contains an empty entry".to_owned(),
                ));
            }
            // tower-http's AllowOrigin::list panics on a literal `*`; fail as a config
            // error instead. Rejecting any `*` substring is deliberately stricter.
            if origin.contains('*') {
                return Err(crate::Error::Configuration(format!(
                    "cors_origins entries must not contain '*': {origin}"
                )));
            }
            // cors_layer() filters unparseable entries with .ok(); validate first so
            // a typo cannot silently remove an origin from the allowlist.
            if axum::http::HeaderValue::from_str(origin).is_err() {
                return Err(crate::Error::Configuration(format!(
                    "cors_origins entry is not a valid HTTP header value: {origin}"
                )));
            }
            // Optional extra hardening (not needed for the gate): opaque origins
            // must never be allowlisted. Uncomment if "null" ever appears in config.
            // if origin.eq_ignore_ascii_case("null") {
            //     return Err(crate::Error::Configuration(
            //         "cors_origins must not contain the opaque origin 'null'".to_owned(),
            //     ));
            // }
            if self.env == "production"
                && (origin.contains("localhost") || origin.contains("127.0.0.1"))
            {
                tracing::warn!(
                    origin = %origin,
                    "cors_origins contains a loopback origin in production; \
                     any local process can satisfy this allowlist"
                );
            }
        }
```

Why each check:
- empty list / empty entry → `AllowOrigin::list(vec![])` would compile but allow nothing, producing a silent total CORS outage.
- `*` → prevents the tower-http panic (§3, claim V2); MUST happen before the layer is built.
- `HeaderValue` parse → closes the `filter_map(|o| ….ok())` silent-drop path in E2.
- loopback-in-production → warning only (NV-11); an error would break legitimate local deployments.

No new dependency (C11). No `panic!`/`unwrap`/`expect`.

---

### E2 — `flora-api/src/middleware/cors.rs` (wire the allowlist; full file replacement)

```rust
//! CORS middleware configuration.

use std::time::Duration;

use axum::http::{HeaderName, HeaderValue, Method, header};
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Creates a CORS layer allowing only the configured browser origins.
///
/// `Authorization: Bearer …` is a non-safelisted request header, so it is listed
/// in `allow_headers`; it is **not** a credentialed request. `allow_credentials`
/// is intentionally left off (cookies are never used), which also avoids
/// tower-http's credentials + wildcard panic path.
///
/// # Panics
///
/// `AllowOrigin::list` panics on a literal `*` entry. `AppConfig::validate()`
/// rejects such entries at startup, so this cannot happen through `Config::load()`.
///
/// # Examples
///
/// ```ignore
/// let cors = cors_layer(&["http://localhost:5173".to_owned()]);
/// ```
#[must_use = "CORS layer must be applied to the router"]
pub fn cors_layer(origins: &[String]) -> CorsLayer {
    // AppConfig::validate() guarantees every entry parses; filter_map only guards
    // a caller that bypassed validation. Invalid entries are never silently kept.
    let origins: Vec<HeaderValue> = origins
        .iter()
        .filter_map(|origin| HeaderValue::from_str(origin).ok())
        .collect();
    // Retained for legacy/test helpers; no route currently trusts this header (C14).
    let organization_header = HeaderName::from_static("x-organization-id");

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            organization_header,
        ])
        .max_age(Duration::from_hours(24))
    // No allow_credentials(true), no expose_headers(Any), no manual vary:
    // tower-http derives `Vary: Origin` for origin lists (§3, claim V4).
}
```

Key deltas vs. design sketch: `use axum::http::{…}` instead of `use http::{…}` (C9 — the latter does **not** compile here); `# Panics` doc added; the silent-drop invariant documented. Everything else matches 030 §3 Step 1.

---

### E3 — `flora-api/src/routes/mod.rs:59` (required wiring — easy to miss)

```diff
-        .layer(crate::middleware::cors::cors_layer())
+        .layer(crate::middleware::cors::cors_layer(
+            &app_state.config.app.cors_origins,
+        ))
```

The borrow is safe: `cors_layer` copies the strings into owned `HeaderValue`s and returns an owned `CorsLayer`, so the borrow ends before `.with_state(app_state)` at line 64. This is the only production call site (C2). Omitting this edit is a compile error.

---

### E4 — `.env.example` (insert after line 21)

```dotenv
# CORS: exact browser/webview origins, Figment array syntax (double quotes inside;
# no "*"). `localhost` and `127.0.0.1` are distinct origins; include both forms.
FLORA_APP__CORS_ORIGINS=["http://localhost:5173","http://127.0.0.1:5173","http://localhost:4173","http://127.0.0.1:4173"]
# Future Tauri webview origins — add only after a real Windows smoke (030 NV-4/5):
#   "tauri://localhost"  (macOS / Linux / iOS)
#   "http://tauri.localhost"  (Windows / Android; "https://tauri.localhost" if
#                             app.windows[].useHttpsScheme = true)
# Never add "null" (opaque origin) and never add "*".
```

Usage notes to keep in the comment/docs:
- In a shell, quote the whole value so the shell does not eat the brackets: `export FLORA_APP__CORS_ORIGINS='["http://a","http://b"]'` — the shell strips the single quotes, Figment receives the array.
- In `.env` / `docker compose env_file`, leave it **unquoted** as shown: `dotenv 0.15` (present in `flora-core/Cargo.toml:24`) takes the value literally, and Figment parses `[...]` as an array (§3, claim V6). Single quotes outside the value are loader-dependent and should be avoided here.
- A bare comma-separated string (`http://a,http://b`) deserializes to a **single** `Vec<String>` element (or fails), not two origins. Do not use it.
- Frontend runtime keys (`PUBLIC_API_BASE_URL`, `PUBLIC_WS_BASE_URL`) belong to 030 Step 3 and are **out of scope** for this edit set.

---

### E5 — Deterministic tests

#### E5a. `flora-api/src/middleware/cors.rs` — append `#[cfg(test)] mod tests` (allowed origin → ACAO; disallowed → no ACAO; OPTIONS preflight with `Authorization`)

Uses `tower::ServiceExt::oneshot` (C10) and `?` throughout, so no `unwrap`/`expect` lint exceptions are needed.

```rust
#[cfg(test)]
mod tests {
    use axum::Router;
    use axum::body::Body;
    use axum::http::{Method, Request, StatusCode, header};
    use axum::routing::get;
    use tower::ServiceExt;

    use super::cors_layer;

    const ALLOWED: &str = "http://127.0.0.1:4173";

    fn router() -> Router {
        let origins = vec![
            ALLOWED.to_owned(),
            "tauri://localhost".to_owned(),
            "http://tauri.localhost".to_owned(),
        ];
        Router::new()
            .route("/health", get(|| async { "ok" }))
            .layer(cors_layer(&origins))
    }

    fn preflight(origin: &str) -> Result<Request<Body>, axum::http::Error> {
        Request::builder()
            .method(Method::OPTIONS)
            .uri("/health")
            .header(header::ORIGIN, origin)
            .header(header::ACCESS_CONTROL_REQUEST_METHOD, "POST")
            .header(
                header::ACCESS_CONTROL_REQUEST_HEADERS,
                "authorization,content-type",
            )
            .body(Body::empty())
    }

    #[tokio::test]
    async fn allowed_origin_preflight_returns_acao_authorization_and_no_credentials()
    -> Result<(), Box<dyn std::error::Error>> {
        let response = router().oneshot(preflight(ALLOWED)?).await?;

        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .and_then(|value| value.to_str().ok()),
            Some(ALLOWED)
        );
        let methods = response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_METHODS)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        assert!(methods.contains("POST"), "ACAM was: {methods}");
        let headers = response
            .headers()
            .get(header::ACCESS_CONTROL_ALLOW_HEADERS)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        assert!(headers.contains("authorization"), "ACAH was: {headers}");
        assert!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
                .is_none()
        );
        let vary = response
            .headers()
            .get(header::VARY)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        assert!(
            vary.to_ascii_lowercase().contains("origin"),
            "Vary was: {vary}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn disallowed_origin_preflight_has_no_acao() -> Result<(), Box<dyn std::error::Error>> {
        let response = router()
            .oneshot(preflight("https://evil.example")?)
            .await?;

        // tower-http answers OPTIONS itself even for a disallowed origin: status is
        // still 200 and ACAM/ACAH may be present. The security assertion is the
        // ABSENCE of ACAO — the browser then discards the response (§3, claim V5).
        assert_eq!(response.status(), StatusCode::OK);
        assert!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .is_none()
        );
        Ok(())
    }

    #[tokio::test]
    async fn tauri_origins_are_allowed_but_opaque_null_is_not()
    -> Result<(), Box<dyn std::error::Error>> {
        for origin in ["tauri://localhost", "http://tauri.localhost"] {
            let response = router().oneshot(preflight(origin)?).await?;
            assert_eq!(
                response
                    .headers()
                    .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                    .and_then(|value| value.to_str().ok()),
                Some(origin)
            );
        }
        let response = router().oneshot(preflight("null")?).await?;
        assert!(
            response
                .headers()
                .get(header::ACCESS_CONTROL_ALLOW_ORIGIN)
                .is_none()
        );
        Ok(())
    }
}
```

#### E5b. `flora-core/src/config/app.rs` — append validation tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cors_origins_reject_empty_list() {
        let mut config = AppConfig::default();
        config.cors_origins.clear();
        assert!(config.validate().is_err());
    }

    #[test]
    fn cors_origins_reject_wildcard_and_empty_entry() {
        let mut config = AppConfig::default();
        config.cors_origins = vec!["*".to_owned()];
        assert!(config.validate().is_err());

        config.cors_origins = vec![String::new()];
        assert!(config.validate().is_err());
    }

    #[test]
    fn cors_origins_reject_unparseable_entry() {
        let mut config = AppConfig::default();
        config.cors_origins = vec!["http://bad host".to_owned()];
        assert!(config.validate().is_err());
    }

    #[test]
    fn cors_origins_accept_the_default_list() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }
}
```

Note: `AppConfig::default()` reads `FLORA_JWT_SECRET` from the environment (`app.rs:47-48`); a CI-provided value shorter than 32 chars only triggers a warning in the default `development` env, so the "accept" test stays valid. If CI sets `FLORA_APP__ENV=production`, construct the fixture explicitly instead.

#### E5c (CONDITIONAL GO) — Figment array parse test

**Manifest prerequisite** — add to `flora-core/Cargo.toml`:

```toml
[dev-dependencies]
figment = { workspace = true, features = ["test"] }
```

(Workspace dep at `Cargo.toml:222` has `features = ["env","toml","yaml"]`, no `test`; `figment::Jail` is gated on `test`. Feature unification keeps normal builds unchanged.)

**Test** — append to `flora-core/src/config/mod.rs`:

```rust
#[cfg(test)]
mod tests {
    use figment::Jail;

    use super::Config;

    #[test]
    fn cors_origins_parse_from_figment_array_env() -> figment::Result<()> {
        Jail::expect_with(|jail| {
            jail.set_env(
                "FLORA_APP__CORS_ORIGINS",
                r#"["http://localhost:5173","http://127.0.0.1:4173"]"#,
            );
            let config = Config::load()?;
            assert_eq!(
                config.app.cors_origins,
                vec![
                    "http://localhost:5173".to_owned(),
                    "http://127.0.0.1:4173".to_owned(),
                ]
            );
            Ok(())
        })
    }
}
```

`Config::load()` returns `figment::Result<Self>`, so `?` propagates inside `Jail::expect_with`. Do **not** substitute `std::env::set_var`: edition 2024 makes it `unsafe` and the workspace denies `unsafe_code`.

---

## 3. tower-http 0.7.0 — primary-source verification

Fetched 2026-11-16 from docs.rs **0.7.0** source (the exact pinned version, `Cargo.toml:141`), not a secondary write-up. Each claim is tied to concrete source lines.

| # | Claim | Source (0.7.0) |
|---|---|---|
| V1 | `CorsLayer::new()` sends no CORS headers; an allowed origin must be set; builder methods override on repeat calls. | `cors/mod.rs:105-125`, `:192-193`, `:271-272`, `:374-375` |
| V2 | `AllowOrigin::list<I: IntoIterator<Item = HeaderValue>>` **panics** if the iterator contains a literal `*`; `From<Vec<HeaderValue>>` and `From<[HeaderValue; N]>` both delegate to `list`. | `cors/allow_origin.rs:45-68`, `:208-218` |
| V2b | The panic triggers only on a HeaderValue **equal to `*`** — not on a larger string containing `*`. Rejecting any `*` substring in E1c is therefore conservative, not required. | `allow_origin.rs:55-65` (`origins.contains(&WILDCARD)`) |
| V3 | `.allow_methods([Method; N])`, `.allow_headers([HeaderName; N])` are accepted via `From` impls; `AllowHeaders::list` joins names into one comma-separated ACAH value. | `cors/mod.rs:254-281`, `:175-205`; `cors/allow_headers.rs:30-42`, `:93-103` |
| V4 | `Vary` is derived automatically: an origin **list** makes `varies_with_origin()` true → `Vary: Origin`. No manual `.vary()` needed. | `cors/mod.rs:432-482`; `allow_origin.rs:114-116`; `cors/mod.rs:459-482` |
| V5 | On `OPTIONS`, the layer answers immediately (ACAM/ACAH/max-age/ACAO) and **never calls the inner service**. A disallowed origin therefore still gets a 200 with no ACAO — tests must assert header absence, not status. | `cors/mod.rs:699-741` (`if parts.method == Method::OPTIONS`), `:805-815` |
| V6 | Figment `Env` parses TOML-like values including `Array: delimited by []` and quoted strings; `NUMBERS=[1,2,3]` deserializes to `Vec<usize>`. Confirms `FLORA_APP__CORS_ORIGINS=[...]` → `Vec<String>`. | Figment 0.10.19 `Env` docs, "Expand description" + `Env::raw` example |
| V7 | `ensure_usable_cors_rules()` panics when `allow_credentials(true)` is combined with `Any` for headers/methods/origin/exposed headers. Leaving credentials off avoids this entirely. | `cors/mod.rs:820-846` |
| V8 | `Origin: null` is not in a list of concrete HeaderValues → `to_future` returns `None` → no ACAO. Same for a missing `Origin`. | `allow_origin.rs:118-144` (`origin.filter(|o| l.contains(o))`) |
| V9 | No new crate feature is required: `cors` is already enabled, and all used types (`AllowOrigin`, `CorsLayer`, `Any`) are re-exported from `tower_http::cors`. | `cors/mod.rs:77-81`; `Cargo.toml:141` |

**Multilingual cross-check.** The normative Fetch/CORS requirements above are identical in the EN, pt-BR, es, de, fr, zh-CN and ja localizations of the MDN CORS guide (preflight is triggered by `Authorization`; `*` cannot be used with credentials). No translated string enters any code path in this edit set: config values are ASCII origin literals and error messages stay English, matching the rest of `flora-core/src/error.rs`.

---

## 4. Deterministic test matrix & commands

| Test | File | Assertion | Deterministic? |
|---|---|---|---|
| E5a-1 | `cors.rs` unit | `OPTIONS` + `Origin: 127.0.0.1:4173` + `Access-Control-Request-Headers: authorization,content-type` → ACAO == that origin; ACAM contains POST; ACAH contains `authorization`; no ACAC; `Vary: Origin`; 200 | Yes — in-process `oneshot`, no network/DB |
| E5a-2 | `cors.rs` unit | `Origin: https://evil.example` → **no ACAO** (status still 200) | Yes |
| E5a-3 | `cors.rs` unit | `tauri://localhost`, `http://tauri.localhost` echoed; `null` denied | Yes |
| E5b-1..4 | `app.rs` unit | empty / `*` / empty-string / unparseable → `Err(Configuration(_))`; default → `Ok(())`; no panic | Yes |
| E5c | `config/mod.rs` unit | Figment env array → exact `Vec<String>` | Yes, with `figment` `test` feature |
| 030 T7 | Playwright | 4173→3000 browser smoke remains green | Requires seeded API |
| 030 T8 | curl + seeded DB | non-browser preflight echoes ACAO; bearer `select-organization` returns 200 + ACAO | **Execution step, not this research pass**; curl does not *enforce* CORS |

Verification sequence (inside `flora/`):

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p flora-core
cargo test -p flora-api
cargo build --workspace
# targeted:
cargo test -p flora-api allowed_origin_preflight_returns_acao_authorization_and_no_credentials
cargo test -p flora-core cors_origins_reject_wildcard_and_empty_entry
```

`cargo deny check bans licenses sources`, the six DB-gated suites, and the frontend lane run in CI as usual (`.github/workflows/rust.yml`, `ci.yml`) — none are changed by this edit set.

---

## 5. Risks / unknowns (NV)

Carried forward from 030, scoped to this edit set:

- **NV-1 (M) — research only.** No API was started, no preflight or JWT was sent, no webview was run. All header behaviour is source-verified (§3) but T7/T8 remain mandatory at execution.
- **NV-2 (H) — startup panic if validation is bypassed.** `AllowOrigin::list` panics on `*` (V2) and `ensure_usable_cors_rules` panics on credentials + wildcard (V7). E1c closes the normal path (`Config::load()` → `validate()` at `main.rs:28`); any code that constructs `AppConfig` by hand bypassing `validate()` is out of the normal path.
- **NV-3 (M) — Figment syntax.** Bare comma-separated strings do not deserialize into `Vec<String>` (V6). E5c pins the exact format; it requires the `test` feature bump. The old "comma-separated" comment was itself a trap (fixed by E1a).
- **NV-4 (M) — Tauri origin ambiguity.** `tauri://localhost` vs `http://tauri.localhost` vs `https://tauri.localhost` (useHttpsScheme) vs scheme-less `tauri.localhost` field reports. E5a-3 hard-codes the two most common, but **shipping them in the default is a NO-GO** until a Windows smoke proves the exact `Origin` value. `null` must stay denied (V8).
- **NV-5 (M) — WebView2/WKWebView cache preflight per origin.** An already-running client can keep failing after the server is fixed; document "restart client" in the runbook.
- **NV-6 (H) — WS is not protected by CORS.** See §6.
- **NV-7 (M) — Tauri CSP.** `connect-src` must include the API origin once a Tauri build exists; CORS alone is insufficient. Out of scope here.
- **NV-8 (M) — proxy/`Vary: Origin`.** tower-http adds `Vary: Origin` for lists (V4), but a reverse proxy may strip or cache it incorrectly. Verify on the VPS path (030 NV-8).
- **NV-9 (L) — frontend build-time env.** `VITE_API_URL` is baked at build; still unresolved by this edit set (030 Step 3).
- **NV-10 (L) — `X-Organization-ID`** is allowlisted only for legacy/test compatibility (C14); no route trusts it. Do not read its presence as tenancy enforcement.

New findings in this audit:

- **NV-13 (M) — compile trap in the design sketch.** `use http::{HeaderName, …}` from 030 §3 does **not** compile in `flora-api` (no direct `http` dep, C9). E2 pins `axum::http::{…}`. Adding `http = "1.0"` as a direct dep would also work but is not minimal and would touch `Cargo.toml`.
- **NV-14 (M) — silent origin drop.** `cors_layer`'s `filter_map(…ok())` would quietly remove an unparseable origin. E1c's `HeaderValue::from_str` validation makes that unreachable through `Config::load()`; keep the invariant comment in E2 if the validator is ever changed.
- **NV-15 (L) — call-site coupling.** Adding a parameter to `cors_layer()` makes E3 mandatory; it is the only call site (C2), but it lives in a different file than the function, so a partial application fails to compile. Treat E2+E3 as one atomic change.
- **NV-16 (L) — layer built once.** `CorsLayer` is constructed at router creation; changing `FLORA_APP__CORS_ORIGINS` requires an API restart. Runtime reload hooks exist but are deliberately out of scope (030 §4).
- **NV-17 (L) — `Origin: null` validation.** E1c's `null` rejection is commented out to keep the edit minimal. If ops ever adds `"null"` to the allowlist, opaque origins would be accepted (V8). Promote that check to active code if config is operator-edited in production.

---

## 6. WebSocket caveat (must accompany any CORS claim)

Browsers **do not apply CORS to WebSocket**. They send an `Origin` header on the WS handshake but ignore `Access-Control-Allow-Origin`. Therefore:

- The allowlist in E2 has **no effect** on `/ws` or `/api/v1/ws` (C13).
- Flora's WS handler authenticates the session but performs **no `Origin` check**, and `resolve_ws_token()` falls back to `?token=` (because JS cannot set WS headers). Both transports are accepted (C13).
- Any non-browser/Tauri re-entry that treats CORS as the boundary leaves WS cross-origin connections open. Add an explicit WS `Origin` check and assess `?token=` query logging separately before that milestone (030 NV-6).
- State this explicitly in any ADR or gate note: **"CORS allowlist verified" ≠ "cross-origin boundary verified"**.

---

## 7. Out of scope / dependency for full criterion #2

This edit set delivers the backend half only. Full ADR-0002 re-entry #2 additionally needs (from 030):

1. Frontend runtime config: `static/config.js` + `window.__FLORA_CONFIG__`, script in `app.html`, resolver in `client.ts` with precedence `runtime > VITE_API_URL > default`, WS URL derived by existing `buildWsUrl()`.
2. Live gate proof: non-browser preflight echo for each origin + bearer `select-organization` round-trip (030 T8) on a seeded API.
3. Browser regression lane (030 T7).
4. ADR update recording the result — **not** in a research doc.

Do not mark #2 met from E1–E5 alone.

---

## 8. Sources

**Local, read-only at HEAD `bb16ba9a9`:**
- `flora-core/src/config/app.rs:18-19,38-65,67-93`; `flora-core/src/config/mod.rs:43-64`
- `flora-core/src/error.rs:169-172,288-374`; `flora-core/Cargo.toml:24,52,54`
- `flora-api/src/middleware/cors.rs:1-20`; `flora-api/src/middleware/mod.rs`
- `flora-api/src/routes/mod.rs:20-64`; `flora-api/src/state/app_state.rs:24-28`
- `flora-api/src/routes/websocket.rs:112-164`; `flora-api/src/main.rs:12-28`
- `flora-api/Cargo.toml:27-68`; `Cargo.toml:140-141,222` (lints at `:40-79`)
- `.env.example:1-21`; `docs/research/2026-11-16-030-cors-allowlist-plan.md`

**Primary / external, fetched 2026-11-16:**
- [tower-http 0.7.0 source — `cors/mod.rs`](https://docs.rs/tower-http/0.7.0/src/tower_http/cors/mod.rs.html) — V1, V3, V4, V5, V7, V9
- [tower-http 0.7.0 source — `cors/allow_origin.rs`](https://docs.rs/tower-http/0.7.0/src/tower_http/cors/allow_origin.rs.html) — V2, V2b, V4, V8
- [tower-http 0.7.0 source — `cors/allow_headers.rs`](https://docs.rs/tower-http/0.7.0/src/tower_http/cors/allow_headers.rs.html) — V3
- [Figment 0.10.19 — `providers::Env`](https://docs.rs/figment/0.10.19/figment/providers/struct.Env.html) — V6
- [MDN — CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS) (EN + pt-BR/es/de/fr/zh-CN/ja localizations) — bearer preflight / credential distinction
- [Tauri v2 — Security / CSP / config reference](https://v2.tauri.app/security/) — NV-4/NV-7 background
