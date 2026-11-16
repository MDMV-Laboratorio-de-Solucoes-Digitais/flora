# Release: 029 — Seed integrity and executable evidence

**Branch/PR:** `feat/029-seed-integrity` -> `dev`, PR #238 (merged as `b8972c0e`)  
**Baseline:** `github/dev@246a18a`

## Delivered
- Strict clippy now runs with `--all-targets` in CI (test-only `expect_used` scoped, no reversions).
- ADR-0002 adopted (Tauri deferral, Accepted); ADR-0001 consequences updated; 028 doc drift closed.
- Integrated dev smoke executed end-to-end with a fresh JWT on isolated PostgreSQL 18/alt ports: **rc=0**.

## Evidence
See `.specify/validations/029-seed-integrity/validate.md` for commands, run evidence and cleanup record. PR #238 all checks green; merged at `b8972c0e`.

## NV / follow-ups (030)
Browser e2e with minted token (`retries: 0` due to jti rotation), release/tag process + CycloneDX SBOM,
cargo-vet blocking + CODEOWNERS/branch protection, OTLP exporter, 2vCPU/4GB validation.
