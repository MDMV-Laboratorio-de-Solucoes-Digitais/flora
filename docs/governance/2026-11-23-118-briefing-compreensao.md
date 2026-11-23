# Briefing de Compreensão — 118 / #399..#402

**Data**: 2026-11-23 · **Domínio crítico**: auth
**Issues**: [#399](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/399)–[#402](https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora/issues/402) · **Autor**: orquestrador

## Intenção

Extrair e endurecer a troca HTTP de tokens OIDC no `GET /api/v1/auth/callback` (`POST {issuer}/oauth/v2/token` PKCE `S256` → Discovery `/.well-known/openid-configuration` → JWKS assimétrico `EdDSA` → normalização de `email`/`name` → persistência de `users`/`sessions`) e construir um servidor IdP HTTP mock determinístico (`MockOidcIdpServer` em `flora-tests/src/oidc_mock.rs`) que elimine os testes placeholder de `registration.rs` e cubra bloqueios `401 Unauthorized` para contas inativas e conflitos cross-subject em PostgreSQL 18.

## Invariantes

- **Antes**:
  - `callback` (`flora-api/src/routes/auth.rs`) executava inline o `POST /oauth/v2/token`, aceitava `email` não normalizado no claim do JWT ao criar/assinar tokens locais, não bloqueava colisão quando `find_by_email` encontrava uma conta já vinculada a outro `oidc_subject`, e não verificava `!user.is_active` antes de gravar `Session` e emitir `session_token`.
  - `flora-api/src/oidc.rs` testava apenas `verify_id_token_inner(..., allow_symmetric = true)` em HS256 em memória; `verify_id_token` (`allow_symmetric = false`) e `fetch_jwks` não tinham testes exercitando HTTP/JWKS assimétricos reais.
  - `flora-tests/src/integration/registration.rs` (`test_registration_flow` e `test_registration_flow_with_mock_oidc`) continha apenas stubs gerando `Uuid::now_v7()`.

- **Depois**:
  - **Troca e verificação isoladas (`flora-api/src/oidc.rs`)**: `oidc_http_client()` usa `Policy::none()` (sem redirecionamentos), `connect_timeout(5s)` e `timeout(10s)`; `exchange_code_and_verify_id_token` executa `POST {issuer}/oauth/v2/token` (`grant_type=authorization_code`, `client_id`, `client_secret`, `redirect_uri`, `code_verifier`), busca Discovery + JWKS via `fetch_jwks` validando correspondência de `metadata["issuer"]` e `validate_jwks_uri` (`url::Url::parse`, bloqueando credenciais na URL e `http://` fora de loopback estrito `localhost`/`127.0.0.1`/`::1`), valida assinatura assimétrica via `verify_id_token` (`allow_symmetric = false`, rejeitando JWKs `oct`), exige `email_verified == Some(true)` e normaliza `email` (`trim().to_lowercase()`), sanitizando mensagens de `Error::OidcProvider` (log detalhado via `tracing::warn!`).
  - **Proteção contra takeover de e-mail, conta desativada e vinculação atômica (`flora-api/src/routes/auth.rs`, `flora-core/src/{traits/repository.rs,repositories/user_repository.rs}`)**:
    1. Se `user_repo.find_by_oidc_subject(&sub)` não encontrar vínculo direto e `find_by_email(&email)` retornar usuário cujo `oidc_subject` (`Some(bound_sub)`) difere de `sub`, o handler retorna `401 Unauthorized` (`Error::Unauthorized`) sem criar sessão nem sobrescrever vínculo;
    2. Se o usuário encontrado por `email` verificado ainda tiver `oidc_subject: None` (ex.: admin local de bootstrap), `UserRepository::set_oidc_subject_if_absent` realiza `UPDATE users SET oidc_subject = $2, updated_at = $3 WHERE id = $1 AND oidc_subject IS NULL AND is_active = true` de forma atômica, impedindo corridas ou re-vinculação posterior por outro `sub` (retorna `401` se não vinculou);
    3. Se `user.is_active == false`, retorna `401 Unauthorized` antes de chamar `PgSessionRepository::create`;
    4. Em contas válidas e ativas, assina `FloraClaims` com `&user.email` persistido.
  - **Servidor IdP OIDC HTTP mock (`flora-tests/src/oidc_mock.rs`)**: escuta em `http://127.0.0.1:0` assinando tokens com chave `Ed25519` (`OKP`, `EdDSA`), validando `client_id`/`client_secret`/`redirect_uri`/`code` uso único/PKCE `S256` (`derive_s256_challenge`), além de simular assinatura adulterada (no interior do segmento de assinatura), `nonce` divergente, JWK `oct` simétrico, ausência de `email`, `email_verified = Some(false)`, token expirado, `jwks_uri` `http://localhost.evil.com/keys` e ausência de `id_token`.

## Decisões implícitas

1. **Chave `Ed25519` (`OKP` / `EdDSA`) no servidor mock em vez de `HS256`**: mantém em produção e em testes a regra de segurança que proíbe JWKs simétricos (`oct`) para ID tokens sem enfraquecer `verify_id_token`.
2. **Loopback restrito por `url::Host` em `validate_jwks_uri`**: permite `http://` apenas para `localhost` / IP loopback em ambiente de desenvolvimento/testes, bloqueando prefix bypass como `http://localhost.evil.com` ou credenciais embutidas.
3. **Sem novas dependências no `Cargo.lock`**: reutiliza apenas `jsonwebtoken` e `serde_urlencoded` já declarados no `[workspace.dependencies]`.

## Comando de verificação

```bash
cargo fmt --all -- --check && \
cargo clippy --workspace --all-targets -- -D warnings && \
cargo deny check bans licenses sources && \
cargo test --workspace && \
python3 scripts/ci/check_gate_integrity.py && \
cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1
```

## Pergunta-teste ao SE

**Pergunta**: Se um invasor obtiver um `id_token` válido no provedor OIDC com um `oidc_subject` diferente mas contendo o mesmo e-mail de um usuário já cadastrado com outro `oidc_subject`, ou se um administrador desativar a conta de um usuário (`is_active = false`), o `GET /api/v1/auth/callback` emite uma sessão?
**Resposta esperada**: Não. `callback` (`flora-api/src/routes/auth.rs`) detecta `existing.oidc_subject.as_deref().is_some_and(|bound_sub| bound_sub != sub)` e `!user.is_active`, retornando `401 Unauthorized` em ambos os casos sem gravar linha em `sessions` nem emitir JWT, coberto pelo contrato DB `db_oidc_callback_existing_user_login_and_rejects_deactivated_or_conflicting_subject`.

## [NV]

- Integração HTTP contra uma instância Zitadel externa real em staging/produção (`workflow_dispatch` / ambiente Coolify fora do gate de PR).
