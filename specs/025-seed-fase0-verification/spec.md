# Feature Specification: 025 — Seed v0.1 reconciliation and FASE 0 basics

**Branch**: `feat/025-seed-fase0-verification`  
**Status**: In progress

## Goal
Make the Seed milestone statement honest and complete the cheap part of FASE 0.

## Scope
- ADR-0001: authority order, mechanism substitutions (PG FTS / local FS / in-app Postgres notifications), verification gate and drift rule.
- README Seed/FASE labels rewritten to capability names with authoritative backend; checkmarks only where CI evidence exists.
- Canonical full AGPL-3.0 LICENSE text (including the "How to Apply" appendix) and a NOTICE file.
- Minimal `deny.toml` dependency policy (`cargo deny check bans sources` verified locally; full license/advisory enforcement requires crates.io access in CI and is NV here).
- Finalize cycle 024 evidence and update planning for 026.

## Non-goals
CI enforcement of cargo-deny, cargo-vet exemption resolution, OTel wiring, notification transport changes, new services, frontend work.
