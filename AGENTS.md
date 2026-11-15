# AGENTS.md

## Repo reality (read first)
- Rust modular-monolith workspace (`resolver = "3"`, Rust `1.98`, edition `2024`, AGPL-3.0-or-later) with crates: `flora-core`, `flora-api`, `flora-messaging`, `flora-tasks`, `flora-files`, `flora-search`, `flora-notifications`, `flora-tests`.
- Real API entrypoint is `flora-api` (`flora-api/src/main.rs`, bin `flora-api`, lib `flora_api`): loads config, creates Postgres pool, runs migrations, builds `AppState`, serves Axum router. Root `src/main.rs` is only a placeholder.
- `flora-core` owns shared config (`src/config/*`), DB pool/migrations (`src/database.rs`, `src/migrations/mod.rs`), models, repositories, services, traits, utils, and storage. Route modules live in `flora-api/src/routes/*`.
- Frontend/apps exist under `apps/`: `apps/frontend` is the canonical Flora Svelte 5 rewrite target (SvelteKit/Svelte 5, per spec 002/constitution); `apps/web` is a separate imported `@coparse/web` (SolidJS/Macro-style) app and is not the Flora canonical frontend. `.github/workflows/ci.yml` runs Bun install/check/lint/unit-test/build in `apps/frontend`. Rust CI is `.github/workflows/rust.yml` (`cargo build`, `cargo test`, `cargo build --release`, toolchain `1.97.1`).
- README/old docs are partly aspirational: no `lefthook.yml`, `deny.toml`, `cargo-vet` config, or nextest config are present in this checkout. Prefer `Cargo.toml`, workflows, and code over prose.

## Commands
- Rust (workspace root): `cargo build`, `cargo test`, `cargo build --release`.
- Format/lint: `cargo fmt --all` (CI/workspace standard); check with `cargo fmt --all -- --check`. `cargo clippy --workspace --all-targets -- -D warnings` (workspace lints are very strict).
- Focused runs: `cargo test -p <crate>` (e.g. `flora-core`, `flora-api`, `flora-tests`), `cargo test -p <crate> <test_name_or_path_fragment>`, `cargo run -p flora-api`.
- Canonical frontend flow (`apps/frontend`): from `apps/frontend` run `bun install`, `bun run check`, `bun run lint`, `bun run test:unit -- --run`, `bun run build` (see `apps/frontend/package.json` and `.github/workflows/ci.yml`). `apps/web` (`@coparse/web`) has its own scripts/toolchain; do not treat it as the Svelte 5 rewrite target.
- `test-flora.sh` is a manual local smoke script against `http://localhost:3000` using fixed IDs from `dev-seed.sql` and `X-Organization-ID` / `X-User-ID` headers; it assumes a running, seeded API.

## Config, environment, and runtime quirks
- Copy `.env.example` to `.env` for local config. `flora_core::config::Config::load()` merges defaults + env vars prefixed `FLORA_` with `__` as nesting separator (e.g. `FLORA_DATABASE__POSTGRES_URL`, `FLORA_MESSAGING__VALKEY_URL`, `FLORA_OIDC__REDIRECT_URI`).
- Startup validates config, connects Postgres, and applies migrations (`sqlx::migrate!("./src/migrations/sql")` from `flora-core/src/migrations/mod.rs`). Missing DB/migration prerequisites will break API startup/tests that hit DB.
- `FLORA_ENV`, host/port/base URL, JWT/session settings, OIDC/Zitadel, optional Meilisearch, storage path, and OTEL vars are defined in `.env.example`.

## Testing and code constraints
- Tests live in crate-local `tests/`/`#[cfg(test)]` modules and in `flora-tests/src/{contract,integration,...}`. There is no checked-in snapshot-test workflow implied by the tree; do not invent DB snapshot requirements.
- Workspace lints deny `unsafe_code`, `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `dbg_macro`, `print_stdout` is warned, and most clippy groups are denied. Prefer `Result`/`?`, `tracing`, and explicit error handling; if `#[allow(...)]` is unavoidable, include a `reason` (and satisfy `allow_attributes` rules).
- Keep changes crate-local when possible: API wiring in `flora-api`, domain/data/config/migrations in `flora-core`, messaging concerns in `flora-messaging`, etc.

## Existing instruction sources
- Issue tracking conventions: `docs/agents/issue-tracker.md` (GitHub + `gh`).
- Domain layout conventions: `docs/agents/domain.md` (multi-context; mentions `CONTEXT-MAP.md`/`CONTEXT.md` patterns when present). If `CONTEXT-MAP.md` or package `CONTEXT.md` files exist in scope, consult them before architectural changes.
- Ignore generated/noisy dirs (`target/`, `node_modules/`, `.clean-room/`, inactive tooling dirs) unless the task targets them.
