# Assess — define: 118

**Data**: 2026-11-23 · **Escopo fechado**:

- Extrair `VerifiedOidcIdentity` + `exchange_code_and_verify_id_token` em `flora-api/src/oidc.rs` para isolar troca `/oauth/v2/token`, Discovery, JWKS, `verify_id_token` (`allow_symmetric = false`) e normalização de e-mail/nome.
- Endurecer `callback` em `flora-api/src/routes/auth.rs`: bloquear `email` cuja conta possua `oidc_subject` distinto do `sub` verificado (`401 Unauthorized`), bloquear usuário `is_active == false` (`401 Unauthorized`) e emitir claims usando `&user.email` persistido.
- Construir `MockOidcIdpServer` HTTP local (`http://127.0.0.1:0`) em `flora-tests/src/oidc_mock.rs` assinando ID Tokens reais em `EdDSA` (`Ed25519`), servindo `/.well-known/openid-configuration`, `/oauth/v2/keys` e `POST /oauth/v2/token` com checagem `client_id`/`client_secret`/`redirect_uri`/`code` uso único/PKCE S256, além de modos de falha configuráveis.
- Substituir ambos os placeholders em `flora-tests/src/integration/registration.rs` por testes HTTP reais sem necessidade de Postgres e adicionar `2` contratos DB (`db_oidc_callback_*`) em `flora-tests/src/contract/auth.rs`, atualizando o piso `auth: 23 -> 25` em lockstep.
- **Fora de escopo**: provedor OIDC externo real (Zitadel), fluxo passkey/WebAuthn (`ADR-0008`) e endpoint `/auth/refresh` em browser E2E.
