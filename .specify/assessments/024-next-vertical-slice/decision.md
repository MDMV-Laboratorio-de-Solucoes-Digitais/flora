# Decision: 024 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice
**S1 — Verified OIDC ID token (Seed v0.1 identity closure).** Replace base64-only claim decoding with real
signature/`iss`/`aud`/`exp`/`nonce` verification via the already-declared `openidconnect` crate, using mocked
discovery/JWKS fixtures; add redirect allowlist, identity by `sub`, and fail-closed config. Add deterministic
DB-gated/router contracts including a tampered-token negative case.

## Justification
- R1 research convergence: the largest decisive Seed gap; security-critical; bounded effort; reuses declared deps.
- Complements 023 (search mechanism) without overlapping the MDMV-Autonoma agent's `/metrics` work.
- Makes "Users & Roles (OIDC)" evidence-backed rather than aspirational.

## Scope (in)
- `flora-api/src/routes/auth.rs`: discovery/JWKS-based verification; `nonce` in transaction + callback;
  exact redirect allowlist; `sub`-based identity lookup; fail-closed config errors.
- `flora-core/src/config/oidc.rs`: explicit issuer/audience/redirect validation where needed.
- Tests: mocked discovery/JWKS + tampered token + missing nonce/wrong aud/exp; keep existing auth DB contracts green.
- SDD/validation/release; update planning docs; PR/CI green; merge/sync.

## NVs / non-goals
- NV: Phase 0 CI enforcement, OTel honest wiring, Seed decision ADR/docs reconciliation, notification transport
  fork, RustFS/S3, Meilisearch re-enablement, browser e2e/Tauri, real-OP e2e and JWKS rotation drills.
- Non-goals: new mandatory services, frontend changes, touching the other agent's `/metrics` files.
