# Validation: 031 — Seed release readiness and integrity closure

**Branch:** `feat/031-release-sbom` (from `github/dev@856a334`)

## Evidence executed
- `cargo metadata --no-deps`: exactly 8 packages, all `0.1.0`, lockstep after the dotted-key change.
- `cargo check --workspace`: clean; `Cargo.lock` unchanged (versions identical).
- `.github/workflows/release.yml`: parsed as valid YAML; 1 job / 15 steps; tag-only trigger. **No tag exists and none was pushed.**
- Dotted-key finding re-confirmed: adding `[workspace.package]` to the root would produce `duplicate key` (scratch reproduction in research doc).
- No Rust behavior change: the merged 023–030 fmt/clippy/deny/vet/DB/e2e gates remain the baseline.
PR #242: all checks green (4 build, 2 db-tests, dependency-review, 2 supply-chain-vet, 2 e2e-browser; GitHub runs 43952410204 / 43952419180); merged as `aaa45536`.

## Contract coverage
- Governance docs now state the truth: ADR-0001 exemption count 455 and browser e2e resolved; AGENTS lists the e2e job; OBJETIVO/ROADMAP no longer list it as future work; 030 snapshots carry supersession banners.

## NV
Workflow never executed; immutable-release setting, tag push, generator dry run, `--target` match, SBOM `metadata.component` shape; CORS, cargo-vet blocking, load/restore, OTLP.
