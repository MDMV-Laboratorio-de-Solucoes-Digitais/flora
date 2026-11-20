# Release: 026 — Executable Seed verification gate

**Branch/PR:** `feat/026-seed-ci-gate` -> `dev`, PR #232 (merged as `9a60766f`)  
**Baseline:** `github/dev@d09d39c1`

## Delivered
- `deny.toml` fixed (`AGPL-3.0-or-later`); cargo-deny bans/licenses/sources gate added to CI together with fmt check and strict clippy on production targets.
- Cycle 025 artifacts finalized with real PR #231 evidence.
- Constitution v1.3.0 mechanism-neutral Seed wording; ROADMAP and AGENTS drift fixed.
- Phantom OTel/openidconnect workspace declarations removed (no code used them).

## Evidence
Local verification green; PR #232 all required checks green (runs 43875417574 / 43875425127); merged at `9a60766f`.

## NV / follow-ups (027)
cargo-vet store/imports, advisories job/SBOM, PGMQ/Valkey scaffolding retirement, browser e2e (fresh JWT needed), Tauri deferral record, full OTLP exporter, 2vCPU/4GB load/restore.
