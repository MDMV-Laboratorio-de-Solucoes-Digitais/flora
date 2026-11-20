# Plan: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

**Spec:** `specs/011-frontend-api-contract-session-hardening/spec.md`  
**Assessment:** `.specify/assessments/011-frontend-api-contract-session-hardening/`

## Approach
1. **Backend (tasks):** em `flora-api/src/routes/tasks.rs`, ao criar task, aplicar `description` e `assignee_id` do `CreateTaskRequest` (`task.description = ...; task.assignee_id = ...`) antes de persistir.
2. **Frontend API client:** introduzir `ApiError` (status, code, message, field, body/raw, url/statusText) e `parseBody` seguro; `request` lança `ApiError` preservando dados; 401 continua chamando `authState.lockSession()`; 204 retorna `{}` como hoje.
3. **Frontend estado/rotas:** `AuthState.setOrganization` define `isInvalidated = false`; em `tasks/+page.svelte` adicionar guard (sem auth/token → `/`, sem org → `/select-org`) antes de `refresh()`; ajustar tratamento de erro para usar mensagem útil de `ApiError` quando disponível.
4. **Testes frontend:** atualizar `tests/unit/AuthState.test.ts` (cobrir `setOrganization`/limpeza de flags conforme contrato atual) e `tests/integration/api-interceptor.test.ts` (verificar lock em 401 e preservação de status/corpo/estrutura do erro; cleanup de mocks/globals).
5. **Docs:** acrescentar em `docs/dev-smoke.md` exemplo de `docker compose -f docker-compose.yml -f <override.yml> ...` com mapeamento alternativo de portas e ajuste de URLs/env/psql.
6. **Verificação:** `cargo fmt --all -- --check`, `cargo check -p flora-api -p flora-core -p flora-tests`, `cargo test -p flora-core --lib`, `cargo test -p flora-tests --lib`; `bun run check`, `bun run lint`, `bun run test:unit -- --run --project server`, `bun run build` em `apps/frontend`. Smoke compatível se viável.

## Risks / NV
- Mudança de shape de erro no frontend pode afetar consumidores que dependem de string exata; mitigar mantendo `message` útil e atualizando pontos/testes do slice.
- CI remoto/porta 5432/volume PG16→18/OIDC e2e permanecem NV conforme ambiente.
