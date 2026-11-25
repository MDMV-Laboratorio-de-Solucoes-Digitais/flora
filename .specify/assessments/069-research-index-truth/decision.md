# Decision: Corrigir nota falsa de dívida no índice de research

- **Slug**: 069-research-index-truth
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | shape.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `docs/research/index.md:172-179` afirma dívida de indexação 047+, mas a verificação de conjunto mostra cobertura completa (`intake.md:7`, `research.md:9`). |
| Evidence strength | strong | 137 arquivos = 137 links, `missing=[]`, `extra=[]`, seções `3+124+8+2=137` — evidência local determinística e reproduzível (`research.md:7-10`). |
| Value vs. inaction | strong | A nota falsa induz backfill de trabalho já feito e desconfiança de cobertura inexistente (`define.md:5`). |
| Feasibility / appetite | strong | Um único apêndice datado; sem renomeação em massa, sem código — apetite small (`shape.md:5-12`). |
| Strategic fit | adequate | Disciplina append-only preservada; emenda constitucional escalada `[NV]` por exigir maintainer (`intake.md:22`). |
| Risk posture | strong | Adição pura (`+N/-0`), zero mudança em código/workflows/gates/ADRs; merge só com CI verde observada ao vivo (`shape.md:16-20`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown` e os artefatos não apresentam contradição. O resíduo real não é cobertura de links, mas rastreabilidade de ciclo nos títulos das entradas 047–066 (`research.md:15`); a correção documental resolve isso sem reescrever histórico. CI futura, Forgejo, produção, aprovações e a emenda constitucional permanecem `[NV]`.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma; a única incógnita (emenda constitucional) está escalada `[NV]` e fora do escopo.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: nota falsa de dívida de indexação 047+ no índice de research pode levar a backfill de trabalho inexistente.
- **Chosen approach**: apêndice datado único ao fim de `docs/research/index.md`, com resultado 137/137 e comando reprodutível, preservando as notas antigas como histórico (`shape.md:5-12`).
- **In scope / out of scope**: dentro = `docs/research/index.md`, artefatos do ciclo e `STATE.md`; fora = backfill de entradas, renomeação em massa de títulos, emenda constitucional, indexação remota/Forgejo e merge sem CI verde ao vivo (`define.md:19-21`).
- **Success metrics**: SC-001 verificação continua `missing=[]`, `extra=[]` e 137 links; SC-002 gate autoritativo local/`--remote` PASS e gates Rust/frontend do alvo verdes; SC-003 `git diff --name-only dev..HEAD` contém apenas os paths declarados (`define.md:13-17`).
- **Carried-forward open questions**: emenda constitucional escalada `[NV]` (requer maintainer); CI futura/Forgejo/produção/aprovações `[NV]` — reconsultar ao vivo antes do merge.
