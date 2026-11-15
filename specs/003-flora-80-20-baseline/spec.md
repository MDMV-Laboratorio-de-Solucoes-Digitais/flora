# Feature Specification: Flora 80/20 Baseline, PKCE, and Scope Guardrails

**Feature Branch**: `[003-flora-80-20-baseline]`  
**Created**: 2026-11-15  
**Status**: Draft (assessment-backed, go)  
**Input**: User request to research Macro/REA/Craft-Anything fit and execute next 80/20 cycle for Flora per mdmv-dark-factory SDD workflow; local repo/spec inspection.

## Context
Flora aims to be a viable, performant, self-hosted (cheap VPS) SME alternative to Macro using Rust + PostgreSQL and Svelte 5 (Tauri later), debloating proprietary/heavy dependencies. Existing specs (001 backend seed, 002 frontend rewrite) show substantial claimed progress but have unverified runtime baselines, an open security follow-up (**T116 OIDC PKCE**), and frontend/CI/doc drift.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Verified baseline (Priority: P1)
As a maintainer, I want deterministic verification of the current Rust workspace and frontend toolchains so that future work starts from known-good or explicitly known-broken state.

**Why this priority**: Without executed evidence, task/checklist completion claims are untrustworthy; strict lints mean regressions are likely.

**Independent Test**: Run and record outcomes of format/lint/build/test commands per crate/app; capture failures as actionable items.

**Acceptance Scenarios**:
1. **Given** the workspace, **When** `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` (or targeted subsets with rationale) are run, **Then** results are recorded with pass/fail and blocking errors listed.
2. **Given** `apps/frontend` and `apps/web`, **When** their configured checks/build/tests are run (per package manifests/CI), **Then** canonical frontend status and gaps are documented.

---

### User Story 2 - OIDC PKCE hardening (Priority: P1)
As a security-conscious deployer, I want the OIDC login flow to use PKCE so that public-client/code-interception risks are reduced.

**Why this priority**: `specs/001-flora-seed-v0-1/tasks.md` leaves **T116** open; it is a focused, high-value security improvement.

**Independent Test**: Auth route/unit/integration tests demonstrate code verifier/challenge generation, storage/validation, and correct token exchange behavior.

**Acceptance Scenarios**:
1. **Given** login initiation, **When** redirect is created, **Then** PKCE `code_challenge`/method and verifier handling follow spec/config.
2. **Given** callback, **When** code is exchanged, **Then** verifier is required/validated and failures are handled safely.

---

### User Story 3 - Canonical frontend and CI alignment (Priority: P2)
As a contributor, I want a single documented source-of-truth frontend path and matching CI/docs so that work is not split across diverging apps.

**Why this priority**: `apps/frontend` (spec 002 target) and `apps/web` (historically referenced by CI) create ambiguity and wasted effort.

**Independent Test**: Docs/CI point to the chosen app, and build/test commands succeed or have tracked issues.

**Acceptance Scenarios**:
1. **Given** repo docs/workflows, **When** audited, **Then** canonical app, run commands, and migration/deprecation note for the other app are explicit.
2. **Given** CI config, **When** aligned, **Then** it exercises the canonical frontend path (or documents intentional split).

---

### User Story 4 - Macro-informed, debloated scope guardrails (Priority: P2)
As a product owner, I want Flora’s next functional increments chosen from Macro’s open-source (AGPL) concepts but implemented minimally for cheap-VPS SMEs so that we avoid reintroducing bloat.

**Why this priority**: Research shows Macro’s value is product primitives, not its heavy architecture; REA/Craft-Anything are poor primary fits.

**Independent Test**: Decision/research artifacts and next backlog reflect: Postgres-first modular monolith, optional subsystems, no core dependency on REA/Craft-Anything, source/docs-based Macro learning.

**Acceptance Scenarios**:
1. **Given** proposed features, **When** evaluated, **Then** they map to Seed→Sprout needs (org/workspace, identity, messaging, tasks, files/search/notifications basics) with explicit deps/ops cost.
2. **Given** tooling proposals, **When** evaluated, **Then** REA/Craft-Anything are allowed only as optional, authorized, out-of-band tactical aids with documented rationale.

## Requirements *(mandatory)*

### Functional Requirements
- **FR-001**: Produce a recorded verification baseline for Rust (`fmt`, `clippy`, `test/build`) and frontend app(s) with environment prerequisites noted (DB/services as applicable).
- **FR-002**: Implement and test PKCE in `flora-api` OIDC auth flow (T116), consistent with constitution security/rigor.
- **FR-003**: Document and, where safe, align canonical frontend (`apps/frontend` vs `apps/web`) and CI/doc references.
- **FR-004**: Maintain an assessment-backed decision log for this cycle (research + go verdict) and keep Macro/REA/Craft-Anything guidance traceable.
- **FR-005**: Prioritize thin vertical slices over broad parity with Macro’s heavy stack (no Kafka/OpenSearch/FusionAuth/DO/agent-harness requirements for Seed).

### Non-goals
- Full Macro feature parity, hosted SaaS integrations breadth, CRDT-everywhere collab, or adopting REA/Craft-Anything as product dependencies.
- Unauthorized/proprietary reverse engineering; AGPL-incompatible reuse.

## Key Entities
- **Assessment record**: `.specify/assessments/003-flora-80-20-cycle/{research,decision}.md`
- **Verification evidence**: command outputs/log references (to be captured during implementation/validate).
- **Auth flow**: OIDC login/callback with PKCE verifier/challenge state.

## Success Criteria *(mandatory)*
- **SC-001**: Baseline commands executed and outcomes documented; blockers enumerated with owners/next actions.
- **SC-002**: T116 closed with tests covering PKCE happy/failure paths; `cargo fmt/clippy/test` remain green for touched crates.
- **SC-003**: Single canonical frontend path stated in docs/CI with reproducible commands.
- **SC-004**: Subsequent feature proposals reference Seed constraints and AGPL-compliant Macro learning (no RE-first strategy).

## Assumptions
- Postgres/Valkey/Meilisearch/OIDC dependencies may be unavailable in all environments; tests requiring them must be categorized (unit vs integration) and not falsely claimed green.
- `apps/frontend` is the intended Svelte 5 rewrite target per spec 002 unless verification shows `apps/web` is the maintained path; reconciliation is an explicit output of this cycle.