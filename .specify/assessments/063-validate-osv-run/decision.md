# Decision: 063 — Validate OSV run

- **Slug**: 063-validate-osv-run
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | define.md | shape.md | decision.md (anterior) | specs/063-validate-osv-run/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | O pin `@v2.7.1` do ciclo 052 foi validado só estaticamente; sem execução real o sinal advisory de supply-chain não tem evidência de funcionamento (intake.md:9-11). |
| Evidence strength | strong | `gh api` confirma `state=active`, id `420259367`, `total_count: 0`; run `44239662140` terminou `completed/success` com timestamps; verificador 6/6 PASS (converge.md). |
| Value vs. inaction | adequate | Fecha o **[NV]** de 052 sem esperar o cron, mas `fail-on-vuln: false` torna `success` ≠ "zero vulnerabilidades" — ganho real porém limitado (I=2). |
| Feasibility / appetite | strong | Um `workflow_dispatch` no gatilho já declarado + apêndice de 27 linhas em `STATE.md`; nenhum código (shape.md; converge.md). |
| Strategic fit | adequate | Reforça a evidência de supply-chain dentro da governança, sem mudar código, gate ou workflow; permanece sinal advisory. |
| Risk posture | adequate | Efeito remoto controlado (run + upload de SARIF, `security-events: write`), sem credenciais copiadas; o run não é reversível, mas o registro é append-only e reversível por `git revert`. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado**: o gatilho `workflow_dispatch` é parte do próprio workflow — não é merge, force-push nem deleção —, logo não era necessária autorização adicional para **executar e registrar**. O desfecho foi registrado como evidência real, inclusive a ressalva de que `fail-on-vuln: false` impede ler `success` como ausência de vulnerabilidades (converge.md). Âncora `github/dev@e1ff815bf`, branch `feature/063-validate-osv-run`; factory fonte `b5bc7265…`, nada transplantado. Nenhuma alteração de workflow, ADR, gate ou config foi autorizada neste ciclo.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go`.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: O pin `@v2.7.1` do scan OSV carecia de execução real que provasse seu funcionamento.
- **Chosen approach**: Option A de `concept.md` — disparar via `gh workflow run osv-scan.yml` o gatilho já declarado, acompanhar até conclusão e registrar `id`, `status`, `conclusion` e timestamp em `STATE.md`, sem maquiar falhas.
- **In scope / out of scope**: 1 apêndice em `STATE.md` + artefatos do ciclo; fora código, workflow, pin, config, ADRs, constituição, gates, outros workflows e branches/PRs remotos.
- **Success metrics**: run criado e observado até conclusão; `conclusion` e timestamp colados em `STATE.md`; verificador/gates locais verdes; diff sem código (define.md SC-001–SC-004).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: desfecho real do run antes de obtido — **[NV]**, preenchido com `44239662140`/`success`]; [NEEDS CLARIFICATION: o upload de SARIF é comportamento esperado do workflow]; [NEEDS CLARIFICATION: as notas anteriores em `STATE.md` permanecem integralmente preservadas].
