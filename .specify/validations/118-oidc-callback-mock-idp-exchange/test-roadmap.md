# Roteiro de Testes (`test-roadmap.md`) — Ciclo 118 (`118-oidc-callback-mock-idp-exchange`)

**Data**: 2026-11-23 · **Spec**: `specs/118-oidc-callback-mock-idp-exchange/spec.md` · **Superfície**: CLI/Rust HTTP (`axum` + `MockOidcIdpServer` + PostgreSQL 18; nenhuma alteração de UI no frontend SvelteKit).

## Pré-condições

- Postgres 18 ativo (`FLORA_DATABASE__POSTGRES_URL` / `DATABASE_URL`).
- `CARGO_TARGET_DIR=/home/luis/development/MDMV/projetos/flora/target` (local) e GitHub Actions `rust-build` / `db-tests` (remoto).

## Passos

| # | Ação / Comando | Resultado Esperado | Superfície | Captura | Automatizável? |
|---|---|---|---|---|---|
| 1 | `cargo test -p flora-tests` pré-hardening (`TDD RED`) | Falha em cenários de JWK `oct` / conta inativa / conflito `oidc_subject` contra implementação anterior (quando testados) ou após substituir placeholders | HTTP/Rust | Log CLI | Sim |
| 2 | `cargo test -p flora-api oidc::` | Unitários passam (`9 passed; 0 failed`; `50` em `flora-api`), cobrindo `verify_id_token` (`EdDSA` `allow_symmetric=false`), rejeição `oct` JWK, `validate_jwks_uri` estrita (`http://localhost.evil.com`, credenciais na URL) e divergência `issuer` no Discovery | Rust unit | Log CLI | Sim |
| 3 | `cargo test -p flora-tests test_registration_flow` | `2 passed; 0 failed`: troca PKCE S256 → Discovery → JWKS `EdDSA` + falhas HTTP `callback` (`503` em PKCE inválido, assinatura adulterada no interior do bloco, nonce divergente, JWK `oct`, `email` ausente, `email_verified=false`, token expirado, `http://localhost.evil.com/keys`, `id_token` ausente; `401` reuso state) | HTTP/Mock IdP | Log CLI | Sim |
| 4 | `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1` | `25 passed; 0 failed`: inclui criação de usuário/sessão OIDC em PG 18 (`db_oidc_callback_exchanges_code_registers_new_user_and_creates_session`) e login existente + vinculação atômica `set_oidc_subject_if_absent` + bloqueio `401` cross-subject/conta inativa (`db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject`) | HTTP + Postgres 18 | Log CLI | Sim |
| 5 | `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo deny check bans licenses sources` | Exit `0`, zero warnings, zero violações de licença/banimentos/fontes | Rust CLI | Log CLI | Sim |
| 6 | `python3 scripts/ci/check_gate_integrity.py` | `gate-integrity: PASS`, `all 10 db contract floors intact (82 total)` (`auth: 25`) | Gate CI | Log CLI | Sim |
| 7 | `gh pr checks <PR> --watch` + inspeção ao vivo de `db-tests` / `rust-build` + ACR independente (`research`, `pro`) | CI 100% GREEN (`auth: passed=25 floor=25`, `82/82`) e ACR `APROVADO` | GitHub Actions | Links CI/ACR | Sim |
| 8 | Troca OIDC HTTP contra instância Zitadel externa real / staging | `[NV]` (fora do escopo do servidor mock em CI) | Externo | N/A | Não (depende de IdP externo) |

## Justificativa de ausência de capturas de tela

Este ciclo altera exclusivamente crates Rust (`flora-api`, `flora-tests`), gate e documentação de governança; nenhuma rota ou componente visual em `apps/frontend` foi modificado (superfície HTTP/CLI pura, Bloco 0.3). Logo, a evidência consiste em logs determinísticos de CLI, `db-tests` Postgres 18 e CI remoto.

## Rollback

Commit de reversão dos arquivos do ciclo 118 (reduzindo `auth: 25 -> 23` em lockstep); zero migrations SQL e zero dependências novas no `Cargo.lock`.
