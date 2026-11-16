# Tasks: 024 — OIDC ID-Token Verification

- [x] T001 Research Seed milestone gaps and decide OIDC verification as the highest-value slice.
- [x] T002 Add `oidc.rs` with JWKS verification (iss/aud/exp/sub/nonce, kid selection, symmetric rejection) and deterministic unit contracts.
- [x] T003 Wire nonce + redirect allowlist into login/transaction and verified-token identity into callback.
- [ ] T004 Run local verification (fmt, clippy, unit + all DB-gated suites) and fix fallout.
- [ ] T005 Record validation/release, update OBJETIVO/ROADMAP, PR/CI green, merge, sync, plan 025.
