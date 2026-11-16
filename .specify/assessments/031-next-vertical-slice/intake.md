# Intake: 031 — Seed release readiness and integrity closure

**Date:** 2026-11-16  
**Baseline:** `github/dev@856a334` (cycles 023–030 merged).

## Inputs
- R1 (6 subagents): `docs/research/2026-11-16-031-{release-sbom-readiness,cors-exact-edits,cargo-vet-promotion,observability-final,load-restore-feasibility}.md` + post-030 audit.
- Audit: e2e-browser green twice and minter gating VERIFIED; residual doc drift (ADR-0001 NV count, AGENTS job list, OBJETIVO, 030 snapshots) remains.

## Candidates
1. **S1 — Release readiness + integrity closure (recommended):** centralize workspace version (8 manifests, dotted `package.*` key), add tag-only `release.yml` with CycloneDX SBOM (file edits only, no tag), close residual doc drift.
2. S2 — CORS allowlist backend (GO, but ADR-0002 #2 not claimable; separate slice).
3. S3 — cargo-vet blocking promotion (DEFER: authorization, CODEOWNERS, observation window, 13 informational labels).
4. S4 — Load/restore execution (DEFER: host not 2vCPU/4GB, tools absent).

## Constraints
No tag creation or release publication (needs explicit authorization); no new services; CI green; explicit staging; only `dev`/`main`; no reverting parallel work.
