# Baseline evidence (2026-11-15, local environment)

## Toolchain
- `rustc 1.99.1`, `cargo 1.99.1`
- `bun 1.4.4`, `node v22.23.1`, `npm 10.9.8`
- Repo: `flora` workspace (Rust 1.98/edition 2024 per `Cargo.toml`, CI rust.yml uses 1.97.1)

## Rust workspace
- `cargo fmt --all -- --check` → **initially FAIL (exit 1)** on `flora-api/src/routes/auth.rs:75` (line-wrapping). **Fixed** via `cargo fmt --all`; check now passes.
- Cargo registry repair: removed corrupted `~/.cargo/registry/{src,cache}/.../cc-1.4.4*` (incomplete `target.rs` referencing missing submodules). Subsequent builds refetched successfully.
- `cargo clippy -p flora-core --all-targets -- -D warnings` → **PASS** after fixes in `flora-core` (lint expects in `storage/local.rs`, `services/permission_service.rs`; pedantic fixes in `tests/models/{user.rs,workspace.rs}`).
- `cargo clippy --workspace --all-targets -- -D warnings` → **PASS (exit 0)**.
- `cargo test --workspace` → **PASS** (background completion): **90 passed, 0 failed, 1 ignored** across 19 suites (doc-test ignored in `flora-api/src/middleware/cors.rs`). `flora-tests` contract/integration-named suites passed in-process/mocked in this run; no environment-prerequisite failures observed.

## Frontend
- **`apps/frontend`** (`frontend`, SvelteKit 2 + Svelte 5 + TS 6 + Tailwind 4 + Vite 8): canonical per spec 002/constitution.
  - `bun install` **PASS** (287 pkgs; lockfile migrated to `bun.lock`, untracked).
  - `bun run check` (`svelte-check`) **PASS** — 0 errors, 0 warnings (earlier run).
  - `bun run format` (`prettier --write .`) **executed** (reformatted files); post-format `lint`/`check` reruns timed out in session capture (pass not conclusively recorded).
  - `bun run lint` **previously FAIL** (48 Prettier files) — likely resolved by format, needs confirmed rerun.
  - `bun run test:unit -- --run` **FAIL (env)**: `bunx playwright install chromium` failed with **ENOSPC** (downloaded Chrome for Testing v1234 blocked); cache had v1248 and temporary symlink workaround led to `Failed to connect to browser session` timeout. Server test passed; browser/client project did not complete. Requires disk space + matching Playwright browser install.
  - `bun run build` **PASS**.
- **`apps/web`** (`@coparse/web`, SolidJS/Macro import): non-canonical.
- **CI (`.github/workflows/ci.yml`)**: targets `apps/frontend` (`install/check/lint/test:unit/build`). Lint/test:unit stability depends on formatting and Playwright browser availability in CI.

## PKCE (T116) implementation (completed, 2026-11-15)
- Implemented in: `flora-api/src/routes/auth.rs`, `flora-api/src/state/{app_state.rs,mod.rs,oidc_transactions.rs}` (new, Valkey-preferred + in-memory fallback, TTL 600s, single-use), `flora-core/src/{config/oidc.rs,utils/mod.rs,utils/pkce.rs (new),error.rs}`, `flora-tests/src/contract/auth.rs` (and extractor test wiring).
- Login: generates state + verifier + S256 challenge, persists transaction, returns authorize URL with `state`, `code_challenge`, `code_challenge_method=S256`, configured scopes, redirect preserved.
- Callback: requires `code`+`state`, consumes state once before token exchange/DB work, rejects missing/invalid/reused state, exchanges with stored `code_verifier` and stored redirect URI.
- Verification (reported): `cargo fmt --all -- --check`, `cargo test -p flora-core`, `cargo test -p flora-api`, `cargo test -p flora-tests`, `cargo clippy --workspace --all-targets -- -D warnings` **PASS**.
- Gaps/risks: no full mocked-provider success callback/integration test (needs token/JWKS mock + DB-backed flow); no explicit expired-state test (TTL enforced); in-memory fallback is process-local (use Redis/Valkey for multi-process/restart durability); ID token JWKS signature verification still unchanged. No commits made; spec checkboxes updated separately.

## Autonomous progress summary
- Completed: dark-factory workflow mapping, multi-source research synthesis, assessment+decision (go), spec/plan/tasks/checklist for cycle 003, baseline probes, frontend/CI canonical analysis, PKCE implementation analysis.
- Pending (next execution): fix env cargo cache to obtain real clippy/test signal; run `cargo fmt` fix if desired (single known diff) then recheck; implement PKCE T116 with tests; align CI/docs to canonical frontend and run frontend checks. No actions requiring explicit human authorization were taken.