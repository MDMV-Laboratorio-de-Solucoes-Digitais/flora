# Release: 028 — FASE 0 closure and Seed verification unblock

**Branch/PR:** `feat/028-fase0-closure` -> `dev`, PR #236 (merged as `4ecf1459`)  
**Baseline:** `github/dev@3bc60ca`

## Delivered
- Versioned cargo-vet store + one `multer` exemption; informational CI gate; PR dependency-review; weekly OSV sweep.
- Fresh-JWT mint procedure in `docs/dev-smoke.md` (no more expired static token).
- Residual drift closed in ADR-0001, README and 027 research snapshots.

## Evidence
Local verification green; PR #236 all checks green (runs 43891907628 / 43891907644 / 43891922471 / 43891922523); merged at `4ecf1459`.

## NV / follow-ups (029)
Browser e2e with feature-gated minter; release SBOM (needs tag process); formal Tauri ADR-0002; cargo-vet blocking + CODEOWNERS; OTLP exporter; 2vCPU/4GB validation.
