# Decision: Refresh verificado de estado (069/070)

- **Slug**: 072-verified-state-refresh
- **Decided**: 2026-11-25
- **Verdict**: go
- **Artifacts reviewed**: intake.md | research.md | define.md | concept.md

## Scorecard

| Criterion | Rating | Justification |
|-----------|--------|---------------|
| Problem validity | adequate | Quatro âncoras documentais param sua última nota “Estado verificado” no ciclo 068 (01:58Z); 069/070 fechados e mesclados sem nota (`intake.md:9`, `research.md:20-24`). |
| Evidence strength | strong | Contagens por `find` (61/58/57/57/4/137), gate local `PASS` com 4 checks e 8 pisos/51, e diff de nomes de artefatos — tudo local e reproduzível (`research.md:9-23`). |
| Value vs. inaction | adequate | Evita que agentes reabram a dívida 047+ já eliminada em 069 e citem números mortos (58/55/54/54) em decisões futuras (`define.md:38-40`). |
| Feasibility / appetite | strong | Opção A: quatro apêndices datados + artefatos; apetite small (`concept.md:7-11`). |
| Strategic fit | adequate | Constituição §V (Spec-Driven) e disciplina append-only preservadas; a correção de 069 é preservada, não reescrita (`research.md:28-30`, `define.md:21`). |
| Risk posture | strong | Nenhuma mudança de código/CI; risco principal (reescrita de histórico) mitigado por SC-003 (`concept.md:11`, `define.md:36`). |

## Verdict & Rationale

Veredito histórico `go (scoped)` de 2026-11-19 preservado como `go`: nenhum critério é `weak`/`unknown` e os artefatos não trazem contradição. A opção A satisfaz SC-001/SC-003 com menor risco e é o padrão já usado nos ciclos 068/069; a opção B falha SC-001 e a C viola a convenção append-only (`concept.md:28-30`). Começou-se com `feature.json` apontando 051 e `check-prerequisites.sh --json` confirma a identidade — a divergência é registrada, não reescrita (`define.md:43`). CI ao vivo, merge, proteção de branch, Forgejo e produção permanecem `[NV]`.

## If needs-clarification

- **Blocking questions**: N/A — nenhuma (`define.md:42-43`).
- **Revisit stage**: N/A

## If go — Handoff to `/speckit.specify`

- **Problem**: notas de “Estado verificado” desatualizadas em `OBJETIVO.md`, `ROADMAP.md`, `docs/PROGRESSO.md` e `docs/research/index.md` após os ciclos 069/070.
- **Chosen approach**: Opção A — apêndice datado, append-only, nos quatro documentos, com âncora local, contagens reais e intervalo 001–070 (`concept.md:7-11`).
- **In scope / out of scope**: dentro = `OBJETIVO.md`, `ROADMAP.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `.specify/memory/STATE.md` e artefatos do ciclo 072; fora = código, CI, gates, pisos, ADRs, constituição, worktree/branch `feature/071-dead-auth-service`, leg RBAC (depende de maintainer), emenda constitucional, regeneração de `.generated.md` e afirmações remotas (`concept.md:32-34`, `define.md:23-29`).
- **Success metrics**: SC-001 apêndice datado nos quatro documentos com 61/58/57/57/4/137 e intervalo até 070; SC-002 diff só nos quatro documentos, artefatos 072 e `STATE.md`; SC-003 nenhuma linha histórica removida/modificada; SC-004 gate PASS com mesmas 4 checks e 8 pisos/51 e nenhum `[NV]` convertido em fato (`define.md:31-36`).
- **Carried-forward open questions**: nenhuma bloqueante; `[NV]` para CI ao vivo/Forgejo/produção; dívida da leg RBAC escalada a maintainer; emenda constitucional fora de escopo.
