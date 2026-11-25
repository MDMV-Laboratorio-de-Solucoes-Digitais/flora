# Decision: 065 — ADR floor alignment

- **Slug**: 065-adr-floor-alignment
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | problem.md | concept.md | define.md | shape.md | decision.md (anterior) | specs/065-adr-floor-alignment/converge.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | strong | `docs/adr/0003:19` cita `12/4/8/8/5/6 — total 43` e `0006:10,52` cita "piso 43", enquanto o gate determinístico exige 51 (research.md:5-10). |
| Evidence strength | strong | `FLOORS` em `check_gate_integrity.py:42-51` dão `13/4/8/8/5/6/4/3 = 51`; `grep` localiza as três citações antigas; autorização SE registrada em STATE 13:04Z (intake.md:11-17). |
| Value vs. inaction | strong | Decisões futuras poderiam usar 43 ou o detalhamento `/4/2` (que soma 50); E=10,0, maior candidato autorizado (intake.md:29). |
| Feasibility / appetite | strong | Três citações literais em 2 ADRs, sem tocar numeração/versão (shape.md). |
| Strategic fit | adequate | Alinha documentação de governança à autoridade determinística dentro da autorização; constituição, código, C5/C6 ficam de fora. |
| Risk posture | strong | Documentação reversível por `git revert`, sem código/gate; único **[NV]** é o estado remoto/CI. |

## Verdict & Rationale

Veredito histórico **`go (scoped — autorização SE registrada em STATE 13:04Z)` preservado**: o SE citou "total 50", valor vigente antes do ciclo 064, mas o valor verificado é 51 (workspace passou de 2 para 3); a divergência 50→51 foi registrada e o alinhamento seguiu o gate, não a citação (intake.md:9; converge.md). A correção pós-verificação das 13:12Z fixou o detalhamento correto `13/4/8/8/5/6/4/3 — total 51`, substituindo `/4/2` que somava 50 e contradizia o próprio total. Âncora `github/dev@aef13277a`, branch `feature/065-adr-floor-alignment`; factory fonte `c81f0f81…`. Estado remoto/CI permanece **[NV]**.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma pergunta bloqueia o avanço; veredito é `go` e a autorização SE está registrada.
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: ADRs de governança citam pisos obsoletos (43) enquanto o guardião determinístico exige 51.
- **Chosen approach**: Option A de `concept.md` — atualizar as citações ao valor vigente do gate (`0003:19` → `13/4/8/8/5/6/4/3 — total 51`; `0006:10,52` → `piso 51`) e registrar autorização/divergência 50→51 em `STATE.md`.
- **In scope / out of scope**: 2 ADRs + artefatos do ciclo e notas datadas; fora constituição, código, gate, workflows, C5/C6 e reescrita do histórico de `STATE.md`/converge.
- **Success metrics**: `grep -n 43` nas citações de piso → 0; cifra impressa pelo guardião = 51; detalhamento soma 51; diff = 2 ADRs + artefatos; gates locais verdes (define.md SC-001–SC-004 e correção 13:12Z).
- **Carried-forward open questions**: [NEEDS CLARIFICATION: a divergência 50→51 exige nova aprovação de maintainer além da registrada — **[NV]**]; [NEEDS CLARIFICATION: as três citações são as únicas com "43"]; [NEEDS CLARIFICATION: em qual PR o estado remoto/CI será observado — **[NV]**].
