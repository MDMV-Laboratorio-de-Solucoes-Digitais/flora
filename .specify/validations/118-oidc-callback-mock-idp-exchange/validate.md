# Validação R4 — ciclo 118

**Data**: 2026-11-23 UTC · **Veredito: APROVADO com ressalva `[NV]`** *(Passos 1–6 do roteiro verificados em execução local completa em `task-1190`; Passo 7 CI/ACR em execução ao abrir PR para `dev`; Passo 8 Zitadel externo `[NV]`)*

| Comando | Resultado observado |
|---|---|
| `cargo test -p flora-tests test_registration_flow` pré-implementação (`TDD RED`, `2026-11-23`) | exit `101`: `error[E0432]: unresolved import flora_api::oidc::exchange_code_and_verify_id_token` |
| `cargo fmt --all -- --check` (`task-1190`) | exit `0` |
| `cargo clippy --workspace --all-targets -- -D warnings` (`task-1190`) | exit `0`, `0` erros |
| `cargo deny check bans licenses sources` (`task-1190`) | `bans ok, licenses ok, sources ok`, exit `0` |
| `cargo test --workspace` (`task-1190`) | `flora-api` `48 passed` (incl. `7/7` em `oidc::tests`), `flora-core` `35 + 21 passed`, `flora-tests` `36 passed; 0 failed; 82 ignored` (incl. `test_registration_flow_with_mock_oidc` e `test_registration_flow`) |
| `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` (PG 18.7 `127.0.0.1:15433`, `task-1190`) | **`test result: ok. 25 passed; 0 failed`** (`finished in 55.34s`), incl. `db_oidc_callback_exchanges_code_registers_new_user_and_creates_session` e `db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject` |
| `python3 scripts/ci/check_gate_integrity.py` (`task-1190`) | `gate-integrity: PASS`, `all 10 db contract floors intact (82 total)` (`auth: 25`) |

Roteiro de testes: `.specify/validations/118-oidc-callback-mock-idp-exchange/test-roadmap.md` · Detalhes passo a passo: `.specify/validations/118-oidc-callback-mock-idp-exchange/evidence.md`.
Briefing de Compreensão (`auth`): `docs/governance/2026-11-23-118-briefing-compreensao.md`.
Issues R6 vinculadas: `#399`–`#402`.

**Conclusão**: todas as portas locais Rust, supply-chain (`cargo deny`), testes de integração OIDC HTTP mock (`test_registration_flow*`) e contratos PostgreSQL 18 (`auth: 25/25`, total `82`) estão verdes; permanecem **`[NV]`** até abertura do PR o CI remoto/ACR (Passo 7) e, fora do gate de PR, instância Zitadel externa real (Passo 8).

## Revalidação Rodada 2 — Remediação ACR PR #403 (`task-1323`, append-only)

| Comando | Resultado observado (`task-1323`) |
|---|---|
| `cargo fmt --all -- --check` | exit `0` |
| `cargo clippy --workspace --all-targets -- -D warnings` | exit `0`, `0` erros |
| `cargo deny check bans licenses sources` | `bans ok, licenses ok, sources ok`, exit `0` |
| `cargo test --workspace` | `flora-api` **`50 passed`** (incl. `9/9` em `oidc::tests`), `flora-core` `35 + 21 passed`, `flora-tests` `36 passed; 0 failed; 82 ignored` |
| `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`, `all 10 db contract floors intact (82 total)` (`auth: 25`) |
| `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` | **`test result: ok. 25 passed; 0 failed`** (`finished in 77.95s` em PG 18.7) |

**Pós-Rodada 2**: aguarda push no PR `#403`, re-validação ACR Rodada 2 (`72547119`) e CI remoto no novo HEAD (Passo 7); instância Zitadel externa real permanece `[NV]` (Passo 8).
