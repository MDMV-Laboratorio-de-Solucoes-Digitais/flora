# Repository Guidelines

## Project Overview

Flora Workspace is an AGPL-3.0, self-hostable collaboration workspace: Rust Axum API + PostgreSQL core, with one canonical SvelteKit 3 / Svelte 5 frontend in `apps/frontend`. The active checkout is `/home/luis/development/MDMV/projetos/flora` — **not** `projetos/Arquivados/flora` (that path is only a docs fragment of the HQ repo).

The real API entry point is `flora-api/src/main.rs`; the workspace root `src/main.rs` is only a placeholder. Production web is `https://flora.mdmv.qzz.io`, API is `https://flora-api.mdmv.qzz.io`. Coolify deploys are manual and outside the merge gate.

Canonical frontend language is pt-BR (`src/lib/i18n/pt-BR.ts`). English is allowed only for technical terms such as OIDC, OAuth, API, JWT, REST, SSO, AI, slug.

## Architecture & Data Flow

Rust workspace (`Cargo.toml`, resolver 3, edition 2024):

- `flora-core`: config, migrations, models, repositories, services, traits, `Error`/`Result`.
- `flora-api`: HTTP entry point, routes, extractors, middleware, OIDC, state, realtime hub.
- `flora-messaging` / `flora-search`: optional Valkey Pub/Sub and search services.
- `flora-tests`: DB-backed contract tests and integration tests.

Request flow:

`flora-api/src/main.rs` → `Config::load()` → `run_migrations()` → `AppState::new()` → `routes::create_router()` → handler → `AuthExtractor`/`OrgContext` → service traits → SQLx repository → PostgreSQL → DTO/`Json` response.

Key patterns:

- `AppState` (`flora-api/src/state/app_state.rs`) holds `Arc<PgPool>`, `Arc<Config>`, `Arc<dyn Trait + Send + Sync>` repositories/services, and optional search/messaging capabilities. Handlers receive it via `State<AppState>`/`.with_state()`.
- Tenant safety is explicit: `flora-api/src/tenant.rs` (`ensure_workspace_in_org`, `ensure_user_in_org`) guards BOLA/cross-org access.
- Errors use `flora_core::Error` (`thiserror`) and `flora_core::Result<T>`; `impl IntoResponse` maps to `{ error, message, field? }` with canonical statuses. `anyhow` is for process boundaries/tests only.
- Config is Figment-based: `FLORA_APP__BASE_URL`, `FLORA_DATABASE__POSTGRES_URL`, etc. Prefix `FLORA_`, nested keys with `__`. No `.env` is auto-loaded. Unknown keys are silently ignored. CORS uses Figment array syntax; never `*` or `null`.
- Identity: OIDC PKCE S256 plus DB sessions; local password fallback exists for Seed/bootstrap recovery. `FLORA_BOOTSTRAP__ADMIN_EMAIL=*` is an explicit, temporary recovery opt-in: it targets the oldest active user at startup and allows `POST /api/v1/auth/password/login` with `email="*"`; remove it after use.
- Frontend: `src/app.html` loads `static/config.js` before the bundle; runtime `window.__FLORA_CONFIG__.apiBaseUrl` overrides build-time config. `ApiClient` adds Bearer auth, converts errors to `ApiError`, and maps `401` to `authState.lockSession()`.
- Svelte 5 uses runes (`$props`, `$state`, `$effect`) and singleton stores in `*.svelte.ts`; no legacy `export let` or `on:click`. UI errors go through `apiErrorMessage()`/`backendErrorMessage()` from `src/lib/i18n`, never raw backend messages.

## Key Directories

|Path|Purpose|
|---|---|
|`flora-core/src/config/`|Figment config modules and startup validation|
|`flora-core/src/{models,repositories,traits,services}/`|Domain model, SQLx repos, trait seams, business services|
|`flora-core/src/migrations/sql/`|SQLx migrations; PostgreSQL 18+ required (`uuidv7()`)|
|`flora-api/src/routes/`|Axum routers and handlers (`auth`, `bootstrap`, `tasks`, `channels`, `files`, `search`, `ws`, …)|
|`flora-api/src/{extractors,state,middleware,tenant,oidc}/`|Auth extractors, `AppState`, CORS/rate limit, tenant guards, OIDC|
|`flora-tests/src/contract/`|DB-gated contract tests; use `db_<action>_<result>` naming|
|`apps/frontend/src/routes/`|SvelteKit routes; `(app)` and `(auth)` groups do not affect URLs|
|`apps/frontend/src/lib/{api,state,i18n,components}/`|API client/WebSocket, rune stores, pt-BR dictionary, UI components|
|`apps/frontend/tests/{unit,integration,e2e}/`|Vitest browser/server unit tests and Playwright smoke|
|`scripts/ci/`|Authoritative gate (`check_gate_integrity.py`) only|
|`specs/<NNN-slug>/`, `.specify/{assessments,validations,releases}/`|SDD artifacts, append-only lifecycle records|
|`docs/governance/`, `docs/adr/`|SE decisions and architecture decision records|

## Development Commands

Rust (from repo root):

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources
cargo build --workspace
cargo test --workspace
cargo run -p flora-api
python3 scripts/ci/check_gate_integrity.py
```

DB-backed contract tests (Postgres 18 required):

```bash
docker compose up -d postgres
export FLORA_DATABASE__POSTGRES_URL='postgres://postgres:postgres@127.0.0.1:5432/flora'
export DATABASE_URL="$FLORA_DATABASE__POSTGRES_URL"
cargo test -p flora-tests contract::bootstrap::tests::db_ -- --ignored --test-threads=1
```

Frontend (`cd apps/frontend`):

```bash
bun install
bun run check
bun run lint
bun run test:unit -- --run --project server
bun run build
bun run test:e2e
```

Deploy/build helpers:

```bash
docker compose up -d                 # lean Seed: postgres only
docker compose --profile realtime up -d
docker compose --profile search up -d
```

There is no root `verify.sh`, no nextest config, and no Python pytest suite. Factory scripts such as `check_sdd_completeness.py`, `hub_docs_gate.py`, and `check_production_gate.py` are **absent** here (exit 2) — do not import them as gates.

## Code Conventions & Common Patterns

- Rust lints are strict: `unsafe_code`, `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `dbg_macro`, `missing_docs`, clippy `all`/`pedantic` are denied. Use `Result`/`?`; use `#[expect(clippy::lint, reason = "...")]`, never `#[allow]`.
- Prefer explicit DTOs (`TaskResponse`, `BootstrapResponse`) over serializing domain models directly. Never expose password hashes or secrets.
- Repositories implement traits from `flora-core/src/traits/repository.rs`; use `Uuid::now_v7()` for IDs, lowercase emails, and `sqlx` `.bind()`/`.await`/`map_err(Error::from_sqlx)`.
- Handlers return `flora_core::Result<T>`; tests use `anyhow::Result<()>`. Generic 401 (`invalid_credentials`) avoids user enumeration. Rate limiting classes are explicit in `flora-api/src/middleware/rate_limit.rs`.
- Migration files are append-only SQL under `flora-core/src/migrations/sql/`; schema changes require matching model, repository, contract tests, and gate floor updates.
- Conventional Commits in Portuguese are common. Branch names: `feature/NNN-slug`, `fix/NNN-slug`, `username/task-description`. Use `gh` for GitHub issues/PRs; `origin` is a Forgejo mirror. Never `git add -A` / `git add .`, force-push, or rewrite others' work.
- SDD cycles use `.specify/feature.json` for identity, never the branch name. `assess` (intake→research→define→shape→decide) precedes `specify → plan → tasks → analyze → implement → converge`; R4 `validate.md` and R7 `release.md` are written artifacts, not slash commands.

## Important Files

|Path|Why it matters|
|---|---|
|`Cargo.toml`|Workspace members, strict lints, Rust 1.98.1 pin|
|`flora-api/src/main.rs`|Real API startup and router assembly|
|`flora-api/src/routes/mod.rs`|Public/protected route map and middleware layers|
|`flora-api/src/extractors/auth.rs`|Bearer JWT + session validation, `AuthExtractor`, org context|
|`flora-api/src/state/app_state.rs`|Dependency wiring and degraded-capability logic|
|`flora-core/src/error.rs`|Canonical error enum and HTTP mapping|
|`flora-core/src/config/mod.rs`|`FLORA_*` Figment loading and validation|
|`flora-core/src/migrations/sql/20240818000001_init_schema.sql`|Base schema; PostgreSQL 18 `uuidv7()`|
|`.github/workflows/rust.yml`|Rust CI, DB floors, advisory jobs|
|`.github/workflows/ci.yml`|Frontend required check|
|`scripts/ci/check_gate_integrity.py`|Sole authoritative merge gate oracle|
|`apps/frontend/vite.config.ts`|Inline SvelteKit config; no `svelte.config.*`|
|`apps/frontend/src/lib/api/client.ts`|API origin resolution, Bearer auth, error conversion|
|`apps/frontend/src/lib/i18n/{index.ts,pt-BR.ts}`|pt-BR dictionary and backend error mapping|
|`apps/frontend/docker-entrypoint.sh`|Runtime `config.js` generation for adapter-node|
|`.specify/memory/constitution.md`, `docs/adr/`|Governance and architectural decisions|

## Runtime/Tooling Preferences

- Rust: 1.98.1, edition 2024, resolver 3. No `rust-toolchain` file; CI installs 1.98.1 explicitly.
- Frontend: Node 22 (`node:22-alpine`), `engine-strict=true`. Vite 8, SvelteKit 3, Svelte 5, TypeScript 6, Tailwind 4, Vitest 4, Playwright.
- Package managers coexist: `bun.lock` is used by CI and local frontend commands; `package-lock.json` is used by the Docker image with `npm ci`. Keep both in sync when dependencies change.
- PostgreSQL 18+ is mandatory for Seed. `docker-compose.yml` profiles: default `postgres`, `realtime` (Valkey), `search` (Meilisearch).
- Required checks: `frontend-build`, `rust-build`, `db-tests`, `dependency-review` (PR-only). `e2e-browser`, `supply-chain-vet`, OSV scan, Forgejo mirror, and branch-protection audit are advisory.
- Deploy uses Coolify via push/merge to `dev` (staging) and `main` (production); `workflow_dispatch` is the manual backup. Deploy steps fail closed when webhook secrets are missing; never assume a deploy happened without its workflow evidence.
- Absolute rule: no credentials in docs, logs, commits, or tool output. Use local token files only via shell variables.

## Testing & QA

- Rust unit tests: `#[cfg(test)]`, `#[test]`, `#[tokio::test]`; names like `test_<target>_<scenario>` or behavioral (`hash_uses_argon2id_owasp_parameters`).
- DB contract tests: `flora-tests/src/contract/*.rs`, `#[ignore = "requires Postgres..."]`, `db_<action>_<result>`. Arrange with `db_state()`/`seed_*`, act through `oneshot`, assert status + JSON. Full isolation uses temporary databases for bootstrap tests.
- Gate floors are dynamic: `check_gate_integrity.py` runs `cargo test -p flora-tests -- --ignored --list` and compares exact counts (current total 77: auth 23, messaging 4, websocket 8, search 8, notifications 5, files 6, tasks 4, workspace 3, rbac 3, bootstrap 13). Adding/removing ignored tests requires updating both `FLOORS` and `.github/workflows/rust.yml`.
- Frontend unit tests use Vitest projects `server` and `client`; selectors must follow the pt-BR dictionary. E2E setup applies `dev-seed.sql` + `scripts/dev-smoke-session.sql`, mints browser auth with `flora-mint-session`, and validates ACME org/session drift.
- No percentage coverage tool exists. QA is contractual: required checks green, DB floors satisfied, R4 validate with pasted command outputs, R7 release with SemVer/changelog/rollback, and `[NV]` markers for anything not directly verified.
- Docs and state records are append-only and dated. Cite `file:line`; never claim remote/CI/deploy status without live evidence.
