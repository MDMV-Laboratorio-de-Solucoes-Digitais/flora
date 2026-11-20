# Release: 025 — Seed reconciliation and FASE 0 basics

**Branch/PR:** `feat/025-seed-fase0-verification` -> `dev`, PR #231 (merged as `d09d39c1`)  
**Baseline:** `github/dev` after cycle 024 (`bf1766cb`)

## Delivered
- ADR-0001 (authority order, mechanism substitutions, Seed verification gate, drift rule) and README milestone/FASE labels rewritten to capability names.
- Canonical full AGPL-3.0 LICENSE + NOTICE; minimal `deny.toml` dependency policy.
- Cycle 024 finalization and OBJETIVO/ROADMAP updated for 026.

## Evidence
Local verification green; PR #231 all required checks green (runs 43870100343 / 43870107684); merged at `d09d39c1`.

## NV
cargo-deny CI enforcement (crates.io access required), cargo-vet exemptions, OTel, notification transport, browser e2e, load/restore.
