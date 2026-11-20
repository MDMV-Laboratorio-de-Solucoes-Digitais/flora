# Shape: E2E dedicado de criação de tarefa

- **Slug**: 075-task-create-e2e · **Recomendado**: A

## Opção A — teste E2E focado (recomendada)

- **Sketch**: um arquivo `tasks.e2e.ts` com seleção de org, verificação do botão desabilitado, criação com descrição e asserção do card na coluna Todo. Appetite: small.
- **Trade-offs**: cobre o que o smoke não cobre; depende do mesmo seed/setup. Risco: duplicação parcial do smoke — mitigado com asserções distintas.
- **Rabbit holes**: tentar substituir o setup; usar `.spec.ts` (ignorado); reutilizar statuses do unit test.

## Opção B — estender o smoke

- **Sketch**: acrescentar asserções ao teste existente. Barato, mas acopla ainda mais o smoke e não cria caso independente.

## Opção C — só teste unitário

- **Sketch**: reforçar `TaskStore.test.ts`. Não cobre UI/API — fora do objetivo.

## Fora de escopo

Código de produção, API, workflows, outros testes, worktree 073, fechamento de issues.

## Premissas

`testMatch` exige `.e2e.ts`; seed/setup são a única via de autenticação determinística; execução completa depende de Postgres/API/web isolados.
