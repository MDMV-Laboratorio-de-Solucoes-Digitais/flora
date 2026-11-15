# Plan: 008 — Select-Organization Tests & Reproducible Dev Smoke

**Assessment:** `.specify/assessments/008-select-org-tests-and-dev-smoke/`  
**Spec:** `specs/008-select-org-tests-and-dev-smoke/spec.md`

## Approach
1. **Contract tests (CI-safe):** extend `flora-tests/src/contract/auth.rs` with in-process `oneshot` tests for `POST /auth/select-organization` that do not require a live database:
   - missing `Authorization` → `401 Unauthorized` (extractor).
   - valid bearer (nil-org claims) + body `organization_id=nil` → `422 Unprocessable Entity` (`Error::Validation` mapping) before DB/session/membership access.
2. **Dev smoke bootstrap (no OIDC):** provide deterministic local artifacts:
   - `scripts/dev-smoke-session.sql`: upsert fixed pre-selection session for seeded Alice (`2222...`, nil org, fixed `jti`) with long expiry.
   - `docs/dev-smoke.md`: docker compose + migrations/startup + `dev-seed.sql` + session bootstrap + pre-signed JWTs (matched to common `FLORA_JWT_SECRET` values) + `test-flora.sh` flow and known nuances.
   - `test-flora.sh`: clarify assignee-scoped list nuance.
3. **Verification:** `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core -p flora-tests`, `cargo test -p flora-tests --lib contract::auth` (and record broader/NV where relevant).
4. **SpecKit artifacts:** intake/research/decision, spec/plan/tasks/checklist updated with evidence and NV.

## Explicit constraints / NV
- CI has no Postgres (`.github/workflows/rust.yml`) → DB-backed success/non-member/invalid-session automated coverage **NV** in CI; local integrated smoke execution **NV** in this environment.
- No runtime/API behavior changes; no OIDC verification hardening; no frontend alignment.

## Risks & mitigations
- **Secret/token mismatch:** document secret-dependent tokens and fixed `jti/session` coupling.
- **Task list semantics:** document assignee-scoped `GET /tasks` vs unassigned create in smoke.
