#!/usr/bin/env python3
"""Deterministic contract guard for the four manual Flora deploy workflows.

ADR-0003 keeps deploy manual (`workflow_dispatch`) and outside the merge gate.
This script refuses drift that would make a deploy silent, automatic, or able to
cross environments: push triggers, wrong source branch, reused webhook secrets,
shared image tags, cancellation of an in-flight deploy, or a missing-webhook
step that would let a run look green while skipping the deploy.

Stdlib only: no PyYAML, no network, no GitHub API in the CI path.

Run from the repo root:
    python3 scripts/ci/check_deploy_workflows.py
    python3 scripts/ci/check_deploy_workflows.py --self-test
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / ".github" / "workflows"

# filename -> (source branch, image tag, webhook secret, environment)
SPEC: dict[str, tuple[str, str, str, str]] = {
    "docker-staging.yml": (
        "dev",
        "flora:staging",
        "COOLIFY_STAGING_API_DEPLOY_WEBHOOK",
        "staging",
    ),
    "docker-staging-web.yml": (
        "dev",
        "flora-web:staging",
        "COOLIFY_STAGING_WEB_DEPLOY_WEBHOOK",
        "staging",
    ),
    "docker-frontend.yml": (
        "main",
        "flora-web:production",
        "COOLIFY_FRONTEND_DEPLOY_WEBHOOK",
        "production",
    ),
    "docker-production-api.yml": (
        "main",
        "flora:production",
        "COOLIFY_PRODUCTION_API_DEPLOY_WEBHOOK",
        "production",
    ),
}
WEBHOOKS = {value[2] for value in SPEC.values()}
OTHER_TRIGGERS = {"push", "pull_request", "schedule", "workflow_run", "repository_dispatch"}


def _on_block(text: str) -> list[str]:
    lines = text.splitlines()
    for index, line in enumerate(lines):
        if re.match(r"^on:\s*$", line):
            block: list[str] = []
            for following in lines[index + 1 :]:
                if following.strip() and not following.startswith((" ", "\t")):
                    break
                block.append(following.strip())
            return block
    return []


def validate_text(filename: str, text: str) -> list[str]:
    spec = SPEC.get(filename)
    if spec is None:
        return [f"unexpected deploy workflow: {filename}"]
    branch, tag, webhook, environment = spec
    errors: list[str] = []
    triggers = _on_block(text)

    if "workflow_dispatch:" not in triggers:
        errors.append(f"{filename}: manual workflow_dispatch trigger is missing")
    for trigger in triggers:
        key = trigger.split(":", 1)[0]
        if key in OTHER_TRIGGERS:
            errors.append(f"{filename}: forbidden automatic trigger {key!r}")

    guard = f"if: ${{{{ github.ref == 'refs/heads/{branch}' }}}}"
    if guard not in text:
        errors.append(f"{filename}: missing job guard for refs/heads/{branch}")
    other_branch = "main" if environment == "staging" else "dev"
    if f"refs/heads/{other_branch}" in text:
        errors.append(f"{filename}: cross-environment branch ref refs/heads/{other_branch}")

    found_webhooks = [name for name in sorted(WEBHOOKS) if name in text]
    if found_webhooks != [webhook]:
        errors.append(
            f"{filename}: webhook secrets {found_webhooks!r} != expected [{webhook!r}]"
        )
    if tag not in text:
        errors.append(f"{filename}: missing image tag {tag!r}")
    if ":latest" in text:
        errors.append(f"{filename}: mutable ':latest' tag is not allowed")
    foreign_env = "production" if environment == "staging" else "staging"
    for prefix in ("flora:", "flora-web:"):
        if f"{prefix}{foreign_env}" in text:
            errors.append(f"{filename}: {foreign_env} image tag in a {environment} workflow")

    if "cancel-in-progress: false" not in text:
        errors.append(f"{filename}: deploy concurrency must not cancel in progress")
    if "cancel-in-progress: true" in text:
        errors.append(f"{filename}: cancel-in-progress: true can abort a live deploy")
    marker = f"::error::{webhook} is not configured"
    if marker not in text:
        errors.append(f"{filename}: missing fail-closed step for {webhook}")
    if "continue-on-error: true" in text:
        errors.append(f"{filename}: continue-on-error makes the deploy step vacuous")
    if "packages: write" not in text:
        errors.append(f"{filename}: packages: write permission is required for GHCR push")
    return errors


def validate_all(workflows: Path = WORKFLOWS) -> list[str]:
    errors: list[str] = []
    for filename in SPEC:
        path = workflows / filename
        if not path.is_file():
            errors.append(f"missing deploy workflow: {filename}")
            continue
        errors.extend(validate_text(filename, path.read_text(encoding="utf-8")))
    for path in sorted(workflows.glob("*.yml")):
        if path.name in SPEC:
            continue
        text = path.read_text(encoding="utf-8")
        for webhook in sorted(WEBHOOKS):
            if webhook in text:
                errors.append(f"{path.name}: unexpected deploy webhook {webhook}")
    return errors


SELF_TEST_BASE = """name: fixture
on:
  workflow_dispatch:
concurrency:
  group: fixture
  cancel-in-progress: false
permissions:
  contents: read
  packages: write
jobs:
  build-and-push:
    if: ${{ github.ref == 'refs/heads/dev' }}
    runs-on: ubuntu-24.04-arm
    env:
      DEPLOY_WEBHOOK: ${{ secrets.COOLIFY_STAGING_API_DEPLOY_WEBHOOK }}
    steps:
      - name: Fail closed when the deploy webhook is missing
        run: echo "::error::COOLIFY_STAGING_API_DEPLOY_WEBHOOK is not configured; refusing a deploy that would silently skip."
      - name: Build
        run: echo ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora:staging
"""


def self_test() -> int:
    filename = "docker-staging.yml"
    failures: list[str] = []
    if validate_text(filename, SELF_TEST_BASE):
        failures.append("valid fixture was rejected")
    mutations = {
        "automatic push trigger": ("  workflow_dispatch:", "  push:"),
        "wrong source branch": ("refs/heads/dev", "refs/heads/main"),
        "reused production webhook": (
            "COOLIFY_STAGING_API_DEPLOY_WEBHOOK",
            "COOLIFY_FRONTEND_DEPLOY_WEBHOOK",
        ),
        "cancellation of live deploy": (
            "cancel-in-progress: false",
            "cancel-in-progress: true",
        ),
        "missing fail-closed step": (
            "::error::COOLIFY_STAGING_API_DEPLOY_WEBHOOK is not configured",
            "::error::WEBHOOK missing",
        ),
        "mutable latest tag": (
            "        run: echo ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora:staging",
            "        run: echo ghcr.io/mdmv-laboratorio-de-solucoes-digitais/flora:latest",
        ),
        "foreign environment tag": ("flora:staging", "flora:production"),
    }
    for label, (old, new) in mutations.items():
        mutated = SELF_TEST_BASE.replace(old, new)
        if mutated == SELF_TEST_BASE:
            failures.append(f"mutation {label!r} did not change the fixture")
        elif not validate_text(filename, mutated):
            failures.append(f"mutation {label!r} was not rejected")
    print("deploy-workflow-self-test: " + ("PASS" if not failures else "FAIL"))
    for line in failures:
        print(f"  ERR {line}")
    return 1 if failures else 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true", help="test the validator with inline fixtures")
    args = parser.parse_args(argv)
    if args.self_test:
        return self_test()
    failures = validate_all()
    print("deploy-workflows: " + ("PASS" if not failures else "FAIL"))
    for line in failures:
        print(f"  ERR {line}")
    if not failures:
        print(f"  ok  {len(SPEC)} manual, branch-guarded, fail-closed deploy workflows")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
