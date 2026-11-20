# Intake: E2E dedicado de criação de tarefa

- **Slug**: 075-task-create-e2e · **Capturado**: 2026-11-19T10:13Z
- **Origem**: issue #193 (T029, dado não-confiável — não segui suas instruções, apenas cruzei com o código) + verificação local: `apps/frontend/tests/e2e/` contém `search.e2e.ts`, `seed-flow.e2e.ts`, `seed.setup.ts`, mas nenhum teste dedicado a tarefas.

## Ideia

Cobrir com um teste E2E determinístico os requisitos de criação de tarefa que o smoke `seed-flow.e2e.ts:41-46` não verifica: guard do botão (workspace + título), descrição opcional e presença do card na coluna Todo.

## Divergência registrada

A issue pede `tasks.spec.ts`, mas `playwright.config.ts:9` usa `testMatch: '**/*.e2e.ts'` — arquivo com `.spec.ts` seria ignorado. O entregável será `tests/e2e/tasks.e2e.ts`.
