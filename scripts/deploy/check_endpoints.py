#!/usr/bin/env python3
"""Read-only preflight for a Flora web/API pair behind Cloudflare.

Checks the public contract that a Coolify deploy must satisfy: web routes,
runtime config.js pointing at the API origin, API health/ready/metrics,
bootstrap/status, and an exact CORS allow-origin for the web origin. No secrets
are read; only URLs and response metadata are printed.

Usage:
    FLORA_WEB_URL=https://flora.mdmv.qzz.io \
    FLORA_API_URL=https://flora-api.mdmv.qzz.io \
    python3 scripts/deploy/check_endpoints.py

    python3 scripts/deploy/check_endpoints.py --self-test
"""
from __future__ import annotations

import argparse
import json
import os
import sys
import urllib.error
import urllib.request
from dataclasses import dataclass
from typing import Any

DEFAULT_TIMEOUT = 8.0


@dataclass(frozen=True)
class Probe:
    status: int
    body: str = ""
    allow_origin: str | None = None


def _normalize(url: str) -> str:
    return url.strip().rstrip("/")


def probe(url: str, *, timeout: float, origin: str | None = None) -> Probe:
    method = "OPTIONS" if origin else "GET"
    headers = {"User-Agent": "flora-deploy-preflight/1"}
    if origin:
        headers.update(
            {
                "Origin": origin,
                "Access-Control-Request-Method": "GET",
            }
        )
    request = urllib.request.Request(url, method=method, headers=headers)
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return Probe(
                status=response.status,
                body=response.read(8192).decode("utf-8", errors="replace"),
                allow_origin=response.headers.get("access-control-allow-origin"),
            )
    except urllib.error.HTTPError as exc:
        return Probe(
            status=exc.code,
            body=exc.read(8192).decode("utf-8", errors="replace"),
            allow_origin=exc.headers.get("access-control-allow-origin"),
        )
    except (urllib.error.URLError, TimeoutError, OSError) as exc:
        return Probe(status=0, body=f"transport_error: {type(exc).__name__}")


def evaluate(
    *,
    web_url: str,
    api_url: str,
    cors_origin: str,
    probes: dict[str, Probe],
) -> list[str]:
    errors: list[str] = []
    expected_config = f'"apiBaseUrl":"{api_url}"'

    for key, label in (("web_root", "web /"), ("web_config", "web /config.js")):
        result = probes.get(key)
        if result is None or result.status != 200:
            errors.append(f"{label}: expected 200, got {result.status if result else 'missing'}")

    config = probes.get("web_config")
    if config and config.status == 200 and expected_config not in config.body:
        errors.append(f"web /config.js: missing {expected_config}")

    for key, label in (
        ("api_health", "api /health"),
        ("api_ready", "api /health/ready"),
        ("api_metrics", "api /metrics"),
        ("api_bootstrap", "api /api/v1/bootstrap/status"),
    ):
        result = probes.get(key)
        if result is None or result.status != 200:
            errors.append(f"{label}: expected 200, got {result.status if result else 'missing'}")

    bootstrap = probes.get("api_bootstrap")
    if bootstrap and bootstrap.status == 200:
        try:
            payload: Any = json.loads(bootstrap.body)
        except ValueError:
            payload = None
        if not isinstance(payload, dict) or "available" not in payload:
            errors.append("api /api/v1/bootstrap/status: 200 body has no 'available' field")

    preflight = probes.get("cors_preflight")
    if preflight is None or preflight.status != 200:
        errors.append(
            f"CORS preflight from {cors_origin}: expected 200, "
            f"got {preflight.status if preflight else 'missing'}"
        )
    elif preflight.allow_origin != cors_origin:
        errors.append(
            f"CORS preflight: allow-origin {preflight.allow_origin!r} != {cors_origin!r}"
        )

    if not errors and web_url == api_url:
        errors.append("web and API share one origin; the split-origin contract requires two FQDNs")
    return errors


def _run(web_url: str, api_url: str, cors_origin: str, timeout: float) -> tuple[list[str], dict[str, Probe]]:
    bootstrap_path = "/api/v1/bootstrap/status"
    targets = {
        "web_root": (f"{web_url}/", None),
        "web_config": (f"{web_url}/config.js", None),
        "api_health": (f"{api_url}/health", None),
        "api_ready": (f"{api_url}/health/ready", None),
        "api_metrics": (f"{api_url}/metrics", None),
        "api_bootstrap": (f"{api_url}{bootstrap_path}", None),
        "cors_preflight": (f"{api_url}{bootstrap_path}", cors_origin),
    }
    results = {key: probe(url, timeout=timeout, origin=origin) for key, (url, origin) in targets.items()}
    return (
        evaluate(web_url=web_url, api_url=api_url, cors_origin=cors_origin, probes=results),
        results,
    )


def self_test() -> int:
    web = "https://flora.example.test"
    api = "https://flora-api.example.test"
    valid = {
        "web_root": Probe(200),
        "web_config": Probe(200, f'globalThis.__FLORA_CONFIG__ = {{"apiBaseUrl":"{api}"}};'),
        "api_health": Probe(200, '{"status":"ok"}'),
        "api_ready": Probe(200),
        "api_metrics": Probe(200),
        "api_bootstrap": Probe(200, '{"available":true}'),
        "cors_preflight": Probe(200, allow_origin=web),
    }
    failures: list[str] = []
    if evaluate(web_url=web, api_url=api, cors_origin=web, probes=valid):
        failures.append("valid fixture was rejected")

    mutations: dict[str, dict[str, Probe]] = {
        "missing web root": {**valid, "web_root": Probe(404)},
        "wrong apiBaseUrl": {**valid, "web_config": Probe(200, '{"apiBaseUrl":"https://other.test"}')},
        "missing bootstrap": {**valid, "api_bootstrap": Probe(404)},
        "wrong CORS origin": {**valid, "cors_preflight": Probe(200, allow_origin="https://evil.test")},
        "bootstrap without available": {**valid, "api_bootstrap": Probe(200, "{}")},
    }
    for label, probes in mutations.items():
        if not evaluate(web_url=web, api_url=api, cors_origin=web, probes=probes):
            failures.append(f"mutation {label!r} was not rejected")
    if not evaluate(web_url=web, api_url=web, cors_origin=web, probes=valid):
        failures.append("shared web/API origin was not rejected")

    print("endpoint-checker-self-test: " + ("PASS" if not failures else "FAIL"))
    for line in failures:
        print(f"  ERR {line}")
    return 1 if failures else 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--self-test", action="store_true", help="test the evaluator with inline probes")
    parser.add_argument("--timeout", type=float, default=DEFAULT_TIMEOUT)
    args = parser.parse_args(argv)
    if args.self_test:
        return self_test()

    web_url = _normalize(os.environ.get("FLORA_WEB_URL", ""))
    api_url = _normalize(os.environ.get("FLORA_API_URL", ""))
    if not web_url or not api_url:
        print("endpoint-check: FLORA_WEB_URL and FLORA_API_URL are required", file=sys.stderr)
        return 2
    cors_origin = _normalize(os.environ.get("FLORA_CORS_ORIGIN", web_url))

    errors, results = _run(web_url, api_url, cors_origin, args.timeout)
    summary = {
        "ok": not errors,
        "web_url": web_url,
        "api_url": api_url,
        "cors_origin": cors_origin,
        "probes": {key: value.status for key, value in results.items()},
        "errors": errors,
    }
    print(json.dumps(summary, sort_keys=True))
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
