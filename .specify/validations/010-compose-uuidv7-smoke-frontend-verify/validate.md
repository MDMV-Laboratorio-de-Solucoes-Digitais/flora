# Validation: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

**Date:** 2026-11-15  
**Base:** `dev@03de975e1` (cycle 009) + cycle 010 changes (uncommitted at validation time)  
**Verdict:** **Approved (local deterministic + smoke evidence captured; remote CI NV pending PR)**

## Deterministic checks (local)

| Check | Result |
|---|---|
| `cargo fmt --all -- --check` | pass |
| `cargo check -p flora-api -p flora-core -p flora-tests` | pass |
| `cargo test -p flora-core --lib` | pass (8 passed, 0 failed) |
| `cargo test -p flora-tests --lib` | pass (56 passed, 0 failed, 5 ignored — DB-backed `#[ignore]`) |
| `cd apps/frontend && bun run check` | pass (0 errors, 0 warnings) |
| `cd apps/frontend && bun run lint` | pass |
| `cd apps/frontend && bun run test:unit -- --run --project server` | pass (7 files, 14 tests passed) |
| `cd apps/frontend && bun run build` | pass |

## Smoke evidence (integrated, compatible documented path)

- **Stack:** `docker compose -p flora_smoke010` with PostgreSQL **18-alpine** (`postgres:18-alpine`, healthy) + Valkey + Meilisearch. Default host port `5432` was occupied externally (NV/environmental), so an isolated override mapped Postgres/Valkey/Meili to `127.0.0.1:15433/16379/17700`; `FLORA_DATABASE__POSTGRES_URL`, `FLORA_MESSAGING__VALKEY_URL`, `FLORA_SEARCH__MEILISEARCH_URL` pointed accordingly. API ran on `127.0.0.1:3000`.
- **Migrations:** applied successfully on startup against fresh PG18 DB (no `uuidv7()` error).
- **Seed/bootstrap:** `dev-seed.sql` and `scripts/dev-smoke-session.sql` applied (alice/acme membership/session bootstrap present).
- **Flow (`./test-flora.sh`, corrected token for `FLORA_JWT_SECRET=change-me-to-a-random-secret-at-least-32-characters-long`):** health OK → `POST /api/v1/auth/select-organization` **200** with rotated org-scoped session (`user_id` alice, `organization_id` acme, `expires_in` 3600) → `GET /api/v1/workspaces` listed `Design Workspace`, `Main Workspace` → `POST /api/v1/tasks` **created** task (`Smoke test task`, status `Todo`) → list-after-create empty as expected (assignee-scoped semantics). **Result: `smoke_rc=0`** (log: `/tmp/flora-smoke010-ok.log`). Temp compose project torn down (`down -v`) after capture.
- **Bug found & fixed during 010:** `docs/dev-smoke.md` contained an invalid pre-signed bootstrap JWT for the `.env.example` secret (caused `401 InvalidSignature`); corrected token documented and verified.

## Changes validated

- `docker-compose.yml`: `postgres:16-alpine` → `postgres:18-alpine`; volume mount `flora_postgres_data:/var/lib/postgresql` (PG18 layout).
- `docs/dev-smoke.md`: PG18 requirement, PG16→PG18 disposable-volume reset note, port-conflict guidance, Meilisearch key/URL consistency notes, corrected bootstrap JWT for `.env.example` secret.
- `apps/frontend/vite.config.ts`: server Vitest project includes `tests/**/*.{test,spec}.{js,ts}` and excludes `tests/e2e/**` for deterministic unit runs.

## NV / limitations

- **Default-port documented path:** host `5432` occupied by non-flora external process in this environment; smoke evidenced via equivalent isolated alt-port mapping (same PG18 image/migrations/flow). Operator on a free default-port host can run `docker compose up -d` as documented.
- **PG16→PG18 data:** existing local PG16 volumes are incompatible with PG18 image layout; reset requires `docker compose down -v` on disposable env (not performed on non-disposable data).
- **DB-backed ignored tests/remote CI:** `cargo test -p flora-tests --lib` ignores 5 DB-backed cases by default (ran in prior cycles on PG18); GitHub Actions `db-tests`/frontend CI and PR required checks are **NV** until PR runs.
- **OIDC/JWKS real e2e:** remains NV (smoke uses deterministic bootstrap, as designed).
