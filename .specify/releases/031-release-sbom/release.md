# Release: 031 — Seed release readiness and integrity closure

**Branch/PR:** `feat/031-release-sbom` -> `dev`, PR #242 (merged as `aaa45536`)  
**Baseline:** `github/dev@856a334`

## Delivered
- Centralized workspace version (8 manifests / 9 lines, dotted `package.*` key).
- Tag-only `release.yml` with CycloneDX SBOM, draft-first immutable release flow (file edits only; no tag/publish).
- Residual doc drift closed (ADR-0001, AGENTS, OBJETIVO, ROADMAP, 030 artifacts/snapshot banners).
- Six R1 research docs recorded.

## Evidence
See `.specify/validations/031-release-sbom/validate.md`. PR #242 all checks green (runs 43952410204 / 43952419180); merged at `aaa45536`.

## NV / follow-ups (032)
CORS allowlist, cargo-vet blocking + CODEOWNERS, load/restore, OTLP; actual tag/release (explicit authorization required), SBOM dry run.
