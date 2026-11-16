# Intake: 012 — Session Authority Revalidation & Contract Confidence (80/20)

**Date:** 2026-11-16  
**Base:** `dev@598f08152` (cycle 011 merged, PR #214, CI 6/6 green)  
**Workflow:** mdmv-dark-factory SDD (R1→R7), autonomous 80/20

## Context
- Cycle 011 aligned the core slice contract and frontend error handling; CI green and `dev` synced to `origin`.
- `ROADMAP.md` P0 now prioritizes **session authority**: protected routes currently trust JWT signature/claims without revalidating the session row (`jti`) in PostgreSQL. Logout/revocation/expiry can therefore fail to propagate before JWT `exp`.
- Evidence: `flora-api/src/extractors/auth.rs` only decodes JWT; `POST /auth/refresh` and `select-organization` perform partial DB checks; `sessions` table has `jti`, `is_active`, `expires_at`, `user_id`, `organization_id` with indexes.
- Additional 011 gap: `ApiClient` handles 401 before parsing the response body, discarding structured `code/message/body` on 401.

## Request
Execute cycle 012 with minimal blast radius: make session/JWT authority DB-backed on protected requests, add deterministic DB-backed tests (revoked/expired/mismatch), fix 401 body preservation, and keep the Seed/VPS profile lean (no auth cache initially).

## Inputs
- Code: `flora-api/src/extractors/auth.rs`, `flora-api/src/routes/auth.rs`, `flora-core/src/services/session_service.rs`, `flora-core/src/repositories/session_repository.rs`, `flora-core/src/models/session.rs`, migration `20240818000001_init_schema.sql`, `flora-tests/src/contract/auth.rs`, CI workflows.
- Research (in progress, background): implementation audit, multilingual session-security research, DB-test design, frontend↔API audit, VPS Seed profile, Macro gap analysis.
- Strategic: `OBJETIVO.md` / `ROADMAP.md` P0 (security/tenancy, deterministic evidence, cheap VPS).

## Constraints
- No applied-migration edits unless strictly additive/forward-safe and justified.
- No global auth cache in this cycle; one indexed DB lookup per protected request is acceptable for SMB/VPS.
- No OIDC/JWKS real e2e; no broad RBAC/product expansion; no `apps/web` work.
- No `git add -A`; PR to `dev`, CI green, no blind auto-merge; all unverified environment/remote claims marked NV.
- Keep error responses generic on auth failures (avoid leaking revocation vs signature details).

## Initial scope candidates
1. Shared authenticate helper used by `AuthExtractor`/`UserContext`/`OrgContext`: JWT decode + `jti` DB lookup, active/expiry/user/org validation, nil-org pre-selection handling.
2. Frontend: parse 401 body before `lockSession()`/throw, preserving `ApiError` code/message/body; deterministic test.
3. DB-backed contract tests for valid/revoked/expired/mismatched sessions; wire into `db-tests` CI.
4. Docs/evidence: validation with local disposable PG18 (alt port if 5432 busy), update roadmap/assessment artifacts.
5. Deferred: caching, distributed invalidation, full tenant-header removal, broad RBAC.
