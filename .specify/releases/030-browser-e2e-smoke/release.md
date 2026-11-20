# Release: 030 — Browser e2e smoke and Seed doc integrity

**Branch/PR:** `feat/030-browser-e2e-smoke` -> `dev`, PR #240 (merged as `ab3fc007`)  
**Baseline:** `github/dev@0bef919`

## Delivered
- Feature-gated `flora-mint-session` bin and deterministic Playwright storageState injection.
- One Chromium smoke (setup → select-org → workspace/channel → message → task), `retries: 0`, `workers: 1`, non-required CI job.
- Stale clippy prose and 029 smoke evidence repaired.
- Six R1 research docs recorded.

## Evidence
See `.specify/validations/030-browser-e2e-smoke/validate.md`. PR #240 all checks green including `e2e-browser` (runs 43914284405 / 43914293666); merged at `ab3fc007`.

## NV / follow-ups (031)
Tag/release + CycloneDX SBOM (exact plan ready), CORS allowlist, 2vCPU/4GB load/restore, cargo-vet blocking, OTLP.
