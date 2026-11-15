# Plan: 009 — DB-backed Select-Org Tests, Integrated Smoke & Minimal Frontend Slice

**Assessment:** `.specify/assessments/009-db-smoke-frontend-slice/`  
**Spec:** `specs/009-db-smoke-frontend-slice/spec.md`

## Approach
1. **DB-backed tests (`flora-tests/src/contract/auth.rs`)**
   - Add `#[ignore]` + env-gated module (`FLORA_DATABASE__POSTGRES_URL`/`DATABASE_URL`) using `PgPool::connect` + `flora_core::migrations::run_migrations`.
   - Create unique fixtures per test (user, org, role, membership when needed, active/inactive/mismatched pre-selection session with matching `jti` and JWT signed with `config.app.jwt_secret`).
   - Cases: success (200, rotated token/org, old session revoked, new active), non-member (403), missing session (401), inactive session (401), mismatched session user (401).

2. **CI (`.github/workflows/rust.yml`)**
   - Keep existing `build` job unchanged.
   - Add `db-tests` job with `postgres:18` service + healthcheck, DB env vars, and `cargo test -p flora-tests contract::auth::db_select_organization -- --ignored --test-threads=1`.

3. **Integrated smoke execution/evidence**
   - Use disposable Postgres/Valkey/Meilisearch on non-conflicting ports when compose ports are unavailable; run `flora-api` with env overrides (migrations on startup), load `dev-seed.sql` and `scripts/dev-smoke-session.sql`, execute `test-flora.sh` flow (`select org → workspace → task`), capture logs/results and NV details.

4. **Frontend minimal slice (`apps/frontend`)**
   - `lib/state/AuthState.svelte.ts`: persist token/user/org/workspace, `setOrganization` token rotation, `setWorkspace`.
   - `(auth)/+page.svelte`: persist OAuth `state` + redirect URI; `(auth)/callback/+page.svelte`: validate/pass `state`, exchange code, store nil-org session, route to `/select-org`.
   - `(app)/select-org/+page.svelte`: list/create org, `POST /api/v1/auth/select-organization`, store rotated token/org.
   - `(app)/+layout.svelte`: client guard (auth + org selected).
   - `(app)/tasks/+page.svelte`: load workspaces/tasks, select workspace, create task via API (assign-to-me option), display assignee-scoped list.

## Verification
- `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core -p flora-tests`, `cargo test -p flora-tests --lib` (default), `cargo test -p flora-core --lib`.
- DB-backed tests: `cargo test -p flora-tests contract::auth::db_select_organization -- --ignored --test-threads=1` against Postgres.
- Integrated smoke: scripted run with captured rc/output.
- Frontend: deterministic checks as toolchain permits (`bun run check/build/test:unit` in `apps/frontend`) — record NV if unavailable.

## NV / risks
- Host `5432` conflict prevented compose-as-documented; migration `uuidv7()` incompatible with `postgres:16-alpine` (compose), evidenced with pg18 alternative.
- Frontend toolchain availability may limit full deterministic run in this environment.
