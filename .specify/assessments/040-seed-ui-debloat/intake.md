# Idea Intake: 040 — Seed UI debloat + real board

- **Slug**: 040-seed-ui-debloat
- **Created**: 2026-11-25
- **Source**: repositório local — `.specify/assessments/040-seed-ui-debloat/decision.md` (+ `specs/040-seed-ui-debloat/spec.md`)
- **Type**: improvement

## Idea (as captured)

O ciclo 040 registra em `decision.md:6-17` e `specs/040-seed-ui-debloat/spec.md:6-23` dois movimentos frontend de maior confiança para o objetivo "alternativa self-hosted em VPS barata": remover o cliente Macro original importado (`apps/web`, 5.844 arquivos / 43 MB, sem referências funcionais) e converter o `KanbanBoard` órfão em componente controlado, alimentado por tarefas reais e com persistência de status pelo endpoint `PATCH /api/v1/tasks/{id}` já existente. O artefato registra também que a mudança deve preservar o build e ficar protegida por verificações determinísticas congeladas antes da implementação (`decision.md:10-11`; `spec.md:33-36`).

## Restated

Propõe-se reduzir o checkout frontend e dar uso real ao quadro Kanban existente, em apoio à alternativa self-hosted em VPS barata. A iniciativa é registrada como preservadora do comportamento de build, sem alterações de backend ou autorização.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: Objetivo "alternativa self-hosted em VPS barata" registrado em `decision.md:6` e `spec.md:7-8`: cliente Macro importado (`apps/web`, 5.844 arquivos / 43 MB, zero refs funcionais) e `KanbanBoard` órfão sem dados reais.

## First-Glance Unknowns

- [NEEDS CLARIFICATION: Qual inventário exato de referências autoriza a remoção integral de `apps/web` em todos os checkouts?]
- [NEEDS CLARIFICATION: Os dois arquivos `.spec.ts` citados são os únicos e2e mortos, ou existem outros fora do `testMatch`?]
- [NEEDS CLARIFICATION: Qual comportamento esperado quando uma transição de status inválida chega ao backend?]
- [NEEDS CLARIFICATION: Como serão preservados os estados de empty/loading e a ordem das colunas ao trocar o `<ul>` pelo board?]
- [NEEDS CLARIFICATION: O valor histórico de UX de `apps/web` foi auditado antes da remoção ou permanece [NV]?]
- [NEEDS CLARIFICATION: Quem valida a auditoria cega em contexto fresco e qual critério define o board como "real"?]
