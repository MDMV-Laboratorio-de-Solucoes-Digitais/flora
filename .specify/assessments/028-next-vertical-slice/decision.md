# Decision: 028 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice
**S1 — FASE 0 closure + Seed verification unblock.**

## Scope (in)
- Version `supply-chain/` (force-add despite .gitignore), add `multer 3.1.0` exemption, add an informational `cargo vet --locked --no-minimize-exemptions` CI job (`continue-on-error: true`); no mass audit, no blocking.
- Add PR dependency-review and one weekly OSV-Scanner sweep (both non-blocking; public repo, $0).
- Replace the expired static JWT in `docs/dev-smoke.md` with a deterministic mint command; record why a fresh local smoke run is NV here (ports 3000/5432 occupied).
- Close residual drift: ADR-0001 observability/consequences, README FASE/infra line, status banners on 027 research snapshots, specs/001 plan stale crate path.
- PR/CI green; merge/sync; plan 029.

## NV / deferred (029)
e2e browser with feature-gated minter; release SBOM (needs tag process); Tauri ADR-0002; cargo-vet blocking + CODEOWNERS; OTLP exporter; 2vCPU/4GB load/restore; strict `--all-targets` clippy.
