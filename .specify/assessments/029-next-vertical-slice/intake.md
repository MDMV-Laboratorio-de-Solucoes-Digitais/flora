# Intake: 029 — Seed integrity and executable evidence

**Date:** 2026-11-16  
**Baseline:** `github/dev@246a18a` (cycles 023–028 merged).

## Inputs
- R1 (6 subagents): `docs/research/2026-11-16-029-{e2e-slice-validation,release-process-minimal,tauri-adr-adoption,smoke-alt-port-runbook,strict-clippy-metrics-closure}.md` + post-028 acceptance audit.
- Audit: CI + six DB suites + artifacts VERIFIED; remaining: strict `--all-targets` clippy NV, formal Tauri ADR, 028-introduced doc drift, end-to-end smoke not executed.

## Candidates
1. **S1 — Seed integrity closure (recommended):** allow test-only `expect_used` at module level and promote CI clippy to `--all-targets`; adopt ADR-0002 Tauri deferral; close 028 doc drift; attempt the integrated smoke on isolated ports and record evidence (NV if blocked).
2. S2 — Browser e2e slice (GO conditional; needs `retries: 0` due to jti rotation; larger CI job).
3. S3 — Release/tag process + SBOM (design ready, separate milestone concern).
4. S4 — cargo-vet blocking + CODEOWNERS + branch protection (needs observation window).

## Constraints
No new services; no reverting parallel work; CI green; explicit staging; only `dev`/`main`.
