# Research: 028 — R1 synthesis

- **cargo-vet:** GO for Fatia A — commit gitignored `supply-chain/` store, add exactly one `multer 3.1.0` exemption (`cargo vet --locked --no-minimize-exemptions` then succeeds), informational CI job (`continue-on-error: true`); imports optional (skip if network fails); blocking + CODEOWNERS only after a week (Fatia B).
- **Advisories:** GO — repo is public so no GHAS needed; Dependabot alerts already active; add `dependency-review-action` on PRs (license-check off) and one weekly OSV-Scanner sweep (SARIF, non-blocking). No required checks (none exist today); no duplicate RustSec gate.
- **SBOM:** blocked on a release/tag process — defer.
- **E2E:** GO conditional but larger — feature-gated minter + Playwright setup project + non-required CI job; defer to 029.
- **Smoke/drift:** `docs/dev-smoke.md` carries an expired JWT (2026-11-16); replace with deterministic mint. ADR-0001:32 still calls OTel unusable; README:34 stale; 027 research snapshots predate TraceLayer. Local execution blocked by ports 3000/5432 in this environment — record NV.

## Decision drivers
Closes FASE 0 (the last literal milestone item not yet machine-enforced) and removes the two residual audit findings without adding services or blocking checks.
