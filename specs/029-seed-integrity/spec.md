# Feature Specification: 029 — Seed integrity and executable evidence

**Branch**: `feat/029-seed-integrity`  
**Status**: Completed (PR #238)  
**Assessment**: `.specify/assessments/029-next-vertical-slice/decision.md`

## Goal
Close the last two integrity gaps of the Seed gate: make strict clippy executable across all targets, record the Tauri deferral formally, eliminate 028 documentation drift, and execute the integrated dev smoke with a fresh JWT on isolated infrastructure.

## Scope
- Scope test-only `expect_used` with module-level `#[expect(..., reason = "...")]` in `health.rs`/`oidc.rs`; promote CI clippy to `--all-targets`.
- Adopt ADR-0002 (Tauri deferral, Accepted) and update ADR-0001 Consequences.
- Close 028 doc drift (README, AGENTS, ADR-0001, supersession banners on research snapshots).
- Execute the integrated smoke on a disposable PostgreSQL 18 project with alt ports (`-p flora-smoke-alt`, `ports: !override`, API on 33000) and record rc=0 evidence; clean up only our project and PID.

## Non-goals
Browser e2e (030), release/tag + SBOM (030), cargo-vet blocking + CODEOWNERS, OTLP exporter, 2vCPU/4GB load/restore.

## NV
E2E browser, SBOM, 2vCPU/4GB; CORS is permissive `Any` (Tauri re-entry gate); no branch protection.
