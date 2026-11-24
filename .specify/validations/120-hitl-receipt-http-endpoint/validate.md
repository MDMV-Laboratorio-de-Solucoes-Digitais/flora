# Validação R4 — ciclo 120

**Data**: 2026-11-24 UTC · **Baseline**: `github/dev@e47983874` · **Veredito: APROVADO com ressalva `[NV]`** *(Passos 1–6 verificados em execução local nas tasks `task-1870` e `task-1883`; Passo 7 CI/ACR em execução após abertura do PR para `dev`; Passo 8 staging/produção `[NV]`)*

| Comando | Resultado observado |
|---|---|
| `cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1` (`TDD RED`, `task-1870`) | exit `101`: `FAILED. 0 passed; 3 failed` (`404 Not Found: null`) |
| `cargo fmt --all -- --check` (`task-1883`) | exit `0` |
| `cargo clippy --workspace --all-targets -- -D warnings` (`task-1883`) | exit `0`, `0` erros |
| `cargo deny check bans licenses sources` (`task-1883`) | `bans ok, licenses ok, sources ok`, exit `0` |
| `cargo test --workspace` (`task-1883`) | `flora-api` `53 passed` (incl. `3/3` em `routes::hitl::tests`), `flora-core` `36 + 21 passed` (incl. `service_unavailable_returns_sanitized_503_without_internal_detail`), `flora-tests` `36 passed; 85 ignored` |
| `python3 scripts/ci/check_gate_integrity.py && python3 scripts/ci/check_deploy_workflows.py && python3 scripts/ci/check_deploy_workflows.py --self-test` (`task-1883`) | `gate-integrity: PASS`, `all 11 db contract floors intact (85 total)` (`hitl: 3`), `deploy-workflows: PASS`, `deploy-workflow-self-test: PASS` |
| `cargo test -p flora-tests contract::hitl::db::db_ -- --ignored --test-threads=1` (PG 18.7 `127.0.0.1:15433`, `task-1883`) | **`test result: ok. 3 passed; 0 failed`** (`finished in 1.11s`) |

Roteiro de testes: `.specify/validations/120-hitl-receipt-http-endpoint/test-roadmap.md` · Detalhes: `.specify/validations/120-hitl-receipt-http-endpoint/evidence.md`.
Briefing de Compreensão (`auth / rbac / hitl`): `docs/governance/2026-11-24-120-briefing-compreensao.md`.
Issues R6 vinculadas: `#411`–`#413`.

**Conclusão**: todas as verificações Rust (`fmt`, `clippy -D warnings`, `cargo deny`), integridade do gate (`85` pisos DB com `hitl: 3`, validadores de deploy) e os 3 contratos PostgreSQL 18 para `POST /api/v1/hitl/receipts` estão verdes; permanecem **`[NV]`** até abertura do PR o CI remoto/ACR (Passo 7) e, fora do gate de PR, o deploy manual no Coolify (Passo 8).
