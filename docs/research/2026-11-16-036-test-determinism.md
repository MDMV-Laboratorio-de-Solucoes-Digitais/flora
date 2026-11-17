# Local verification: `cargo test --workspace` determinism, env independence, `#[ignore]`, and filesystem writes

- **Date:** 2026-11-16
- **Method:** local execution in this checkout (no edits to code/config); static inspection of test setup. Work was research-only.
- **Scope:** (1) repeatability of `cargo test --workspace` under a clean env; (2) whether ignored tests run; (3) whether any test writes outside `target/`.

---

## Environment

- Toolchain actually used: `rustc 1.99.1` / `cargo 1.99.1` (local). Note: `Cargo.toml` declares `rust-version = 1.98` / `package.rust-version = "1.98"`; this run used a newer, semver-compatible toolchain.
- Clean env for both primary runs: `env -i PATH="$PATH" HOME="$HOME" TERM=xterm cargo test --workspace`. This removes all `FLORA_*`, `DATABASE_URL`, `STORAGE_*`, `MEILISEARCH_*`, `VALKEY_*`, `ZITADEL_*`, `SQLITE_PATH`, `S3_*`, `RUSTFLAGS`, etc. No `.cargo/config.toml` exists locally that could inject flags.
- Pre-run check: no `FLORA_*` config vars and no `DATABASE_URL` were set in the ambient shell either.

## 1. Determinism and env independence — PASS

Two independent clean-env runs both exited `0` and produced **byte-identical** per-test lines (`diff` of sorted `test … ... ok` lines: no differences). Aggregate counts, identical in both runs:

| Binary | Passed | Failed | Ignored |
|---|---:|---:|---:|
| `flora` (root bin) | 0 | 0 | 0 |
| `flora_api` (lib) | 25 | 0 | 0 |
| `flora_api` (bin) | 0 | 0 | 0 |
| `flora_core` (lib) | 12 | 0 | 0 |
| `flora_core` (`tests/models.rs`) | 21 | 0 | 0 |
| `flora_files` / `flora_messaging` / `flora_search` / `flora_tasks` (lib) | 0 | 0 | 0 |
| `flora_tests` (lib) | 46 | 0 | 43 |
| Doc-tests (all 7 crates) | 0 | 0 | 1 (`flora_api` CORS example) |
| **Total** | **104** | **0** | **44** |

Selection is exactly the 7 workspace members plus the root package; `apps/web/tauri` is not a member and did not run. Because the runs used a fully emptied environment and still passed with stable counts, the suite does not depend on stray env vars in this checkout. Configuration constructors use `unwrap_or`/`unwrap_or_default` fallbacks (`flora-core/src/config/{app,database,storage,messaging,search,oidc}.rs`), and `Config::load` merges `Figment` with `Env::prefixed("FLORA_")` optional input.

## 2. No `#[ignore]` test ran — PASS

- `cargo test --workspace` was invoked without `--ignored` / `--include-ignored`; no local `.cargo/config.toml` adds them.
- Reported ignored counts: 43 in `flora_tests` (matching the 43 `#[ignore = "requires Postgres; …"]` attributes, all in `flora-tests/src/contract/*.rs`) and 1 ignored doctest in `flora-api/src/middleware/cors.rs`. Zero ignored tests reported as executed in any run.
- CI's DB-gated suites are separate explicit invocations (`cargo test -p flora-tests contract::<area>::db_ -- --ignored --test-threads=1` in `.github/workflows/rust.yml`) and require a live Postgres; they are outside the plain `--workspace` command.

## 3. No test writes outside `target/` — PASS (with one ignored-only caveat)

Empirical: a third clean-env run was bracketed by snapshots of `/tmp` and of the repo tree excluding `target/` and `.git/`. Diffs after the run were empty (apart from the harness's own `/tmp/opencode` log files). No file was created or removed outside `target/`.

Static inspection:

- `tempfile` is declared in the workspace (`Cargo.toml`: `tempfile = "3.14"`) and in `flora-files/Cargo.toml` (`tempfile.workspace = true`), but **no Rust source uses `tempfile`, `TempDir`, or `NamedTempFile`** anywhere in the workspace. No test uses `std::env::set_var` or `std::env::temp_dir()`.
- The only test-owned absolute-path write is `config.storage.local_path = format!("/tmp/flora-test-storage-{uniq}")` in `flora-tests/src/contract/files.rs:154`, inside `db_state()`. Every caller of `db_state()` (6 tests at lines 254, 326, 369, 434, 483, 556) carries `#[ignore = "requires Postgres; …"]`, so it cannot execute under `cargo test --workspace` without `--ignored`. The `apps/web/tauri/**` `/tmp` writes are in a non-workspace crate and are not exercised here.
- Filesystem write implementations (`flora-core/src/storage/local.rs`, `flora-files/src/lib.rs`) are production code with no `#[cfg(test)]` modules driving them against real paths; `flora-core/tests/models/file.rs` only compares an in-memory `storage_path` string.

## Conclusion

In this checkout, `cargo test --workspace` under a clean environment is deterministic (104 passed / 0 failed / 44 ignored in both runs, identical per-test lines), does not depend on stray `FLORA_*`/`DATABASE_URL` env vars, does not execute any ignored test, and creates no files outside `target/`. The sole `/tmp` write path is reachable only from Postgres-gated `#[ignore]`d tests run explicitly in CI.

## Caveats

- Results reflect the local toolchain (1.99.1), not the pinned 1.98.0; CI pins 1.98.0.
- "Does not depend on stray env vars" is demonstrated for the clean-vs-clean comparison; a deliberately poisoned-env run was not performed.
