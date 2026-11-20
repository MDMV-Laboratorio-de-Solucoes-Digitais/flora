# Decision: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

**Date:** 2026-11-15  
**Verdict:** **go**

## Rationale
- Consolida o slice núcleo evidenciado em 010 (auth→org→workspace→task) com correções contratuais de alto valor e baixo risco: persistir `description`/`assignee_id` na criação de tasks (backend), melhorar tratamento tipado de erros/status no `ApiClient`, alinhar estado/fluxo frontend e atualizar testes determinísticos obsoletos.
- Melhora ergonomia/reprodutibilidade do smoke documentado (orientação concreta de override de portas) sem depender de alterações ambientais fora de controle.
- Mantém hardening de sessão/tenancy como incremental: registrar e, quando viável com baixo risco, aplicar ajustes pontuais; **não** introduzir revalidação DB global no `AuthExtractor` neste ciclo (impacto amplo, sessão pré-seleção `org=nil`, custo/perf e necessidade de verificação DB extensiva) — documentado como follow-up priorizado com base em auditoria/pesquisa.

## In-scope (011)
1. **Backend contrato (tasks):** `POST /api/v1/tasks` deve aplicar `description` e `assignee_id` do request ao criar a task (mantendo semântica atual de org/membership).
2. **Frontend cliente/estado:** `ApiClient` preserva status e corpo de erro (JSON/texto) via erro tipado, mantendo `lockSession()` em 401; `AuthState.setOrganization` limpa `isInvalidated`.
3. **Frontend slice:** guards mínimos auth/org em `/tasks` (consistentes com app), ajustes de uso de contrato/erros onde necessário no slice; alinhamento de testes unitários/integration frontend (`AuthState`, interceptor) para comportamento atual/determinístico.
4. **Docs/smoke:** `docs/dev-smoke.md` com exemplo concreto de override de portas para conflitos (equivalente ao caminho documentado), mantendo notas NV.
5. **Verificação/evidência:** fmt/check/tests Rust, frontend check/lint/test:unit(server)/build, smoke compatível se ambiente permitir; artefatos SDD (spec/plan/tasks/checklists/validate/release).

## Out of scope / follow-up
- Revalidação DB global de sessão no extractor/middleware para todas as rotas protegidas (hardening importante, planejado com cuidado).
- Expansão ampla de testes DB-backed/CI além do viável sem risco; OIDC/JWKS e2e real; realtime/files/search/notificações wiring; mudanças de migração aplicada.

## NV
- Porta host padrão `5432` pode estar ocupada externamente; CI remoto/merge/sync verificados apenas no PR; volume PG16→18 em ambientes não descartáveis.
