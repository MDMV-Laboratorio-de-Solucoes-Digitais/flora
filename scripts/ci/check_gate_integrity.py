#!/usr/bin/env python3
"""Deterministic guard for the required checks that gate autonomous merges.

Cycle 037 made push/PR/merge a machine decision (ADR-0003). That decision is only
safe while the required checks keep their names, actually run, and exercise the
suite. This script fails if any of those properties is lost. Stdlib only: no
PyYAML, no network, no GitHub API in the CI path.

Run from the repo root:
    python3 scripts/ci/check_gate_integrity.py

Optional live verification of branch protection (local, needs `gh` + network):
    python3 scripts/ci/check_gate_integrity.py --remote
"""
from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / ".github" / "workflows"

# Exact check names installed as required status checks on dev/main (ADR-0003).
REQUIRED = {
    "frontend-build": "ci.yml",
    "rust-build": "rust.yml",
    "db-tests": "rust.yml",
    "dependency-review": "rust.yml",
}

# The only allowed job-level `if:` on a required check: PR-only dependency review.
# A skipped required check counts as success on GitHub; every other skip is a
# vacuous gate. Keep this allowlist as narrow as possible and justify changes in ADR-0003.
ALLOWED_JOB_IF = {
    "dependency-review": "if: github.event_name == 'pull_request'",
}

FLOORS = {
    "auth": 23,
    "messaging": 4,
    "websocket": 8,
    "search": 8,
    "notifications": 5,
    "files": 7,
    "tasks": 4,
    "workspace": 3,
    "rbac": 3,
    "bootstrap": 13,
}

JOB_RE = re.compile(r"^  ([A-Za-z0-9][A-Za-z0-9_-]*):\s*$")

# A CI db leg: `run_leg <label> <filter> <floor>`. Horizontal whitespace only so
# the regex never swallows newlines and accidentally spans multiple lines.
LEG_RE = re.compile(r"^[ \t]*run_leg[ \t]+(\S+)[ \t]+(\S+)[ \t]+(\d+)[ \t]*$", re.M)


def _jobs(text: str) -> dict[str, list[str]]:
    lines = text.splitlines()
    starts = [(i, m.group(1)) for i, line in enumerate(lines) if (m := JOB_RE.match(line))]
    out: dict[str, list[str]] = {}
    for idx, (start, name) in enumerate(starts):
        end = starts[idx + 1][0] if idx + 1 < len(starts) else len(lines)
        out[name] = lines[start:end]
    return out


def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--remote", action="store_true", help="also compare live branch protection via gh")
    args = ap.parse_args(argv)

    failures: list[str] = []
    checks: list[str] = []

    texts: dict[str, str] = {}
    for name in sorted({*REQUIRED.values(), "ci.yml", "rust.yml"}):
        path = WORKFLOWS / name
        if not path.is_file():
            failures.append(f"missing workflow: {name}")
            continue
        texts[name] = path.read_text(encoding="utf-8")

    for check, filename in REQUIRED.items():
        if filename not in texts:
            continue
        jobs = _jobs(texts[filename])
        if check not in jobs:
            failures.append(f"{filename}: required check '{check}' has no matching job id")
            continue
        block = jobs[check]
        if check == "db-tests":
            for var in ("FLORA_DATABASE__POSTGRES_URL:", "DATABASE_URL:"):
                if not any(line.lstrip().startswith(var) for line in block):
                    failures.append(f"rust.yml:db-tests: missing {var} env (DB legs vacuous)")
        if any("continue-on-error: true" in line for line in block):
            failures.append(f"{filename}:{check}: continue-on-error makes a required check vacuous")
        job_ifs = [line.strip() for line in block if line.strip().startswith("if:")]
        allowed = ALLOWED_JOB_IF.get(check)
        for line in job_ifs:
            if line != allowed:
                failures.append(f"{filename}:{check}: unexpected job-level if: {line!r}")
        if allowed and not job_ifs:
            failures.append(f"{filename}:{check}: expected the documented PR-only if: is gone")
        checks.append(f"job {check} present and non-vacuous")

    # Workflow-level path filters leave required checks PENDING on other pushes.
    for filename in ("ci.yml", "rust.yml"):
        if filename in texts and re.search(r"^    paths:", texts[filename], re.M):
            failures.append(f"{filename}: workflow-level paths: filter leaves required checks pending")

    rust = texts.get("rust.yml", "")
    for need in ("cargo build --workspace", "cargo test --workspace"):
        if need not in rust:
            failures.append(f"rust.yml: missing '{need}' (the cycle-036 gate fix regressed)")
        else:
            checks.append(f"rust.yml contains '{need}'")
    floor_failures: list[str] = []
    # 088: dynamic oracle. The floors must match the tests libtest actually
    # discovers, not just a number copied into rust.yml. Only stdout is parsed
    # for test names: cargo may emit warnings on stderr.
    proc = subprocess.run(
        ["cargo", "test", "-p", "flora-tests", "--", "--ignored", "--list"],
        cwd=ROOT, capture_output=True, text=True, check=False,
    )
    if proc.returncode != 0:
        detail = proc.stderr.strip().splitlines()
        tail = detail[-1] if detail else "no stderr output"
        floor_failures.append(f"cargo --ignored --list failed (rc={proc.returncode}): {tail}")

    ignored: list[str] = []
    for raw in proc.stdout.splitlines():
        line = raw.strip()
        if line.endswith(": test"):
            ignored.append(line[: -len(": test")])
    if not ignored:
        floor_failures.append("cargo --ignored --list discovered no tests (empty oracle)")

    legs = [(m.group(1), m.group(2), int(m.group(3))) for m in LEG_RE.finditer(rust)]
    if not legs:
        floor_failures.append("rust.yml: no run_leg <label> <filter> <floor> invocations found")

    seen: dict[str, int] = {}
    for label, filter_, floor in legs:
        if label not in FLOORS:
            floor_failures.append(f"rust.yml: run_leg label {label!r} is not in FLOORS")
            continue
        seen[label] = seen.get(label, 0) + 1
        expected = FLOORS[label]
        if floor != expected:
            floor_failures.append(
                f"rust.yml: db floor for {label} is {floor}, FLOORS says {expected}"
            )
        # startswith is intentional: a prefix filter covers its submodules.
        count = sum(1 for name in ignored if name.startswith(filter_))
        if count != expected:
            floor_failures.append(
                f"rust.yml:db-tests:{label}: {count} ignored tests match {filter_!r}, "
                f"floor is {expected}"
            )
    for label in FLOORS:
        if seen.get(label, 0) != 1:
            floor_failures.append(
                f"rust.yml: expected exactly one run_leg for {label!r}, found {seen.get(label, 0)}"
            )

    if floor_failures:
        failures.extend(floor_failures)
    else:
        checks.append(
            f"all {len(FLOORS)} db contract floors intact ({sum(FLOORS.values())} total)"
        )

    if args.remote:
        repo = subprocess.run(
            ["gh", "repo", "view", "--json", "nameWithOwner", "--jq", ".nameWithOwner"],
            cwd=ROOT, capture_output=True, text=True, check=False,
        ).stdout.strip()
        for branch in ("dev", "main"):
            proc = subprocess.run(
                ["gh", "api", f"repos/{repo}/branches/{branch}/protection",
                 "--jq", "[.required_status_checks.contexts[]] | sort | join(\",\")"],
                cwd=ROOT, capture_output=True, text=True, check=False,
            )
            live = proc.stdout.strip()
            expected = ",".join(sorted(REQUIRED))
            if proc.returncode != 0 or live != expected:
                failures.append(f"{branch}: live required contexts {live!r} != {expected!r}")
            else:
                checks.append(f"{branch}: live required contexts match")

    print("gate-integrity: " + ("PASS" if not failures else "FAIL"))
    for line in checks:
        print(f"  ok  {line}")
    for line in failures:
        print(f"  ERR {line}")
    if args.remote:
        print("  NV  GitHub runner semantics and Forgejo live state not verified by this script")
    return 1 if failures else 0


if __name__ == "__main__":
    raise SystemExit(main())
