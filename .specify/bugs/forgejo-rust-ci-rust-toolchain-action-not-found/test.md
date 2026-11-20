# Bug Verification: Forgejo Rust CI fails resolving `dtolnay/rust-toolchain` action

- **Slug**: forgejo-rust-ci-rust-toolchain-action-not-found
- **Tested**: 2026-11-15
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: partial

## Summary

The proximate workflow configuration issue (dependency on `dtolnay/rust-toolchain` action resolution via Forgejo mirror) was removed and the updated workflow YAML is syntactically valid. However, the original failure cannot be reproduced or confirmed resolved from this local environment because it requires execution on the Forgejo Actions runner (`coolify-runner-v13`) against `data.forgejo.org`/runner image behavior.

## Checks Performed

| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (post-fix, original environment) | Re-run Forgejo Actions `rust.yml` for commit/`dev` on https://forgejo.mdmv.qzz.io/MDMV/flora-workspace | not-run | Requires Forgejo CI trigger/execution (remote); not executed in this local session. |
| Workflow syntax validation | `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/rust.yml')); print('yaml-ok')"` | pass | Output: `yaml-ok`. |
| Local Rust toolchain availability (sanity) | `rustc --version`; `cargo --version` (local host) | pass | Confirms local environment has Rust tooling; not equivalent to Forgejo runner/container environment. |
| Fix scope review | Manual diff/review of `.github/workflows/rust.yml` | pass | Rust setup no longer uses `dtolnay/rust-toolchain`; pinned `1.98.0`, build/test steps preserved. |
| GitHub Actions CI (post-fix) | Observe PR/push CI run on GitHub | not-run | Not executed yet in this session; should be monitored after commit/push. |
| Regression suite (`cargo build/test`) in runner context | CI job steps on Forgejo/GitHub | not-run | To be validated by CI runs. |

## Output Excerpts

- YAML validation: `yaml-ok`
- Local toolchain (host): `rustc`/`cargo` present (host environment; Forgejo job uses `node:20-bookworm` runner image and may differ).

## Residual Risks

- `actions/checkout@v4` remains action-mirror dependent; if unavailable on the Forgejo mirror/runner, job may still fail at setup (different cause).
- `rustup` may not be preinstalled in the Forgejo runner image; fix includes bootstrap via rustup installer, which depends on network access to install sources.
- Runner environment differences (PATH, permissions, preinstalled packages, outbound network policy) could affect the shell-based setup.
- GitHub Actions parity not yet observed for the modified workflow.

## Recommendation

Hold for remote CI verification. Commit/push the fix and **verify on Forgejo Actions** that `rust.yml` completes (toolchain setup, `cargo build`, `cargo test`) for the target commit/branch; also confirm **GitHub Rust CI** remains green. If Forgejo still fails, re-run `speckit.bug.assess` with the new log (likely checkout mirror/access or rustup bootstrap/network issue) and iterate.
