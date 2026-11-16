# Decision: 031 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice
**S1 — Release readiness + integrity closure.**

## Scope (in)
- Centralize version via dotted `package.version` at root + `version.workspace = true` in 7 members; regenerate Cargo.lock.
- Add `.github/workflows/release.yml` (tag-only, draft-first immutable release, CycloneDX 0.5.10 generation + v1_5 validation, lockstep version assert, asset upload, verify). **No tag creation, no publish.**
- Close residual doc drift (ADR-0001 exemption count + e2e resolved, AGENTS job list, OBJETIVO/ROADMAP, 030 validation wording, research banners).
- PR/CI green; merge/sync; plan 032.

## NV / deferred (032)
CORS allowlist, cargo-vet blocking + CODEOWNERS, load/restore, OTLP, actual tag/release/bump, SBOM dry run.

## Risk controls
Adding a tag-triggered workflow with `contents: write` is latent capability only; no tag exists today. Do not push tags or publish releases.
