# Shape: 071 — remover stub morto AuthService

## Forma escolhida

Remoção mecânica e completa das referências ao stub, em 11 arquivos:

1. `git rm` do arquivo de definição; remoção de `mod`/`pub use` em `services/mod.rs`.
2. `AppState`: remover import `AuthService`, campo `pub auth_service`, `let auth_service = ...` e a entrada no `Self { ... }`.
3. 7 construtores de teste: remover `AuthService` do `use` e a linha `auth_service: Arc::new(AuthService::new())`.
4. Manter `Error::ServiceUnavailable` (usado por OIDC) e a trait `AuthProvider`/`UserInfo`.

Nenhuma linha de lógica de autenticação é reescrita; a remoção é verificada por compilação e gates.

## Riscos e mitigação

- **Risco:** esquecer um construtor de `AppState` → mitigado por `cargo check --workspace` e `grep` final zero ocorrências.
- **Risco:** remover `ServiceUnavailable` por engano → mitigado por FR-004 + verificação de uso em `oidc_transactions.rs`.
- **Risco:** lint de imports não usados → mitigado por remoção dos `use` correspondentes e clippy `--all-targets -D warnings`.

## Entrega

Um PR para `dev/`, título conventional em inglês, evidências coladas, riscos e rollback; merge deliberado após CI verde ao vivo (ADR-0003).
