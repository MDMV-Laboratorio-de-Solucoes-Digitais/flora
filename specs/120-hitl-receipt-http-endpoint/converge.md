# Converge: 120 — Endpoint HTTP RBAC para emissão de recibo HITL (`POST /api/v1/hitl/receipts`)

**Data**: 2026-11-24 · **Modo**: append-only · **Status: Converged (pré-CI)**

## Mapeamento R6 (`T### → Issue`)

- `T-001` → [#411](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/411) (`flora-core/src/error.rs`, `flora-api/src/routes/hitl.rs`, `flora-api/src/routes/mod.rs`)
- `T-002` → [#412](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/412) (`flora-tests/src/contract/mod.rs`, `flora-tests/src/contract/hitl.rs`, `scripts/ci/check_gate_integrity.py`, `.github/workflows/rust.yml`)
- `T-003` → [#413](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/413) (`docs/governance/2026-11-24-120-briefing-compreensao.md`, governança, docs append-only, R4/R7 `0.12.0`, `test-roadmap.md`/`evidence.md`, ACR)

## Entrega e verificação local (`task-1870`, `task-1883`)

- **TDD RED → GREEN**:
  - Execução pré-implementação (`task-1870`) com os contratos `contract::hitl::db::db_` montados contra `create_router` sem a rota `/api/v1/hitl/receipts` falhou conforme esperado (`FAILED. 0 passed; 3 failed`: todos os `3` retornaram `404 Not Found: null`).
  - Após mapear `Error::ServiceUnavailable` para HTTP `503` sanitizado (`flora-core/src/error.rs`), implementar `flora-api/src/routes/hitl.rs` (`create_hitl_router`, `EmitHitlReceiptRequest` com `#[serde(deny_unknown_fields)]`, `can_emit_hitl_receipt`, `build_decision`, `require_hitl_operator`, `emit_hitl_receipt`) e montar `/api/v1/hitl` em `flora-api/src/routes/mod.rs`, todos os portões passaram sobre `github/dev@e47983874`:
    - `cargo fmt --all -- --check`: exit `0` (`task-1883`).
    - `cargo clippy --workspace --all-targets -- -D warnings`: `0` erros/warnings (`task-1883`).
    - `cargo deny check bans licenses sources`: `bans ok, licenses ok, sources ok` (`task-1883`).
    - `cargo test --workspace`: `flora-api` `53 passed` (incl. `3/3` em `routes::hitl::tests`), `flora-core` `36 + 21 passed` (incl. `error::tests::service_unavailable_returns_sanitized_503_without_internal_detail`), `flora-tests` `36 passed; 85 ignored` (`task-1883`).
    - `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test`: `gate-integrity: PASS`, `all 11 db contract floors intact (85 total)` (`hitl: 3`), `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS` (`task-1883`).
    - `cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1` (PostgreSQL 18.7 `127.0.0.1:15433`): **`test result: ok. 3 passed; 0 failed`** em `1.11s` (`task-1883`).
- **Dispensa de capturas de UI (Bloco 0.3)**: ciclo backend Rust/HTTP sem alterações em `apps/frontend`.
- **Briefing de Compreensão (`auth / rbac / hitl`)**: `docs/governance/2026-11-24-120-briefing-compreensao.md`.
- **[NV]**: CI remoto do PR em `dev`, ACR independente (`research`, `pro`), PR de promoção `dev -> main` e deploy manual em staging/produção no Coolify.
