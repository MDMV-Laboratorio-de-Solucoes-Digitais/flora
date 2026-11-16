# Validation: 032 — CORS allowlist backend

**Branch:** `feat/032-cors-allowlist` (from `github/dev@966fb19`)

## Evidence executed
- `cargo fmt --all -- --check`: OK; `cargo clippy --workspace --all-targets -D warnings`: clean.
- Unit tests: flora-api 21, flora-core 12 (4 new CORS preflight cases + 4 config validation cases).
- Six DB-gated PG18 suites: auth 12, messaging 4, websocket 8, search 8, notifications 5, files 6 — all passed.
- Static invariant: no `Any` remains in `flora-api/src/middleware/cors.rs`; single call site passes config origins.

## Contract coverage
Allowed preflight returns `Access-Control-Allow-Origin` for the configured origin, includes `authorization` in ACAH, sets no `Access-Control-Allow-Credentials`, and derives `Vary: Origin`; disallowed origins get no ACAO; wildcard and unparseable origins fail at config validation.

## Boundary / NV
This does **not** satisfy ADR-0002 re-entry #2 (no runtime FE URL config, no live non-browser proof, no WS `Origin` check). Cargo-vet stays informational; no tag/release; no load/restore; OTLP unchanged.

## CI result
PR #244: all checks green (4 build, 2 db-tests, dependency-review, 2 supply-chain-vet, 2 e2e-browser; GitHub runs 43955991790 / 43956000665); merged as `cecaf686`.
