# Research: 071 — remover stub morto AuthService

**Data:** 2026-11-19T03:49Z · **Evidência local determinística** · remoto futuro **[NV]**.

## Primária (local)

1. `flora-core/src/services/auth_service.rs:33-70` — `impl AuthProvider` com 5 métodos `Err(Error::ServiceUnavailable("... not implemented"))`, `is_available() -> false`, `issuer_url()/client_id()` vazias.
2. `flora-core/src/services/auth_service.rs:17-27` — construtor e `Default` triviais; nenhum estado.
3. `flora-core/src/services/mod.rs:3,16` — `mod auth_service` e `pub use auth_service::AuthService`.
4. `flora-api/src/state/app_state.rs:13,30,97,184` — import, campo público `auth_service`, inicialização e struct literal.
5. `flora-api/src/extractors/auth.rs:272-278` — único uso em `flora-api` é um `AppState` de teste.
6. `flora-tests/src/contract/{auth,files,messaging,notifications,search,websocket}.rs` — seis `make_test_state` com o campo; remoção é mecânica (import + inicializador).
7. `grep -rn 'auth_service' flora-api/src/routes/` → **0**. Auth efetiva: `routes/auth.rs` (OIDC/JWKS) e `extractors/auth.rs` (sessão `jti`).
8. `Error::ServiceUnavailable` continua usado em `flora-api/src/state/oidc_transactions.rs:85,119` — a variante **deve permanecer**.
9. `AuthProvider`/`UserInfo` (`flora-core/src/traits/auth_provider.rs`) ficam sem implementadores após a remoção, mas são pontos de extensão documentados e exportados publicamente; mantidos (traço de arquitetura, sem custo de lint).

## NV

CI remota, Forgejo, produção e aprovações: **[NV]**. Cobertura idiomática: pt-BR (artefatos) + inglês (código).
