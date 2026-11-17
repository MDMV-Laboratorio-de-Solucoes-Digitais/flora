# Bug Fix: CI build/test steps run only the root package

- **Slug**: ci-test-step-runs-zero-tests
- **Fixed**: 2026-11-16
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Added explicit workspace package selection to the two no-op steps in the Rust CI `build` job. The strict clippy step already used `--workspace --all-targets`; build and test now cover the same set of members.

## Changes

| File | Line | Change | Notes |
|---|---|---|---|
| `.github/workflows/rust.yml` | 33 | `cargo build` → `cargo build --workspace` | Same root cause as the test step; both compiled only the root placeholder |
| `.github/workflows/rust.yml` | 35 | `cargo test` → `cargo test --workspace` | Main fix requested by cycle 036 |

No Rust source, Cargo manifest, lockfile, or other workflow was changed.

## Local verification (fresh, 2026-11-16)

| Check | Command | Result |
|---|---|---|
| Workflow syntax | `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/rust.yml'))"` | `yaml-ok` |
| Formatting | `cargo fmt --all -- --check` | exit 0 |
| Workspace build | `cargo build --workspace` | `Finished dev profile`; 18 crates compiled |
| Workspace tests | `cargo test --workspace` | **104 passed, 0 failed, 43 ignored** (+1 doctest ignored) |
| Step text | `grep -n -A1 -E 'name: (Build|Test)' .github/workflows/rust.yml` | lines 33 and 35 show `--workspace` |

`#[ignore]` tests did not run, as confirmed by the official Cargo semantics documented in `docs/research/2026-11-16-036-cargo-docs-independent.md`. They remain in the separate `db-tests` job.

## Deviations from assessment

- The assessment proposed only the test step. The build step on line 33 had the identical default-member defect and was fixed in the same change. The audit is in `docs/research/2026-11-16-036-ci-change-risk.md`.
- `--all-targets` was deliberately NOT added to the test step: the workspace has no benches or examples, and the flag excludes doc tests.

## Follow-ups (not applied here)

- **Rust pin**: CI pins exactly `1.98.0` in nine places in `rust.yml` and three in `release.yml`; 1.98.1 fixed a trait-object vtable UB (current stable 1.100.0). Recommend a patch bump to `1.98.1` in a separate bug. `Dockerfile` needs no change: `rust:1.98-slim` already resolves to 1.98.1. Evidence: `docs/research/2026-11-16-036-rust-pin-bump.md` and `-docker-tag-check.md`.
- **DB filter legs**: each `cargo test -p flora-tests contract::…::db_` command in the `db-tests` job exits 0 when the filter matches zero tests. Add an assertion on the reported count in a separate bug. Evidence: `docs/research/2026-11-16-036-international-seed-verification.md`.
- **docker-staging.yml**: uses `secrets.*` in a step-level `if:` and fails every run. Not touched: fixing it can publish to GHCR and trigger the Coolify webhook, which requires explicit authorization.
