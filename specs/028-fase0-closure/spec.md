# Feature Specification: 028 — FASE 0 closure and Seed verification unblock

**Branch**: `feat/028-fase0-closure`  
**Status**: Completed (PR #236)  
**Assessment**: `.specify/assessments/028-next-vertical-slice/decision.md`

## Goal
Close the last literal Seed milestone item not yet machine-enforced (dependency audit) and unblock the integrated smoke documentation.

## Scope
- Version `supply-chain/` store (previously gitignored), add one `multer 3.1.0` `safe-to-deploy` exemption; `cargo vet --locked --no-minimize-exemptions` succeeds locally (443 exempted).
- Informational CI job (`continue-on-error: true`) for cargo-vet; PR dependency-review (license-check off) and one weekly non-blocking OSV-Scanner sweep. No new required checks, no CODEOWNERS yet (Fatia B).
- Replace expired static JWT in `docs/dev-smoke.md` with a deterministic Node mint (fresh `exp`, same secret as `Config::load()`); document that execution is NV in this environment (ports 3000/5432 occupied).
- Close residual drift: ADR-0001 observability/consequences, README infra line, 027 research snapshot banners, specs/001 stale crate path.

## Non-goals
Browser e2e bin, release SBOM (no release process), formal Tauri ADR, cargo-vet blocking, OTLP exporter, new services, reverting parallel work.

## NV
Local integrated smoke execution; cargo-vet imports/CODEOWNERS/blocking; Dependabot config beyond existing alerts; SBOM; e2e; 2vCPU/4GB; strict all-targets clippy.
