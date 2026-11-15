# Decision: 003 Flora 80/20 Cycle

**Date:** 2026-11-15  
**Verdict:** **go** (narrow, autonomous cycle)  
**Scope:** Flora repo (`/home/luis/development/MDMV/projetos/Arquivados/flora`) — apply mdmv-dark-factory SDD discipline pragmatically for next 80/20.

## Decision
Proceed with **baseline stabilization + security follow-up + scope clarification** as the next cycle, and explicitly **do not** make REA or Craft-Anything core dependencies/strategies for learning/reimplementing Macro.

## Rationale
1. **Value/effort (80/20):** specs/tasks claim broad convergence (001/002) but runtime verification is unproven in this environment; proving the toolchain and closing **T116 (OIDC PKCE)** yields immediate trust/security value and unblocks real feature delivery.
2. **Macro strategy:** Macro is public **AGPL-3.0**; source/docs/behavior analysis is the valid, high-signal path. REA is not a primary fit; Craft-Anything is stack/product-misaligned for core work. Both remain optional tactical references only under legal/authorized, narrow use.
3. **Alignment:** constitution emphasizes VPS-first debloat, trait-driven modular monolith, extreme rigor, spec/test-first, and Svelte5/Tauri direction — consistent with a small, verifiable stabilization cycle before expanding scope.

## In scope (autonomous)
- Capture assessment artifacts (intake/research/decision) and a concrete spec package for cycle 003: baseline verification, PKCE completion, canonical frontend/CI/docs reconciliation, and Macro-informed seed-scope guardrails.
- Documentation/planning changes only in this step; implementation gated by verification results and normal repo workflow.

## Out of scope / requires explicit authorization or separate decisions
- Deployments, production/infra changes, secrets/credentials handling, merges/releases, adding external paid/proprietary services.
- Any reverse-engineering of unauthorized/proprietary targets; any AGPL-incompatible reuse/copying of Macro code/assets.
- Large architecture rewrites or claiming “green” without executed evidence.

## Acceptance for this decision step
- `research.md`, `decision.md` recorded; `specs/003-flora-80-20-baseline/` created with spec/plan/tasks focused on executable 80/20 outcomes and clear verification gates.