# 036 — mdmv-dark-factory tools: applicability to the Flora bug-fix cycle

- **Date**: 2026-11-16
- **Type**: research only (no Flora source/CI edits performed)
- **Hub inspected**: `/home/luis/development/MDMV/projetos/mdmv-dark-factory` (git repo, Forgejo remote)
- **Target**: `/home/luis/development/MDMV/projetos/Arquivados/flora` (archived Rust workspace)
- **Trigger**: upcoming bug-fix cycle for `CI cargo test` → `cargo test --workspace` (the gate in `.github/workflows/rust.yml:35` is plain `cargo test`, which only exercises the root placeholder `src/main.rs`; clippy already uses `--workspace` at line 27).

## 1. Bottom line

1. `mdmv-dark-factory` is a **docs/governance hub, not a library or a CI toolkit for other repos**. Every gate script derives `ROOT = Path(__file__).resolve().parents[2]` (the hub checkout) and reads hub-only artifacts (`AGENTS.md`, `specs/NNN-*`, `.specify/{assessments,validations,releases}`, `.forgejo/workflows/`, `docs/governance/*.json`, hub telemetry/holdouts). Running them from Flora still scans/validates the **hub**, not Flora.
2. Exactly **one** named tool is genuinely portable as-is: `scripts/policy/check_policy.py`, because it takes an explicit `--policy` path and evaluates an arbitrary command string. Even there, the policy is the *hub runner's* allowlist, so the result is advisory, not a Flora gate.
3. `check_secrets_hygiene.py` is portable **as a method** (stdlib, single file, no args) but must be copied into Flora (or its `ROOT` line patched) to scan Flora.
4. `check_sdd_completeness.py`, `check_policy_drift.py`, `hub_docs_gate.py`, and `check_production_gate.py` are **hub-specific / inapplicable** to Flora. Pointing `--root` at Flora yields either a vacuous green or a meaningless stale-baseline failure — the exact "green without verifying anything" anti-pattern the hub tracks as its own BUG-002.
5. The **legitimately transferable asset is the bug-flow convention** in `.specify/bugs/` (assess → fix → test, three Markdown artifacts, evidence-before-done). It can be adopted manually in Flora with no hub code, and it fits the `cargo test` → `cargo test --workspace` bug well.

Shorthands below: `DF` = hub root, `FL` = Flora root.

```bash
DF=/home/luis/development/MDMV/projetos/mdmv-dark-factory
FL=/home/luis/development/MDMV/projetos/Arquivados/flora
```

## 2. Executable tool inventory (hub)

No `Cargo.toml`/`package.json`/`pyproject.toml` at hub root. Executable code = stdlib Python 3 + POSIX shell.

| Area | Scripts (under `DF`) | Purpose |
|---|---|---|
| CI gates (`scripts/ci/`) | `hub_docs_gate.py`, `check_production_gate.py`, `check_sdd_completeness.py`, `check_secrets_hygiene.py`, `check_policy_drift.py`, `check_code_review_readiness.py`, `check_no_human_eligibility.py` | deterministic structural/policy gates, all hub-root-bound |
| Policy (`scripts/policy/`) | `check_policy.py` + `policy.yaml` | allow/deny/escalate a proposed command; the one portable checker |
| Runner (`scripts/runner/`) | `job_queue.py`, `job_dispatch.py`, `ingress_daemon.py`, `micro_harness.py`, `worktree_pool.py`, `worktree_hygiene.py`, `task_eligibility.py`, `job_scope_gate.py`, `check_job_schema.py`, `prepr_verifier.py`, `token_fence.py`, `retry_cap.py`, `shadow_digest.py`, `mark_pr_issues.py` (most with `--self-test`) | autonomous job intake/dispatch; hub-internal |
| Holdout (`scripts/holdout/`) | `run_holdout.py`, `checker.py`, `integrity.py` | scenario replay (casv-clippy-fmt) against a target repo |
| Sandbox (`scripts/sandbox/`) | `path_guard.py`, `run_bwrap.sh` | bubblewrap path allowlist / version floor 0.12.0 |
| Telemetry (`scripts/telemetry/`) | `validate_telemetry.py`, `ingest_telemetry.py`, `hash_chain.py`, `audit_chain.py`, `journal_relay.py`, `pr_join.py`, `telemetry_report.py`, `cost_report.py`, `scan_project_returns.py`, `emit_telemetry.py`, `wrap_telemetry.py`, `eval_stealth_*.py` | append-only execution evidence; hub-internal |
| Review / git / ops / automation / commercial | `scripts/review/code_review_engine.py` + `ocr_*.py`, `scripts/git_gateway/{router,ports,adapters}.py`, `scripts/ops/factory_health_server.py`, `ops/factory-cron/entrypoint.sh`, `scripts/automation/docs_merge_controller.py`, `scripts/commercial/*` | hub operations, not applicable to Flora |
| Spec Kit extension | `.specify/extensions/bug/commands/speckit.bug.{assess,fix,test}.md` + `.specify/bugs/README.md` | the transferable bug workflow |
| Verification (hub) | `python3 -m pytest -q` (tests/ mirrors scripts/; needs PyYAML + bubblewrap) | tests the **hub**, not Flora |

Note: `production-gate-hub.policy.json` references `scripts/ci/check_task_id_uniqueness.py` and `scripts/ci/check_converge_append_only.py`, which **do not exist in this checkout** (verified by glob) — further evidence that the hub gate is a drifting, self-referential control plane, not a reusable harness.

## 3. Focus tools — anatomy, exact commands, verdicts

### 3.1 `scripts/policy/check_policy.py` + `scripts/policy/policy.yaml` — APPLICABLE (advisory)

- **What it does**: loads YAML policy; forbidden-pattern regex check (`password|token|secret|api_key|access_key` followed by `=` or `:`) → `deny`; program name (basename of first token) not in allowlist → `escalate`; else `allow`. Prints JSON; exits 0 allow / 1 deny / 2 escalate. `policy.yaml` allowlist = `cargo, git, fj, python3, python, bash, bwrap`; `secret_env_vars: [CASV_LLM_REMOTE_API_KEY]`; rest is hub model-routing config.
- **Dependency**: `import yaml` (PyYAML required). It is the only named script with a hard `import yaml` in its own logic (`hub_docs_gate.py` degrades gracefully without it).
- **Exact command (safe to run now, reads only)**:

```bash
python3 "$DF/scripts/policy/check_policy.py" \
  --command "cargo test --workspace" \
  --policy "$DF/scripts/policy/policy.yaml"
# -> {"decision": "allow", "reason": "command allowed by policy", "triggered_by_rule": "allowlist"}  (exit 0)
```

- **Applicability**: legitimate as a cheap deterministic pre-flight "is this command within factory policy?" check for the agent driving the bug fix. **Not** a Flora CI gate: the allowlist/denylist encode the hub's autonomous runner (incl. `fj`, `bwrap`, `CASV_LLM_REMOTE_API_KEY`), and the verdict is informative only under hub constitution Principle V (LLM output never routes state; deterministic gates are authoritative *within the hub*). Do not copy `policy.yaml` into Flora CI — it would couple an archived repo to hub policy and add a PyYAML dependency for no test-coverage benefit.

### 3.2 `scripts/ci/check_secrets_hygiene.py` — APPLICABLE AS METHOD (requires relocation)

- **What it does**: single-file stdlib regex scan over the whole script-relative `ROOT` (skips `.git/node_modules/target/.venv/__pycache__`, binary suffixes, and paths containing `.tokens/` or its own filename). Patterns: PEM private keys, `gh[pousr]_…`, 40-hex forgejo token-like, `AKIA…`, `xox[baprs]-…`, quoted `password|secret|api[_-]?key|token = '…'` assignments (placeholders `CHANGEME/example/REDACTED/TODO/${...}` exempt). Prints `secrets-hygiene: PASS/FAIL (scanned N, M findings)`; exit 0/1.
- **No args and `ROOT` is derived from `__file__`** → running it from `DF` scans the hub; `cd $FL` does not change that. To scan Flora you must place the script inside Flora (a **future edit**, not done here) or patch one line to `ROOT = Path.cwd()`:

```bash
# FUTURE action (not performed in this research task):
install -D -m 0644 "$DF/scripts/ci/check_secrets_hygiene.py" "$FL/scripts/ci/check_secrets_hygiene.py"
cd "$FL" && python3 scripts/ci/check_secrets_hygiene.py
# or, without adding a tracked file:
python3 -c 'import runpy, pathlib, sys; sys.argv=[...]; exec(pathlib.Path("'$DF'/scripts/ci/check_secrets_hygiene.py").read_text().replace("Path(__file__).resolve().parents[2]", "Path.cwd()"))'
```

- **Applicability**: the *method* is appropriate for the bug cycle — a CI/workflow change is a sensitive diff, so a one-shot secret scan of the touched tree is cheap evidence. Caveats: heuristic, can false-positive on Flora's `dev-seed.sql`, test fixtures, and docs/`docs/research/*`; the harness already runs `cargo deny check bans licenses sources`, which covers supply-chain policy, not credential leaks. Run and triage, never auto-block on a single regex hit.

### 3.3 `scripts/ci/check_sdd_completeness.py` — INAPPLICABLE / VACUOUS for Flora

- **What it does**: for every `specs/[0-9][0-9][0-9]-*` dir, requires `spec.md`, `plan.md`, `tasks.md`, a `checklists/*.md`, and by slug matching dirs under `.specify/{assessments,validations,releases}`; compares against `docs/governance/sdd-completeness-baseline.json`; fails on regressions **and** stale baseline entries. This is an SDD-lifecycle completeness gate for the hub's own 80+ numbered specs (currently `--root` and `--baseline` are overridable).
- **Exact commands and why both are useless for Flora**:

```bash
# (a) default baseline (hub's) + Flora root -> meaningless FAIL: every hub baseline key is "stale"
python3 "$DF/scripts/ci/check_sdd_completeness.py" --root "$FL" --json
# (b) no baseline -> vacuous PASS: spec_dirs=0, gaps=0, ok=true  (NOT evidence of anything)
python3 "$DF/scripts/ci/check_sdd_completeness.py" --root "$FL" --baseline /nonexistent --json
```

- **Applicability**: none. Flora has no `specs/NNN-*` dirs and no `.specify/assessments|validations|releases`; case (a) reports hub-specific stale entries, case (b) is a tautological green. Do not cite either as a gate result for the bug fix. (The hub's own AGENTS.md warns counts drift; trust script output — here the script output is empty by construction.)

### 3.4 `scripts/ci/check_policy_drift.py` — INAPPLICABLE (hub↔satellite only)

- **What it does**: compares `hub-policy-manifest.json` (`required_policies`, `hub_sha`, `required_files`) against satellite manifests (`policies_affected`/`adopted_policies`, `files_present`, `hub_sha`); exit 0 aligned/no-satellites (prints explicit `NV`), 1 drift, 2 usage error. Manifest lists `required_policies: [022,023,024,025,026,028,031]` and `required_files: [.forgejo/workflows/hub-docs-gate.yml, scripts/ci/hub_docs_gate.py]`.
- **Exact command (hub root only)**:

```bash
cd "$DF" && python3 scripts/ci/check_policy_drift.py docs/governance/hub-policy-manifest.json
# -> policy-drift: PASS (no satellite manifests provided; NV)
```

- **Applicability**: none. `docs/governance/satellites.yaml` registers only `casv-rust`, `communal`, `crsdd-fabro` — **Flora is not a satellite**. Flora has no adopted-policy manifest, no `hub_sha` agreement, and no Forgejo federation. At most use the one-shot no-arg invocation while in `DF` to record "Flora = NV / not governed by hub policy"; do not fabricate a Flora manifest to make the check pass.

### 3.5 `scripts/ci/hub_docs_gate.py` — INAPPLICABLE as a gate (portable idea only)

- **What it does**: requires `AGENTS.md`, `.specify/memory`, `specs/`; warns if constitution missing; rglobs **every** `*.json`/`*.yml`/`*.yaml` outside `.git|node_modules|target|generated|.direnv` and JSON/YAML-parses it (PyYAML optional, degrades to NUL/tab heuristic); requires `spec.md` in every numbered `specs/` dir; exit 0/1.
- **Exact command (hub root only)**:

```bash
cd "$DF" && python3 scripts/ci/hub_docs_gate.py
```

- **Applicability**: none as a Flora gate. If copied into Flora it would fail on `missing required path: specs` (and `.specify/memory` absent), while its value-add — "all JSON/YAML parse" — is redundant with Flora's own CI/tooling. Useful only as a **design pattern** for a future Flora-native docs gate (require `AGENTS.md`, parse checked-in JSON/YAML), which is out of scope for a bug-fix cycle.

### 3.6 `scripts/ci/check_production_gate.py` — INAPPLICABLE (self-referential hub control plane)

- **What it does**: reads `docs/governance/production-gate-hub.policy.json`, runs each `required_scripts_pass` entry via `subprocess.run(["python3"] + spec.split(), cwd=ROOT)`, flags `continue-on-error: true` in `.forgejo/workflows/hub-docs-gate.yml`, optionally checks `--release <slug>` evidence, emits JSON (`schema 043.2.0`, `human_approval_required: false`). The policy list includes all of §2 plus pytest, holdout, telemetry, and job self-tests.
- **Exact command (hub root only)**:

```bash
cd "$DF" && python3 scripts/ci/check_production_gate.py --check
```

- **Applicability**: none. ROOT and policy are hub-bound; it cannot be aimed at Flora. Its policy is already drifted (two referenced scripts absent, see §2), and a green result is explicitly **not** a merge license (hub AGENTS.md: required checks/branch protection are `NV`). For the Flora cycle, use the *pattern* — one deterministic script that runs a fixed list of checks and emits JSON — not the script.

## 4. Bug-flow conventions in `.specify/bugs/` — TRANSFERABLE (recommended)

Source: `DF/.specify/bugs/README.md`, `DF/.specify/extensions/bug/README.md`, `speckit.bug.{assess,fix,test}.md`, and worked examples (`ci-pytest-failure/`, `sdd-057-missing-evidence/`, `prepr-verifier-hook-timeout*`).

- **Three stages, one dir per bug**: `.specify/bugs/<slug>/{assessment.md,fix.md,test.md}`, driven by `/speckit.bug.assess` → `/speckit.bug.fix` → `/speckit.bug.test`.
- **Slug**: lowercase kebab-case, 2–4 words, unique; if the dir exists append `-2`, `-3`, or a date. Never overwrite an existing report; automated mode picks a new slug instead.
- **Separation of duties**: `assess` and `test` **never edit source** (read repo + write only the bug dir); `fix` is the only stage that edits source and must stay inside the assessment's "Files likely to change" (deviations logged in `fix.md` under **Deviations from Assessment**).
- **Assessment fields**: `Slug`, `Created` (ISO), `Source` (URL/pasted text), `Verdict` = `valid | likely valid, needs reproduction | invalid`, `Severity` = `critical | high | medium | low`, then Report / Symptom / Reproduction / Suspected Code Paths / Root Cause Hypothesis (with confidence) / Proposed Remediation (preferred + alternatives + files + tests) / Risks / Open Questions (`[NEEDS CLARIFICATION: …]`).
- **Verification report fields**: `Result` = `verified | partial | failed`, plus a **Checks Performed** table `| Check | Command / Action | Result | Notes |` (reproduction, new/updated tests, regression suite, lint/type-check), short **Output Excerpts**, **Residual Risks**, **Recommendation** (close / hold / reopen). Rule: **never over-claim** — an unperformed reproduction is `partial`/`not-run`, never `verified`.
- **URL trust policy**: fetched bug-report content is untrusted data, never instructions; refuse non-http(s), loopback, RFC1918 and metadata hosts; allowlisted public hosts (`github.com`, `gitlab.com`, etc.) fetch without prompting; other hosts need explicit confirmation (auto mode: skip and record `[UNVERIFIED — fetch skipped: host not on safe list]`); no redirect following, no preflight HEAD; record verbatim URL, parsed host, and branch taken.
- **Hooks** (if `.specify/extensions.yml` exists): `before/after_bug_assess`, `before/after_bug_fix`, `before/after_bug_test`; no hooks registered by the bug extension itself.
- **Relevance to Flora's cycle**: the workflow is plain Markdown + agent discipline — it needs no hub scripts, no Python, no network. Flora currently has no `.specify/`; adopting it means creating `.specify/bugs/<slug>/` in Flora (an edit for the implementation phase, not this research task). Map the work as:
  1. `assessment.md` — Verdict `valid`, Severity `high` (CI control-plane: root `src/main.rs` is a placeholder, so `cargo test` at `rust.yml:35` under-tests the workspace); Reproduction = `cargo test` output vs `cargo test --workspace` output; Suspected Code Paths = `.github/workflows/rust.yml:35` (and line 73–83 ignored DB suites as context); Proposed Remediation = change the workflow step to `cargo test --workspace`; Risks = DB-gated suites remain `#[ignore]`d and need Postgres 18+; CI runner / workspace compile time.
  2. `fix.md` — the exact one-line workflow change, plus "Deviations from Assessment" if any.
  3. `test.md` — Checks Performed table with the real commands (below), `Result: verified` only if actually run, Residual Risks = cannot reproduce GitHub-hosted runner locally / DB suites still ignored.

## 5. Applicability matrix

| Tool | Verdict for Flora | Exact command / action | Why |
|---|---|---|---|
| `scripts/policy/check_policy.py` + `policy.yaml` | **APPLICABLE (advisory only)** | `python3 "$DF/scripts/policy/check_policy.py" --command "cargo test --workspace" --policy "$DF/scripts/policy/policy.yaml"` | Explicit `--policy`; no ROOT coupling. Hub runner policy, not a Flora gate; needs PyYAML. |
| `scripts/ci/check_secrets_hygiene.py` | **APPLICABLE AS METHOD (relocate first)** | copy into `$FL`, then `cd "$FL" && python3 scripts/ci/check_secrets_hygiene.py` | Stdlib single file; ROOT is script-relative. Heuristic; triage false positives. |
| `.specify/bugs/` + bug extension | **APPLICABLE (convention, manual)** | create `$FL/.specify/bugs/<slug>/{assessment,fix,test}.md` | Pure Markdown discipline; no hub dependency; fits evidence-before-done. |
| `scripts/ci/check_sdd_completeness.py` | **INAPPLICABLE (vacuous/meaningless)** | `--root "$FL"` → hub-baseline stale FAIL; `--baseline /nonexistent` → `ok:true, spec_dirs:0` | Flora has no numbered `specs/` or SDD evidence dirs. |
| `scripts/ci/check_policy_drift.py` | **INAPPLICABLE (NV)** | `cd "$DF" && python3 scripts/ci/check_policy_drift.py docs/governance/hub-policy-manifest.json` | Flora not in `satellites.yaml`; no adopted-policy manifest. |
| `scripts/ci/hub_docs_gate.py` | **INAPPLICABLE (would FAIL if copied)** | `cd "$DF" && python3 scripts/ci/hub_docs_gate.py` | Requires `specs/` + `.specify/memory`; scans hub only. |
| `scripts/ci/check_production_gate.py` | **INAPPLICABLE (self-referential)** | `cd "$DF" && python3 scripts/ci/check_production_gate.py --check` | Hub-bound ROOT/policy; policy lists absent scripts; green ≠ merge. |
| `check_code_review_readiness.py`, `check_no_human_eligibility.py`, holdout/telemetry/runner/sandbox scripts | **INAPPLICABLE** | (hub-root-only; not needed for Flora) | Forgejo protections, hub workflows, hub telemetry, job queue — none exist in Flora. |

## 6. Recommended sequence for the `cargo test` → `cargo test --workspace` cycle

Flora-native evidence (run from `$FL`, no hub dependency):

```bash
cd "$FL"
cargo test                 # baseline: what CI currently does (root placeholder package)
cargo test --workspace     # target: real workspace coverage
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo deny check bans licenses sources
```

Hub tools, used only in their advisory roles:

```bash
python3 "$DF/scripts/policy/check_policy.py" --command "cargo test --workspace" --policy "$DF/scripts/policy/policy.yaml"
# optionally, after relocating the scanner into Flora:
cd "$FL" && python3 scripts/ci/check_secrets_hygiene.py
```

Then record the three bug artifacts in `$FL/.specify/bugs/<slug>/` per §4. Cite **only** commands actually executed; mark any GitHub-runner-only step `not-run`/`NV`.

## 7. Caveats / NV

- All hub gate scripts were **read, not executed**, during this research (executing `check_production_gate.py --check` would run the hub's full pytest/holdout suite and mutate hub caches; not needed to establish applicability). Glob/grep/read evidence is sufficient and reproducible.
- Hub AGENTS.md states required checks / branch protections / auto-merge semantics are **NV (not verified live)**; a green hub gate is never a merge license. The same discipline applies to Flora: a green `cargo test --workspace` is test evidence, not deployment authorization.
- `check_policy.py` needs PyYAML; `check_secrets_hygiene.py` / `hub_docs_gate.py` / `check_sdd_completeness.py` / `check_policy_drift.py` / `check_production_gate.py` are stdlib-only (`hub_docs_gate.py`'s YAML parse degrades gracefully).
- Nothing in this document changes Flora source, CI, config, or dependencies; the sole write is this research file.
