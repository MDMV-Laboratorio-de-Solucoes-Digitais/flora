# Plan: 024 — OIDC ID-Token Verification

## Design
1. New `flora-api/src/oidc.rs`: `IdTokenClaims`, `verify_id_token` (pure, JWKS injected for deterministic tests) and `fetch_jwks` (discovery + JWKS, HTTPS/localhost guard). Internal `verify_id_token_inner` allows symmetric keys only for unit fixtures.
2. `OidcTransaction` gains `nonce`; `login` generates/stores it, appends `nonce` to the authorize URL and enforces the redirect allowlist.
3. `callback` fetches JWKS, verifies the token with expected issuer/audience/nonce and uses `sub` for identity (email fallback), then issues the existing Flora session JWT.
4. Remove the legacy `decode_id_token_claims` helper.
5. Verification: fmt, strict clippy (`-p flora-api --lib`), unit tests (incl. 5 new OIDC tests), all six DB-gated suites, CI green.

## Verification
- `cargo fmt --all -- --check`
- `cargo clippy -p flora-api --lib -- -D warnings` (and core/tests all-targets)
- `cargo test -p flora-api --lib`, `cargo test -p flora-core --lib`, `cargo test -p flora-tests --lib`
- All six `contract::*::db_` suites on disposable PG18, `--ignored --test-threads=1`

## Risks/NVs
- Discovery/JWKS availability is now part of the callback path (fail-closed); caching/rotation NV.
- Existing users matched by email are not backfilled with `oidc_subject` in this slice (NV).
