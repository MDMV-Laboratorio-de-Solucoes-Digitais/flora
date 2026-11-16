# Intake: 028 — FASE 0 closure and Seed verification unblock

**Date:** 2026-11-16  
**Baseline:** `github/dev@3bc60ca` (cycles 023–027 merged).

## Inputs
- R1 (6 subagents): `docs/research/2026-11-16-028-{cargo-vet-gate-plan,advisory-job-plan,sbom-release-plan,e2e-jwt-implementation-plan,tauri-smoke-closure}.md` + post-027 acceptance audit.
- Audit findings: Seed gates VERIFIED except integrated smoke (expired static JWT) and residual doc drift; repo is PUBLIC with Dependabot already enabled (83 alerts, 4 on root Cargo.lock); no branch protection; no release process.

## Candidates
1. **S1 — FASE 0 closure + Seed doc unblock (recommended):** version cargo-vet store + one `multer` exemption + informational CI job; add PR dependency-review and one weekly OSV sweep; replace expired static JWT in dev-smoke with a deterministic mint procedure; finish residual ADR/README/research-snapshot drift.
2. S2 — Browser e2e with feature-gated mint bin (GO conditional; larger, non-required CI job).
3. S3 — Release SBOM (blocked: no tag/release process).
4. S4 — Tauri ADR-0002 + load/restore validation (needs executed evidence).

## Constraints
No new mandatory services; no reverting parallel work; CI green; explicit staging; only `dev`/`main`; advisory jobs must not block merges.
