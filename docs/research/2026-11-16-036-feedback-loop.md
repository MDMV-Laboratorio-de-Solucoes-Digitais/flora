# 036 — Tightest red/green feedback loop: CI `Test` step runs zero workspace tests

- **Date:** 2026-11-16
- **Scope:** research + design only. No edits to source, workflow, or bug artifacts. This file is the sole artifact written.
- **Bug slug:** `ci-test-step-runs-zero-tests` (target: `.specify/bugs/ci-test-step-runs-zero-tests/{assessment,fix,test}.md`)
- **State caveat:** the one-line fix was applied to the working tree **concurrently during this research** (parent session). At the time of first inspection `.github/workflows/rust.yml:35` was `run: cargo test`; it is now `run: cargo test --workspace` (uncommitted, together with `cargo build --workspace` at line 33). The RED excerpt below was actually observed earlier in this same session, before that edit; the GREEN numbers were observed after. The loop below is state-independent: it reports RED or GREEN from whatever the file currently says.

---

## 1. The loop contract (what "red" and "green" mean)

| Phase | Sole predicate | Expected observation | Gate |
|---|---|---|---|
| RED (pre-fix) | The Test step's `run` from YAML, executed verbatim, selects **0** tests | `CI_TEST_STEP_RUN=cargo test`, `EXECUTED=0`, `running 0 tests` | exit 1 |
| GREEN (post-fix) | Same predicate, same gate script, now selects **104** non-ignored tests | `CI_TEST_STEP_RUN=cargo test --workspace`, `EXECUTED=104`, `IGNORED=44`, exit 0 | exit 0 |

**Key design choice:** the gate reads the command *out of the workflow YAML* and executes that exact string, rather than hardcoding `cargo test --workspace`. That way GREEN proves the **CI step's actual behavior**, not a locally assumed command. A static-only `--workspace in run` check is also given (Gate A) but is weaker: it proves the text changed, not that tests run.

**Why 104 and not 148:** verified locally this checkout has `148` discovered cases (`cargo test --workspace -- --list | grep -cE ': test$'`) of which `44` are ignored (`... --ignored ...` → `44`; 43 DB-gated `#[ignore]` unit tests + 1 ignored doc fence). `cargo test --workspace` reports `104 passed; 0 failed; 44 ignored` → **104 executed**. Assert on `EXECUTED == 104` (pass count), not on discovered count: ignored tests are out of scope (owned by the `db-tests` job, lines 72–83), and a pass count proves execution.

**Root cause (confirmed, for both phases):** `Cargo.toml` has `[package] name = "flora"` (line 8) alongside `[workspace]` (line 17) with **no `default-members`** → non-virtual workspace → Cargo selects only the root package. `cargo metadata --no-deps --format-version 1` gives `workspace_default_members = ['path+file:///…/flora#0.1.0']` out of `8` packages. Root `src/main.rs` is `fn main() { println!("Hello, world!"); }` → zero tests. Bare `cargo test` therefore exits 0 with `running 0 tests`, making the required check green by construction.

---

## 2. Preconditions / reproducibility caveats

- Python 3 + PyYAML required for the YAML gate: verified `pyyaml-ok 6.0.3` (`python3 -c "import yaml; print(yaml.__version__)"`).
- `cargo` on PATH here is `/home/luis/.local/bin/cargo` (a wrapper), version `1.99.1`; CI pins `1.98.0` via rustup in `rust.yml:13–19`. For maximum determinism use the real binary: `~/.cargo/bin/cargo --version` (also `1.99.1` locally). The gate commands below call `cargo` as the workflow does; record the version used as evidence.
- All counts below were measured on a warm `target/` cache with **no** Postgres/Valkey/Meilisearch running; the 104 non-ignored tests need no external service (helpers use `PgPool::connect_lazy` and issue no queries). CI's cold-cache compile time is the only new cost, not test semantics.
- The parser keys on the step with `name: Test` inside `jobs.build.steps` — that label is explicit (`rust.yml:34`) and stable across command-text edits. Do not key on line number; line numbers shift.

---

## 3. Gate A — static YAML check (fastest; text-level RED/GREEN)

One-liner (exit 0 = GREEN, 1 = RED):

```bash
python3 -c "import yaml,sys; r=next(s['run'].strip() for s in yaml.safe_load(open('.github/workflows/rust.yml',encoding='utf-8'))['jobs']['build']['steps'] if s.get('name')=='Test'); print('CI_TEST_STEP_RUN='+r); print('WORKSPACE_SCOPED='+str('--workspace' in r)); sys.exit(0 if '--workspace' in r else 1)"
```

- RED output: `CI_TEST_STEP_RUN=cargo test` / `WORKSPACE_SCOPED=False` / exit 1.
- GREEN output: `CI_TEST_STEP_RUN=cargo test --workspace` / `WORKSPACE_SCOPED=True` / exit 0.
- Also validate syntax once: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/rust.yml')); print('yaml-ok')"` → `yaml-ok`.

## 4. Gate B — tightest combined loop (RECOMMENDED)

Parses the Test step, executes its `run` string, aggregates `test result:` lines, and asserts 104 executed tests. Single command, single exit status, covers both RED and GREEN:

```bash
python3 - <<'PY'
import re, subprocess, sys, yaml
steps = yaml.safe_load(open('.github/workflows/rust.yml', encoding='utf-8'))['jobs']['build']['steps']
run = next(s['run'].strip() for s in steps if s.get('name') == 'Test')
print(f'CI_TEST_STEP_RUN={run}')
p = subprocess.run(run, shell=True, capture_output=True, text=True)
counts = [tuple(map(int, m)) for m in re.findall(r'test result:.*?(\d+) passed;.*?(\d+) ignored', p.stdout)]
executed = sum(a for a, _ in counts)
ignored = sum(b for _, b in counts)
print(f'EXECUTED={executed} IGNORED={ignored} EXIT={p.returncode}')
ok = p.returncode == 0 and executed == 104
print('VERDICT=' + ('GREEN' if ok else 'RED'))
sys.exit(0 if ok else 1)
PY
echo "gate_exit=$?"
```

Outputs in this checkout:

```
RED:  CI_TEST_STEP_RUN=cargo test
      running 0 tests
      EXECUTED=0 IGNORED=0 EXIT=0
      VERDICT=RED            (gate_exit=1)
GREEN: CI_TEST_STEP_RUN=cargo test --workspace
      EXECUTED=104 IGNORED=44 EXIT=0
      VERDICT=GREEN         (gate_exit=0)
```

GREEN was run end-to-end after the concurrent edit. RED is reconstructed from two components **actually observed earlier in this session** — the parser returning `CI_TEST_STEP_RUN=cargo test` from the then-current file, and the real bare-`cargo test` output `running 0 tests` / `0 passed` — because the file had already changed before Gate B was assembled. It is deterministic, not `[NV]`, but mark it in `test.md` as "RED mirrored, not re-run end-to-end" for full honesty.

Note RED exits 0 from cargo yet the gate exits 1 — this is the whole point: a green CI check can mask zero coverage. The predicate is "tests executed", not "process exit code".

## 5. Gate C — authoritative direct runs (audit-grade evidence)

Use these to produce the literal stdout lines quoted into the bug artifacts:

```bash
# RED mirror of rust.yml:35 (pre-fix)
cargo test 2>&1 | grep -E 'Running|running [0-9]+ tests|test result:'

# GREEN: authoritative workspace run (post-fix)
cargo test --workspace 2>&1 | grep 'test result:'

# Aggregate exactly (avoid brittle sed):
cargo test --workspace 2>&1 | grep 'test result:' \
  | awk '{for(i=1;i<=NF;i++){if($i=="passed;")p+=$(i-1); if($i=="ignored;")ig+=$(i-1)}} \
          END{print "passed="p, "ignored="ig, "selected="p+ig}'
# -> passed=104 ignored=44 selected=148

# Discovery / ignore inventory (counts only, never dump the name list):
cargo test --workspace -- --list | grep -cE ': test$'              # 148
cargo test --workspace -- --list --ignored | grep -cE ': test$'    # 44
cargo metadata --no-deps --format-version 1 \
  | python3 -c "import json,sys; d=json.load(sys.stdin); print('default_members=',d['workspace_default_members'],'members=',len(d['workspace_members']))"
```

Per-package GREEN breakdown (from the `test result:` lines, safe to tabulate): root `flora` 0; `flora-api` lib 25 + main 0; `flora-core` lib 12 + `tests/models.rs` 21; `flora-files`/`flora-messaging`/`flora-search`/`flora-tasks` 0 each; `flora-tests` lib 46 passed + 43 ignored; doc-tests 0 passed + 1 ignored. Sum: 104 passed, 44 ignored.

**Do not add `--all-targets`**: it drops the doc-test phase and adds no coverage (0 benches, 0 examples). Keep the fix as the single flag `--workspace`.

---

## 6. Evidence capture matrix (exact items for each artifact)

### `assessment.md` — prove the bug (RED)
1. Verbatim `rust.yml:34–35`:
   ```
   - name: Test
     run: cargo test
   ```
   plus current line number and the note that the `db-tests` job (lines 72–83) does use `--ignored` filters — the defect is only the `build` job's required check.
2. Verbatim `Cargo.toml:7–8` (`[package] name = "flora"`) + `:17–34` (`[workspace]`, `members`, **no `default-members`**) and root `src/main.rs` placeholder body.
3. `cargo metadata` line: `workspace_default_members = ['path+file:///…/flora#0.1.0']`, `members = 8`.
4. RED stdout, verbatim:
   ```
   Running unittests src/main.rs (target/debug/deps/flora-4a60960422547d4e)
   running 0 tests
   test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
   ```
5. Verdict `valid`; severity `high` (control-plane: required check green by construction). Source = local inspection + local run; **no URL supplied** → record "host N/A; no remote fetch" and mark all remote confirmation `[NV]`. Unknowns as `[NEEDS CLARIFICATION: …]` (e.g. exact Forgejo run id / displayed Test-step line, branch-protection status).

### `fix.md` — prove the change (GREEN)
1. Exact diff hunk (from `git -C . diff -- .github/workflows/rust.yml`), limited to the Test step:
   ```
   -        run: cargo test
   +        run: cargo test --workspace
   ```
   Note: the same working-tree diff also changes line 33 to `cargo build --workspace`; that is **out of scope** for this slug and the Gate B predicate deliberately ignores it (keys only on the Test step). Record it under `Deviations from Assessment` if the parent session treats it as part of the same edit.
2. `yaml-ok` from the parse check.
3. Gate B GREEN line set: `CI_TEST_STEP_RUN=cargo test --workspace`, `EXECUTED=104 IGNORED=44 EXIT=0`, `VERDICT=GREEN`, `gate_exit=0`.
4. `Status: applied` only while the edit is actually present; `not-applied` otherwise. Do not backdate.
5. `Tests Added or Updated: none` (workflow-only fix).

### `test.md` — prove verification honestly
A checks table with these rows (exact command → exact observed result):

| Check | Command | Expected result |
|---|---|---|
| RED reproduction (mirrors old `rust.yml:35`) | `cargo test` | `running 0 tests`; `0 passed` — pass row = bug reproduced |
| Static YAML gate | Gate A one-liner | RED pre-fix (`WORKSPACE_SCOPED=False`), GREEN post-fix (`True`) |
| Combined gate | Gate B heredoc | `EXECUTED=0 → 104`, `VERDICT=RED → GREEN`, exit `1 → 0` |
| Authoritative workspace run | Gate C `cargo test --workspace` | `104 passed; 0 failed; 44 ignored` |
| Discovery inventory | `-- --list` counts | `148` discovered, `44` ignored |
| YAML syntax | `yaml.safe_load` | `yaml-ok` |
| Toolchain used | `~/.cargo/bin/cargo --version` | `cargo 1.99.1` locally; CI pins `1.98.0` |
| Reproduction on Forgejo runner | re-run `rust.yml` | `not-run` / `[NV]` |
| GitHub Actions parity (`Rust CI`) | observe after push | `not-run` / `[NV]` |
| Required-check status on dev/main | branch-protection view | `not-run` / `[NV]` |

`Result: partial` + `Hold` until a remote CI log shows a non-zero `test result:` line; then append an append-only `## Converge` line and upgrade to `verified`. Include the RED and GREEN stdout excerpts verbatim in `## Output Excerpts`. Residual risks to state: 44 ignored tests still not run here (db-tests owns them); no remote confirmation; `--list` counts are discovery, not execution; local toolchain `1.99.1` vs CI `1.98.0`.

---

## 7. Redaction rules (no secrets)

**Never capture or paste into any artifact:**

- `FLORA_APP__JWT_SECRET` / `FLORA_APP__SESSION_TTL_SECS` values (`rust.yml:133–134`); the JWT value is a dev placeholder but is still an env secret-shaped value — do not echo it.
- `FLORA_DATABASE__POSTGRES_URL`, `DATABASE_URL`, `PGPASSWORD` (`rust.yml:54–55, 132, 140`) and any connection string with credentials.
- `secrets.*`, `GITHUB_TOKEN`, `${{ … }}` expressions, or any `env:` block dump.
- Bearer/session tokens used by `test-flora.sh`, cookies, or any auth header values.
- Remote run URLs, run ids, job ids, PR numbers, or commit SHAs **unless actually supplied and observed** — otherwise `[NV]`, never invented.
- Full `cargo test` output dumps, full `env`, or full `--list` name dumps: capture only the enumerated `test result:` / `running N tests` / `CI_TEST_STEP_RUN=` / `EXECUTED=` lines and the four count integers. (Binary names with hashes are benign but unnecessary; keep the short `unittests src/main.rs` form.)

**Safe to capture:** the Test step `run` string and its line number, fixed test-result lines (they contain only counts/timings), the `cargo --version` string, `cargo metadata` default-member list, boolean gate verdicts/exit codes, and the YAML parse token `yaml-ok`.

**Habit:** before writing an artifact, grep the captured block for `SECRET|TOKEN|PASSWORD|POSTGRES_URL|DATABASE_URL|Bearer|github_token|secrets\.` and confirm zero hits. If any hit, remove the line and record `[redacted]` — do not paraphrase the value.

---

## 8. Limits of this design

- Gate B executes shell from YAML (`shell=True`); safe here because the source is the checked-in workflow under review, not untrusted input. Do not point it at a PR from a fork without review.
- The 104 constant is checkout-specific. If tests are added/removed/`#[ignore]` changes, the gate will (correctly) go RED; update the constant only with a documented recount of `148 discovered / 44 ignored`.
- A GREEN local gate is necessary but not sufficient: the decisive evidence is a remote `rust.yml` run showing a non-zero `test result:` line. Until then `test.md` stays `partial`/Hold.
- Measurements used local `cargo 1.99.1` (both PATH wrapper and `~/.cargo/bin/cargo`); CI's pinned `1.98.0` was not executed locally — mark that difference, do not assume equivalence.
