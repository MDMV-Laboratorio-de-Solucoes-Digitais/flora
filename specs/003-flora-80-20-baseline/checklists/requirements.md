# Requirements Checklist: 003 Flora 80/20 Baseline

- [X] CHK001 Scope is narrow and 80/20 (baseline verification, PKCE T116, frontend/CI reconciliation, guardrails)
- [X] CHK002 Research inputs include Macro, REA, Craft-Anything, lightweight stacks, and local repo state with sources/confidence framing
- [X] CHK003 Decision recorded with `go` verdict and explicit authorization/out-of-scope boundaries
- [X] CHK004 Functional requirements are testable (command evidence, PKCE behavior, CI/doc alignment)
- [X] CHK005 Non-goals prevent REA/Craft-Anything core adoption and unauthorized/AGPL-risky reuse
- [X] CHK006 Baseline evidenced: `cargo fmt`/workspace `cargo clippy -D warnings`/`cargo test --workspace` **PASS** (90 passed, 0 failed, 1 ignored) in this environment; frontend evidence recorded with some reruns pending (see baseline)
- [~] CHK007 PKCE implemented and partially covered by tests (state/challenge/login+callback failure/reuse covered; full mocked-provider success callback/integration path pending, ID token signature verification unchanged) — see baseline/tasks
- [~] CHK008 Canonical frontend path enforced in CI/agent/docs (`.github/workflows/ci.yml`, `AGENTS.md`, `README.md`); frontend checks executed (install/check/build pass, format applied) but lint confirmation and test:unit blocked by Playwright **ENOSPC**/browser mismatch in env