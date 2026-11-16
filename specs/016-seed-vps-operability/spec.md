# Feature Specification: 016 — Seed/VPS Operability Hardening (80/20)

**Feature Branch**: `feat/016-seed-vps-operability`  
**Created**: 2026-11-16  
**Status**: In progress  
**Assessment**: `.specify/assessments/016-next-vertical-slice/{intake,research,problem,concept,decision}.md` (verdict: go, slice C)

## Goal
Make the Seed runtime truthful and operable on a cheap single-node VPS (~2 vCPU/4 GB): Postgres as the only mandatory dependency, optional Meilisearch/Valkey profile-gated with degraded behavior, single init/pool path with config-driven DB pool, compose/env alignment for lean profiles, and minimal backup/restore guidance.

## Scope
- Single pool/init path; inject/reuse pool in `AppState`; wire `DatabaseConfig` into pool creation with conservative defaults.
- Search optionality: gate on `config.search.enabled`, represent as optional in state, handle unavailable search in routes/boot without hard-fail in lean profile.
- Messaging optionality alignment: explicit enabled/optional semantics consistent with degraded runtime behavior.
- Health/readiness improvements (DB-aware readiness, capability visibility as feasible).
- Compose profiles/limits/healthchecks guidance for `core`/`realtime`/`search`, localhost binds, persistent volumes, conservative optional-service settings.
- Canonical `FLORA_*__*` env alignment in `.env.example`/docs; truthful notes for unsupported runtime paths where applicable.
- Minimal backup/restore runbook/docs updates (Postgres + storage; Valkey/Meili rebuildable/optional).
- Deterministic tests for config/gating/pool behavior and targeted regressions.

## Non-goals
- Binary file upload/download implementation (slice B).
- New mandatory services, multi-node/durable realtime, search indexing feature work.
- Full OIDC/Zitadel redesign, SQLite runtime implementation, S3 rollout.
- Performance benchmarking/load testing on real 2vCPU/4GB, deploy/release automation.

## NV
- Runtime load/OOM validation on 2vCPU/4GB, compose resource enforcement by runtime, exhaustive route coupling, off-host restore drills, full optional-service CI matrix.
