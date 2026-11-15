# Bug Fix: Forgejo Rust CI fails resolving `dtolnay/rust-toolchain` action

- **Slug**: forgejo-rust-ci-rust-toolchain-action-not-found
- **Fixed**: 2026-11-15
- **Assessment**: ./assessment.md
- **Status**: applied

## Summary

Replaced the Forgejo-mirror-dependent `dtolnay/rust-toolchain@stable` GitHub Action in `.github/workflows/rust.yml` with an explicit, portable `rustup`-based setup step that installs/selects Rust `1.98.0` and prints tool versions, removing the action-resolution failure observed on Forgejo while preserving the pinned toolchain and GitHub Actions compatibility intent.

## Changes

| File | Change | Notes |
|------|--------|-------|
| `.github/workflows/rust.yml` | modified | Replaced `uses: dtolnay/rust-toolchain@stable` with a `run` step: installs rustup if missing, installs/defaults toolchain `1.98.0` (minimal profile), and outputs `rustc`/`cargo` versions. Kept `actions/checkout@v4`. |

## Diff Highlights (optional)

- Before: action-based setup (`uses: dtolnay/rust-toolchain@stable`, `with: toolchain: '1.98.0'`).
- After: shell-based setup with `rustup toolchain install 1.98.0 --profile minimal --no-self-update` and `rustup default 1.98.0`, plus conditional rustup bootstrap via official installer if `rustup` is absent.

## Tests Added or Updated

- None (CI workflow fix; no repository unit/integration test harness for Forgejo runner action resolution).

## Local Verification

- Commands run: `python3 -c "import yaml,sys; yaml.safe_load(open('.github/workflows/rust.yml')); print('yaml-ok')"` → `yaml-ok`.
- Manual checks: reviewed workflow structure (checkout retained; build/test steps unchanged; pinned toolchain preserved). Cross-environment CI execution on Forgejo/GitHub not run from this local session.

## Deviations from Assessment

- Assessment suggested either rustup-based setup or fully qualified action URL/alternative action. Implemented the **rustup-based portable path** (preferred in assessment) to avoid dependence on third-party action mirrors/URLs in Forgejo runner environments.
- `actions/checkout@v4` left unchanged because the reported failure was specific to rust-toolchain resolution; checkout fetch appeared in runner logs. If checkout mirror issues arise, a follow-up workflow adjustment may be needed.

## Follow-ups

- Re-run Forgejo Actions `rust.yml` on `dev` (or trigger via PR) to confirm job setup, `cargo build`, and `cargo test` pass on the Forgejo runner.
- Monitor GitHub Actions `Rust CI` for parity/regressions with the new setup step.
- Consider pinning/qualifying action sources (including checkout) consistently if this Forgejo instance restricts mirrors or external fetches.
- Background research (subagent) may provide additional Forgejo-specific best practices; incorporate if they suggest a safer/clearer setup variant.
