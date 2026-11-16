# Decision: 029 — Next vertical slice

**Verdict:** go (scoped)

## Chosen slice
**S1 — Seed integrity closure.**

## Scope (in)
- Add module-level `#[expect(clippy::expect_used, reason = "...")]` to `health.rs` and `oidc.rs` test modules; promote CI clippy from `--lib --bins` to `--all-targets`; verify locally + in CI.
- Adopt ADR-0002 (Tauri deferral, Accepted) and update ADR-0001 Consequences.
- Close 028 doc drift (README:34/74, AGENTS:7-8, ADR-0001:45, supersession banners on 028 research docs).
- Attempt integrated dev smoke on isolated ports per runbook; record rc=0 evidence or explicit NV with blocker.
- PR/CI green; merge/sync; plan 030.

## NV / deferred (030)
Browser e2e (`retries: 0`), release/tag + SBOM, cargo-vet blocking/CODEOWNERS, OTLP exporter, 2vCPU/4GB load/restore.

## Risk controls
Do not edit test bodies or revert parallel work; use `-p flora-smoke-alt` for any local stack; no production access.
