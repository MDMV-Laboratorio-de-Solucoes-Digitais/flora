# 118 — servidor IdP OIDC HTTP mock e endurecimento do callback

**Data**: 2026-11-23 · **Critério 80/20**: `2000/3125` · **Issues**: `#399`..`#402` · **Domínio crítico**: `auth`

## O que mudou

- **Troca e verificação OIDC extraídas e testadas (`T-001` / `#399`)**:
  - `flora-core/src/traits/repository.rs` & `flora-core/src/repositories/user_repository.rs`: método atômico `UserRepository::set_oidc_subject_if_absent` (`WHERE id = $1 AND oidc_subject IS NULL AND is_active = true`) para vincular contas locais preexistentes apenas no primeiro login OIDC com e-mail verificado.
  - `flora-api/src/oidc.rs`: adicionados `VerifiedOidcIdentity`, `email_verified: Option<bool>` (`Some(true)` obrigatório), `oidc_http_client()` (`Policy::none()`, `connect_timeout=5s`, `timeout=10s`), validação de `issuer` no Discovery, `validate_jwks_uri` estrita contra prefix bypass/SSRF, sanitização de mensagens `Error::OidcProvider` e `9` testes unitários em `oidc::tests`.
  - `flora-api/src/routes/auth.rs`: `callback` refatorado para consumir `exchange_code_and_verify_id_token`, vincular atomicamente `oidc_subject` quando `NULL` via `set_oidc_subject_if_absent`, rejeitar `401 Unauthorized` em colisão cross-subject (`existing.oidc_subject` distinto do `sub` verificado) e em contas desativadas (`!user.is_active`), e emitir `FloraClaims` com `&user.email`.
- **Servidor IdP OIDC HTTP mock (`T-002` / `#400`)**:
  - `flora-tests/Cargo.toml` e `flora-tests/src/lib.rs`: expostos `jsonwebtoken` e `serde_urlencoded` do workspace e exportado `pub mod oidc_mock`.
  - `flora-tests/src/oidc_mock.rs`: servidor Axum efêmero `MockOidcIdpServer` em `http://127.0.0.1:0` com Discovery, JWKS `Ed25519` (`OKP`), troca `POST /oauth/v2/token` com PKCE `S256` (`derive_s256_challenge`), uso único do código, `email_verified`, registro de `ObservedTokenRequest` e modos negativos configuráveis.
- **Integração, contratos DB e trava lockstep (`T-003` / `#401`)**:
  - `flora-tests/src/integration/registration.rs`: substituídos ambos os stubs pelos fluxos HTTP reais contra `MockOidcIdpServer` (`test_registration_flow_with_mock_oidc` e `test_registration_flow`, cobrindo inclusive `email_verified = false`, `exp` vencido e `http://localhost.evil.com/keys`).
  - `flora-tests/src/contract/auth.rs`: adicionados `db_oidc_callback_exchanges_code_registers_new_user_and_creates_session` e `db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject` (incluindo vinculação atômica de usuário local sem `oidc_subject` e bloqueio de hijack posterior), elevando `auth: 23 → 25` (`82` total) em lockstep entre `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
- **Governança, R4 e R7 (`T-004` / `#402`)**:
  - Briefing de Compreensão (`auth`), roteiro/evidências R4, release R7 (`0.10.0 → 0.10.1` PATCH) e atualização append-only com recontagem determinística.

## [NV]

Execução remota em CI (pós-remediação ACR Rodada 2) de `db-tests`/`rust-build`/`dependency-review`, ACR Rodada 2 e fluxo contra instância Zitadel externa real em staging/produção.

## Rollback

Novo commit revertendo os arquivos do ciclo 118 (retornando em lockstep `auth: 25 → 23` caso os contratos `db_oidc_callback_*` sejam revertidos); zero alterações em migrations SQL e zero dependências novas no `Cargo.lock`.
