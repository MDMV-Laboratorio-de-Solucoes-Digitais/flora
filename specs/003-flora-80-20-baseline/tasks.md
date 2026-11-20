# Tasks: 003 Flora 80/20 Baseline, PKCE, and Scope Guardrails

## Phase 0 — Assessment (completed in this cycle)
- [X] T001 Research Macro, REA, Craft-Anything, lightweight stacks, dark-factory workflow, and local Flora state via parallel subagents and repo inspection
- [X] T002 Create `.specify/assessments/003-flora-80-20-cycle/research.md`
- [X] T003 Create `.specify/assessments/003-flora-80-20-cycle/decision.md` (verdict: go)

## Phase 1 — Baseline evidence (P1)
- [X] T010 Capture environment/prereq notes and toolchain (`rustc/cargo/bun/node`, `.env.example`, migrations/services)
- [X] T011 Run/fix `cargo fmt --all -- --check` (**was FAIL** in `flora-api/src/routes/auth.rs`; formatted and now passing)
- [X] T012 Repair cargo registry cache (`cc-1.4.4`) and run clippy: **workspace clippy PASS** (`cargo clippy --workspace --all-targets -- -D warnings`, exit 0) after fixes in `flora-core` and `flora-api`
- [X] T013 Run tests and record pass/fail by crate/suite: `cargo test --workspace` **PASS** (90 passed, 0 failed, 1 ignored doctest; no env-prereq failures observed in this run) — see baseline
- [~] T014 Ran `apps/frontend` checks (install/check/build PASS; format applied). Lint post-format and test:unit remain unconfirmed/failing in env: Playwright install hit **ENOSPC**, browser-session timeout after cache workaround — see baseline
- [X] T015 Inspected/compared `apps/web` (`@coparse/web` Macro/SolidJS app, not Svelte rewrite target)
- [X] T016 Documented canonical frontend recommendation and aligned CI/docs to `apps/frontend` (`.github/workflows/ci.yml`, `AGENTS.md`, `README.md`, baseline notes)

## Phase 2 — PKCE T116 (P1)
- [X] T020 Inspected `flora-api/src/routes/auth.rs` and related OIDC/config/session/state flow
- [X] T021 Implemented PKCE S256 (login state/verifier/challenge generation + persisted transaction store; callback validates/consumes state once and sends `code_verifier`), config `code_challenge_method` validation, and core PKCE utils
- [X] T022 Added/updated tests: login contract asserts state/challenge/method; callback failure cases (missing code/state, invalid/reused state). Full mocked-provider success-path callback test not added (requires OIDC token/JWKS mock + DB-backed flow)
- [X] T023 Verification recorded by implementation pass: `cargo fmt --all -- --check`, `cargo test -p flora-core`, `cargo test -p flora-api`, `cargo test -p flora-tests`, `cargo clippy --workspace --all-targets -- -D warnings` **PASS** (see `.specify/assessments/003-flora-80-20-cycle/baseline.md`)

## Phase 3 — Canonical frontend & CI/docs (P2)
- [~] T030 Update CI/docs to reflect canonical frontend (`.github/workflows/ci.yml` and `AGENTS.md` updated to `apps/frontend`; `README.md` still needs explicit note if desired)
- [~] T031 Add deprecation/migration note for non-canonical frontend app if retained (`README.md` and `AGENTS.md` clarify `apps/web` as non-canonical; dedicated migration/deprecation note can be expanded if app is retained)
- [~] T032 Aligned frontend CI commands executed in `apps/frontend` and recorded (install/check/build pass; format applied; lint/test:unit blocked by unconfirmed rerun and Playwright/ENOSPC env) — see baseline

## Phase 4 — Guardrails & handoff (P2)
- [X] T040 Spec/assessment reflect: Macro AGPL source/docs-first learning, REA/Craft-Anything non-core/tactical-only, Seed VPS-first dependency constraints (see assessment research/decision + spec non-goals)
- [~] T041 Validation summary: workspace fmt/clippy pass; PKCE + touched crate tests pass (per implementation pass); full `cargo test --workspace` and frontend checks pending/env-gated; PKCE success-callback mock/integration and README/apps-web note pending (see baseline.md)