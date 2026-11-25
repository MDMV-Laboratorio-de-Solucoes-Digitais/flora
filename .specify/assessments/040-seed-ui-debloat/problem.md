# Problem Definition: Peso de checkout do Seed e quadro Kanban sem dados reais

- **Slug**: 040-seed-ui-debloat
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

No objetivo registrado de uma alternativa self-hosted em VPS barata, o checkout do Seed carrega o cliente Macro original importado em `apps/web` — 5.844 arquivos / 43 MB sem referências funcionais — e, ao mesmo tempo, a interface de tarefas exibe uma lista simples enquanto o componente `KanbanBoard` permanece órfão e alimentado por mocks (intake.md:10,18-19; research.md:11,18). O problema não é a ferramenta de remoção nem a reescrita do componente em si: é a impossibilidade de reduzir o peso morto do checkout com segurança e de dar ao quadro um uso real apoiado nas tarefas e no endpoint de persistência já existentes (research.md:12,19). A auditoria author-blind em contexto fresco emitiu NO-GO para merge "dark" pleno embora nenhum gate determinístico tenha falhado, condicionando a aprovação a CI remoto verde e a decisão de produto sobre `Archived` (research.md:44); também encontrou que o caminho `changeStatus`→PATCH não era coberto, apenas os helpers puros (research.md:45). Qualquer afirmação de redução de custo financeiro seria inferência: os artefatos provam apenas I/O de checkout e contagem de arquivos (research.md:31).

## Affected Users & Stakeholders

- **Users**: maintainer/SE que mantém o Seed em VPS de recursos limitados; pessoa que cria e acompanha tarefas na rota `(app)/tasks` e hoje não dispõe de um quadro útil (research.md:12).
- **Stakeholders**: mantenedores e revisores de CI — precisam de gates determinísticos congelados antes da mudança; responsáveis por produto — precisam decidir o destino de status `Archived` e o critério que define o quadro como "real".

## Goals

- Reduzir o peso morto do checkout sem remover algo que ainda tenha uso funcional não identificado.
- Fazer o quadro de tarefas refletir tarefas reais e registrar mudanças de status pelo mecanismo de persistência já disponível.
- Congelar verificações determinísticas antes da implementação, de modo que build, lint, testes e integridade de gate permaneçam equivalentes ao baseline.
- Tornar explícito o que a mudança não entrega — visão de time, ordem persistida, regras de transição e paridade com o Macro.

## Non-Goals

- Não alterar backend, migration, RBAC/RLS/tenancy/authz, Tauri (ADR-0002), Documentos FASE 7 ou paridade Macro (decision.md, "Non-goals / non-claims").
- Não alterar ADR, constituição, deploy, tag, release, secrets ou required checks/proteções.
- Não afirmar quadro de time, ordem persistida de colunas ou regras de transição de status; não afirmar ganho financeiro medido em VPS.
- Não atribuir ao ciclo a auditoria do valor histórico de UX de `apps/web`, registrado como [NV] (decision.md, "NV"; research.md:48).

## Success Metrics

- Remoção de `apps/web` e dos arquivos e2e mortos resulta em diff somente `D`, com zero referências a `apps/web`/`@coparse` (baseline: 5.844 arquivos / 43 MB e dois `.spec.ts` fora do `testMatch`).
- O caminho `changeStatus`→PATCH passa a ter cobertura determinística no projeto `server` do Vitest (baseline: defeito MEDIUM, apenas helpers puros cobertos; 36 testes antes da correção, 39 depois).
- Check, lint, build, fmt e `check_gate_integrity.py` permanecem verdes e idênticos ao baseline (baseline: check 0/0, lint limpo, build ok, gate PASS).
- Auditoria cega em contexto fresco não emite NO-GO por ausência de evidência determinística (baseline: NO-GO para merge "dark" pleno; CI remoto [NV]).

## Cost of Inaction

O checkout continua pagando o peso de um cliente sem uso e o quadro permanece uma affordance isolada, sem ligação com tarefas reais; sem cobertura do caminho de persistência, regressões de status podem passar despercebidas. Como a remoção é irreversível no checkout e o valor histórico de UX não foi auditado, adiar também não elimina a incerteza — apenas a mantém sem evidência [NV].

## Open Questions

- [NEEDS CLARIFICATION: qual inventário exato de referências autoriza a remoção integral de `apps/web` em todos os checkouts?]
- [NEEDS CLARIFICATION: os dois arquivos `.spec.ts` citados são os únicos e2e mortos, ou existem outros fora do `testMatch`?]
- [NEEDS CLARIFICATION: qual comportamento esperado quando uma transição de status inválida chega ao backend, dado o fallback silencioso?]
- [NEEDS CLARIFICATION: como preservar estados de empty/loading e a ordem das colunas ao substituir a lista pelo quadro?]
- [NEEDS CLARIFICATION: o valor histórico de UX de `apps/web` foi auditado ou permanece [NV]?]
- [NEEDS CLARIFICATION: quem valida a auditoria cega em contexto fresco e qual critério define o quadro como "real"?]
