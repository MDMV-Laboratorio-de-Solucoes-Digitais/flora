# Research: 029 — R1 synthesis

- **Strict clippy:** test-only `expect_used` in `health.rs` (4) and `oidc.rs` (3); module-level `#[expect(clippy::expect_used, reason=...)]` matches the existing precedent in `extractors/auth.rs`. Zero test-body edits, no reversions. Then CI can use `--all-targets`, closing a long-running NV.
- **Tauri:** ADR-0002 draft is consistent with ADR-0001/constitution/README/code; adopt verbatim with Accepted status and update ADR-0001 Consequences.
- **Smoke:** runbook found `FLORA_APP__PORT`, `ports: !override` + `-p flora-smoke-alt` (volume safety), readiness gate before jq, nested JWT secret. Ports 3000/5432 occupied by other processes → use isolated project/ports.
- **E2E:** GO conditional but `retries: 0` mandatory (select-organization rotates jti); defer to 030.
- **SBOM:** blocked on a version/tag contract; design ready in `029-release-process-minimal.md`; defer to 030.
- **Doc drift:** README:34 vs :74, AGENTS:7-8, ADR-0001:45/55-57, 028 research snapshots lack supersession banners.

## Decision drivers
Highest integrity-per-effort: make the strict lint gate fully executable, record the Tauri deferral formally, and eliminate 028's self-contradictions; then attempt the last unexecuted ADR gate (smoke) on isolated infrastructure.
