# Implementation Plan: Flora 80/20 Baseline, PKCE, and Scope Guardrails

**Branch**: `[003-flora-80-20-baseline]` | **Date**: 2026-11-15 | **Spec**: `specs/003-flora-80-20-baseline/spec.md`

## Summary
Execute a narrow stabilization cycle: (1) establish executable verification baselines for Rust and frontend(s), (2) implement OIDC PKCE (T116), (3) reconcile canonical frontend/CI/docs, (4) codify Macro-informed debloat guardrails. No core dependency on REA/Craft-Anything.

## Technical Context
- **Backend:** Rust workspace (Axum 0.8, sqlx/Postgres, tokio), strict workspace lints (`deny` unwrap/expect/panic/todo/dbg etc., pedantic clippy). API in `flora-api`, domain/config/migrations in `flora-core`.
- **Frontend:** `apps/frontend` = SvelteKit/Svelte 5/TS/Tailwind (spec 002 target); `apps/web` also present and referenced historically by CI — reconcile.
- **Verification:** `cargo fmt --all`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test` (scoped as needed); frontend `bun/npm` scripts per app manifests.
- **Constraints:** VPS-first, Postgres-first Seed, optional subsystems; AGPL compliance; do not claim unexecuted passes.

## Constitution/Workflow Check
- Aligns with constitution: VPS test mindset, trait-driven modular monolith, rigor/linting, spec/test-first, atomic/traceable changes. Follows dark-factory spirit: intake/research/decision before implementation claims; R4-like evidence before release claims (not in scope).

## Phases

### Phase A — Baseline evidence (P1)
- Inventory commands/env needs (`.env.example`, DB/migrations/services). Run fmt/clippy/tests/build in smallest reproducible steps; classify failures (lint, compile, unit, integration/env-gated).
- Run frontend checks for `apps/frontend` and `apps/web` as configured; identify canonical path.

### Phase B — Security fix: PKCE T116 (P1)
- Update `flora-api` OIDC login/callback flow: generate/store verifier, send challenge (S256), validate on exchange; handle missing/invalid verifier safely.
- Add/adjust tests (unit/integration/contract as feasible) without violating strict lints.

### Phase C — Frontend/CI/doc reconciliation (P2)
- Decide/document canonical frontend; update `AGENTS.md`/README/workflow docs and CI (`.github/workflows/ci.yml`) accordingly or document split with rationale.
- Deprecation/migration note for non-canonical app if applicable.

### Phase D — Guardrails (P2)
- Record ADR-lite/notes in assessment/spec: Macro source/docs-first learning, optional subsystems, no REA/Craft-Anything core use, cheap-VPS dependency discipline.

## Risks & Mitigations
- **Env-gated test failures:** separate code defects from missing Postgres/Valkey/OIDC/Meilisearch; document prerequisites.
- **Lint strictness blocks quick fixes:** prefer correct `Result`/`?` handling and minimal, reasoned allows only if unavoidable.
- **Dual frontend confusion:** make one path authoritative; avoid partial migrations in same change.

## Deliverables
- `.specify/assessments/003-flora-80-20-cycle/{research.md,decision.md}`
- `specs/003-flora-80-20-baseline/{spec.md,plan.md,tasks.md,checklists/requirements.md}`
- (Implementation phase, follow-up) code/tests/doc/CI changes with verification evidence.