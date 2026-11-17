# Bug Verification: CI build/test steps run only the root package

- **Slug**: ci-test-step-runs-zero-tests
- **Tested**: 2026-11-16
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: pass (local evidence + remote CI observation, 2026-11-16)

## Summary

The red/green signal is closed on both sides. Before the change, `cargo test` selected only the root placeholder and reported `running 0 tests`; after the change, `cargo test --workspace` reports 104 passed. GitHub Actions observed the same commit with green `build` and `db-tests` jobs.

## Checks performed

| Check | Command | Result | Notes |
|---|---|---|---|
| Original symptom (pre-fix) | `cargo test` | red: `running 0 tests` | Root package is the sole default member |
| Workflow text | `grep -n -A1 -E 'name: (Build|Test)' .github/workflows/rust.yml` | pass | Lines 33/35 contain `--workspace` |
| YAML syntax | `python3 -c "import yaml; yaml.safe_load(...)"` | pass | `yaml-ok` |
| Formatting | `cargo fmt --all -- --check` | pass | exit 0 |
| Workspace build | `cargo build --workspace` | pass | 18 crates compiled |
| Workspace tests | `cargo test --workspace` | pass | 104 passed, 0 failed, 43 ignored |
| Ignore semantics | `cargo test --workspace` output | pass | `flora_tests` reports 43 ignored; DB suites stayed out |
| Remote CI (`Rust CI` job `build`) | PR #251, run `43986194004` | pass | https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/43986194004/job/133714834751 |
| Remote CI (`CI` frontend `build`) | PR #251, run `43986188630` | pass | https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/actions/runs/43986188630/job/133714814552 |
| Remote CI (`db-tests`) | PR #251, runs `43986188630` / `43986194004` | pass | Unchanged; 2m15s / 2m20s |
| Remote CI (`e2e-browser`, `supply-chain-vet`, `dependency-review`) | PR #251 | pass | Informational lanes also green |

PR #251 state: `OPEN`, `MERGEABLE`, `CLEAN`. Merge is not performed in this bug: the authorization matrix requires explicit human approval for merges.

## Output excerpts

```text
$ cargo test --workspace
     Running unittests src/lib.rs (target/debug/deps/flora_api-...)
test result: ok. 25 passed; 0 failed; 0 ignored
     Running unittests src/lib.rs (target/debug/deps/flora_core-...)
test result: ok. 12 passed; 0 failed; 0 ignored
     Running tests/models.rs (target/debug/deps/models-...)
test result: ok. 21 passed; 0 failed; 0 ignored
     Running unittests src/lib.rs (target/debug/deps/flora_tests-...)
test result: ok. 46 passed; 0 failed; 43 ignored
```

Total: 104 passed, 0 failed, 43 ignored, plus one ignored doctest.

## Residual risks

- CI compile time increases because all members are now compiled and tested.
- Future DB-gated tests must remain `#[ignore]`d so the default workspace run needs no services.
- The six `contract::…::db_` legs in `db-tests` can still exit 0 with zero matched tests; tracked as a follow-up in `fix.md`.

## Recommendation

The fix is verified. Merge remains gated on explicit approval per `docs/research/2026-11-16-036-authorization-matrix.md`.

## Converge

- 2026-11-16: reproduced locally, applied two-line workspace fix, verified locally and remotely, opened PR #251. No product code changed.
- Residual work is tracked in `fix.md` follow-ups; this entry does not claim the Seed milestone is fully verified.
