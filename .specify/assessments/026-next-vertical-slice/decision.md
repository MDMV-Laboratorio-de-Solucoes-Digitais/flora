# Decision: 026 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice
**S1 — Executable Seed verification gate + governance drift closure.**

## Scope (in)
- Fix `deny.toml` to allow `AGPL-3.0-or-later`; verify `cargo deny check bans licenses sources` locally.
- Add `cargo fmt --all -- --check`, strict `cargo clippy --workspace --all-targets -- -D warnings` and the cargo-deny check to `.github/workflows/rust.yml`.
- Finalize cycle 025 evidence (tasks, validate, release with real PR #231 result).
- Amend constitution to mechanism-neutral milestone/FASE wording (version bump), fix ROADMAP cycle-025 line, AGENTS.md stale notes, and Seed contract header docs where practical.
- Remove phantom OTel declarations from `Cargo.toml`; document as NV/follow-up.
- Record R1 research and planning updates; PR/CI green; merge/sync.

## Justification
Highest value per effort: turns the ADR-0001 gate from prose into a machine-enforced CI gate, closes self-contradictions in authoritative docs, and removes a dependency trap — directly satisfying FASE 0 and the Seed verification claim.

## NV / deferred
- cargo-vet store/imports/exemptions; cargo-deny advisories job requiring RustSec network; SBOM.
- PGMQ/Valkey scaffolding retirement; browser e2e (fresh JWT prerequisite); Tauri; full OTLP exporter; 2vCPU/4GB load/restore; Meilisearch parity.
- Non-goals: new services, frontend changes, reverting parallel-agent work.
