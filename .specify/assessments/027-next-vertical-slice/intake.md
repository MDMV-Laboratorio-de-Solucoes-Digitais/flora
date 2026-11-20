# Intake: 027 — Honest Seed closure

**Date:** 2026-11-16  
**Baseline:** `github/dev@cda98041` (cycles 023–026 merged; local dev additionally carries parallel work, preserved).

## Inputs
- R1 (6 subagents): exact PGMQ/Valkey retirement plan, cargo-vet gate feasibility, e2e JWT bootstrap, observability/Tauri closure, advisory/SBOM alternatives, post-026 acceptance audit — `docs/research/2026-11-16-027-*.md`.
- ADR-0001, constitution v1.3.0, cycles 023–026 artifacts.

## Candidates
1. **S1 — Honest Seed closure (recommended):** retire unwired PGMQ/Valkey notification scaffolding (crate + config only; keep core service, dedup path and Valkey realtime profile), add zero-cost `TraceLayer`, and close the remaining doc drift (ADR/README/roadmap/specs). Low risk, behavior-preserving, fully verifiable.
2. S2 — cargo-vet new-dependency gate (unignore store + multer exemption + 3 imports; network-sensitive).
3. S3 — Browser e2e with fresh JWT (feature-gated mint bin + Playwright CI).
4. S4 — Advisory job (Dependabot/OSV) + release SBOM.

## Constraints
No new mandatory services; no reverting parallel work; CI green; explicit staging; only `dev`/`main`.
