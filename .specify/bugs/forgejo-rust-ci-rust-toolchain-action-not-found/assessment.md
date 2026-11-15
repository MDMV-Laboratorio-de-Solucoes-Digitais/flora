# Bug Assessment: Forgejo Rust CI fails resolving `dtolnay/rust-toolchain` action

- **Slug**: forgejo-rust-ci-rust-toolchain-action-not-found
- **Created**: 2026-11-15
- **Source**: https://forgejo.mdmv.qzz.io/MDMV/flora-workspace/actions/runs/2/jobs/0 (pasted failure log provided by user; fetched page content was mostly UI chrome/non-log)
- **Verdict**: valid
- **Severity**: high

## Report (verbatim or summarized)

User-provided context and log excerpt (commit `3d70befe65`, merge of PR #207 to `dev`, Forgejo Actions workflow `rust.yml`, job `build`):

- Failure occurs during job setup/action resolution on Forgejo runner (`coolify-runner-v13`, `v13.2.1`, image `node:20-bookworm`).
- Runner fetches `actions/checkout@v4` from `https://data.forgejo.org/actions/checkout` and clones `dtolnay/rust-toolchain` ref `stable` from `https://data.forgejo.org/dtolnay/rust-toolchain`.
- Error:
  - `⚙️ [runner]: unable to probe object format for https://data.forgejo.org/dtolnay/rust-toolchain: ls-remote failed https://data.forgejo.org/dtolnay/rust-toolchain: remote: Not found.`
  - `fatal: repository 'https://data.forgejo.org/dtolnay/rust-toolchain/' not found: exit status 128`
- Subsequent steps skipped (`actions/checkout@v4` post, `Set up Rust`, `Build`, `Test`); job fails after ~4s.

URL policy note: host `forgejo.mdmv.qzz.io` is not on the default public allowlist; fetch was attempted via available tooling but did not yield log content beyond page chrome. Assessment relies on the verbatim log/context pasted by the user (trusted report input for this session) and local workflow inspection.

## Symptom

**Observed:** Forgejo Actions `Rust CI` (`.github/workflows/rust.yml`) fails before executing workflow steps because the runner cannot resolve/clone the `dtolnay/rust-toolchain@stable` action from the Forgejo action mirror (`data.forgejo.org`).

**Expected:** The Rust CI workflow should set up Rust 1.98.0 and run `cargo build` and `cargo test` successfully on Forgejo (as the equivalent workflow does on GitHub for merged PRs).

## Reproduction

1. Push/merge to `dev` on Forgejo remote `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` (observed at commit `3d70befe65`).
2. Forgejo Actions triggers `.github/workflows/rust.yml` (`on: [push, pull_request]`).
3. Runner prepares job and attempts to resolve actions referenced by the workflow.
4. Resolution of `dtolnay/rust-toolchain@stable` via `https://data.forgejo.org/dtolnay/rust-toolchain` fails with "remote: Not found".
5. Job fails; build/test steps never run.

[NEEDS CLARIFICATION: Whether `actions/checkout@v4` resolution from `data.forgejo.org/actions/checkout` completed successfully in this environment in general, or if only the rust-toolchain action is missing/unmirrored. Log excerpt shows checkout fetched but job still failed at setup due to rust-toolchain probe.]

## Suspected Code Paths

- `.github/workflows/rust.yml:1-15` — defines Rust CI job and uses `actions/checkout@v4` and `dtolnay/rust-toolchain@stable` with `toolchain: '1.98.0'`; this is the direct trigger of the failed action resolution.
- Forgejo runner/action resolution configuration (external to repo): determines how `uses:` references are mirrored/fetched (e.g. via `data.forgejo.org`); likely root environmental factor, but workflow can be made more portable.

## Root Cause Hypothesis

**Confidence: high (for proximate cause), medium (for best portable fix without live Forgejo action-mirror inventory).**

Proximate cause: the Forgejo runner attempts to obtain third-party actions from its configured mirror (`data.forgejo.org`) and the repository/mirror entry for `dtolnay/rust-toolchain` is unavailable/not found, so job setup fails before steps execute. This is environment/mirror-specific: GitHub-hosted CI for the same workflow/changes passed during PR validation (#204–#207). The workflow currently assumes GitHub-style action availability for `dtolnay/rust-toolchain`.

## Proposed Remediation

**Preferred:** Replace the `dtolnay/rust-toolchain` action step in `.github/workflows/rust.yml` with an explicit, portable Rust toolchain installation that does not depend on a third-party GitHub Action mirror (e.g. install via `rustup` in a `run:` step: ensure rustup is present, `rustup toolchain install 1.98.0 --profile minimal` or default as needed, `rustup default 1.98.0`, add cargo bin to PATH if required). Keep `actions/checkout@v4` if confirmed available on this Forgejo instance; otherwise switch to a Forgejo-compatible checkout mechanism/full URL supported by the runner.

This removes the missing-mirror dependency while preserving the pinned toolchain requirement (`1.98.0`) already declared in the workflow.

**Alternatives (trade-offs):**
- Use a fully qualified action source supported by Forgejo (e.g. `uses: https://github.com/dtolnay/rust-toolchain@stable` or a specific commit/tag) if the runner allows direct GitHub fetches — simpler diff but depends on runner policy/network and action compatibility.
- Use an alternative setup action known to be mirrored/available on Forgejo (e.g. `actions-rust-lang/setup-rust-toolchain`) — may work if mirrored, but requires verifying availability on `data.forgejo.org`/this runner.
- Configure Forgejo instance to mirror/cache `dtolnay/rust-toolchain` — fixes environment but requires infra/admin access (out of repo scope; may need explicit authorization).

**Files likely to change:**
- `.github/workflows/rust.yml`

**Tests to add or update:**
- No Rust unit/integration tests apply directly. Verification is CI/workflow execution: re-run Forgejo Actions for `rust.yml` on `dev` (push) or via a test PR/dispatch if available and confirm `Set up Rust` (or equivalent), `Build`, and `Test` complete. Optionally validate YAML syntax locally.

## Risks & Considerations

- **Cross-platform CI parity:** Changes must remain compatible with GitHub Actions CI (`.github/workflows/rust.yml` is also used there). `rustup`-based setup is generally portable on `ubuntu-latest` images (rustup often preinstalled; may need install step).
- **Toolchain/profile/components:** Ensure installed toolchain matches prior behavior (`1.98.0`) and includes components needed by `cargo build/test` (and clippy/fmt if added later).
- **Action availability uncertainty:** `actions/checkout@v4` appeared to be fetched from `data.forgejo.org` in logs; if it becomes unavailable, workflow may need a similar portability adjustment.
- **External dependency:** Installing rustup from network (if not preinstalled) introduces network dependency; acceptable for CI but should use official installer practices.
- **Observability:** Current failure gives fast, clear signal; after fix, ensure logs still show selected Rust version (`rustc --version`, `cargo --version`).

## Open Questions

- [NEEDS CLARIFICATION: Is `actions/checkout@v4` reliably available on this Forgejo runner/mirror, or should checkout also be made mirror-independent?]
- [NEEDS CLARIFICATION: Does the Forgejo runner environment allow fetching actions/scripts directly from GitHub (`https://github.com/...`) or only from `data.forgejo.org`?]
- [NEEDS CLARIFICATION: Is `rustup` preinstalled on the `node:20-bookworm` job image used by this runner?]
- [NEEDS CLARIFICATION: Is there a preferred org-wide Forgejo Actions pattern for Rust toolchain setup in MDMV repos?]
