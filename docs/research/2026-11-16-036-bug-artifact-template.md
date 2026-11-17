# Bug-artifact template guidance — slug `ci-test-step-runs-zero-tests`

- **Date**: 2026-11-16
- **Scope**: research only. No bug files created. Guidance for the parent session that will create
  `.specify/bugs/ci-test-step-runs-zero-tests/{assessment,fix,test}.md`.
- **Sources compared**: `projetos/mdmv-dark-factory/.specify/bugs/` (README + 8 entries) and
  Flora `.specify/bugs/forgejo-rust-ci-rust-toolchain-action-not-found/{assessment,fix,test}.md`.
- **Bug premise (confirmed locally, see evidence §):** `.github/workflows/rust.yml:35` runs bare
  `cargo test` at the virtual-ish root that also has `[package] name = "flora"`; Cargo selects only
  that root package, whose `src/main.rs` has zero tests ⇒ the required `Test` check is green by
  construction. This is the Flora instance of dark-factory `BUG-002-ci-gate-vacuous.md`.

## 1. Convention comparison (what to carry over)

| Axis | dark-factory (older/Portuguese family: `ci-gate-057-*`, `ci-gate-060-*`, `forgejo-open-prs-gate-red`) | dark-factory (newer/English family: `ci-pytest-failure`, `forgejo-pr-102`, `sdd-057-missing-evidence`, `prepr-verifier-hook-timeout-2`) | Flora existing entry |
|---|---|---|---|
| Filename/layout | `BUG-<NNN>-<slug>.md` standalone **and** `<slug>/{assessment,fix,test}.md` | `<slug>/{assessment,fix,test}.md` | `<slug>/{assessment,fix,test}.md` — follow this |
| Header metadata | `**Bug ID**`, `**Data**`, `**Reporter**`, `**Status**: assessed` | bullets `- **Slug**`, `- **Created**`, `- **Source**`, `- **Verdict**`, `- **Severity**` | same as newer family — copy it verbatim |
| Assessment sections | `## Sintoma`, `## Causa raiz`, `## Impacto`, `## Classificação` | `## Report (verbatim or summarized)`, optional `## URL Trust Policy record`, `## Symptom`, `## Reproduction`, `## Suspected Code Paths`, `## Root Cause Hypothesis` (+Confidence), `## Proposed Remediation` (Preferred+Alternatives), `## Risks & Considerations`, `## Open Questions` | same as newer family — copy it |
| Fix sections | `## Fix aplicado`; `**Status**: fixed` | `## Summary`, `## Changes` (table File/Change/Notes), `## Diff Highlights (optional)`, `## Tests Added or Updated`, `## Local Verification`, `## Deviations from Assessment`, `## Follow-ups`; `**Status**: applied\|partial` | same as newer family |
| Test sections | `## Testes executados`, `## Conclusão`; `**Status**: verified` | `## Summary`, `## Checks Performed` (table Check/Command/Result/Notes), `## Output Excerpts`, `## Residual Risks`, `## Recommendation`; `- **Result**: verified\|partial` | same as newer family; Flora used `Result: partial` |
| Unknowns | `[NV]` (não verificado), `[registrado pós-merge]`, append-only `## Converge` | `[NEEDS CLARIFICATION: ...]` inline; never fill with guesses | Flora uses `[NEEDS CLARIFICATION: ...]` — keep |
| Remote CI | records run/job/PR ids and durations; negative control optional → `[NV]` if not run | `Result: partial` + `not-run` rows when repro requires remote runner | Flora marked Forgejo re-run and GitHub parity `not-run` and `Result: partial` — keep |

**Net rule:** copy Flora's existing entry structure exactly (it is already the newer dark-factory
format), and additionally adopt dark-factory's honesty markers: `[NV]` for anything not observed,
`[NEEDS CLARIFICATION: ...]` for unknowns, `Result: partial` + `not-run` rows when the decisive
check is a remote CI run, and an append-only `## Converge` line when later evidence arrives.

## 2. Evidence to quote (all already verified for this slug)

Quote these **verbatim**; do not paraphrase counts.

1. Workflow defect — `.github/workflows/rust.yml`:
   ```
   32:      - name: Build
   33:        run: cargo build
   34:      - name: Test
   35:        run: cargo test
   ```
   (Contrast `db-tests` job lines 72–83, which do use `--ignored` + explicit filters; the bug is
   specifically the `build` job's required `Test` check.)
2. Root manifest shape — `Cargo.toml`: `[package] name = "flora"` coexists with `[workspace]`
   `members = [flora-core, flora-api, ...]` and **no `default-members`**; root `src/main.rs` is
   `fn main() { println!("Hello, world!"); }`.
3. Live local proof (run 2026-11-16 in this checkout):
   ```
   $ cargo test
        Running unittests src/main.rs (target/debug/deps/flora-4a60960422547d4e)
   running 0 tests
   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   ```
   and `cargo test --workspace -- --list | grep -cE ': test$'` → **148**;
   `cargo test --workspace -- --list --ignored | grep -cE ': test$'` → **44**.
4. Primary-doc quotes already captured in
   `docs/research/2026-11-16-035-security-supplychain-global.md:99-104`:
   cargo-test(1): *"If the manifest is the root of a workspace then the workspaces default members
   are selected ..."* / *"--workspace = Test all members in the workspace."*; Cargo Workspaces:
   *"When a root package is present ... the root package will be used."* Research verdict:
   *"fix only (a) — `.github/workflows/rust.yml:35`: `cargo test` → `cargo test --workspace`"*,
   confidence high. Its §101 states 149 test attributes (43 `#[ignore]` DB-gated) ⇒ ~106 actually
   execute, up from 0; note the small discrepancy with the live 148/44 count as
   `[NEEDS CLARIFICATION: attribute count vs discovered test-case count]`, not as a contradiction.
5. Do **not** quote any Forgejo run URL/id for this bug — none was supplied. Record
   `Source` as local inspection + research doc, with a URL-policy note: "no URL supplied; host N/A;
   no remote fetch". Mark all remote confirmation `[NV]`.

## 3. assessment.md outline (ready to use)

```
# Bug Assessment: CI required "Test" step runs zero tests

- **Slug**: ci-test-step-runs-zero-tests
- **Created**: 2026-11-16
- **Source**: local inspection (.github/workflows/rust.yml, Cargo.toml, src/main.rs) + docs/research/2026-11-16-035-security-supplychain-global.md:99-104 (no URL supplied; host N/A; no remote fetch)
- **Verdict**: valid
- **Severity**: high (control-plane: required check green by construction; blocks trust in CI, not merges)

## Report (verbatim or summarized)
[quote evidence §2.1–2.3 verbatim, incl. the `running 0 tests` line; state that the required check
passed despite executing zero test cases]

## Symptom
Observed: `Test` step (`cargo test`) exits 0 with `running 0 tests` in the `flora` root package.
Expected: it exercises the workspace suites (148 discovered cases; 44 `#[ignore]` DB-gated left to
`db-tests`). Contrast: dark-factory BUG-002 "required check verde sem verificar nada".

## Reproduction
1. From workspace root, run `cargo test` (mirrors rust.yml:35).
2. Observe only `unittests src/main.rs` → `running 0 tests`.
3. Run `cargo test --workspace -- --list` → 148 cases (44 ignored).
[NEEDS CLARIFICATION: exact Forgejo run/job id and displayed test count for the `Test` step — [NV]]

## Suspected Code Paths
- `.github/workflows/rust.yml:34-35` — required `Test` step, bare `cargo test`.
- `Cargo.toml` — root `[package]` + `[workspace]`, no `default-members`.
- `src/main.rs` — placeholder `main`, 0 tests.
- (out of scope, do not conflate) `db-tests` job lines 72-83; staging `if:` in docker-staging.yml.

## Root Cause Hypothesis
Confidence: high (primary Cargo docs + local reproduction). Cargo's default-member rule selects the
root `flora` package; its binary has no tests; no `--workspace`, so no member suites run.

## Proposed Remediation
**Preferred:** one-line change at rust.yml:35 — `cargo test` → `cargo test --workspace`.
**Alternatives:** delete root `[package]` (virtual workspace) or add `default-members = [...]`
(larger blast radius, changes local `cargo test` semantics); pin the gate in a script. Note DB-gated
tests remain `#[ignore]` and are covered by the separate `db-tests` job — do not add `--ignored` here.
**Files likely to change:** `.github/workflows/rust.yml` (line 35 only).
**Tests to add or update:** none; verification is CI execution showing a non-zero test count.

## Risks & Considerations
- Raises executed tests from 0 to ~106; previously "green" checks were meaningless.
- Must keep GitHub Actions parity and the 6 DB-gated `db-tests` steps intact.
- Staging/deploy `if:` and branch protection are separate items — explicitly out of scope.

## Open Questions
- [NEEDS CLARIFICATION: Forgejo run id + displayed Test-step output for a post-fix run]
- [NEEDS CLARIFICATION: whether the `Test` check is a required branch-protection check on dev/main]
```

## 4. fix.md outline

```
# Bug Fix: CI required "Test" step runs zero tests

- **Slug**: ci-test-step-runs-zero-tests
- **Fixed**: 2026-11-16   (use `Status: applied` ONLY after the edit is actually made;
                          otherwise `Status: not-applied` + hold — do not backdate)
- **Assessment**: ./assessment.md
- **Status**: applied | not-applied

## Summary
[one paragraph: replaced bare `cargo test` with `cargo test --workspace` at rust.yml:35 so the
required check exercises the 7 workspace members; DB-gated tests stay in db-tests.]

## Changes
| File | Change | Notes |
|------|--------|-------|
| `.github/workflows/rust.yml` | modified (1 line, line 35) | `run: cargo test` → `run: cargo test --workspace`; no other steps changed. |

## Diff Highlights (optional)
- Before: `run: cargo test`
- After:  `run: cargo test --workspace`

## Tests Added or Updated
- None (CI workflow fix; no unit/integration test harness for the runner itself).

## Local Verification
- `cargo test --workspace -- --list | grep -cE ': test$'` → 148 (44 ignored) — workspace suites are discovered.
- YAML parse: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/rust.yml')); print('yaml-ok')"` → `yaml-ok`.
[If the workflow edit has not been made, say so plainly here and set Status: not-applied.]

## Deviations from Assessment
- None expected (one-line preferred fix). Record any deviation explicitly, as Flora's prior entry
  did for the rustup path and dark-factory `ci-pytest-failure` did for skipped optional assertions.

## Follow-ups
- Re-run Forgejo Actions `rust.yml` and confirm the `Test` step prints a non-zero `test result:` line.
- Monitor GitHub Actions `Rust CI` for parity.
- Consider branch protection only after the check is real (per research doc §103-104).
```

## 5. test.md outline (honest status)

```
# Bug Verification: CI required "Test" step runs zero tests

- **Slug**: ci-test-step-runs-zero-tests
- **Tested**: 2026-11-16
- **Assessment**: ./assessment.md
- **Fix**: ./fix.md
- **Result**: partial        <- keep `partial` until a remote CI run is observed; never `verified`
                                on local evidence alone (Flora precedent)

## Summary
The vacuous step is reproduced locally (`running 0 tests`) and the one-line fix makes workspace
suites discoverable locally. The original failure environment is remote CI; post-fix CI execution
has not been observed, so this closes as partial / Hold, not verified.

## Checks Performed
| Check | Command / Action | Result | Notes |
|-------|------------------|--------|-------|
| Reproduction (pre-fix, mirrored) | `cargo test` at root | pass (bug reproduced) | `running 0 tests`; quote verbatim in §Output Excerpts. |
| Workspace discovery post-fix | `cargo test --workspace -- --list` | pass | 148 cases; 44 ignored (DB-gated, covered by db-tests). |
| Workflow syntax | `python3 -c ... yaml.safe_load(...)` | pass | `yaml-ok` (if run). |
| Reproduction on Forgejo runner | re-run `rust.yml` on `dev` / PR | not-run | [NV] — no remote trigger from this session. |
| GitHub Actions parity | observe `Rust CI` | not-run | [NV] — should be monitored after push. |
| Required-check status on dev/main | branch protection view | not-run | [NV] — admin config, not observable locally. |

## Output Excerpts
```
running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
```
148   # cargo test --workspace -- --list | grep -cE ': test$'
44    # ... --ignored | grep -cE ': test$'
```

## Residual Risks
- 44 DB-gated `#[ignore]` tests still do not run in this step; they depend on the `db-tests` job and
  a live Postgres 18 — a green `Test` is not proof those ran.
- No remote CI confirmation; runner environment (rustup bootstrap, PATH, pinned 1.98.0) may differ.
- Test counts from `--list` are discovered cases, not a guarantee of execution; confirm via CI log.
- [NEEDS CLARIFICATION: exact Forgejo run id / displayed Test-step line — [NV]]

## Recommendation
Hold — do not mark verified. Commit/push the one-line fix, then verify on Forgejo that the `Test`
step prints a non-zero `test result:` line and that GitHub `Rust CI` stays green; if so, append a
`## Converge` line and upgrade `Result` to `verified`. If it still shows zero tests, re-run
`/speckit.bug.assess` with the new log.
```

## 6. Verification-status honesty checklist

1. `Verdict: valid` only when reproduced locally or quoted from a trusted log; otherwise
   `likely valid, needs reproduction` (dark-factory `forgejo-pr-102` precedent).
2. `fix.md` `Status`: `applied` = edit really made; `partial` = mechanism proven but not end-to-end;
   `not-applied` = proposed only. Never write `applied` for an unapplied change.
3. `test.md` `Result`: `verified` only when the decisive check was actually executed and observed;
   use `partial` + `Hold` when it depends on remote CI; `not-run` is a valid table cell, not a pass.
4. Quote exact command + exact output line; distinguish "0 tests executed" from "tests passed".
5. Mark unknowns `[NEEDS CLARIFICATION: ...]`; mark unperformed remote checks `[NV]`; never fill
   with inferred run ids, counts, or "registered post-merge" placeholders presented as facts.
6. Keep an append-only `## Converge` line when later evidence upgrades the status; do not rewrite
   earlier sections (dark-factory BUG-002 / research-doc convention).
