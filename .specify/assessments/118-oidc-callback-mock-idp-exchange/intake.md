# Assess — intake: 118 (`118-oidc-callback-mock-idp-exchange`)

**Data**: 2026-11-23 · **Origem**: solicitação explícita do SWE + encerramento de gap `BF-01` do `backlog_de_fechamento`.

## Problema observado

1. Em `flora-tests/src/integration/registration.rs:10-23`, `test_registration_flow` e `test_registration_flow_with_mock_oidc` são apenas placeholders que geram `Uuid::now_v7()` sem iniciar nenhum servidor IdP HTTP nem exercitar `/api/v1/auth/login` ou `/api/v1/auth/callback`.
2. Em `flora-api/src/routes/auth.rs:179-307`, o handler `callback` mistura a lógica HTTP de troca do authorization code (`POST {issuer}/oauth/v2/token` + `fetch_jwks` + `verify_id_token`) com a persistência de usuário/sessão e possui dois riscos latentes:
   - Se `user_repo.find_by_email(email)` encontra usuário já vinculado a outro `oidc_subject`, o fallback atual retorna esse usuário sem checar conflito de `oidc_subject`;
   - Contas com `is_active = false` não são bloqueadas antes da emissão de `session_token` JWT e criação de `Session` no PostgreSQL.
3. Em `flora-api/src/oidc.rs:165-331`, todos os testes unitários usam apenas `verify_id_token_inner(..., allow_symmetric = true)` com JWKS HS256 em memória; a rota pública `verify_id_token` (que bloqueia chaves `oct` simétricas e valida assinaturas assimétricas como `EdDSA`) e `fetch_jwks` não possuem cobertura unitária ou de integração HTTP real.
