# Decision: 062 — Escalation register

- **Slug**: 062-escalation-register
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | define.md | shape.md | decision.md (anterior) | specs/062-escalation-register/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | Quatro itens bloqueados (ADR-0003/0006, N3, C5, C6) são reavaliados toda sessão, com ~10 menções repetidas em `STATE.md` e nenhuma tabela única (intake.md:9-16). |
| Evidence strength | strong | `gh api` ao vivo: workflow `active`, id `420259367`, 0 runs, cron `37 14 * * 1`, pin `@v2.7.1` sem validação; verificador 6/6 PASS (research.md:5-8; converge.md). |
| Value vs. inaction | strong | Reduz gasto recorrente de análise e o risco de um ciclo iniciar trabalho não autorizado; E=5,0 (intake.md:27). |
| Feasibility / appetite | strong | Um apêndice de 25 linhas em `STATE.md`: tabela de 4 linhas + parágrafo OSV (shape.md; converge.md). |
| Strategic fit | strong | Disciplina de governança: consolida evidência, bloqueio e ação sem executar nenhum item escalado (define.md:9). |
| Risk posture | strong | Risco nulo: só metadados públicos do workflow, nenhuma credencial copiada, nenhum efeito remoto, reversível por `git revert`. |

## Verdict & Rationale

Veredito histórico **`go (scoped)` preservado**: o problema é de registro, não de execução; os valores do OSV já estavam verificados ao vivo e permitem satisfazer SC-002 sem efeito remoto. O escopo permaneceu um apêndice em `.specify/memory/STATE.md` (25+/0−), com âncora `github/dev@104c581f3` e factory fonte `4a5de2ce…`. Nenhuma autorização de maintainer/SE/dono foi simulada: ADRs, N3, C5/C6 e o disparo do OSV seguem escalados **[NV]** e intactos.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go`.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: Itens bloqueados por governança não têm registro canônico e o estado do scan OSV permanece **[NV]** sem contexto.
- **Chosen approach**: Option A de `concept.md` — tabela append-only em `STATE.md` com os 4 itens (evidência, bloqueio, ação necessária, status) e parágrafo com o estado OSV verificado via `gh api`.
- **In scope / out of scope**: 1 arquivo existente (`STATE.md`), artefatos do ciclo e notas datadas; fora ADRs, migração N3, código, CI, C5/C6, constituição, deploy e disparo do workflow OSV.
- **Success metrics**: tabela com 4 linhas e colunas completas; números/estados do OSV conferem com `gh api`; verificador isolado PASS; gates do alvo verdes (define.md SC-001–SC-004).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: quem aprova cada item e em qual canal a autorização é registrada — **[NV]**]; [NEEDS CLARIFICATION: quando ocorrerá a primeira execução real do OSV — **[NV]**]; [NEEDS CLARIFICATION: a tabela permanece append-only também para mudanças de status].
