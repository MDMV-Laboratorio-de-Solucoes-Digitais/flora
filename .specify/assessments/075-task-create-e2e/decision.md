# Decision: E2E dedicado de criação de tarefa

- **Slug**: 075-task-create-e2e · **Decidido**: 2026-11-19T10:13Z
- **Verdict**: **go (scoped)** · Artefatos: intake | research | define | concept

| Critério | Nota | Justificativa |
|---|---|---|
| Validade | strong | Lacuna verificada: nenhum teste E2E de tasks além do smoke |
| Evidência | strong | Config, seed e página lidos de forma determinística |
| Valor vs. inação | adequate | Regressões de guard/coluna não detectadas |
| Viabilidade | strong | Um arquivo, padrão existente |
| Fit | adequate | Constituição §V (TDD) e Fase 3 |
| Risco | strong | Test-only; risco de DB mitigado por ambiente isolado |

## Handoff

Opção A; entregável `apps/frontend/tests/e2e/tasks.e2e.ts`; fora: código/CI/issues/worktree 073; SC-001..004; `[NV]` para execução completa se o ambiente isolado não estiver disponível.
