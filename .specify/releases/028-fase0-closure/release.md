# Release: 028 — FASE 0 closure and Seed verification unblock

**Branch/PR:** `feat/028-fase0-closure` -> `dev` (pending)  
**Baseline:** `github/dev@3bc60ca`

## Delivered
- Versioned cargo-vet store + one `multer` exemption; informational CI gate; PR dependency-review; weekly OSV sweep.
- Fresh-JWT mint procedure in `docs/dev-smoke.md` (no more expired static token).
- Residual drift closed in ADR-0001, README and 027 research snapshots.

## Evidence
Local verification in `validate.md`; PR CI result appended after green.

## NV / follow-ups (029)
Browser e2e with feature-gated minter; release SBOM (needs tag process); formal Tauri ADR-0002; cargo-vet blocking + CODEOWNERS; OTLP exporter; 2vCPU/4GB validation.
