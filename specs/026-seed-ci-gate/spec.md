# Feature Specification: 026 — Executable Seed verification gate

**Branch**: `feat/026-seed-ci-gate`  
**Status**: Completed (PR #232)  
**Assessment**: `.specify/assessments/026-next-vertical-slice/decision.md`

## Goal
Turn ADR-0001's Seed verification gate from prose into machine-enforced CI checks, close governance drift, and remove latent dependency traps — without new services or reverting parallel work.

## Scope
- `deny.toml`: allow `AGPL-3.0-or-later` (crates declare it; cargo-deny ≥0.18.4 matches pedantically). Verified locally: `bans ok, licenses ok, sources ok`.
- `.github/workflows/rust.yml` build job: `cargo fmt --all -- --check`, strict `cargo clippy --workspace --lib --bins -- -D warnings`, `cargo deny check bans licenses sources` via `taiki-e/install-action@cargo-deny`. Production targets only for clippy because a parallel agent's test code trips `expect_used` under `--all-targets` (NV; not reverted).
- Finalize cycle 025 evidence (real PR #231 run IDs, merge commit, task checkbox).
- Amend `.specify/memory/constitution.md` to v1.3.0 with mechanism-neutral Seed milestone/phases (ADR-0001), fix `ROADMAP.md` cycle-025 drift and `AGENTS.md` stale notes.
- Remove phantom OTel/`openidconnect` workspace declarations (unused, absent from Cargo.lock; `opentelemetry-otlp 0.22` does not exist).

## Non-goals
cargo-vet store/imports, advisories job, SBOM, PGMQ/Valkey scaffolding removal, browser e2e, Tauri, full OTLP exporter, new services.

## NV
`--all-targets` clippy blocked by parallel-agent test code; `cargo vet` unavailable on clean checkout (gitignored store); real-OP e2e; 2vCPU/4GB validation.
