# 118 — servidor IdP OIDC HTTP mock e endurecimento do callback

**Data**: 2026-11-23 · **Critério 80/20**: `2000/3125` · **Issues**: `#399`..`#402` · **Domínio crítico**: `auth`

## O que mudou

- **Troca e verificação OIDC extraídas e testadas (`T-001` / `#399`)**:
  - `flora-api/src/oidc.rs`: adicionados `VerifiedOidcIdentity` e `exchange_code_and_verify_id_token` (`POST /oauth/v2/token` → Discovery → JWKS → `verify_id_token` → normalização de `email`/`name`) além de testes unitários para verificação assimétrica `EdDSA` (`allow_symmetric = false`), rejeição de JWK simétrico (`oct`) e bloqueio de `jwks_uri` HTTP não-local.
  - `flora-api/src/routes/auth.rs`: `callback` refatorado para consumir `exchange_code_and_verify_id_token`, rejeitar `401 Unauthorized` em colisão cross-subject (`existing.oidc_subject` distinto do `sub` verificado) e em contas desativadas (`!user.is_active`), e emitir `FloraClaims` com `&user.email`.
- **Servidor IdP OIDC HTTP mock (`T-002` / `#400`)**:
  - `flora-tests/Cargo.toml` e `flora-tests/src/lib.rs`: expostos `base64`, `jsonwebtoken` e `serde_urlencoded` do workspace e exportado `pub mod oidc_mock`.
  - `flora-tests/src/oidc_mock.rs`: servidor Axum efêmero `MockOidcIdpServer` em `http://127.0.0.1:0` com Discovery, JWKS `Ed25519` (`OKP`), troca `POST /oauth/v2/token` com PKCE `S256` (`derive_s256_challenge`), uso único do código, registro de `ObservedTokenRequest` e modos negativos configuráveis.
- **Integração, contratos DB e trava lockstep (`T-003` / `#401`)**:
  - `flora-tests/src/integration/registration.rs`: substituídos ambos os stubs pelos fluxos HTTP reais contra `MockOidcIdpServer` (`test_registration_flow_with_mock_oidc` e `test_registration_flow`).
  - `flora-tests/src/contract/auth.rs`: adicionados `db_oidc_callback_exchanges_code_registers_new_user_and_creates_session` e `db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject`, elevando `auth: 23 → 25` (`82` total) em lockstep entre `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
- **Governança, R4 e R7 (`T-004` / `#402`)**:
  - Briefing de Compreensão (`auth`), roteiro/evidências R4, release R7 (`0.10.0 → 0.10.1` PATCH) e atualização append-only com recontagem determinística.

## [NV]

Execução remota em CI de `db-tests`/`rust-build`/`dependency-review`, ACR independente e fluxo contra instância Zitadel externa real em staging/produção.

## Rollback

Novo commit revertendo os arquivos do ciclo 118 (retornando em lockstep `auth: 25 → 23` caso os contratos `db_oidc_callback_*` sejam revertidos); zero alterações em migrations SQL e zero dependências novas no `Cargo.lock`.
