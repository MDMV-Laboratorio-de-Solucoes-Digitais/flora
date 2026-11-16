# Decision: 032 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice
**CORS allowlist backend + residual governance drift.**

## Scope (in)
- `AppConfig.cors_origins`: 4 exact dev/preview origins, fail-fast validation (empty/`*`/unparseable), production loopback warning.
- `cors_layer(&[String])`: `AllowOrigin::list`, explicit methods/headers incl. `authorization`, no credentials; single call site in `routes/mod.rs` updated atomically.
- Deterministic tests: allowed preflight (ACAO/ACAM/ACAH/no credentials/Vary), disallowed origin (no ACAO), wildcard rejection, default accepted.
- `.env.example`: Figment array syntax + explicit "export into process env" note; no Tauri origins by default.
- Residual doc drift: supersession banner on 031 release doc, ROADMAP ranges, OBJETIVO exemption count, ADR-0002 refs.

## Explicit non-goals
Do NOT claim ADR-0002 re-entry #2 met (needs runtime URLs + live non-browser proof + WS Origin check); do NOT promote cargo-vet to required (needs explicit authorization); do NOT cut a tag/release; no load/restore execution; no OTLP.

## Risks/NVs
CORS does not protect WebSocket; bearer is not a credentialed request; FE runtime URLs and WS Origin check remain NV; 13 informational vet exemptions and 455-count baseline unchanged.
