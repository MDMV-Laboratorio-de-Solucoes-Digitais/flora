# Plan: 012 — Session Authority Revalidation & Contract Confidence (80/20)

**Spec:** `specs/012-session-authority-revalidation/spec.md`  
**Assessment:** `.specify/assessments/012-session-authority-revalidation/`

## Approach
1. **Backend:** novo `VerifiedSession` + `authenticate_bearer` em `flora-api/src/extractors/auth.rs`; os três extractors passam a usá-lo. Consulta via `AppState.session_service.validate_session(jti)`, depois checagem de `is_active`, `expires_at`, `user_id`, `organization_id`. Erros genéricos; log detalhado via `tracing`.
2. **Testes unitários do extrator:** remover o caso de JWT válido sem DB (coberto por DB tests) e adicionar rejeição por assinatura inválida sem DB.
3. **Frontend:** `parseResponseBody` antes do branch 401; `apiErrorFromResponse` preserva `code`/`field`/`message`/`body`; fallback `unauthorized` quando o corpo não tem `error`.
4. **DB tests:** módulo `contract::auth::db_session_authority` em `flora-tests` com 7 casos, reutilizando `make_test_state`/`run_migrations`; rota protegida `GET /api/v1/tasks`.
5. **CI:** `.github/workflows/rust.yml` → `cargo test -p flora-tests contract::auth::db_ -- --ignored --test-threads=1`.
6. **Verificação:** fmt/check/clippy/testes Rust; DB tests em Postgres 18 descartável (`127.0.0.1:15434`, porta alternativa por conflito externo); frontend check/lint/test:unit(server)/build.

## Risks / NV
- `select-organization` faz segunda consulta à sessão; aceitável.
- Sem cache: latência sob carga não medida; documentado.
- WebSocket continua com autoridade só via JWT (follow-up).
- CI remoto/merge NV até o PR; smoke integrado não re-executado neste ciclo.
