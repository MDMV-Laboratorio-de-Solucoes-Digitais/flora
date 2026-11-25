# Decision: 040 — Seed enxuto com quadro real

- **Slug**: 040-seed-ui-debloat
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (antigo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O checkout carrega `apps/web` (5.844 arquivos / 43 MB, zero refs funcionais) e o `KanbanBoard` segue órfão com mocks, no objetivo de VPS barata (intake.md:10,18-19; research.md:11,18; problem.md:9). |
| Evidence strength | adequate | Pesquisa local de alta confiança com gates determinísticos congelados e validate auditável, mas auditoria author-blind emitiu NO-GO para merge "dark" pleno e encontrou defeito MEDIUM no caminho `changeStatus`→PATCH (research.md:44-45). |
| Value vs. inaction | strong | Reduz peso morto e dá uso real ao quadro com endpoint já existente; adiar mantém a affordance isolada e sem cobertura o caminho de persistência (problem.md:37-39). |
| Feasibility / appetite | adequate | Large: duas frentes somadas a auditoria em contexto fresco, sem medição de esforço nos artefatos (concept.md:11). |
| Strategic fit | strong | Ataca diretamente o norte estratégico de alternativa self-hosted em VPS barata, sem tocar backend ou autorização (research.md:27; concept.md:12). |
| Risk posture | adequate | Remoção irreversível no checkout, valor histórico de UX de `apps/web` não auditado [NV], lanes client/e2e não são gates e backend aceita status inválido com fallback silencioso (research.md:46-48). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`: o escopo foi justamente a delimitação que responde ao NO-GO da auditoria — duas movimentações 80/20 com verificações congeladas antes da implementação (c040-10). Validade, valor e fit são `strong`; evidência e risco `adequate` porque o merge "dark" pleno dependia de CI remoto verde e de decisão de produto sobre `Archived` (research.md:44), e o defeito do caminho de PATCH foi corrigido elevando 36→39 testes. A Option A de `concept.md` é a única que liga as duas metas do problema; o critério que define o board como "real" e a auditoria cega ficam como carried-forward, não bloqueantes do recorte aprovado (decision.md antigo:19-29).

## If needs-clarification

N/A — veredito `go`. Incertezas registradas como `[NEEDS CLARIFICATION:]`/`[NV]` no handoff abaixo; nenhuma pergunta bloqueante do recorte aprovado.

## If go — Handoff to `/speckit.specify`

- **Problem**: Checkout pesado com cliente Macro importado sem uso e quadro Kanban órfão alimentado por mocks, sem ligação com as tarefas reais nem persistência protegida por teste.
- **Chosen approach**: Option A — enxugar e dar uso real ao quadro, com verificações congeladas antes da implementação (`concept.md:9-12,29`; decision.md antigo:6-11).
- **In scope / out of scope**: In — remoção de `apps/web/` e de dois arquivos e2e `.spec.ts` mortos (diff somente `D`), módulo puro `kanban.svelte.ts`/`types.ts` + `kanban.test.ts` (Vitest `server`), reescrita controlada de `KanbanBoard.svelte`, `changeStatus()` em `(app)/tasks/+page.svelte`, drift de ROADMAP, validação, release e doc de research. Out — backend/endpoint/migration, RBAC/RLS/tenancy/authz, Tauri (ADR-0002), Documentos FASE 7, paridade Macro, ADR/constituição, deploy/tag/release/secrets, required-check/proteções; não afirmar quadro de time, ordem persistida ou regras de transição de status (decision.md antigo:13-23).
- **Success metrics**: Remoção com diff só `D` e zero refs a `apps/web`/`@coparse`; cobertura determinística de `changeStatus`→PATCH no projeto `server`; check/lint/build/fmt/`check_gate_integrity.py` idênticos ao baseline; auditoria cega sem NO-GO por falta de evidência (baseline: 5.844 arquivos/43 MB, defeito MEDIUM, check 0/0, build ok, gate PASS, CI remoto [NV] — problem.md:30-35; research.md:38,44-45).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: inventário exato de referências que autoriza a remoção integral em todos os checkouts]; [NEEDS CLARIFICATION: os dois `.spec.ts` são os únicos e2e mortos fora do `testMatch`?]; [NEEDS CLARIFICATION: comportamento esperado para transição de status inválida, dado o fallback silencioso do backend]; [NEEDS CLARIFICATION: como preservar empty/loading e ordem das colunas ao trocar o `<ul>`]; [NEEDS CLARIFICATION: valor histórico de UX de `apps/web` foi auditado ou permanece [NV]?]; [NEEDS CLARIFICATION: quem valida a auditoria cega e qual critério define o board como "real"]; [NV: Forgejo, proteções e CI remoto]; ADR-0003 mantém merge como decisão de máquina sob 4 required checks verdes, sem escalada humana para o merge em si (decision.md antigo:25-29).
