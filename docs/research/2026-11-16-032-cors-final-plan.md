# 032 — CORS hardening: final audit + exact edit plan (cycle 032)

**Date:** 2026-11-16. **Repo:** `flora` (archived checkout), branch `dev`.
**Verified HEAD:** `7e801a6b34b7f584c54cc81320cf3e8117f07f52` (`7e801a6b`), tree clean (`git status --short` empty).
**Type:** research / read-only audit. **No source, config, env, manifest, or test file was modified.** This document is the only artifact written.

**Builds on:** docs 030 (design), docs 031 (`2026-11-16-031-cors-exact-edits.md` — the exact-edit source that this plan audits), ADR-0002 re-entry criterion #2, docs 026:NV-6/NV-7, 027:NV-11, 028, 029.

**Question answered:** Do the five edits of doc 031 still apply verbatim at HEAD `7e801a6b`? Are the `axum::http` import fix and the `routes/mod.rs` coupling real? What is the minimal deterministic test set? What is the ordered edit list, the verification command sequence, the primary-source basis, and the go/no-go with risks/NVs?

---

## 0. Go / No-Go

| Decision | Scope | Rationale |
|---|---|---|
| **GO — with two mandatory corrections** | Backend CORS hardening: E1–E4 + E5 at HEAD `7e801a6b`. | Every fact in doc 031 (C1–C15) still holds. The only inter-HEAD delta is a workspace *version* bump in three manifests; no dependency, feature, lint, or source line changed semantically. Two doc-031 details need fixing before copy-paste: the E5b-3 fixture is invalid (§5, V10/NV-18), and the E4 rationale overstates `dotenv` (§2 of §6, NV-19). Neither changes the design. |
| **GO** | Treat **E2 + E3 as one atomic commit**. | `cors_layer()` has exactly one production call site (C2/NV-15). A partial application is a compile error, not a silent regression. |
| **CONDITIONAL GO** | E5c (Figment array parse test). | Needs one dev-only `figment` `test` feature bump in `flora-core/Cargo.toml`. Deterministic env-array proof is otherwise impossible (edition 2024 makes `std::env::set_var` `unsafe`; workspace denies `unsafe_code`). Keep E5c out of the hard gate if the feature bump is not wanted this cycle. |
| **NO-GO** | Claiming ADR-0002 criterion #2 "met" from these edits alone. | Criterion #2 also needs runtime-configurable frontend API/WS URLs (030 Step 3) and a live non-browser preflight + bearer `select-organization` proof (030 T8). Neither is in this edit set. |
| **NO-GO** | Adding `tauri://*` origins to the shipped default this cycle. | Origin-string ambiguity (NV-4) + WebView2 preflight caching (NV-5) need a real Windows smoke. Keep them env-only (E4). |
| **NO-GO (hard block)** | `allow_credentials(true)`, any `"*"` origin entry, `.expose_headers(Any)`. | tower-http panics on credentials + wildcard (`ensure_usable_cors_rules`, V7); `AllowOrigin::list` panics on literal `*` (V2); bearer auth is **not** credentialed (§3). |

**Ship gate (all green before merge):**
`cargo fmt --all -- --check` → `cargo clippy --workspace --all-targets -- -D warnings` → `cargo test -p flora-core` → `cargo test -p flora-api` → `cargo build --workspace` → browser smoke 4173→3000 (docs 028/029) still green.
This repo has **no `verify.sh`** (only `.specify/scripts/...` and no root script); use the cargo commands from `flora/AGENTS.md`.

---

## 1. HEAD-to-HEAD verification (doc 031 `bb16ba9a9` → current `7e801a6b`)

`git log bb16ba9a9..7e801a6b` = two commits, neither touching CORS source:
- `d7bfbc8fc` `build(release): prepare Seed v0.1 tag workflow and SBOM`
- `ffb7cc1ff` `docs(031): record merged PR #242 evidence`

The diff over all files relevant to this edit set is **version-only**:

```
Cargo.toml          | 3 ++-   (package.version → workspace version; + package.version in [workspace.package])
flora-api/Cargo.toml| 2 +-    (version = "0.1.0" → version.workspace = true)
flora-core/Cargo.toml|2 +-    (same)
```

`flora-api/src/**`, `flora-core/src/**`, `.env.example` — **zero byte changes**.

| ID | Doc-031 fact | Verdict at `7e801a6b` | Evidence (current tree) |
|---|---|---|---|
| C1 | `cors_layer()` no args; `Any` origin/methods/headers; `max_age(24h)` | **HOLDS** | `flora-api/src/middleware/cors.rs:1-20` (fn at `:14`, imports `:3-4`) |
| C2 | Only one production call site | **HOLDS** | `flora-api/src/routes/mod.rs:59`; repo-wide grep for `cors_layer` returns only `:59`, the fn def at `cors.rs:14`, and its doc example at `:11` |
| C3 | `AppState.config: Arc<Config>`; borrow before `.with_state` | **HOLDS** | `flora-api/src/state/app_state.rs:24-28`; `routes/mod.rs:20,59,64` |
| C4 | `cors_origins: Vec<String>`, single localhost:5173 default, stale "comma-separated" comment | **HOLDS** | `flora-core/src/config/app.rs:18-19,46` |
| C5 | `validate()` checks base_url/JWT/TTL only; called via `Config::validate()` at startup | **HOLDS** | `app.rs:73-93`; `config/mod.rs:56-64` (`:57` calls `app.validate()`); `flora-api/src/main.rs:27-28` |
| C6 | Error variant `Error::Configuration(String)` → 500 | **HOLDS** | `flora-core/src/error.rs:169-172,363,368` |
| C7 | `Serialized::defaults` + `Env::prefixed("FLORA_").split("__")`; no `test` feature | **HOLDS (line shifted)** | `config/mod.rs:43-49`; workspace `Cargo.toml:223` (`figment = { version = "0.10", features = ["env","toml","yaml"] }`) — was `:222` in doc 031 |
| C8 | `.env.example` has `FLORA_APP__*` through `SESSION_TTL_SECS`, no `CORS_ORIGINS` | **HOLDS** | `.env.example:8-21` (`:21` = `FLORA_APP__SESSION_TTL_SECS`) |
| C9 | no direct `http` dep in `flora-api`; use `axum::http` | **HOLDS** | `flora-api/Cargo.toml:27-68` (no `http = …`); `http 1.5.0` only in `Cargo.lock:1625-1626` transitively; design's `use http::{…}` would not compile |
| C10 | `tower` normal **and** dev dep; tokio macros available | **HOLDS** | `flora-api/Cargo.toml:38,66`; `Cargo.toml:137` (`tokio` `features=["full"]`); `main.rs:15` `#[tokio::main]` |
| C11 | `flora-core` depends on `axum` → zero new deps for validation | **HOLDS** | `flora-core/Cargo.toml:54`; `Cargo.toml:140` (`axum = "0.8"`, lock `0.8.9` at `Cargo.lock:229-230`) |
| C12 | lints deny `unsafe_code`, `unwrap_used`, `expect_used`, `pedantic`, `allow_attributes` | **HOLDS** | `Cargo.toml:41-82` (`:43,56,62,79,80,81`) |
| C13 | WS serves `/ws` + `/api/v1/ws`, prefers `Authorization`, falls back to `?token=`, no `Origin` check | **HOLDS** | `routes/mod.rs:49-50`; `routes/websocket.rs:116-156` (`:150-156` = `resolve_ws_token`) |
| C14 | no route trusts `X-Organization-ID` | **HOLDS** | `routes/mod.rs:30` (stale comment only); no `.ts` client sets the header (only `Authorization` + `Content-Type` at `apps/frontend/src/lib/api/client.ts:78-89`) |
| C15 | `tower-http 0.7.0`, features `["cors","trace"]` | **HOLDS (line shifted)** | `Cargo.toml:142` — was `:141` in doc 031; `flora-api/Cargo.toml:39` |

**Conclusion:** doc 031's edit bodies are textually valid at `7e801a6b`. Only the two corrections in §0/§5 are needed.

---

## 2. Requested confirmation A — the `axum::http` import fix (C9 / doc-031 NV-13)

- `flora-api` has **no direct `http` dependency** (`flora-api/Cargo.toml:27-68`). The `http` crate appears only as a transitive dependency pinned at `1.5.0` (`Cargo.lock:1625-1626`, pulled by axum 0.8.9 / tower-http 0.7.0).
- axum 0.8.9 **re-exports** the whole `http` crate at `axum::http`. Every type the edits need is available there:
  `axum::http::{HeaderName, HeaderValue, Method, Request, StatusCode, Error}` and the `axum::http::header` module (with `AUTHORIZATION`, `CONTENT_TYPE`, `ACCEPT`, `ORIGIN`, `VARY`, `ACCESS_CONTROL_*`).
- Therefore doc 030's sketch `use http::{HeaderName, HeaderValue, Method, header};` **does not compile** in this crate. E2 must use:
  ```rust
  use axum::http::{HeaderName, HeaderValue, Method, header};
  use tower_http::cors::{AllowOrigin, CorsLayer};
  ```
- Rejected alternative: adding `http = "1.0"` as a direct dep of `flora-api`. It works but touches `Cargo.toml` and adds a redundant dependency; not minimal, and it would not help `flora-core` (which already has `axum` — C11).
- Note the two workspaces agree on version semantics: root `[dependencies] axum = "0.8.9"` (`Cargo.toml:115`) and `[workspace.dependencies] axum = "0.8"` (`:140`); members use `axum.workspace = true`, resolving to 0.8.9. No conflict.

## 3. Requested confirmation B — the `routes/mod.rs` coupling (C2 / doc-031 NV-15)

- `create_router(app_state: AppState)` is at `routes/mod.rs:20`; the CORS layer is applied at `:59` and the state is moved at `:64` (`.with_state(app_state)`). The intervening layer at `:63` is `TraceLayer::new_for_http()` and does not touch `app_state`.
- The proposed call `cors_layer(&app_state.config.app.cors_origins)` takes `&[String]`. `cors_layer` converts each entry into an **owned** `HeaderValue` and returns an owned `CorsLayer` (no lifetime parameter). Under non-lexical lifetimes the immutable borrow ends at `:59`, so the move at `:64` is legal.
- **E3 is mandatory**: changing the signature in E2 without E3 is a compile error (`cors_layer()` takes 0 arguments). It is the *only* production call site, but it lives in a different file, so E2+E3 must land atomically. Ordering within the commit is irrelevant to the compiler; ordering in the review checklist matters (verify both files changed).

---

## 4. Ordered exact edit list (copy-paste; **not applied**)

Apply in this order. E1/E2/E3/E4 are the runtime change; E5 is the test proof.

### Step 1 — E1a: `flora-core/src/config/app.rs:18` (stale comment)
```diff
-    /// CORS allowed origins (comma-separated)
+    /// CORS allowed origins (exact `scheme://host[:port]` strings; Figment array syntax)
     pub cors_origins: Vec<String>,
```

### Step 2 — E1b: `flora-core/src/config/app.rs:46` (four browser origins)
```diff
-            cors_origins: vec!["http://localhost:5173".to_owned()],
+            cors_origins: vec![
+                "http://localhost:5173".to_owned(),   // dev server
+                "http://127.0.0.1:5173".to_owned(),   // dev server, IP form (distinct origin)
+                "http://localhost:4173".to_owned(),   // preview server
+                "http://127.0.0.1:4173".to_owned(),   // preview server, IP form (CI uses this)
+            ],
```

### Step 3 — E1c: `flora-core/src/config/app.rs`, insert after the `base_url` block (after `:78`), before the JWT check
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
No new dependency (C11); no `panic!`/`unwrap`/`expect`.

### Step 4 — E2: `flora-api/src/middleware/cors.rs` (full file replacement, includes the `axum::http` fix)
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
    // tower-http derives `Vary: Origin` for origin lists (§5, claim V4).
}
```
Key deltas vs. the design sketch: `axum::http::{…}` (not `http::{…}`, §2), `# Panics` doc, documented silent-drop invariant. No `Any` remains.

### Step 5 — E3: `flora-api/src/routes/mod.rs:59` (atomic with Step 4)
```diff
-        .layer(crate::middleware::cors::cors_layer())
+        .layer(crate::middleware::cors::cors_layer(
+            &app_state.config.app.cors_origins,
+        ))
```

### Step 6 — E4: `.env.example`, insert after `:21`
```dotenv
# CORS: exact browser/webview origins, Figment array syntax (double quotes inside;
# no "*"). `localhost` and `127.0.0.1` are distinct origins; include both forms.
# Export into the PROCESS ENVIRONMENT (shell `export`, systemd, docker env_file);
# this checkout has no Rust code that auto-loads `.env` (see 032 NV-19).
FLORA_APP__CORS_ORIGINS=["http://localhost:5173","http://127.0.0.1:5173","http://localhost:4173","http://127.0.0.1:4173"]
# Future Tauri webview origins — add only after a real Windows smoke (030 NV-4/5):
#   "tauri://localhost"  (macOS / Linux / iOS)
#   "http://tauri.localhost"  (Windows / Android; "https://tauri.localhost" if
#                             app.windows[].useHttpsScheme = true)
# Never add "null" (opaque origin) and never add "*".
```
Usage notes to keep:
- Shell: `export FLORA_APP__CORS_ORIGINS='["http://a","http://b"]'` (single quotes keep the shell from eating the brackets; Figment receives the array).
- A bare comma-separated string deserializes to a single `Vec<String>` element (or fails), not two origins (§5, V6).
- Frontend runtime keys (`PUBLIC_API_BASE_URL`, `PUBLIC_WS_BASE_URL`) belong to 030 Step 3, out of scope.

### Step 7 — E5a: `flora-api/src/middleware/cors.rs`, append `#[cfg(test)] mod tests`
Uses `tower::ServiceExt::oneshot` (C10) and `?` throughout (no lint exceptions).
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
        let response = router().oneshot(preflight("https://evil.example")?).await?;
        // tower-http answers OPTIONS itself even for a disallowed origin: status is
        // still 200 and ACAM/ACAH may be present. The security assertion is the
        // ABSENCE of ACAO (§5, claim V5).
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

### Step 8 — E5b: `flora-core/src/config/app.rs`, append validation tests — **with the corrected fixture**
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
        // CORRECTED (032 NV-18): a SPACE is a valid HeaderValue byte in http 1.5.0,
        // so doc 031's "http://bad host" would parse and this test would FAIL.
        // A control character (< 0x20, here '\n') is genuinely rejected.
        config.cors_origins = vec!["http://bad\nhost".to_owned()];
        assert!(config.validate().is_err());
    }

    #[test]
    fn cors_origins_accept_the_default_list() {
        let config = AppConfig::default();
        assert!(config.validate().is_ok());
    }
}
```
Note: `AppConfig::default()` reads `FLORA_JWT_SECRET` (`app.rs:47-48`); a CI-provided value shorter than 32 chars only warns in the default `development` env, so the "accept" test stays valid. If CI sets `FLORA_APP__ENV=production`, build the fixture explicitly.

### Step 9 — E5c (CONDITIONAL GO): Figment array parse test
Manifest prerequisite — add to `flora-core/Cargo.toml` under `[dev-dependencies]` (`:58-61`):
```toml
figment = { workspace = true, features = ["test"] }
```
(Workspace dep at `Cargo.toml:223` has no `test`; `figment::Jail` is gated on it. Feature unification leaves normal builds unchanged.) Verify `figment 0.10.19` in `Cargo.lock` before relying on the `Jail` API.

Test — append to `flora-core/src/config/mod.rs`:
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
`Config::load()` returns `figment::Result<Self>` (`config/mod.rs:43`), so `?` propagates inside `Jail::expect_with`. Do **not** use `std::env::set_var` (edition 2024 `unsafe`; workspace denies `unsafe_code`). The Figment `Env` docs confirm both the `prefixed(...).split("__")` nesting and `Array: delimited by []` parsing (§5, V6).

---

## 5. Minimal deterministic test set

Two tiers. The **hard gate** is the four tests that prove the security property and the panic guard; the remainder is cheap defense-in-depth. All are in-process (`oneshot`, no network/DB/seed), so all are deterministic.

| Test | File | Assertion | Tier |
|---|---|---|---|
| `allowed_origin_preflight_returns_acao_authorization_and_no_credentials` | `cors.rs` unit | `OPTIONS` + `Origin: http://127.0.0.1:4173` + `ACRH: authorization,content-type` → status 200, ACAO == that origin, ACAM contains POST, ACAH contains `authorization`, **no** ACAC, `Vary: Origin` | **hard gate** |
| `disallowed_origin_preflight_has_no_acao` | `cors.rs` unit | `Origin: https://evil.example` → status 200, **no ACAO** (security assertion is header absence, not status — V5) | **hard gate** |
| `cors_origins_reject_wildcard_and_empty_entry` | `app.rs` unit | `"*"` and `""` → `Err(Configuration(_))`, no panic (V2 guard) | **hard gate** |
| `cors_origins_accept_the_default_list` | `app.rs` unit | default four-origin list → `Ok(())` | **hard gate** |
| `cors_origins_reject_empty_list` | `app.rs` unit | empty `Vec` → `Err` (avoids silent total CORS outage) | full set |
| `cors_origins_reject_unparseable_entry` | `app.rs` unit | **corrected** `http://bad\nhost` → `Err` (V10/NV-18) | full set |
| `tauri_origins_are_allowed_but_opaque_null_is_not` | `cors.rs` unit | two tauri strings echoed; `null` denied (V8) | full set |
| `cors_origins_parse_from_figment_array_env` | `config/mod.rs` unit | env array → exact `Vec<String>` (V6) | conditional (E5c) |
| 030 T7 | Playwright | 4173→3000 browser smoke remains green | execution, needs seeded API |
| 030 T8 | curl + seeded DB | non-browser preflight echoes ACAO; bearer `select-organization` → 200 + ACAO | execution step; curl does not *enforce* CORS |

If only one test could be kept, keep `disallowed_origin_preflight_has_no_acao`: it is the direct proof that the previously-permissive layer now denies an arbitrary origin. Paired with `allowed_origin_preflight_returns_acao_authorization_and_no_credentials` it covers both halves of the invariant.

---

## 6. Primary-source verification (independently re-fetched 2026-11-16)

Pinned versions from this tree: `tower-http 0.7.0` (`Cargo.toml:142`), `http 1.5.0` (`Cargo.lock:1625-1626`), `figment 0.10.19`, `axum 0.8.9` (`Cargo.lock:229-230`). Sources are the exact tagged files, not secondary write-ups.

| # | Claim | Primary source (exact version) |
|---|---|---|
| V1 | `CorsLayer::new()` sends no headers; an allowed origin must be set; builder methods override on repeat calls. | tower-http 0.7.0 `cors/mod.rs` — `new()` docs "No headers are sent by default…"; `allow_headers`/`allow_methods`/`allow_origin` docs "multiple calls … will override any previous calls" |
| V2 | `AllowOrigin::list<I: IntoIterator<Item = HeaderValue>>` **panics** if the iterator contains a literal `*`; `From<Vec<HeaderValue>>` and `From<[HeaderValue; N]>` delegate to `list`. | `cors/allow_origin.rs:45-68,208-218` — `if origins.contains(&WILDCARD) { panic!(…) }` |
| V2b | Panic fires only on a HeaderValue **equal to `*`**, not on a larger string containing `*`; rejecting any `*` substring in E1c is conservative, not required. | `allow_origin.rs:55-65` |
| V3 | `.allow_methods([Method; N])`, `.allow_headers([HeaderName; N])` accepted via `From`/`Into`; `AllowHeaders::list` joins names into one comma-separated ACAH value. | `cors/mod.rs` builder docs; exported `AllowHeaders`/`AllowMethods` types; `cors/allow_headers.rs:30-42,93-103` |
| V4 | `Vary` derived automatically: an origin **list** makes `varies_with_origin()` true → `Vary: Origin`. No manual `.vary()` needed. | `allow_origin.rs:114-116` (`!matches!(…, OriginInner::Const(_))`); `cors/mod.rs` `update_vary_header()` + `Vary` docs |
| V5 | On `OPTIONS`, the layer answers immediately with `Response::new(B::default())` (status 200) and **never calls the inner service**. A disallowed origin still gets 200 with no ACAO — assert header absence. | `cors/mod.rs` `Service::call`: `if parts.method == Method::OPTIONS { … Kind::PreflightCall { … } } else { … self.inner.call(req) … }`; `Kind::PreflightCall` poll arm |
| V6 | Figment `Env` parses TOML-like values including `Array: delimited by []`; docs' `NUMBERS=[1,2,3]` → `Vec<usize>`. `split("__")` nests `FLORA_APP__CORS_ORIGINS` → `app.cors_origins`. | Figment 0.10.19 `providers::Env` docs, "Expand description" + `Env::raw` example + `Env::split` docs |
| V7 | `ensure_usable_cors_rules()` panics when `allow_credentials(true)` is combined with `Any` for headers/methods/origin/exposed headers. Leaving credentials off avoids this. | `cors/mod.rs` `ensure_usable_cors_rules()` — four `assert!(!…is_wildcard(), …)` |
| V8 | `Origin: null` is not in a list of concrete HeaderValues → `to_future` returns `None` → no ACAO; same for a missing `Origin`. | `allow_origin.rs:118-144` — `origin.filter(|o| l.contains(o))` |
| V9 | No new crate feature needed: `cors` already enabled; `AllowOrigin`, `CorsLayer`, `Any` re-exported from `tower_http::cors`. | `cors/mod.rs` `pub use self::{…}`; `Cargo.toml:142` |
| **V10 (new)** | `http 1.5.0` `HeaderValue` validity is `b >= 32 && b != 127 || b == b'\t'`. **Space (0x20) is valid**, so `HeaderValue::from_str("http://bad host")` returns `Ok`. `HeaderValue::from_str` is a byte validator, **not a URL/origin parser**. | `http 1.5.0` `src/header/value.rs` — `try_from_generic` + `is_valid(b)`; doctest `from_str("\n").is_err()` |

**Multilingual cross-check.** The Fetch/CORS requirements (preflight triggered by `Authorization`; `*` cannot be used with credentials) are identical across the EN/pt-BR/es/de/fr/zh-CN/ja localizations of the MDN CORS guide. No translated string enters any code path in this edit set: config values are ASCII origin literals and error messages stay English, matching `flora-core/src/error.rs`.

---

## 7. Verification commands

Run inside `flora/`, in this order (from `flora/AGENTS.md`; note `--all-targets` for clippy):

```bash
# 0. confirm the tree you are auditing/patching
git rev-parse HEAD                       # expect 7e801a6b… at audit time
git status --short                       # expect clean before edits

# 1. after the edits
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p flora-core
cargo test -p flora-api
cargo build --workspace

# 2. targeted proofs (fastest feedback loop)
cargo test -p flora-api allowed_origin_preflight_returns_acao_authorization_and_no_credentials
cargo test -p flora-api disallowed_origin_preflight_has_no_acao
cargo test -p flora-core cors_origins_reject_wildcard_and_empty_entry
cargo test -p flora-core cors_origins_reject_unparseable_entry   # must fail on doc-031 fixture, pass after 032 correction
cargo test -p flora-core cors_origins_parse_from_figment_array_env  # only after E5c feature bump

# 3. CI-owned, unchanged by this edit set
# cargo deny check bans licenses sources
# six DB-gated suites on PostgreSQL 18 (.github/workflows/rust.yml)
# frontend lane (.github/workflows/ci.yml, apps/frontend)
# browser smoke 4173→3000 (docs 028/029) -- requires a running, seeded API
```

**Static pre-merge assertions** (cheap, catch the two known traps):
- `grep -n 'use http::' flora-api/src/middleware/cors.rs` → must return **nothing** (NV-13).
- `grep -rn 'cors_layer' flora-api/src` → must show the def, the **new** E3 call with `&app_state.config.app.cors_origins`, and E5a's test call — never the zero-arg production form.
- `grep -n 'http://bad host' flora-core/src/config/app.rs` → must return **nothing** (NV-18).

---

## 8. Risks / unknowns (NV)

Carried forward from doc 030/031, scoped to this edit set:

- **NV-1 (M) — research only.** No API was started, no preflight/JWT sent, no webview run. All header behaviour is source-verified (§6) but T7/T8 remain mandatory at execution.
- **NV-2 (H) — startup panic if validation is bypassed.** `AllowOrigin::list` panics on `*` (V2) and `ensure_usable_cors_rules` on credentials + wildcard (V7). E1c closes the normal path (`Config::load()` → `validate()` at `main.rs:28`); hand-built `AppConfig` bypassing `validate()` is out of the normal path.
- **NV-3 (M) — Figment syntax.** Bare comma-separated strings do not deserialize to `Vec<String>` (V6). E5c pins the exact format; it requires the `test` feature bump. The old "comma-separated" comment was itself a trap (fixed by E1a).
- **NV-4 (M) — Tauri origin ambiguity.** `tauri://localhost` vs `http://tauri.localhost` vs `https://tauri.localhost` (`useHttpsScheme`) vs scheme-less field reports. E5a-3 hard-codes the two most common, but **shipping them in the default is NO-GO** until a Windows smoke proves the exact `Origin`. `null` must stay denied (V8).
- **NV-5 (M) — WebView2/WKWebView cache preflight per origin.** An already-running client can keep failing after the server is fixed; document "restart client" in the runbook.
- **NV-6 (H) — WS is not protected by CORS.** See §9.
- **NV-7 (M) — Tauri CSP.** `connect-src` must include the API origin once a Tauri build exists; CORS alone is insufficient. Out of scope.
- **NV-8 (M) — proxy / `Vary: Origin`.** tower-http adds `Vary: Origin` for lists (V4), but a reverse proxy may strip or cache it incorrectly. Verify on the VPS path.
- **NV-9 (L) — frontend build-time env.** `VITE_API_URL` is baked at build; unresolved by this edit set (030 Step 3).
- **NV-10 (L) — `X-Organization-ID`** allowlisted only for legacy/test compatibility (C14); no route trusts it. Do not read its presence as tenancy enforcement.
- **NV-11 (M) — loopback origins in production.** E1c only `tracing::warn!`s; an error would break legitimate local deployments. Accepted risk.
- **NV-12 (L) — fixed method/header list.** `.allow_methods([…])` covers GET/POST/PUT/PATCH/DELETE/OPTIONS and `.allow_headers([…])` covers Authorization/Content-Type/Accept/X-Organization-ID. Verified against `apps/frontend/src/lib/api/client.ts:78-89` (only Authorization + Content-Type set by the client). A future custom header would need an edit here.
- **NV-13 (M) — compile trap in the design sketch.** `use http::{…}` from 030 §3 does not compile in `flora-api` (§2). E2 pins `axum::http::{…}`.
- **NV-14 (M) — silent origin drop.** `filter_map(…ok())` would quietly remove an unparseable origin. E1c validation makes that unreachable through `Config::load()`; keep the invariant comment in E2.
- **NV-15 (L) — call-site coupling.** E2+E3 atomic (§3).
- **NV-16 (L) — layer built once.** `CorsLayer` is constructed at router creation; changing `FLORA_APP__CORS_ORIGINS` requires an API restart. Runtime reload hooks deliberately out of scope.
- **NV-17 (L) — `Origin: null` validation.** E1c's `null` rejection is commented out to keep the edit minimal. If ops adds `"null"`, opaque origins would be accepted (V8). Promote to active code if config is operator-edited in production.

**New findings in this audit:**

- **NV-18 (M) — doc 031 E5b-3 fixture is wrong (must fix).** `http 1.5.0` accepts space as a header-value byte (V10), so `HeaderValue::from_str("http://bad host")` is `Ok`. The test as written asserts `Err` and will **fail**. Use a control-character string such as `"http://bad\nhost"` (Step 8). This is the only copy-paste-blocking defect found in doc 031.
- **NV-19 (L) — `.env` is not auto-loaded by Rust.** No `.rs` file in the tree calls `dotenv()` (`flora-core/Cargo.toml:24` declares `dotenv 0.15` but it is never invoked); `Config::load()` reads only `Serialized::defaults` + the process environment (`config/mod.rs:43-49`). No compose file uses `env_file`. E4's env key is correct, but its doc-031 rationale "dotenv 0.15 takes the value literally" overstates what happens: **the value must be exported into the process environment** by the launcher (shell/systemd/container). The corrected E4 comment says so.
- **NV-20 (L) — `HeaderValue::from_str` is not an origin validator (V10).** It accepts spaces, missing schemes, and malformed URLs. Such a configured string passes E1c, becomes a `HeaderValue`, and will simply never match a browser `Origin` (which is `scheme://host[:port]`). Result is fail-closed (no ACAO), so severity is low — but the E1c error text should be read as "not a valid HTTP header value", not "not a valid origin". A stricter `url::Origin`-style check is a follow-up, not a gate item.
- **NV-21 (L) — second `Config::load()` caller skips validation.** `flora-core/src/bin/flora-mint-session.rs:49` calls `Config::load()` without `validate()`. It does not build a CORS layer, so it is irrelevant to this edit — but do not assume "validation always runs" as a global invariant.

---

## 9. WebSocket caveat (must accompany any CORS claim)

Browsers **do not apply CORS to WebSocket**. They send `Origin` on the handshake but ignore `Access-Control-Allow-Origin`. Therefore:
- The E2 allowlist has **no effect** on `/ws` or `/api/v1/ws` (C13, `routes/mod.rs:49-50`).
- Flora's WS handler authenticates the session but performs **no `Origin` check**, and `resolve_ws_token()` falls back to `?token=` because JS cannot set WS headers (`websocket.rs:116-156`). Both transports are accepted.
- Any non-browser/Tauri re-entry that treats CORS as the boundary leaves WS cross-origin connections open. Add an explicit WS `Origin` check and assess `?token=` query logging separately before that milestone (030 NV-6).
- State explicitly in any ADR or gate note: **"CORS allowlist verified" ≠ "cross-origin boundary verified"**.

---

## 10. Out of scope / dependency for full criterion #2

This edit set delivers the backend half only. Full ADR-0002 re-entry #2 additionally needs (from 030):
1. Frontend runtime config: `static/config.js` + `window.__FLORA_CONFIG__`, script in `app.html`, resolver in `client.ts` with precedence `runtime > VITE_API_URL > default`, WS URL via existing `buildWsUrl()`.
2. Live gate proof: non-browser preflight echo for each origin + bearer `select-organization` round-trip (030 T8) on a seeded API.
3. Browser regression lane (030 T7).
4. ADR update recording the result — **not** in a research doc.

Do not mark #2 met from E1–E5 alone.

---

## 11. Sources

**Local, read-only at HEAD `7e801a6b`:**
- `flora-api/src/middleware/cors.rs:1-20`; `flora-api/src/middleware/mod.rs`; `flora-api/src/routes/mod.rs:20-64`; `flora-api/src/state/app_state.rs:24-28`; `flora-api/src/main.rs:12-28`; `flora-api/src/routes/websocket.rs:112-156`
- `flora-core/src/config/app.rs:1-94`; `flora-core/src/config/mod.rs:1-64`; `flora-core/src/error.rs:169-172,355-369`; `flora-core/src/bin/flora-mint-session.rs:49`
- `flora-api/Cargo.toml:27-68`; `flora-core/Cargo.toml:23-67`; `Cargo.toml:41-82,110-145,223`; `Cargo.lock:229-230,1625-1626`; `.env.example:1-71`
- `apps/frontend/src/lib/api/client.ts:78-89`; `docs/research/2026-11-16-030-cors-allowlist-plan.md`; `docs/research/2026-11-16-031-cors-exact-edits.md`

**Primary / external, fetched 2026-11-16:**
- [tower-http 0.7.0 — `cors/mod.rs` (tag `tower-http-0.7.0`)](https://raw.githubusercontent.com/tower-rs/tower-http/tower-http-0.7.0/tower-http/src/cors/mod.rs) — V1, V3, V4, V5, V7, V9
- [tower-http 0.7.0 — `cors/allow_origin.rs`](https://docs.rs/tower-http/0.7.0/src/tower_http/cors/allow_origin.rs.html) — V2, V2b, V4, V8
- [tower-http 0.7.0 — `cors/allow_headers.rs`](https://docs.rs/tower-http/0.7.0/src/tower_http/cors/allow_headers.rs.html) — V3
- [http 1.5.0 — `header/value.rs` (tag `v1.5.0`)](https://raw.githubusercontent.com/hyperium/http/v1.5.0/src/header/value.rs) — V10 (`is_valid`, `from_str("\n").is_err()`)
- [Figment 0.10.19 — `providers::Env`](https://docs.rs/figment/0.10.19/figment/providers/struct.Env.html) — V6
- [MDN — CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS) (EN + localizations) — bearer preflight / credential distinction
- [Tauri v2 — Security / CSP / config reference](https://v2.tauri.app/security/) — NV-4/NV-7 background
