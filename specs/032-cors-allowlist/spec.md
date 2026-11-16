# Feature Specification: 032 — CORS allowlist backend

**Branch**: `feat/032-cors-allowlist` · **Status**: Completed (PR #244)  
**Assessment**: `.specify/assessments/032-next-vertical-slice/decision.md`

## Goal
Replace permissive CORS `Any` with an exact origin allowlist, closing a real security gap in the self-hosted multi-tenant app.

## Scope
- `AppConfig.cors_origins`: four exact dev/preview origins (localhost/127.0.0.1 on 5173/4173), fail-fast validation of empty/`*`/unparseable entries, production loopback warning.
- `cors_layer(&[String])` with `AllowOrigin::list`, explicit methods and headers (including `authorization`), no `allow_credentials`; single call site in `routes/mod.rs` updated atomically.
- Deterministic tests: allowed preflight (ACAO/ACAM/ACAH/no credentials/Vary), disallowed origin (no ACAO), wildcard rejection, default config accepted; four config validation tests.
- `.env.example`: Figment array syntax and explicit "export into process env" note; no Tauri origins by default.
- Residual doc drift closed (031 snapshot banner, ROADMAP ranges, OBJETIVO exemption count, ADR-0002 refs).

## Non-goals / explicit non-claims
Do **not** claim ADR-0002 re-entry #2 met (needs runtime-configurable FE URLs, live non-browser proof and a WS `Origin` check). Do not promote cargo-vet to required (needs explicit authorization), cut a tag, publish a release, run load/restore, or add OTLP.

## NV
CORS does not protect WebSocket; FE runtime URL keys and WS Origin check remain deferred.
