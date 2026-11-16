# Release: 027 — Honest Seed closure

**Branch/PR:** `feat/027-honest-seed-closure` -> `dev` (pending)  
**Baseline:** `github/dev@cda98041`

## Delivered
- Removed the unwired `flora-notifications` crate and `NotificationsConfig` (Cargo.lock: one stanza). Kept the core notification service, Postgres dedup path and Valkey realtime profile.
- Attached zero-cost `TraceLayer` to the API router.
- Reconciled ADR-0001, README, ROADMAP/OBJETIVO, specs/001, specs/026, AGENTS and dev-smoke with ADR-0001 and cycles 026–027 reality.
- Six R1 research docs recorded.

## Evidence
Local verification in `validate.md`; PR CI result appended after green.

## NV / follow-ups (028)
cargo-vet new-dependency gate, RustSec advisories/SBOM, browser e2e with fresh JWT, Tauri decision, OTLP exporter, 2vCPU/4GB validation.
