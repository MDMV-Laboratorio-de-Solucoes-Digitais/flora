# 036 — Cargo policy runner (`mdmv-dark-factory/scripts/policy/check_policy.py`) — read-only

**Date:** 2026-11-16
**Scope:** Can Flora invoke the hub's policy runner to validate `cargo` commands, and does `policy.yaml` hold cargo rules worth adopting?
**Sources:** `mdmv-dark-factory/scripts/policy/check_policy.py`, `.../policy.yaml` (verified by running the script). Complements `2026-11-16-036-dark-factory-tools-applicability.md`.
**Verdict:** Invocable, but advisory-only; **no cargo-specific rules to adopt**.

## Runner behavior (verified)

`check_policy.py --command <str> --policy <path>` is the one genuinely portable hub tool: no hub `ROOT` coupling, explicit `--policy`, stdlib + PyYAML. It loads YAML, then:
1. forbidden-pattern regex `(?i)\b{pat}\s*[=:]` over the whole command → `deny` (exit 1);
2. basename of the first whitespace token not in `allowed_commands` → `escalate` (exit 2);
3. otherwise `allow` (exit 0); prints one JSON object (`decision`, `reason`, `triggered_by_rule`).

Observed:
- `--command cargo ... --policy policy.yaml` → `allow` / exit 0
- `--command "cargo test --workspace" ...` → `allow` / exit 0
- `--command "cargo test --workspace TOKEN=abc" ...` → `deny` / exit 1 (`forbidden secret pattern: token`)
- `--command "curl http://x" ...` → `escalate` / exit 2 (`unknown command: curl`)

So it validates only the **program name** and a secret-pattern guard — it does **not** parse subcommands/flags. `cargo test --workspace`, `cargo build`, and `cargo --help` are indistinguishable; any `cargo ...` is allowed.

## Relevant policy entries (quoted)

```yaml
# scripts/policy/policy.yaml
allowed_commands:          # line 2
  - cargo                  # line 3 — the only cargo "rule": bare allowlist entry
  - git
  - fj
  - python3
  - python
  - bash
  - bwrap

secret_env_vars:           # line 11
  - CASV_LLM_REMOTE_API_KEY # hub runner-specific

forbidden_patterns:        # line 14
  - password
  - token
  - secret
  - api_key
  - access_key

escalation_triggers:       # line 56
  - gate_failure
  - policy_deny
  - unknown_command
```

(The remaining `model_routing` block, lines 20–55, is hub LLM-cost config, unrelated to cargo.)

## Can Flora use it?

- **Yes, mechanically:** `python3 "$DF/scripts/policy/check_policy.py" --command "cargo test --workspace" --policy "$DF/scripts/policy/policy.yaml"` works from Flora (cite the sibling note) and returns `allow`. Needs PyYAML installed.
- **Only as an advisory pre-flight**, never a Flora CI gate: the allowlist/denylist encode the *hub's* autonomous runner (`fj`, `bwrap`, `CASV_LLM_REMOTE_API_KEY`), not Flora's toolchain. Flora's own constraints already live in `Cargo.toml` lints and `.github/workflows/{rust,ci}.yml` (fmt, strict clippy, cargo-deny, build/test, DB-gated suites).
- **No cargo rules worth adopting:** `policy.yaml` has zero cargo subcommand/flag/argument rules. The generic secret-pattern guard and `- cargo` allowlist entry add no coverage beyond existing Flora gates; copying the file would couple an archived repo to hub policy and introduce a PyYAML dep for no test benefit.
