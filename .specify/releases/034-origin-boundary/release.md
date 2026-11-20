# Release: 034 — ADR-0002 re-entry criterion #2

**Branch/PR:** `feat/034-origin-boundary` -> `dev`, PR #248 (merged as `23ca350c`) · **Baseline:** canonical `github/dev@1be723cc`

## Delivered
- Server-side exact-match WebSocket `Origin` guard (403 before auth/DB) sharing the CORS allowlist.
- Runtime FE API/WS config (`/config.js` → `window.__FLORA_CONFIG__`), pure resolvers with runtime > Vite > default precedence.
- Deterministic Rust pure + DB-free oneshot tests and FE Vitest server-project tests.
- T8 live non-browser evidence from a fresh canonical instance isolated on :3100 / `flora_034_t8`.

## Evidence
See `.specify/validations/034-origin-boundary/validate.md`. PR #248 all checks green including the browser lane; merged at `23ca350c`.

## NV / follow-ups (035)
ADR gate note after CI browser lane green; FASE 7 Documents; frontend affordance cleanup; cargo-vet blocking / load-restore / tag-release (authorization).
