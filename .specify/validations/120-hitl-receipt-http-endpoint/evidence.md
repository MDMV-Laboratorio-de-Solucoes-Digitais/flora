# Evidência de Validação — ciclo 120 (`120-hitl-receipt-http-endpoint`)

- **Data**: 2026-11-24 UTC
- **Baseline**: `github/dev@e47983874`
- **Issues R6**: [#411](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/411)–[#413](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/413) (`feature/120-hitl-receipt-http-endpoint` → `dev`)
- **Veredito R4**: **APROVADO com ressalva `[NV]`** *(Passos 1–6 verificados localmente com Postgres 18.7 nas tasks `task-1870` e `task-1883`; Passo 7 CI remoto/ACR aguardando etapa de PR; Passo 8 staging/produção `[NV]`)*

## Tabela do Roteiro de Testes (`test-roadmap.md`, 8 passos)

| # | Passo | Resultado observado | Evidência / Referência |
|---|---|---|---|
| 1 | Pré-implementação (`TDD RED`): `cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1` (`task-1870`) | `PASS (RED esperado)` (exit `101`: `FAILED. 0 passed; 3 failed` — todas as `3` chamadas a `/api/v1/hitl/receipts` retornaram `404 Not Found: null`) | Execução local `task-1870` |
| 2 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` (`task-1883`) | `PASS` (exit `0`, `0` erros/warnings de compilação/clippy) | Execução local `task-1883` |
| 3 | `cargo deny check bans licenses sources` (`task-1883`) | `PASS` (`bans ok, licenses ok, sources ok`, zero crates adicionadas ao `Cargo.lock`) | Execução local `task-1883` |
| 4 | `cargo test --workspace` (`task-1883`) | `PASS` (`flora-api` `53 passed` incl. `3/3` em `routes::hitl::tests`; `flora-core` `36 + 21 passed` incl. `service_unavailable_returns_sanitized_503_without_internal_detail`; `flora-tests` `36 passed; 85 ignored`) | Execução local `task-1883` |
| 5 | `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test` (`task-1883`) | `PASS` (`gate-integrity: PASS`, `all 11 db contract floors intact (85 total)`, `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS`) | Execução local `task-1883` |
| 6 | `cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1` em PG 18.7 (`task-1883`) | `PASS` (**`test result: ok. 3 passed; 0 failed`** em `1.11s`: `201` HMAC + `operator_id` servidor, `401`/`403`/`422` spoof e `422` decisão inválida + `503` sanitizado) | Local PG 18.7 + `oneshot` (`task-1883`) |
| 7 | `gh pr checks <PR> --watch` + logs CI `rust-build` / `db-tests` (`85/85`) / `frontend-build` / `e2e-browser` + ACR independente (`research`, `pro`) | Pendente abertura do PR (`[NV]`) | Atualizado após criação/validação do PR |
| 8 | Staging/produção externos no Coolify após promoção humana | `[NV]` (fora do gate de PR; deploy manual via `workflow_dispatch`) | Ambiente externo |

- **Dispensa de capturas de UI (Bloco 0.3)**: ciclo exclusivamente backend Rust/HTTP sem modificação de `apps/frontend`; evidência composta pelas saídas CLI/HTTP/DB e execuções de CI.
