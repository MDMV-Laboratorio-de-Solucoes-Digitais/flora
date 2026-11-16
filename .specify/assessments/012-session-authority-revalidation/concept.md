# Concept: 012 — Session Authority Revalidation & Contract Confidence (80/20)

## Abordagem
- **Autoridade única de autenticação:** extrair um helper compartilhado em `flora-api/src/extractors/auth.rs` que decodifica o JWT e valida a sessão por `jti` no PostgreSQL via `SessionService`/`PgSessionRepository`:
  - sessão existente, `is_active = true`, `expires_at > now()`;
  - `session.user_id == claims.sub` e `session.organization_id == claims.organization_id` (incluindo o caso nil-org de pré-seleção);
  - erros genéricos `401` para token/sessão inválidos; `500` genérico para indisponibilidade do banco (fail-closed, sem vazar detalhes).
- **Extractors:** `AuthExtractor`, `UserContext` e `OrgContext` passam a usar o helper, evitando drift; `require_org()` permanece igual.
- **Sem cache neste ciclo:** uma consulta indexada por request protegido; documentar que cache curto (Valkey/Moka) é otimização futura somente com medição.
- **Frontend:** mover o parse do corpo para antes do tratamento de 401; preservar `status`, `code`, `field`, `message`, `body` no `ApiError` e manter `authState.lockSession()`; atualizar teste do interceptor.
- **Testes DB-backed:** novo módulo em `flora-tests` com Postgres 18 descartável cobrindo sessão válida (200), revogada (401), expirada (401), `user` mismatch (401), `org` mismatch (401) e sessão ausente (401); executar no job `db-tests` com `--ignored --test-threads=1`.
- **Evidência:** rodar fmt/check/clippy/testes Rust, DB tests em PG18 local (porta alternativa se 5432 ocupada), frontend check/lint/test/build; marcar CI remoto e smoke como NV até verificação.
- **Documentação:** atualizar `OBJETIVO.md`/`ROADMAP.md`, validação/release e (se viável) nota curta em `docs/dev-smoke.md` sobre revogação de sessão.

## Fora de escopo
- Cache de sessão (Valkey/Moka), invalidação distribuída, rate limiting novo, RBAC amplo.
- Revalidação de sessão no WebSocket upgrade (registrar como follow-up) e remoção completa de headers de tenant.
- OIDC/JWKS e2e real, mudanças de migração aplicada, expansão de produto/UI, `apps/web`.
