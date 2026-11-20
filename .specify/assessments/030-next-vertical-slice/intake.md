# Intake: 030 — Browser e2e smoke and Seed doc integrity

**Date:** 2026-11-16  
**Baseline:** `github/dev@0bef919` (cycles 023–029 merged).

## Inputs
- R1 (6 subagents): `docs/research/2026-11-16-030-{e2e-exact-edits,frontend-e2e-selectors,release-sbom-exact-plan,cors-allowlist-plan,vps-load-restore-plan}.md` + post-029 audit.
- Audit: strict clippy/ADR-0002/smoke rc=0 VERIFIED; stale clippy prose and incomplete smoke evidence artifact remain.

## Candidates
1. **S1 — One Chromium smoke + doc integrity (recommended):** feature-gated `flora-mint-session` bin, Playwright setup + one `retries:0` smoke, non-required CI job; fix stale clippy prose and complete the smoke evidence artifact.
2. S2 — Tag/release + SBOM workflow (design ready; critical dotted `package.*` finding).
3. S3 — CORS allowlist (ADR-0002 re-entry #2).
4. S4 — 2vCPU/4GB load/restore execution (needs second host + backup target).

## Constraints
No new services; no reverting parallel work; CI green on required checks; explicit staging; only `dev`/`main`.
