# Validation: 026 — Executable Seed verification gate

**Branch:** `feat/026-seed-ci-gate` (from `github/dev@d09d39c1`)

## Evidence executed
- `cargo metadata --offline` + `cargo-deny --metadata-path ... check licenses bans sources`: **bans ok, licenses ok, sources ok** (exit 0; only non-fatal `license-not-encountered` warnings). Before the fix: `licenses FAILED` (exit 4) because crates declare `AGPL-3.0-or-later`.
- `cargo clippy --workspace --lib --bins -- -D warnings`: clean (production targets).
- `cargo check --workspace`: clean after removing phantom OTel/openidconnect declarations.
- `cargo fmt --all -- --check`: OK.
- Unit + six DB-gated suites: recorded from the merged 023/024 baseline; this cycle changes no Rust behavior.
- PR CI (including the three new gates): recorded after the run.

## Contract coverage
- FASE 0 policy is now machine-enforced in CI (bans/licenses/sources); governance docs no longer contradict ADR-0001; cycle 025 evidence is complete.

## NV
`--all-targets` clippy on CI (parallel-agent test-only `expect_used`); cargo-vet gitignored store + exemptions; advisories job (RustSec network); SBOM; e2e/Tauri; 2vCPU/4GB.
