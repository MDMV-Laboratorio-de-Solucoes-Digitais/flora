# Feature Specification: 041 — Rust dependency debloat

**Branch**: `feat/041-rust-dead-crates` · **Status**: In progress (working tree)
**Assessment**: `.specify/assessments/041-rust-dead-crates/decision.md`

## Goal
Remove Rust dependencies and workspace members that have zero call sites in code,
shrinking the dependency graph and checkout for the "VPS barata" objective — the
backend counterpart of cycle 040's frontend debloat. Pure deletion, no behavior change.

## Scope
- Remove from `Cargo.toml`: workspace members `flora-tasks`, `flora-files`;
  workspace deps `flora-tasks`, `flora-files`, `rust-i18n`, `argon2`, `bcrypt`.
- Remove from `flora-core/Cargo.toml`: `dotenv`, `argon2`, `bcrypt`, `rust-i18n`.
- Delete directories `flora-tasks/`, `flora-files/` (leaf crates, no consumers).
- Remove 6 cargo-vet exemptions (`argon2`, `bcrypt`, `dotenv`, `rust-i18n{,-macro,-support}`);
  **keep** `dotenvy` (still used by sqlx) and `jsonwebtoken/rust_crypto`.
- Regenerate `Cargo.lock`: 4898 → 4529 lines (−369, 34 packages removed, no new package;
  the 9 `+ "bitflags"` diff lines are Cargo disambiguation after `bcrypt`'s old
  `bitflags 1.x` left the graph).
- Governance drift docs (README/AGENTS/ROADMAP), validation + release.

## Non-goals / non-claims
- No code behavior change, no RbacService wiring, no RLS/role changes (separate
  decisions — see c041-02/c041-03 research; RLS stays `[NV]` until SE authorization).
- No required check/protection/ADR/constitution change, no deploy/tag/release.
- Do NOT claim performance measurements — only graph/config size delta is proven.

## Verification (frozen: same protocol as 040, fresh-context audit)
Baseline gates captured before change; after: `cargo fmt --check`,
`cargo clippy --workspace --all-targets -- -D warnings`, `cargo deny check bans licenses
sources`, `cargo test --workspace`, `cargo vet check`; test count identical to baseline
(46 passed / 43 ignored in the main crate); `Cargo.lock` diff contains only deletions;
zero orphan references; author-blind audit.

## NV
- Runtime memory/CPU deltas not measured (only graph size).
- Forgejo protections / remote CI [NV].
