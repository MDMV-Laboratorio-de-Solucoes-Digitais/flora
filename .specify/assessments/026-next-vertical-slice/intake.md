# Intake: 026 — Seed verification gate and governance drift

**Date:** 2026-11-16  
**Baseline:** `github/dev@d09d39c1` (cycles 023–025 merged; local dev additionally carries the other agent's work, preserved).  
**Directive:** reach a verifiable Seed v0.1 via 80/20 cycles, autonomous decisions.

## Inputs
- R1 (6 background agents): `docs/research/2026-11-16-026-{cargo-deny-ci-gate,cargo-vet-exemptions,otel-honest-wiring,notification-transport-decision,browser-e2e-tauri-decision}.md` + Seed verification audit.
- ADR-0001, cycles 023–025 artifacts, `.github/workflows/`, `Cargo.toml`, `deny.toml`, constitution.

## Candidates
1. **S1 — Make the Seed gate executable + close governance drift (recommended):** fix `deny.toml` license mismatch (`AGPL-3.0-or-later`), add `fmt`/strict `clippy`/`cargo deny` CI steps, finalize cycle 025 evidence, amend constitution/README/ROADMAP/AGENTS drift, remove phantom OTel declarations.
2. S2 — Retire unwired PGMQ/Valkey scaffolding (behavior-preserving but multi-crate; 027).
3. S3 — Browser e2e thin slice (blocked by expired dev JWT; 027).
4. S4 — cargo-vet store commit + imports (P1 after cargo-deny gate; 027).

## Constraints
- No new mandatory services; deterministic gates; CI green before merge; preserve other agents' work; only `dev`/`main`.
