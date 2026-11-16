# Flora Dev Smoke (updated 016-C): `select-organization` → workspace → task

Reproducible local smoke path without real OIDC, using `docker compose`, `dev-seed.sql`, and a deterministic pre-selection session/JWT bootstrap.

**Scope:** dev/local verification aid. Not for production. Runtime-integrated OIDC e2e remains **NV**.

## Prerequisites
- Docker + Docker Compose
- Rust toolchain (to run `flora-api`) and `psql` client (or `docker compose exec postgres psql ...`)
- Repo checkout at this revision
- **PostgreSQL 18+** for the documented compose path: `flora-core/src/migrations/sql/20240818000001_init_schema.sql` uses `DEFAULT uuidv7()` (native in PG18). `docker-compose.yml` uses `postgres:18-alpine`.

> **Existing local volume note:** if you previously initialized `flora_postgres_data` with `postgres:16-alpine`, a PG16 data directory is not compatible with PG18. For a disposable dev reset: `docker compose down -v` (removes named volumes) then `docker compose up -d`. Do not reset non-disposable data without backup/migration.
>
> **Port conflict note (NV/environmental):** if host `5432/6379/7700` are already in use, run compose with alternative host ports via an override file and point env vars/`psql` to the chosen ports. The migration/seed/bootstrap/API flow remains the same.
>
> **Lean Seed profile (016-C, updated 023):** default compose brings up **postgres only**. Enable optional dependencies with profiles: `--profile realtime` (Valkey) and/or `--profile search` (Meilisearch), and set `FLORA_MESSAGING__ENABLED=true` / `FLORA_SEARCH__ENABLED=true` accordingly. API starts in degraded/optional mode when disabled.
>
> **Search authority (023):** `GET /api/v1/search` is always answered by **Postgres FTS** (`"source": "postgres"` in the response). Meilisearch is a **dormant, non-authoritative capability**: the `flora-search` crate and the compose `search` profile are kept for a gated future re-enablement, but enabling `FLORA_SEARCH__ENABLED` does **not** change search results. Re-enablement as an authoritative backend requires mutation-indexing hooks, async task-await handling, a Postgres-sourced rebuild/backfill tool, and deterministic Meilisearch-gated tests (all NV; design notes in `docs/research/2026-11-16-023-*.md`).

## 1) Start dependencies
```bash
# Lean core (postgres only)
docker compose up -d

# Optional: include realtime/search
# docker compose --profile realtime --profile search up -d
```
Default services: Postgres `127.0.0.1:5432` (user/pass/db `postgres`/`postgres`/`flora`). Valkey `127.0.0.1:6379` and Meilisearch `127.0.0.1:7700` are profile-gated.

## 2) Configure environment
```bash
cp .env.example .env
# Ensure DB URL matches compose:
# FLORA_DATABASE__POSTGRES_URL=postgresql://postgres:postgres@localhost:5432/flora
# Lean core: FLORA_SEARCH__ENABLED=false, FLORA_MESSAGING__ENABLED=false
# Keep FLORA_APP__JWT_SECRET consistent with the bootstrap token you use (see below).
```
> Note: use **one** JWT secret consistently for minting/validation. If you changed compose host ports, update URLs accordingly.

## 3) Run API (applies migrations on startup)
```bash
cargo run -p flora-api
```
`flora-api` connects to Postgres and runs migrations automatically (`flora_core::migrations::run_migrations`) in a single init path.

## 4) Load seed data
With host `psql`:
```bash
PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -d flora -f dev-seed.sql
```
Or via container:
```bash
docker compose exec -T postgres psql -U postgres -d flora < dev-seed.sql
```
Seeds org `11111111-1111-1111-1111-111111111111` (Acme), users Alice/Bob/Carol, memberships, workspaces `88888888-...` (Main) / `99999999-...` (Design), tasks, etc. No sessions are seeded.

## 5) Bootstrap pre-selection session + token (no OIDC)
Insert a fixed active pre-selection session for Alice (nil org, jti `018f0000-0000-7000-8000-000000000001`):
```bash
PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -d flora -f scripts/dev-smoke-session.sql
```

Use a matching pre-signed JWT for that session (claims: sub=Alice, jti as above, organization_id=nil, email=alice@acme.test). Pick the token that matches your `FLORA_APP__JWT_SECRET`:

- **If `FLORA_APP__JWT_SECRET=change-me-to-a-random-secret-at-least-32-characters-long` (.env.example):**
  `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIyMjIyMjIyMi0yMjIyLTIyMjItMjIyMi0yMjIyMjIyMjIyMjIiLCJqdGkiOiIwMThmMDAwMC0wMDAwLTcwMDAtODAwMC0wMDAwMDAwMDAwMDEiLCJvcmdhbml6YXRpb25faWQiOiIwMDAwMDAwMC0wMDAwLTAwMDAtMDAwMC0wMDAwMDAwMDAwMDAiLCJlbWFpbCI6ImFsaWNlQGFjbWUudGVzdCIsImV4cCI6MTc5NDc4NzIwMCwiaWF0IjoxNzYzMjUxMjAwLCJ2ZXIiOjF9.AgxYRUAu7SpJ4N1HPw_dz4tp6f-YWhNZTdDL2DP1Yq4`
- **If `FLORA_APP__JWT_SECRET=dev-secret-change-in-production` (legacy default, unset env):**
  `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIyMjIyMjIyMi0yMjIyLTIyMjItMjIyMi0yMjIyMjIyMjIyMjIiLCJqdGkiOiIwMThmMDAwMC0wMDAwLTcwMDAtODAwMC0wMDAwMDAwMDAwMDEiLCJvcmdhbml6YXRpb25faWQiOiIwMDAwMDAwMC0wMDAwLTAwMDAtMDAwMC0wMDAwMDAwMDAwMDAiLCJlbWFpbCI6ImFsaWNlQGFjbWUudGVzdCIsImV4cCI6MTc5NDc4NzIwMCwiaWF0IjoxNzYzMjUxMjAwLCJ2ZXIiOjF9.zPbSy7mxjtwpa6nUH0TMuKoMFEuSPl_srbZz2QVTibQ`

Export it for the smoke script:
```bash
export FLORA_TOKEN='<token-matching-your-secret>'
export FLORA_BASE_URL='http://localhost:3000'
export ORG_ID='11111111-1111-1111-1111-111111111111'
export WORKSPACE_ID='88888888-8888-8888-8888-888888888888'
```

## 6) Run smoke (`select org → workspace → task`)
```bash
./test-flora.sh
```
Expected behavior:
- `POST /api/v1/auth/select-organization` rotates the session and returns an org-scoped `session_token`.
- `GET /api/v1/workspaces` lists seeded workspaces for Acme.
- `POST /api/v1/tasks` creates a task in the selected workspace.

**Known semantics (NV/nuance):** `GET /api/v1/tasks` currently lists tasks assigned to the authenticated user in the org (assignee-scoped). A task created via this API defaults to no assignee, so it may not appear in the assignee-scoped list even when creation succeeds; verify creation by the create response (and/or DB) rather than relying solely on list-after-create.

## 7) Minimal backup/restore basics (016-C)
- **Postgres (authoritative):** schedule off-host `pg_dump -Fc` (and globals if needed). Keep retention (e.g. 7 daily / 4 weekly) and periodically restore into a disposable PG18 instance, run migrations/smoke, and verify data.
- **Storage:** back up `FLORA_STORAGE__LOCAL_PATH` (uploaded blobs when implemented) alongside DB snapshots as appropriate.
- **Valkey/Meilisearch:** treat as optional/rebuildable caches/indexes in lean Seed profiles; primary recovery focus is Postgres + storage.
- **NV:** restore drills, off-host retention automation, and 2vCPU/4GB load/OOM validation not executed in this cycle.

## Cleanup
```bash
docker compose down -v
```

## CI / NV notes
- CI (`.github/workflows/rust.yml`) runs build/test without Postgres/Valkey/Meilisearch; DB-integrated runtime smoke is **NV** in CI unless run against a provisioned Postgres service.
- Contract tests for `select-organization` that require no live DB (missing auth, nil org validation) run in `cargo test -p flora-tests`.
