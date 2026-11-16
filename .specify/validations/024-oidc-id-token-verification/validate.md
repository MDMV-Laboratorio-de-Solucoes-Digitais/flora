# Validation: 024 — OIDC ID-Token Verification

**Date:** 2026-11-16  
**Branch:** `feat/024-oidc-id-token-verification` (from `github/dev`)  
**Scope:** S1 per `.specify/assessments/024-next-vertical-slice/decision.md`

## Evidence executed
- `cargo fmt --all -- --check`: OK
- `cargo clippy -p flora-api --lib -- -D warnings`: no issues
- `cargo test -p flora-api --lib`: 17 passed (5 new `oidc::tests`); `flora-core --lib`: 8 passed; `flora-tests --lib`: 44 passed, 43 ignored
- OIDC unit contracts: valid HS256 fixture token accepted; tampered token, wrong issuer, wrong audience, wrong nonce and unknown `kid` rejected; `JwkSet::find` kid-aware
- DB-gated suites on disposable PG18 (`--ignored --test-threads=1`): auth 12, messaging 4, websocket 8, search 8, notifications 5, files 6 — all passed
- Two existing auth login contract tests were updated to declare their custom redirect URI in `oidc.allowed_redirect_uris` (exact allowlist is now enforced)

## Contract coverage
- ID token verified against discovery-derived JWKS with `kid` selection; signature/`iss`/`aud`/`exp`/`sub`/`nonce` enforced; symmetric JWKs rejected in the production path.
- `redirect_uri` accepted only when equal to the configured callback; `nonce` generated, stored and compared.
- Identity resolved by `sub` (email fallback for pre-existing rows); missing `sub`/`email` fails closed.

## Merge evidence
PR #230 all required checks green (4 `build` + 2 `db-tests`); merged to `dev` as commit `bf1766cb` on 2026-11-16.

## NV / residual
- Real-OP e2e and JWKS rotation; discovery/JWKS caching; logout/refresh semantics; RBAC endpoints still stubs;
  `oidc_subject` backfill, browser FE round-trip, 2vCPU/4GB validation.
- A parallel agent's `/metrics` test code in `health.rs` trips `clippy::expect_used` under `--all-targets`; it is
  intentionally excluded from this branch's commit.
