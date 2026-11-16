# AGENTS.md

## Repo reality (read first)
- Rust modular-monolith workspace (`resolver = "3"`, Rust `1.98`, edition `2024`, AGPL-3.0-or-later) with crates: `flora-core`, `flora-api`, `flora-messaging`, `flora-tasks`, `flora-files`, `flora-search`, `flora-notifications`, `flora-tests`.
- Real API entrypoint is `flora-api` (`flora-api/src/main.rs`, bin `flora-api`, lib `flora_api`): loads config, creates Postgres pool, runs migrations, builds `AppState`, serves Axum router. Root `src/main.rs` is only a placeholder.
- `flora-core` owns shared config (`src/config/*`), DB pool/migrations (`src/database.rs`, `src/migrations/mod.rs`), models, repositories, services, traits, utils, and storage. Route modules live in `flora-api/src/routes/*`.
- Frontend/apps exist under `apps/`: `apps/frontend` is the canonical Flora Svelte 5 rewrite target (SvelteKit/Svelte 5). `apps/web` is a separate imported `@coparse/web` (SolidJS/Macro-style) app and is not the canonical target. `.github/workflows/ci.yml` runs in `apps/frontend` (`bun install`, `check`, `lint`, `test:unit -- --run --project server`, `build`); `.github/workflows/rust.yml` runs fmt check, strict clippy (production targets), `cargo deny check bans licenses sources`, `cargo build`/`cargo test` and six DB-gated suites on PostgreSQL 18, with toolchain `1.98.0`.
- README/old docs are partly aspirational: there is **no `lefthook.yml` or nextest config** in this checkout. A `deny.toml` exists (cycle 025) and CI now runs fmt/strict clippy/cargo-deny (cycle 026); `supply-chain/` (cargo-vet) is gitignored. Prefer `Cargo.toml`, workflows, and code over prose. See `docs/adr/0001-seed-v01-mechanism-substitutions.md` for the authoritative Seed mechanisms.

## Commands
- Rust (workspace root): `cargo build`, `cargo test`, `cargo build --release`.
- Format/lint: `cargo fmt --all` (check: `cargo fmt --all -- --check`), `cargo clippy --workspace --lib --bins -- -D warnings` (CI gate on production targets). Locally `--all-targets` is stricter but currently blocked by a parallel agent's test-only `expect_used` violations (NV; do not revert their work).
- Focused runs: `cargo test -p <crate>` (e.g. `flora-core`, `flora-api`, `flora-tests`), `cargo test -p <crate> <test_name_or_path_fragment>`, `cargo run -p flora-api`.
- Canonical frontend (`apps/frontend`): from `apps/frontend` run `bun install`, `bun run check`, `bun run lint`, `bun run test:unit -- --run` (CI uses `--project server`), `bun run build`. `apps/web` has its own toolchain; not the Svelte 5 target.
- `test-flora.sh`: manual smoke script against `http://localhost:3000` using fixed IDs from `dev-seed.sql` and a Bearer session token (`Authorization: Bearer ...`); assumes a running, seeded API. Org context comes from the verified session, not request headers.

## Config, environment, and runtime quirks
- Copy `.env.example` to `.env` for local config. `flora_core::config::Config::load()` merges defaults + env vars prefixed `FLORA_` with `__` as nesting separator (e.g. `FLORA_DATABASE__POSTGRES_URL`, `FLORA_MESSAGING__VALKEY_URL`, `FLORA_OIDC__REDIRECT_URI`).
- Startup validates config, connects Postgres, and applies migrations (`sqlx::migrate!("./src/migrations/sql")` from `flora-core/src/migrations/mod.rs`). Missing DB/migration prerequisites will break API startup/tests that hit DB.
- `FLORA_ENV`, host/port/base URL, JWT/session settings, OIDC/Zitadel, optional Meilisearch, storage path, and OTEL vars are defined in `.env.example`.

## Testing and code constraints
- Tests live in crate-local `tests/`/`#[cfg(test)]` modules and in `flora-tests/src/{contract,integration,...}`. There is no checked-in snapshot-test workflow implied by the tree; do not invent DB snapshot requirements.
- Workspace lints deny `unsafe_code`, `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `dbg_macro`, `print_stdout` is warned, and most clippy groups are denied. Prefer `Result`/`?`, `tracing`, and explicit error handling; if `#[allow(...)]` is unavoidable, include a `reason` (and satisfy `allow_attributes` rules).
- Keep changes crate-local when possible: API wiring in `flora-api`, domain/data/config/migrations in `flora-core`, messaging concerns in `flora-messaging`, etc.

## Existing instruction sources
- Issue tracking: `docs/agents/issue-tracker.md` (GitHub via `gh`).
- Domain layout guidance: `docs/agents/domain.md` (mentions `CONTEXT-MAP.md`/`CONTEXT.md` patterns if present; none currently checked in). Consult them before architectural changes when they exist in scope.
- Ignore generated/noisy dirs (`target/`, `node_modules/`, `.clean-room/`, inactive tooling dirs) unless the task targets them.
