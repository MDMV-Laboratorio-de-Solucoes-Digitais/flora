# Release: 118 — servidor IdP OIDC HTTP mock e endurecimento do callback

**Ciclo**: 118 · **Data**: 2026-11-23 · **Baseline**: `github/dev@2ef8fd97e`

## Entregue

- `FR-001` / `FR-002` / `T-001` ([#399](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/399)): `VerifiedOidcIdentity` + `exchange_code_and_verify_id_token` em `flora-api/src/oidc.rs`, testes unitários cobrindo `verify_id_token` assimétrico (`EdDSA`) + rejeição `oct` + rejeição de `jwks_uri` HTTP não-local em `fetch_jwks`, e endurecimento do `callback` em `flora-api/src/routes/auth.rs` (`401 Unauthorized` para contas `is_active = false` ou conflito de `oidc_subject` no fallback por e-mail, mais `FloraClaims` com `&user.email`).
- `FR-003` / `T-002` ([#400](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/400)): exposição de `base64`, `jsonwebtoken` e `serde_urlencoded` do workspace em `flora-tests/Cargo.toml` e implementação de `MockOidcIdpServer` (`http://127.0.0.1:0`, Discovery, JWKS `Ed25519`/`OKP`, token endpoint PKCE `S256`) em `flora-tests/src/oidc_mock.rs` (`flora-tests/src/lib.rs`).
- `FR-004` / `T-003` ([#401](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/401)): substituição dos dois stubs em `flora-tests/src/integration/registration.rs` por testes HTTP reais (`test_registration_flow_with_mock_oidc` e `test_registration_flow`) e adição de `2` contratos DB (`db_oidc_callback_exchanges_code_registers_new_user_and_creates_session` e `db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject`) em `flora-tests/src/contract/auth.rs`, com trava lockstep `auth: 23 → 25` (`82` total) em `scripts/ci/check_gate_integrity.py` e `.github/workflows/rust.yml`.
- `FR-005` / `T-004` ([#402](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/402)): Briefing de Compreensão (`auth`), governança, roteiro/evidências R4, docs append-only e revisão agêntica independente.

## SemVer

PATCH `0.10.0` → `0.10.1`: refatoração interna e endurecimento de segurança (`401` para contas desativadas e conflito de `oidc_subject`, normalização de e-mail) no callback `/api/v1/auth/callback`, acrescidos de suíte HTTP mock OIDC e `2` novos contratos DB sem alteração de contrato público nem schema SQL.

## Evidência

Portas locais verdes (`task-1190`): `cargo fmt --all -- --check` (exit `0`), `cargo clippy --workspace --all-targets -- -D warnings` (`0` erros), `cargo deny check bans licenses sources` (`bans ok, licenses ok, sources ok`), `cargo test --workspace` (`flora-api` `48/48`, `flora-core` `35+21`, `flora-tests` `36/82 ignored`), `contract::auth::db_` (`25 passed; 0 failed` em Postgres 18.7) e `python3 scripts/ci/check_gate_integrity.py` (`gate-integrity: PASS`, `82` pisos). Evidência de CI remoto e ACR será anexada em `.specify/validations/118-oidc-callback-mock-idp-exchange/evidence.md`.

## Rollback

Novo commit de reversão dos arquivos do ciclo 118 (retornando em lockstep `auth: 25 → 23` caso os `2` novos contratos sejam removidos); nenhuma migration SQL alterada e nenhuma nova dependência externa adicionada ao `Cargo.lock`.

## [NV]

Execução remota em CI (`db-tests`, `rust-build`, `dependency-review`), ACR independente e integração contra Zitadel real em staging/produção.
