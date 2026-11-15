# Flora Dev Smoke (Cycle 008): `select-organization` → workspace → task

Reproducible local smoke path without real OIDC, using `docker compose`, `dev-seed.sql`, and a deterministic pre-selection session/JWT bootstrap.

**Scope:** dev/local verification aid. Not for production. Runtime-integrated OIDC e2e remains **NV**.

## Prerequisites
- Docker + Docker Compose
- Rust toolchain (to run `flora-api`) and `psql` client (or `docker compose exec postgres psql ...`)
- Repo checkout at this revision

## 1) Start dependencies
```bash
docker compose up -d
```
Services: Postgres `127.0.0.1:5432` (user/pass/db `postgres`/`postgres`/`flora`), Valkey `127.0.0.1:6379`, Meilisearch `127.0.0.1:7700` (dev key `flora-dev-master-key`).

## 2) Configure environment
```bash
cp .env.example .env
# Ensure DB URL matches compose:
# FLORA_DATABASE__POSTGRES_URL=postgresql://postgres:postgres@localhost:5432/flora
# Keep FLORA_JWT_SECRET consistent with the bootstrap token you use (see below).
```
> Note: `AppConfig` default secret in code is `dev-secret-change-in-production` when `FLORA_JWT_SECRET` is unset, while `.env.example` sets `change-me-to-a-random-secret-at-least-32-characters-long`. Use **one** secret consistently for minting/validation.

## 3) Run API (applies migrations on startup)
```bash
cargo run -p flora-api
```
`flora-api` connects to Postgres and runs migrations automatically (`flora_core::migrations::run_migrations`).

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

Use a matching pre-signed JWT for that session (claims: sub=Alice, jti as above, organization_id=nil, email=alice@acme.test). Pick the token that matches your `FLORA_JWT_SECRET`:

- **If `FLORA_JWT_SECRET=change-me-to-a-random-secret-at-least-32-characters-long` (.env.example):**
  `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIyMjIyMjIyMi0yMjIyLTIyMjItMjIyMi0yMjIyMjIyMjIyMjIiLCJqdGkiOiIwMThmMDAwMC0wMDAwLTcwMDAtODAwMC0wMDAwMDAwMDAwMDEiLCJvcmdhbml6YXRpb25faWQiOiIwMDAwMDAwMC0wMDAwLTAwMDAtMDAwMC0wMDAwMDAwMDAwMDAiLCJlbWFpbCI6ImFsaWNlQGFjbWUudGVzdCIsImV4cCI6MTc5NDc4NzIwMCwiaWF0IjoxNzYzMjUxMjAwLCJ2ZXIiOjF9.ix0i5Ec3GkxhGdXsCVVfrDHT2yb3gXbslNpKAkwODYo`
- **If `FLORA_JWT_SECRET=dev-secret-change-in-production` (code default, unset env):**
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

## Cleanup
```bash
docker compose down -v
```

## CI / NV notes
- CI (`.github/workflows/rust.yml`) runs build/test without Postgres/Valkey/Meilisearch; DB-integrated runtime smoke is **NV** in CI and was **NV** in this authoring environment.
- Contract tests for `select-organization` that require no live DB (missing auth, nil org validation) run in `cargo test -p flora-tests`.
