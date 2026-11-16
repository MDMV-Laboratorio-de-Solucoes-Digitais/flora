# Release: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

**Date:** 2026-11-15  
**Target:** `dev` (PR #214 from `feat/011-frontend-api-contract-session-hardening`; merged as `598f08152`, CI 6/6 green, mirrored to `origin/dev`)  
**SemVer proposal:** dev-line increment (no intended breaking public API; frontend client error shape enriched while preserving message semantics for common cases).

## Deliveries

- **Tasks API contract:** `POST /api/v1/tasks` persists `description` and `assignee_id` from `CreateTaskRequest` (aligns with frontend create payload and assignee-scoped listing expectations).
- **Frontend API client/state:** `ApiClient` throws typed `ApiError` preserving status, parsed body (JSON/text), error code/field, and message while retaining 401 `lockSession()` behavior; `AuthState.setOrganization` clears `isInvalidated`.
- **Frontend slice UX/guards:** `/tasks` enforces auth/org preconditions before loading (redirect to `/` or `/select-org`).
- **Deterministic tests:** updated `AuthState` and API interceptor tests (CI-equivalent `--project server`) covering org rotation/flags and typed error handling.
- **Docs:** `docs/dev-smoke.md` adds concrete compose port-override example for host-port conflicts.
- **SDD artifacts:** assessment/spec/plan/tasks/checklists, validation, release for cycle 011.

## Evidence

- `.specify/validations/011-frontend-api-contract-session-hardening/validate.md`
- Local gates: `cargo fmt --check`, `cargo clippy -D warnings`, `cargo check`, targeted `cargo test` (flora-core/lib, flora-tests/lib, flora-api), frontend `check/lint/test:unit(server)/build` — all passing.

## Traceability

- Addresses cycle 011 80/20 priorities from `OBJETIVO.md`/`ROADMAP.md`: frontend↔API contract alignment and determinism for auth→org→workspace→task slice, plus smoke ergonomics. Session-authority DB revalidation in extractor remains an explicit follow-up.

## Rollback

- Revert PR merge commit(s) on `dev`. No migrations or irreversible data changes.

## Risks / NV

- Smoke not re-executed this cycle due to external `5432` occupancy (NV); flow unchanged functionally and previously evidenced (010). Remote CI/merge to be verified on PR. OIDC/JWKS e2e NV.
