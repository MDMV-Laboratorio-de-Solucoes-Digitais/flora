# Repository Guidelines

## Project Overview

Flora is an AGPL-3.0, self-hostable modular corporate suite: a Rust/Axum API backed by PostgreSQL 18 and one canonical SvelteKit 3 / Svelte 5 frontend in `apps/frontend`. The goal is lean single-VPS operation ("Seed") that scales to Sprout → Grove → Forest without changing the stack; tenant isolation and trait-driven services are first-class.

Work in `/home/luis/development/MDMV/projetos/flora`; `projetos/Arquivados/flora` is a docs fragment, not this repo. The real API entry point is `flora-api/src/main.rs` (workspace-root `src/main.rs` is a placeholder). Canonical UI language is pt-BR (`apps/frontend/src/lib/i18n/pt-BR.ts`); English is limited to technical terms (OIDC, JWT, API, slug).

## Architecture & Data Flow

Rust workspace: `flora-core` (config, migrations, models, repositories, traits, services, errors, JWT/PKCE), `flora-api` (HTTP surface, extractors, state, middleware, OIDC, tenant guards, realtime), `flora-messaging` / `flora-search` (optional capabilities), `flora-tests`, plus the root package `flora`. Edition 2024, resolver 3, Rust 1.98.1.

Startup (`flora-api/src/main.rs`): `tracing_subscriber` → `Config::load()` + `validate()` → `create_pool()` → `run_migrations()` (append-only SQLx migrations) → `AppState::new()` → `TcpListener::bind()` → `axum::serve(listener, routes::create_router(state))`.

Request path: handler in `flora-api/src/routes/*.rs` takes `State<AppState>` + `AuthExtractor`, calls `require_org()` and `tenant::ensure_*_in_org()` guards, then reaches an `Arc<XService>` or a `PgXRepository` built from `state.db_pool`; SQLx runs the query and the handler maps `flora_core::Result<T>` to DTO/`Json`. Routes are not 1:1 with modules (`org.rs` → `/organizations`, `rbac.rs` → `/roles`, `websocket.rs` → `/ws` and `/api/v1/ws`). Unauthenticated probes: `/health`, `/health/ready`, `/health/live`, `/metrics`, `/llms.txt`.

Non-obvious invariants:

- **Tenant authority:** org comes from the validated DB session (`jti` rechecked on every request), never a trusted header. `x-workspace-id` is a lenient selector (invalid UUID → ignored) that must still be checked with `ensure_workspace_in_org`; `X-Organization-ID` is not an authorization input. RLS is enabled on tenant tables but has no policies — application-level `WHERE organization_id = ?` is the isolation.
- **Identity:** OIDC PKCE S256 + verified ID token/JWKS + DB sessions; JWT signature/expiry alone is never sufficient. Local Argon2id fallback for bootstrap recovery. `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` is a one-shot wildcard opt-in targeting Admin/Owner roles; it expires `2026-11-29T23:59:59Z` and the env vars must then be removed.
- **Realtime:** `/ws` checks `Origin` against an exact allowlist before auth (missing origin allowed, `null` denied), accepts `Authorization: Bearer` then `?token=`, and rejects a nil org. The in-process `broadcast` hub (capacity 256) is best-effort: events are filtered by org + subscribed channels; lagging clients must catch up via REST.
- **Search:** `/api/v1/search` always answers from Postgres FTS with `source: "postgres"`; Meilisearch is dormant. Membership is required, query trimmed, limit 20 by default and capped at 50.
- **Errors:** `flora_core::Error`/`Result<T>` → `{"error": ..., "message": ...}` plus `field` on validation and `size`/`max` on file-too-large. `Error::from_sqlx` maps `23505`→DuplicateEntry, `23503`→Database, `RowNotFound`→Database. Notable statuses: 400 missing context, 403 cross-org, 409 duplicate, 413/415 file errors, 422 validation, 429 quota and rate limit, 503 OIDC, 500 DB/storage/search/config.
- **Config:** exactly eight Figment sections (`app`, `hitl`, `bootstrap`, `database`, `oidc`, `storage`, `search`, `messaging`), prefix `FLORA_`, nested keys with `__` (e.g. `FLORA_DATABASE__POSTGRES_URL`). No `.env` auto-load; unknown keys ignored. CORS must be exact `scheme://host[:port]` values; `*` is rejected. `FLORA_HITL__HMAC_SECRET` must be ≥32 chars (empty disables emission; `Debug` redacts it).
- **Side effects:** notifications, hub publish and Valkey publish are best-effort and must never fail the primary write. HTTP rate limiting uses an in-memory limiter, not `state.valkey`.

## Key Directories

|Path|Purpose|
|---|---|
|`flora-core/src/{config,migrations/sql,models,repositories,traits,services,utils}`|Config validation, forward-only SQL, domain types, `Pg*Repository`, trait seams, business services, JWT/PKCE/validation helpers|
|`flora-core/src/bin/flora-mint-session.rs`|`dev-mint`-gated CLI that mints browser `storageState` for E2E|
|`flora-api/src/{routes,extractors,state,middleware,tenant,oidc,realtime}`|HTTP surface, auth extraction, DI wiring, CORS/rate limit, BOLA guards, realtime hub|
|`flora-tests/src/{contract,integration}`|11 ignored DB contract modules; 9 non-DB integration modules|
|`apps/frontend/src/{lib,routes}`|API/WS clients, rune stores, pt-BR i18n, components, `(app)`/`(auth)` route groups|
|`apps/frontend/tests/{unit,integration,e2e}`, `apps/frontend/playwright/`|Vitest projects, Playwright smoke, session minting and seed setup|
|`scripts/ci/`, `scripts/deploy/`|Authoritative merge oracle, deploy-contract guard, read-only live preflight|
|`specs/<NNN-slug>/`, `.specify/{assessments,validations,releases,bugs,memory}`|SDD lifecycle and dated append-only records|
|`docs/{governance,research,adr}`, `docs/PROGRESSO.md`|Decisions, runbooks, research notes, append-only progress ledger|

## Development Commands

Rust commands from the repo root; frontend commands from `apps/frontend`.

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources
cargo build --workspace && cargo test --workspace
cargo run -p flora-api
python3 scripts/ci/check_gate_integrity.py
python3 scripts/ci/check_deploy_workflows.py
FLORA_WEB_URL=https://flora.mdmv.qzz.io FLORA_API_URL=https://flora-api.mdmv.qzz.io \
  python3 scripts/deploy/check_endpoints.py --timeout 8

docker compose up -d postgres                 # default profile: Postgres 18 only
docker compose --profile realtime up -d       # + Valkey
docker compose --profile search up -d         # + Meilisearch
```

DB contracts need PostgreSQL 18 (`uuidv7()` in the init migration):

```bash
export FLORA_DATABASE__POSTGRES_URL='postgres://postgres:postgres@127.0.0.1:5432/flora'
export DATABASE_URL="$FLORA_DATABASE__POSTGRES_URL"
cargo test -p flora-tests 'contract::tasks::db::db_' -- --ignored --test-threads=1
```

Frontend:

```bash
bun install && bun run check && bun run lint
bun run test:unit -- --run --project server
bun run build
bun run test:e2e
```

## Code Conventions & Common Patterns

- **Rust:** `snake_case`/`PascalCase`/`UPPER_SNAKE_CASE`; `thiserror::Error` in libraries, `anyhow` at process boundaries and in `AppState::new`; always `Result<T, E>`. `Uuid::now_v7()` for IDs, lowercase emails, SQLx `.bind().await.map_err(Error::from_sqlx)`. Migrations are append-only — a schema change also needs matching model, repository, contracts and gate floors.
- **Lints:** workspace denies `unsafe_code`, `missing_docs`, `missing_debug_implementations`, `unreachable_pub`, `unused_results`, `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `allow_attributes`; clippy `all`/`pedantic` denied, `nursery` warn. Use narrow `#[expect(clippy::lint, reason = "...")]`, never a bare `#[allow]`.
- **Dependency injection:** `AppState` (`flora-api/src/state/app_state.rs`) exposes concrete `Arc<XService>` fields plus `Option<...>` capabilities (`search_service`, `messaging_service`, `valkey`, `oidc_transactions`, `realtime_hub`); trait objects are locals inside `AppState::new`, which returns `anyhow::Result<Self>`. Handlers receive `State<AppState>` and often build `Pg*Repository` from `state.db_pool`.
- **Layers:** `Pg<Domain>Repository` implements `<Domain>Repository` from `flora-core/src/traits/repository.rs`; services are `<Domain>Service`; route modules expose `create_<domain>_router()`; HITL uses the `HitlReceiptPort`/`HitlReceiptAdapter` seam.
- **Frontend:** Svelte 5 runes are forced by `vite.config.ts` — no `export let` or `on:click`; use runes, `onclick`, `cn()` from `$lib/utils`, `lucide-svelte`, strict TS props/returns. UI errors go through `apiErrorMessage()`/`backendErrorMessage()`, strings through `t['chave']`.
- **Evidence:** dated append-only notes; `[EV]` = observed, `[NV]` = not validated (remote/CI/production state), `[INF]` = inference. Corrections are new entries, never rewrites.
- **SDD/Git:** assess → specify/clarify/plan/checklist/tasks/analyze → implement → converge → R4 `validate.md` → R7 `release.md`. Conventional Commits (usually pt-BR), explicit `git add <file>` (never `.`/`-A`), no force-push, branches `feature/NNN-slug` or `username/task-description`, PRs via `gh`.
- **Governance:** green required checks allow autonomous merge to `dev`; `dev→main` promotion, release/tagging, branch protection/CODEOWNERS and secrets remain human/SE decisions. Deploy is automatic on push/merge to `dev` (staging) and `main` (production), with `workflow_dispatch` as backup.

## Important Files

|Path|Why it matters|
|---|---|
|`Cargo.toml`|Workspace members, edition 2024, Rust 1.98.1 pin, lint policy, six-manifest version lockstep|
|`flora-api/src/main.rs`, `flora-api/src/routes/mod.rs`|Startup chain and public/protected route map|
|`flora-api/src/extractors/auth.rs`, `flora-api/src/tenant.rs`|Bearer/JWT/session authority and BOLA guards|
|`flora-api/src/state/app_state.rs`, `flora-api/src/realtime.rs`|Service/capability wiring; best-effort realtime hub|
|`flora-core/src/error.rs`, `flora-core/src/config/mod.rs`|Canonical HTTP errors; eight-section `FLORA_*` loading/validation|
|`flora-core/src/migrations/sql/20240818000001_init_schema.sql`|Base schema; PG18 `uuidv7()` defaults|
|`scripts/ci/check_gate_integrity.py`, `scripts/ci/check_deploy_workflows.py`|Sole merge gate (dynamic floors); deploy-contract guard for the automatic workflows|
|`.github/workflows/ci.yml`, `.github/workflows/rust.yml`|Required contexts and exact 11 DB legs (89 contracts, HITL included)|
|`.specify/memory/constitution.md`, `docs/adr/`, `docs/governance/`|Governance authority; ADR-0003 merge rule; human-only boundary|
|`apps/frontend/src/lib/api/client.ts`, `apps/frontend/src/lib/i18n/pt-BR.ts`|API origin/auth/errors; canonical UI strings|
|`apps/frontend/vite.config.ts`, `apps/frontend/docker-entrypoint.sh`|Runes/Vitest config; runtime `config.js` generation|

## Runtime/Tooling Preferences

- Rust 1.98.1, edition 2024, resolver 3; pin in `Cargo.toml`, CI and `Dockerfile` — no `rust-toolchain` file. PostgreSQL 18+ mandatory. Compose profiles: default `postgres`, `realtime` (Valkey 8), `search` (Meilisearch v1.12); no app services in Compose — run the API with `cargo run -p flora-api`.
- Frontend: Node 22 in Docker (`node:22-alpine`, `npm ci`, `engine-strict=true` in `.npmrc`); Bun locally and in CI (`bun.lock`, `bun install`). Both lockfiles must stay in sync when dependencies change; there is no `engines`/`packageManager` field.
- Required checks: `frontend-build` (`ci.yml`), `rust-build`, `db-tests`, `dependency-review` (`rust.yml`, PR-only). Advisory: `e2e-browser`, `supply-chain-vet`, OSV scan, branch-protection audit. Four Coolify deploy workflows run on push/merge to `dev` (staging) and `main` (production), with `workflow_dispatch` as backup; branch guards, immutable tags and fail-closed webhooks.
- Release tags `vX.Y.Z` assert all six Rust manifests share the same version. No root `verify.sh`, no nextest config, no pytest suite and no percentage coverage tool.

## Testing & QA

- Rust unit tests live in `#[cfg(test)]` modules across crates; `flora-core/tests/models.rs` aggregates model tests. DB contracts live in `flora-tests/src/contract/*.rs` (11 modules), are `#[ignore = "requires Postgres; ..."]`, named `db_<action>_<result>`, and run with `-- --ignored --test-threads=1` with both `FLORA_DATABASE__POSTGRES_URL` and `DATABASE_URL` set. OIDC tests use the local `flora-tests/src/oidc_mock.rs` (Ed25519 JWKS) and an inline fixture in `flora-api/src/oidc.rs`.
- `check_gate_integrity.py` is the authoritative oracle: it lists ignored tests with `cargo test -p flora-tests -- --ignored --list` and matches `run_leg <label> <filter> <floor>` entries in `rust.yml` by prefix. Current legs: auth 25, messaging 5, websocket 9, search 8, notifications 6, files 8, tasks 4, workspace 3, rbac 5, bootstrap 13, hitl 3 (89 total). Adding/removing an ignored test requires updating both the workflow and the script's floors.
- Frontend: Vitest 4 with `expect.requireAssertions: true`; `server` (node; the only required project) and `client` (headless Chromium via `@vitest/browser-playwright`; not required). Playwright E2E applies `dev-seed.sql` and mints browser auth with `flora-mint-session --features dev-mint`; `retries: 0` and `workers: 1` because `select-organization` revokes the injected `jti` and rotates the session.
- Axe-core WCAG 2.2 AA scans run across 16 routes in the Playwright smoke; `summarizeWcagAudit` pushes `wcag-axe` annotations and `expect(findings).toEqual([])` makes violations fail the smoke. The `e2e-browser` CI job is advisory.
- QA is contractual: required checks green, DB floors satisfied, evidence pasted into R4 `validate.md`, then R7 `release.md` with SemVer/changelog/rollback. Green local checks never prove remote/CI/deploy status.
