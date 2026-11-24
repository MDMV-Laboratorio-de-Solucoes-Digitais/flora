# Release: 120 — Endpoint HTTP RBAC `POST /api/v1/hitl/receipts`

**Ciclo**: 120 · **Data**: 2026-11-24 · **Baseline**: `github/dev@e47983874`

## Entregue

- `FR-001` / `T-001` ([#411](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/411)): mapeamento explícito de `Error::ServiceUnavailable(_)` em `flora-core/src/error.rs` para HTTP `503 Service Unavailable` com corpo fixo sanitizado (`{"error":"service_unavailable","message":"Service temporarily unavailable"}`) e teste unitário anti-vazamento; implementação de `flora-api/src/routes/hitl.rs` (`create_hitl_router`, `EmitHitlReceiptRequest` com `#[serde(deny_unknown_fields)]`, `can_emit_hitl_receipt`, `build_decision`, `require_hitl_operator`, `emit_hitl_receipt` e `3` testes unitários) montado em `/api/v1/hitl` em `flora-api/src/routes/mod.rs`.
- `FR-002` / `T-002` ([#412](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/412)): suíte `flora-tests/src/contract/hitl.rs` (`flora-tests/src/contract/mod.rs`) com `3` contratos DB PostgreSQL 18 (`db_hitl_*`) cobrindo emissão `201` com assinatura HMAC canônica + `operator_id` do servidor, bloqueio `401`/`403`/`422` por anti-spoofing, validação `422` antes de `503` sanitizado sem vazar `FLORA_HITL__HMAC_SECRET`, e atualização em lockstep `hitl: 3` (`82 -> 85`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
- `FR-003` / `FR-004` / `T-003` ([#413](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/413)): Briefing de Compreensão (`auth / rbac / hitl`), governança, R4 (`test-roadmap.md`, `evidence.md`, `validate.md`), atualização append-only da documentação (`BF-03 -> fechado`) e ACR independente.

## SemVer

MINOR `0.11.0` → `0.12.0`: exposição de novo endpoint público autenticado (`POST /api/v1/hitl/receipts`) na API REST do `flora-api` e mudança de status HTTP sanitizado `503` para `Error::ServiceUnavailable`, sem alterações destrutivas em rotas existentes, sem migrations SQL e sem novas dependências no `Cargo.lock` (`85` pisos DB intactos).

## Evidência

Portas locais verdes (`task-1883` sobre `github/dev@e47983874`): `cargo fmt --all -- --check` (exit `0`), `cargo clippy --workspace --all-targets -- -D warnings` (`0` erros), `cargo deny check bans licenses sources` (`bans ok, licenses ok, sources ok`), `cargo test --workspace` (`flora-api` `53 passed` incl. `3/3` em `routes::hitl::tests`; `flora-core` `36 + 21`; `flora-tests` `36 passed; 85 ignored`), `check_gate_integrity.py` (`gate-integrity: PASS`, `85` pisos), `check_deploy_workflows.py` + `--self-test` (`PASS`), e `contract::hitl::db::db_` (**`3 passed; 0 failed`** em PG 18.7). Evidências de CI remoto e ACR são registradas em `.specify/validations/120-hitl-receipt-http-endpoint/evidence.md`.

## Rollback

Novo commit de reversão removendo `flora-api/src/routes/hitl.rs` e `flora-tests/src/contract/hitl.rs`, restaurando `flora-core/src/error.rs`, `flora-api/src/routes/mod.rs` e `flora-tests/src/contract/mod.rs`, e retornando em lockstep `hitl: 3` (`85 -> 82`) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`; zero migration SQL e zero alteração no `Cargo.lock`.

## [NV]

Execução remota em CI do PR em `dev` (`rust-build`, `db-tests` `85/85`, `frontend-build`, `dependency-review`, `e2e-browser`), ACR independente e deploy manual em staging/produção via Coolify (`workflow_dispatch`).
