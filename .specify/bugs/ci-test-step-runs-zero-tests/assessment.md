# Bug Assessment: CI build/test steps run only the root package

- **Slug**: ci-test-step-runs-zero-tests
- **Created**: 2026-11-16
- **Verdict**: valid
- **Severity**: high (verification gate is a no-op)

## Report

`.github/workflows/rust.yml` runs `cargo build` (line 33) and `cargo test` (line 35) without package-selection flags. The repository root is a non-virtual workspace: `Cargo.toml:7-8` defines `[package] name = "flora"` and `Cargo.toml:17` defines `[workspace]`, with no `workspace.default-members`.

## Symptom

**Observed:** `cargo test` at the workspace root prints `running 0 tests` for `src/main.rs`, the placeholder root crate. The seven member crates are never tested by the CI `build` job. **Expected:** the job exercises the workspace, as the adjacent strict clippy step already does with `--workspace --all-targets`.

## Reproduction

```text
$ cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["workspace_default_members"])'
['path+file:///home/luis/development/MDMV/projetos/Arquivados/flora#0.1.0']

$ cargo test
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored

$ cargo test --workspace
104 passed; 0 failed; 43 ignored; 0 measured; 0 filtered out
```

## Root cause

Cargo's default-member rule for a non-virtual workspace selects the root package when `default-members` is absent. The root package depends only on `axum`; its `src/main.rs` is a placeholder. The build and test steps were written with bare commands and never updated when the workspace grew.

## Evidence (primary)

- Official Cargo docs: a non-virtual workspace with a root package includes only the root crate by default; `--workspace` means "Test all members in the workspace"; `--all-targets` means `--lib --bins --tests --benches --examples` and excludes doc tests. See `docs/research/2026-11-16-036-cargo-docs-independent.md`.
- Local audit: `docs/research/2026-11-16-036-ci-change-risk.md` (10 test binaries, 104 passed, 43 `#[ignore]` stay ignored, no external services required).
- ADR-0001 §3 requires executed evidence and prohibits self-certification.

## Proposed remediation

Change two lines in `.github/workflows/rust.yml`: `cargo build` → `cargo build --workspace` (line 33) and `cargo test` → `cargo test --workspace` (line 35). Do not add `--all-targets` to the test step: there are no benches or examples, and it would silently exclude doc tests.

## Non-goals

- Do not change `db-tests`: those suites are `#[ignore]`d by design and run in their own job with Postgres 18.
- Do not fix `.github/workflows/docker-staging.yml` in this bug: deploying is outside the authorization boundary.
- Do not bump the Rust toolchain here: tracked as a separate follow-up in `fix.md`.

## Risks & considerations

- CI compile time grows by roughly one minute as all members are compiled and tested.
- Future DB tests must remain `#[ignore]`d so the default workspace run stays service-free.

## Open questions

None for the proposed change. The Rust toolchain bump requires a separate decision.
