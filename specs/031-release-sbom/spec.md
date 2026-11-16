# Feature Specification: 031 — Seed release readiness and integrity closure

**Branch**: `feat/031-release-sbom`  
**Status**: In progress  
**Assessment**: `.specify/assessments/031-next-vertical-slice/decision.md`

## Goal
Make the verified Seed v0.1 releasable (draft-first tag workflow with CycloneDX SBOM) without cutting a tag, and make governance docs literally true.

## Scope
- Centralize workspace version: root dotted `package.version = "0.1.0"` + `version.workspace = true` in the root package and 7 members (8 files / 9 lines). No Cargo.lock change (versions identical).
- Add `.github/workflows/release.yml`: tag-only trigger (`v*`), exact `vX.Y.Z` shape assert, lockstep version assert, `cargo-cyclonedx 0.5.10` + `cyclonedx-cli 0.33.1`, v1_5 schema validation, draft-first immutable release, asset upload, verify. **No tag creation, no publish.**
- Close residual drift: ADR-0001 (455 exemptions, e2e resolved), AGENTS (e2e job), OBJETIVO/ROADMAP (stale e2e/typos), 030 validation wording and research banners.

## Non-goals
Creating/pushing a tag, publishing a release, enabling immutable releases, SBOM dry run, CORS allowlist, cargo-vet blocking, load/restore, OTLP.

## NV
Release workflow never executed (no tag); immutability/GH runtime state unverified; `--target` and shipped-arm64 match untested; 13 informational vet labels.
