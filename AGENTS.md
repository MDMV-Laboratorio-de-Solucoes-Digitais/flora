# AGENTS.md

## Repo reality (read first)
- Archived but self-contained Rust modular-monolith workspace (`resolver = "3"`, Rust `1.98`, edition `2024`, AGPL-3.0-or-later). Members: `flora-core`, `flora-api`, `flora-messaging`, `flora-tasks`, `flora-files`, `flora-search`, `flora-tests`. Root `src/main.rs` is a placeholder.
- Real API entrypoint is `flora-api` (`flora-api/src/main.rs`, bin `flora-api`, lib `flora_api`): loads config, connects Postgres, runs migrations, builds `AppState`, serves an Axum router. Route modules live in `flora-api/src/routes/*`.
- `flora-core` owns shared config (`src/config/*`), DB pool/migrations (`src/database.rs`, `src/migrations/mod.rs`), models, repositories, services, traits, utils, storage. Keep changes crate-local: API wiring in `flora-api`, domain/data/config/migrations in `flora-core`, etc.
- Frontend under `apps/`: `apps/frontend` is the canonical (and only) Svelte 5 / SvelteKit target. `apps/web` (imported `@coparse/web` SolidJS/Macro app) was **removed in cycle 040**; it was never the target. CI: `.github/workflows/ci.yml` runs in `apps/frontend`; `.github/workflows/rust.yml` runs fmt check, strict clippy `--all-targets`, `cargo deny check bans licenses sources`, build/test, six DB-gated suites (Postgres 18), plus informational cargo-vet, dependency-review, weekly OSV scan and one Chromium smoke. Toolchain pinned `1.98.0`.
- README/old docs are partly aspirational: there is **no `lefthook.yml` or nextest config** in this checkout. Trust `Cargo.toml`, workflows and code over prose. ADR-0001 (`docs/adr/0001-seed-v01-mechanism-substitutions.md`) is authoritative for Seed mechanisms; ADR-0002 defers Tauri.

## Commands
- Rust (workspace root): `cargo build`, `cargo test`, `cargo build --release`.
- Gate sequence mirrors CI: `cargo fmt --all -- --check` → `cargo clippy --workspace --all-targets -- -D warnings` → `cargo deny check bans licenses sources` → `cargo build` → `cargo test`.
- Focused runs: `cargo test -p <crate>` (e.g. `flora-core`, `flora-api`, `flora-tests`), `cargo test -p <crate> <test_name_or_path_fragment>`, `cargo run -p flora-api`.
- DB-backed contract tests are `#[ignore]`d and need a live Postgres; run individually with `cargo test -p flora-tests contract::<area>::db_ -- --ignored --test-threads=1` (areas: `auth`, `messaging`, `websocket`, `search`, `notifications`, `files`).
- Frontend (`apps/frontend`): `bun install`, `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build`. (`apps/web` removal complete — no separate toolchain remains.)
- `test-flora.sh`: manual smoke against `http://localhost:3000` using fixed IDs from `dev-seed.sql` and a Bearer session token; assumes a running, seeded API. Org context comes from the verified session, not request headers.

## Environment / setup quirks
- Config is **only** `FLORA_*` env vars via Figment (`Env::prefixed("FLORA_").split("__")`); no `.env` is auto-loaded despite the config module's doc comment. Export vars into the process environment (shell/systemd/docker `env_file`) — copying `.env.example` to `.env` alone does nothing.
- CORS uses Figment array syntax: `FLORA_APP__CORS_ORIGINS=["http://localhost:5173","http://127.0.0.1:5173",...]`. `localhost` and `127.0.0.1` are distinct origins and must both be listed; never `*` or `null`. Frontend runtime config (`window.__FLORA_CONFIG__` from `PUBLIC_*`) is injected before the bundle and overrides build-time `VITE_API_URL`.
- PostgreSQL **18+** is required: the init migration uses native `uuidv7()`. `docker compose up -d` gives the lean Seed (postgres only); `--profile realtime` / `--profile search` add Valkey / Meilisearch. Load seed data with `PGPASSWORD=postgres psql -h 127.0.0.1 -U postgres -d flora -f dev-seed.sql`.
- Optional capabilities are gated by matching `FLORA_MESSAGING__ENABLED` / `FLORA_SEARCH__ENABLED` flags. Postgres FTS is always authoritative for `/api/v1/search`; Meilisearch is dormant/non-authoritative. Missing DB/migration prerequisites break API startup and DB tests.

## CodeGraph & architecture diagrams
- `.codegraph/` is a local, gitignored index (`codegraph.db`). Refresh it before relying on `codegraph query/explore/impact/affected`: `codegraph sync` for incremental changes, `codegraph index` for a full rebuild; if `.codegraph/` is absent run `codegraph init`. Stale indexes give wrong answers.
- Architecture/flow diagrams are managed with the `/archify` skill under `.archify/<type>-<slug>-<timestamp>/` (e.g. `flora-workspace.html`). Update the existing graph or create a relevant one via `/archify` rather than hand-editing generated HTML.

## Testing & code constraints
- Tests live in crate-local `tests/`/`#[cfg(test)]` modules and `flora-tests/src/{contract,integration,...}`. No checked-in snapshot workflow; don't invent DB snapshot requirements.
- Workspace lints deny `unsafe_code`, `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `dbg_macro`, most clippy groups; `print_stdout` is warned. Prefer `Result`/`?`, `tracing`, explicit errors. `#[allow(...)]` is denied — use `#[expect(..., reason = "...")]` (test-only `expect_used` is scoped at module level this way).

## Existing instruction sources
- Issues + specs live in **GitHub** issues via `gh` (`docs/agents/issue-tracker.md`); remotes: `github` = GitHub issue repo, `origin` = Forgejo mirror. `gh` resolves the repo from the `github` remote.
- `docs/agents/domain.md` describes a `CONTEXT-MAP.md`/per-package `CONTEXT.md` convention; none are checked in — consult it before architecture changes if they appear. ADRs live in `docs/adr/`.
- Ignore generated/noisy dirs (`target/`, `node_modules/`, `.clean-room/`, `.commandcode.inactive-*/`, `.archify/`, `.codegraph/`) unless the task targets them.
