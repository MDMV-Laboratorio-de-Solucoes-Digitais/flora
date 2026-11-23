# Evidência de Validação — ciclo 118 (`118-oidc-callback-mock-idp-exchange`)

- **Data**: 2026-11-23 UTC
- **Issues R6**: [#399](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/399)–[#402](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/402) (`feature/118-oidc-callback-mock-idp-exchange` → `dev`)
- **Veredito R4**: **APROVADO com ressalva `[NV]`** *(Passos 1–6 verificados localmente com saída colada em `task-1190`; Passo 7 CI/ACR e Passo 8 Zitadel externo aguardam etapa de PR/ambiente externo)*

## Tabela do Roteiro de Testes (`test-roadmap.md`, 8 passos)

| # | Passo | Resultado observado | Evidência / Referência |
|---|---|---|---|
| 1 | `cargo test -p flora-tests test_registration_flow` (pré-implementação TDD RED) | `PASS (RED esperado)` (exit `101`: `no exchange_code_and_verify_id_token in oidc`) | Execução local pré-GREEN em `flora-tests` |
| 2 | `cargo test -p flora-api oidc::` (`task-1190`) | `PASS` (`7/7` dentro de `flora-api` `48 passed; 0 failed` em `0.10s`, incl. `EdDSA` assimétrico e rejeição `http://evil.example.com/keys`) | Local `task-1190` |
| 3 | `cargo test -p flora-tests test_registration_flow` (`task-1190`) | `PASS` (`2/2` dentro de `flora-tests` `36 passed; 82 ignored` em `0.77s`: `test_registration_flow_with_mock_oidc ... ok` e `test_registration_flow ... ok`) | Local `task-1190` |
| 4 | `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` (PG 18.7 `127.0.0.1:15433`, `task-1190`) | `PASS` (`25 passed; 0 failed; 93 filtered out` em `55.34s`, incl. `db_oidc_callback_exchanges_code_registers_new_user_and_creates_session` e `db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject`) | Local `task-1190` |
| 5 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check bans licenses sources` (`task-1190`) | `PASS` (exit `0`, `0` warnings/erros, `bans ok, licenses ok, sources ok`) | Local `task-1190` |
| 6 | `python3 scripts/ci/check_gate_integrity.py` (`task-1190`) | `PASS` (`gate-integrity: PASS`, 4 required checks, `all 10 db contract floors intact (82 total)`, `auth: 25`) | Local `task-1190` |
| 7 | `gh pr checks <PR> --watch` + logs CI `db-tests`/`rust-build` + ACR (`research`, `pro`) | Pendente abertura do PR (`[NV]`) | Atualizado após criação/validação do PR |
| 8 | Troca OIDC HTTP contra instância Zitadel externa real em staging/produção | `[NV]` (fora do escopo do servidor mock `127.0.0.1:0` em CI) | Ambiente externo |

- **Dispensa de capturas de UI (Bloco 0.3)**: ciclo puramente Rust/HTTP sem alterações em `apps/frontend`; evidência composta exclusivamente pelos logs CLI e CI listados acima.
