# Release: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

**Date:** 2026-11-15  
**Target:** `dev` (PR from `feat/010-compose-uuidv7-smoke-frontend-verify`)  
**SemVer proposal:** patch/minor N/A for unreleased dev line; suggest documenting as **cycle 010** maintenance increment on `dev` (no public API contract break intended).

## Deliveries

- **Compose/migration compatibility (documented path):** `docker-compose.yml` uses `postgres:18-alpine` (native `uuidv7()` required by init schema) with PG18-compatible volume mount; `docs/dev-smoke.md` updated (PG18 prerequisite, disposable PG16→PG18 volume reset note, host-port conflict guidance, Meilisearch key consistency, corrected bootstrap JWT for `.env.example` secret).
- **Smoke evidence:** integrated `select org → workspace → task` flow verified end-to-end on PostgreSQL 18 stack (`smoke_rc=0`) via `test-flora.sh` (seed + bootstrap). Evidence captured on isolated alt-port mapping due to external `5432` occupancy (NV/environmental); flow/migrations identical to documented PG18 path.
- **Frontend deterministic checks:** Vitest server project includes `tests/**/*` (excludes `tests/e2e/**`) enabling consistent unit execution; `check`, `lint`, `test:unit -- --run --project server`, and `build` pass locally.
- **SDD artifacts:** `.specify/assessments/010-compose-uuidv7-smoke-frontend-verify/`, `specs/010-compose-uuidv7-smoke-frontend-verify/`, `.specify/validations/.../validate.md`, `.specify/releases/.../release.md`.

## Evidence

- Validation: `.specify/validations/010-compose-uuidv7-smoke-frontend-verify/validate.md`
- Smoke log: `/tmp/flora-smoke010-ok.log` (captured in-session; key outcomes recorded in validation)
- Local gates: fmt/check/tests (Rust + frontend) as tabulated in validation

## Traceability

- Closes cycle 010 priorities from `specs/009-db-smoke-frontend-slice/{spec,plan,tasks}` and `ROADMAP.md` items: compose/migration `uuidv7` reconciliation on documented path, consolidated deterministic/frontend verification, and evidenced minimal auth→org→workspace→task smoke.

## Rollback

- Revert commit(s) on `feat/010-compose-uuidv7-smoke-frontend-verify` / PR merge commit. No irreversible data migration in repo; runtime impact limited to dev compose image/volume layout and docs.

## Risks / NV

- Host default port conflicts and PG16→PG18 local volume reset are environmental/operator actions (documented). Real OIDC/JWKS e2e NV. Remote CI required checks must pass on PR before merge (not auto-merged).

## Communication

- PR should highlight: PG18 now required for documented compose/dev smoke (`uuidv7()`), disposable volume reset note for prior PG16 users, and corrected bootstrap token in `docs/dev-smoke.md`.
