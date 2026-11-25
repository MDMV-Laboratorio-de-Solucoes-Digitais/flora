# Decision: 033 — Busca tipada no Seed e fechamento da deriva

- **Slug**: 033-next-vertical-slice
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | decision.md (antigo)

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O campo de busca do cabeçalho é decorativo: não consulta o backend nem mostra resultado, e a auditoria de frontend o aponta como maior ganho visível por esforço (intake.md:5; research.md:3,5; problem.md:9). |
| Evidence strength | strong | GO com 3 guardrails de CI (Vitest apenas `server`, Playwright só `**/*.e2e.ts` informativo), lista exata de 8 arquivos, código drop-in e backend já verificado (research.md:3) — contrato protegido por teste determinístico. |
| Value vs. inaction | strong | Maior capacidade visível ao usuário com backend pronto; sem teste no gate, regressões do contrato de busca passariam despercebidas (problem.md:37-39). |
| Feasibility / appetite | adequate | Small–medium, mas incerto: depende do formato real dos resultados, já que `type`/`source` são strings abertas no backend (concept.md:11-12; research.md:3). |
| Strategic fit | adequate | Entrega valor visível imediato e difere slices maiores (ADR-0002 #2, FASE 7 Documents) que exigem prova ao vivo (intake.md:5; research.md:4-5). |
| Risk posture | adequate | Não cria novo gate de merge, mas não promete relevância CJK/i18n nem navegação provada em navegador, e testes de componente seguem fora do gate (concept.md:12-13). |

## Verdict & Rationale

Veredito histórico `go (scoped)` preservado como `go`: validade do problema e evidência são `strong`, apetite/ajuste estratégico/risco `adequate`. A Option A de `concept.md` — busca tipada com teste no projeto `server` do Vitest — é a que liga o campo a uma consulta real e protege o contrato sem criar gate novo; a Option B não protege o contrato e a Option C reintroduz um serviço externo que o Seed já substituiu por busca em banco verificada (concept.md:27-29). A capacidade de ADR-0002 #2 e Documentos FASE 7 permanece diferida, e as incertezas sobre relevância/i18n e prova em navegador ficam como carried-forward, não bloqueantes.

## If needs-clarification

N/A — veredito `go`. Incertezas registradas como `[NEEDS CLARIFICATION:]`/`[NV]` no handoff abaixo; nenhuma pergunta bloqueante.

## If go — Handoff to `/speckit.specify`

- **Problem**: O campo de busca do cabeçalho é uma affordance decorativa, sem consulta ao backend nem resultado ao usuário, e o contrato da busca não tem verificação determinística no gate de CI.
- **Chosen approach**: Option A — busca tipada com teste no gate (`concept.md:9-12,29`).
- **In scope / out of scope**: In — modelos `SearchResult/SearchResponse`, `ApiClient.search()`, helpers puros, `SearchBox.svelte` com `$effect` debounced, teardown/`cancelled` guard, estados de vazio/carregamento/erro e a11y, substituindo o input decorativo; `tests/unit/searchApi.test.ts` executado no projeto `server` do Vitest e um `search.e2e.ts` opcional/informativo; drift documental em OBJETIVO/ROADMAP, anotação histórica "442", banners de supersessão nos snapshots de 032 e prosa de validação. Out — afirmar ADR-0002 #2 atendido (faltam URLs de runtime do FE, checagem de `Origin` em WS e prova ao vivo não-browser), implementar Documentos FASE 7, promover cargo-vet a required, criar tag/release, executar load/restore, transformar testes de componente/e2e informativo em gates, reivindicar relevância CJK/i18n ou prova de navegação em navegador real (decision.md antigo:8-14).
- **Success metrics**: Teste unitário do cliente de busca verde no projeto `server`; resultados tipados visíveis nos estados normal/vazio/erro; drift residual corrigido; required checks verdes (baseline: nenhum teste, campo decorativo, drift pendente, resultado de CI remoto/Forgejo [NV] — problem.md:30-35).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: relevância e suporte a i18n/CJK, dado que `type`/`source` são strings abertas]; [NEEDS CLARIFICATION: quais campos do resultado exibir e como tratar valores desconhecidos de `type`/`source`]; [NEEDS CLARIFICATION: é aceitável entregar sem prova em navegador real da navegação para o resultado?]; [NEEDS CLARIFICATION: quem decide quando retomar Documentos FASE 7 e a auditoria das demais affordances falsas]; [NV: resultado de CI remoto e Forgejo].
