# Decision: 060 — Refresh verified counts

- **Slug**: 060-refresh-verified-counts
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | define.md | shape.md | decision.md (anterior) | specs/060-refresh-verified-counts/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | A nota de "estado verificado" em `docs/PROGRESSO.md:11-22` (02:23Z) ficou defasada pelos ciclos 057–059 e `index.md:15` defere explicitamente a contagem a ela (intake.md:9-12). |
| Evidence strength | strong | Contagens por `find` determinísticas (specs 50/224, assessments 47/192, validations 46/47, releases 46/46, bugs 4/12, research 135, adr 6); 12/12 números conferidos e verificador 6/6 PASS (converge.md); remoto **[NV]**. |
| Value vs. inaction | strong | Leitores e a entrada raiz herdam números errados de uma seção rotulada "verificado"; a dívida cresce a cada ciclo (E=5,0 — intake.md:25). |
| Feasibility / appetite | strong | Um único apêndice em 1 arquivo existente; nenhum gerador, nenhuma automação (concept.md Option A). |
| Strategic fit | adequate | Honra a convenção append-only aprovada no ciclo 056, mas deixa ADR-0003/0006 e N3 escalados — fora da autorização deste ciclo. |
| Risk posture | strong | Risco nulo: só acréscimo datado, reversível por `git revert`, sem credenciais e sem código/migration/gate. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado**: o problema é real e mediível, a evidência é local e determinística, e a opção escolhida não apaga histórico. O escopo restou estritamente documental — `docs/PROGRESSO.md`, artefatos do ciclo e notas datadas — com âncora `github/dev@262ac042f`. O próprio ciclo deslocou as contagens ao acrescentar seus artefatos, o que motivou a correção append-only das 03:52Z (specs 51 dirs/230 arquivos, assessments 48/197) sem reescrever a nota das 03:51Z (converge.md). Nada aqui autoriza ADRs ou a migração N3; o estado remoto/CI segue **[NV]**.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go`.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: A seção "estado verificado" de `docs/PROGRESSO.md` ficou desatualizada após os ciclos 057–059 e é a fonte deferida por `index.md`.
- **Chosen approach**: Option A de `concept.md` — apêndice datado com as contagens obtidas por `find`, âncora `262ac042f` e nota **[NV]** para o estado remoto, preservando integralmente a nota anterior.
- **In scope / out of scope**: 1 arquivo existente (`docs/PROGRESSO.md`), artefatos do ciclo e notas datadas; fora reescrita da nota antiga, `index.md`, código, migrations, ADRs, CI e gate.
- **Success metrics**: cada número confere com `find`; `git diff` mostra apenas acréscimos ao final; verificador isolado confirma contagens e preservação; gates do alvo verdes (define.md SC-001–SC-004).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: periodicidade das próximas atualizações da nota]; [NEEDS CLARIFICATION: em qual ciclo chegará a aprovação para corrigir ADR-0003/0006]; [NEEDS CLARIFICATION: em qual PR o estado remoto/CI será verificado — **[NV]**].
