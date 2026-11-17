# Bug Verification: CI build/test steps run only the root package

- **Slug**: ci-test-step-runs-zero-tests
- **Tested**: 2026-11-16
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: partial (local evidence complete; remote CI observation pending)

## Summary

The local red/green signal is closed: before the change, `cargo test` selected only the root placeholder and reported `running 0 tests`; after the change, `cargo test --workspace` reports 104 passed. Remote CI observation is recorded separately and must be filled after the PR runs.

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
| Remote CI (`Rust CI` job `build`) | PR run on GitHub | not-run yet | Fill with run URL and check conclusion |
| Remote CI (`db-tests`) | PR run on GitHub | not-run yet | Expected unchanged |

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

Hold for remote CI. After the PR run, update the two `not-run yet` rows with the run URL and conclusion, then set `Result: pass` if the `build` and `db-tests` jobs are green. Until then, do not claim remote green.
