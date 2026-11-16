# Feature Specification: 027 — Honest Seed closure

**Branch**: `feat/027-honest-seed-closure`  
**Status**: Completed (PR #234)  
**Assessment**: `.specify/assessments/027-next-vertical-slice/decision.md`

## Goal
Make the codebase advertise only what actually runs: retire the unwired PGMQ/Valkey notification adapter, add zero-cost request tracing, and close the remaining governance/documentation drift.

## Scope
- Remove `flora-notifications` from workspace members/dependencies and delete the crate; remove `NotificationsConfig` and its `config/mod.rs` wiring; regenerate `Cargo.lock` (exactly one stanza removed).
- Attach `TraceLayer::new_for_http()` to the API router (tower-http `trace` feature already enabled; no exporter/collector).
- Reconcile ADR-0001, README, ROADMAP/OBJETIVO constitution version, specs/001 Valkey claims, specs/026 status, AGENTS crate list and dev-smoke realtime note.
- Acceptance: zero `pgmq|XADD|flora_notifications` hits in `.rs`/`.toml`; core notification service, dedup path and Valkey realtime profile untouched; all gates green.

## Non-goals
cargo-vet gate, advisories/SBOM, browser e2e, Tauri, OTLP exporter, new services, reverting parallel work.

## NV
cargo-vet store/imports; RustSec advisories; e2e (fresh JWT needed); 2vCPU/4GB validation; strict `--all-targets` clippy (parallel test code).
