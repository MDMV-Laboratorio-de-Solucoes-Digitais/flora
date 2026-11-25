# Decision: E2E dedicado de criação de tarefa

- **Slug**: 075-task-create-e2e
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `apps/frontend/tests/e2e/` só tem `search.e2e.ts` e o smoke `seed-flow.e2e.ts:41-46`; guard do botão, descrição opcional e coluna Todo não são verificados (`intake.md:4,8`, `define.md:5-7`). |
| Evidence strength | strong | Config do Playwright (`playwright.config.ts:3-9,19-23,26-41`), seed/setup e marcação da página lidos de forma determinística (`research.md:7-14`). |
| Value vs. inaction | adequate | Regressões de guard e de mapeamento de status passariam despercebidas nos checks atuais (`define.md:7`, `concept.md:5-9`). |
| Feasibility / appetite | strong | Um arquivo seguindo o padrão existente; apetite small (`concept.md:7`). |
| Strategic fit | adequate | Constituição §V (TDD) e Fase 3 (UI de Tasks) registradas nos artefatos (`decision anterior:12`, `define.md:11`). |
| Risk posture | strong | Test-only; risco de DB mitigado por ambiente isolado e diff restrito a 1 arquivo de teste + artefatos (`concept.md:8`, `define.md:17-18`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown` e os artefatos não apresentam contradição. A issue #193 (T029) é dado não-confiável — apenas cruzada com o código — e pede `tasks.spec.ts`, que seria ignorado por `testMatch: '**/*.e2e.ts'`; o entregável é, portanto, `apps/frontend/tests/e2e/tasks.e2e.ts` (`intake.md:4,10-12`, `research.md:7`). A opção A cobre o que o smoke não cobre com asserções distintas; B acopla o smoke e C não cobre UI/API (`concept.md:11-17`). Execução completa depende de Postgres/API/web isolados e permanece `[NV]` se o ambiente não estiver disponível.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma; a divergência de extensão do arquivo foi resolvida pela config do Playwright.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: nenhum teste E2E independente cobre guard do botão, descrição opcional e presença do card na coluna Todo na criação de tarefa.
- **Chosen approach**: Opção A — um arquivo `tasks.e2e.ts` com seleção de org, verificação do botão desabilitado, criação com descrição e asserção do card na coluna Todo (`concept.md:5-9`).
- **In scope / out of scope**: dentro = `apps/frontend/tests/e2e/tasks.e2e.ts` + artefatos do ciclo; fora = código de produção, API, workflows, outros testes, worktree 073, fechamento de issues e esquema/DB compartilhado (`concept.md:19-21`, `define.md:11`).
- **Success metrics**: SC-001 teste descoberto por `playwright test --list`; SC-002 validação TS/ESLint/Prettier sem erros; SC-003 diff restrito a 1 arquivo de teste + artefatos; SC-004 gates do alvo PASS e nenhuma mutação de DB compartilhado (`define.md:13-18`).
- **Carried-forward open questions**: nenhuma bloqueante; sessão de uso único (select-org rotaciona `jti`), IDs fixos do seed, título com `exact: true`, assignee dependente de `authState.user?.id` (`research.md:16`); `[NV]` para execução completa sem ambiente isolado.
