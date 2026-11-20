# Feature Specification: 024 — OIDC ID-Token Verification (Seed identity closure)

**Feature Branch**: `feat/024-oidc-id-token-verification`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/024-next-vertical-slice/decision.md` (go, S1)

## Goal
Make "Users & Roles (OIDC)" evidence-backed for Seed v0.1 by verifying the ID token before creating users or issuing Flora sessions.

## Scope
- Add `nonce` to the transient OIDC transaction; include it in the authorization URL and require an exact match in the ID token.
- Restrict `redirect_uri` to the single configured callback (exact allowlist).
- Replace base64-only claim decoding with JWKS-based verification: fetch discovery (`/.well-known/openid-configuration`) + JWKS over HTTPS (localhost allowed for dev), select key by `kid`, verify signature, `iss`, `aud`, `exp`, `sub` and `nonce`. Reject symmetric JWKs in the production path.
- Identify users by provider `sub` (with email fallback for existing rows), fail closed when `sub`/`email` are missing.
- Deterministic unit contracts: valid token accepted; tampered token, wrong issuer/audience/nonce and unknown `kid` rejected; `JwkSet::find` kid-aware.

## Non-goals
- Real-OP e2e, JWKS rotation drills, logout/refresh semantics, workspace-scoped RBAC, browser FE round-trip.
- Phase 0 CI enforcement, OTel wiring, Meilisearch, PGMQ/Valkey/RustFS, new services, 2vCPU/4GB load tests.

## NV
- OIDC Core §3.1.3.7 TLS-server validation path is not used; default dev issuer is HTTP so verification is required.
- Provider discovery is fetched per callback (no cache yet); caching/rotation NV.
