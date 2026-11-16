# Release: 030 — Browser e2e smoke and Seed doc integrity

**Branch/PR:** `feat/030-browser-e2e-smoke` -> `dev` (pending)  
**Baseline:** `github/dev@0bef919`

## Delivered
- Feature-gated `flora-mint-session` bin and deterministic Playwright storageState injection.
- One Chromium smoke (setup → select-org → workspace/channel → message → task), `retries: 0`, `workers: 1`, non-required CI job.
- Stale clippy prose and 029 smoke evidence repaired.
- Six R1 research docs recorded.

## Evidence
See `.specify/validations/030-browser-e2e-smoke/validate.md`. PR CI result appended after green.

## NV / follow-ups (031)
Tag/release + CycloneDX SBOM (exact plan ready), CORS allowlist, 2vCPU/4GB load/restore, cargo-vet blocking, OTLP.
