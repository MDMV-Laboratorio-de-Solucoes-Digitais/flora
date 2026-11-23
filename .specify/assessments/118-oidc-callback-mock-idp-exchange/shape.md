# Assess — shape: 118

**Data**: 2026-11-23 · **Desenho escolhido (opção A — servidor Axum local com chaves `Ed25519` em memória)**:

- Usar um servidor `axum` efêmero em `127.0.0.1:0` dentro de `flora-tests/src/oidc_mock.rs` em vez de `wiremock` estático porque:
  1. Valida dinamicamente o `code_verifier` recebido em `application/x-www-form-urlencoded` contra o `code_challenge` (`S256`) emitido por `/api/v1/auth/login` via `flora_core::utils::pkce::derive_s256_challenge`;
  2. Consome `authorization_code` em uso único e registra `ObservedTokenRequest` para inspeção determinística pelos testes;
  3. Reutiliza as mesmas dependências já presentes no `Cargo.toml` raiz (`jsonwebtoken`, `axum`, `tokio`, `serde_urlencoded`) e não exige I/O externo nem portas fixas.
- Alternativa descartada (opção B — manter `HS256` ou stubs com `MockUserRepository` em memória): não exercitaria o caminho real de HTTP (`reqwest` + Discovery + JWKS `EdDSA` + rejeição de chave simétrica + persistência `PgUserRepository`/`PgSessionRepository` no Postgres 18).
