# Validation: 025 — Seed reconciliation and FASE 0 basics

**Branch:** `feat/025-seed-fase0-verification` (from `github/dev`)

## Evidence
- `LICENSE` is the canonical 661-line AGPL-3.0 text including the appendix (source: gnu.org); `NOTICE` present.
- `cargo-deny check bans sources` (direct binary, local config): passes with no findings.
- `cargo-deny check licenses` could not be verified locally: this environment cannot reach the crates.io index
  (HTTP 403); full license/advisory enforcement in CI is therefore NV in this cycle.
- No Rust source changes in this cycle; the existing fmt/clippy/unit/DB gates from cycles 023/024 remain the baseline.
- PR #231: all 4 `build` + 2 `db-tests` checks green (GitHub run IDs 43870100343, 43870100410, 43870107684, 43870107773); merged to `dev` as commit `d09d39c1`.

## NV
cargo-deny CI gate, cargo-vet 442 exemptions, OTel honest wiring, notification transport decision, browser e2e, 2vCPU/4GB validation.
