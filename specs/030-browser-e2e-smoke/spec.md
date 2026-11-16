# Feature Specification: 030 — Browser e2e smoke and Seed doc integrity

**Branch**: `feat/030-browser-e2e-smoke`  
**Status**: In progress  
**Assessment**: `.specify/assessments/030-next-vertical-slice/decision.md`

## Goal
Add the last verification lane (one deterministic Chromium smoke) and repair documentation integrity left by cycle 029.

## Scope
- Feature-gated `flora-mint-session` bin (`dev-mint`, `required-features`), emitting a Playwright storageState with a literal `FloraClaims` via `Config::load()`; absent from default/release builds.
- Playwright config (`retries: 0`, `workers: 1` — `select-organization` rotates the jti), `seed.setup.ts` (seed + mint + positive API drift guard), one `seed-flow.e2e.ts` covering select-org → workspace/channel → message → task.
- Non-required `e2e-browser` CI job (Chromium-only, short artifact retention); no `dev-mint` elsewhere.
- Fix stale clippy prose (README, rust.yml, OBJETIVO, 029 research banners) and complete the 029 smoke evidence artifact (revision pin, secret fingerprint, redacted checked-in transcript).

## Non-goals
Release/tag + SBOM, CORS allowlist, load/restore, cargo-vet blocking, OTLP, Tauri implementation.

## NV
Browser smoke not executed locally (no browser/stack run in this cycle); CI job informational; CORS permissive `Any`; WS has no Origin check.
