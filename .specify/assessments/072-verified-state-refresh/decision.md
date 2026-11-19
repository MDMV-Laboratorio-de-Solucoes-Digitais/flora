# Decision: refresh verificado de estado (072)

- **Slug**: 072-verified-state-refresh
- **Decidido**: 2026-11-19T04:24Z
- **Verdict**: **go (scoped)**
- **Artefatos revisados**: intake.md | research.md | define.md | concept.md

## Pontuação

| Critério | Nota | Justificativa |
|---|---|---|
| Validade do problema | adequate | Quatro âncoras documentais param em 068; 069/070 fechados sem nota. |
| Força da evidência | strong | Contagens por `find`, gate local colado, diff de nomes de artefatos — tudo local e reproduzível. |
| Valor vs. inação | adequate | Evita que agentes reabram dívida 047+ já eliminada e citem números mortos. |
| Viabilidade/apetite | strong | Opção A small, quatro apêndices + artefatos. |
| Fit estratégico | adequate | Constituição §V (Spec-Driven) e disciplina append-only preservadas. |
| Postura de risco | strong | Nenhuma mudança de código/CI; risco principal (reescrita de histórico) mitigado por SC-003. |

## Veredito

`go (scoped)`: avançar para `/speckit.specify`. Nenhum critério `weak`/`unknown`. Escopo = opção A do concept.

## Handoff para specify

- **Problema**: notas de estado desatualizadas em quatro documentos âncora.
- **Opção recomendada**: apêndice datado, append-only, por documento.
- **Dentro**: `OBJETIVO.md`, `ROADMAP.md`, `docs/PROGRESSO.md`, `docs/research/index.md`, `index.md` (quando aplicável), `.specify/memory/STATE.md`, artefatos do ciclo 072.
- **Fora**: código, CI, gates, pisos, ADRs, constituição, worktree 071, leg RBAC, afirmações remotas.
- **Métricas**: SC-001..SC-004 de `define.md`.
- **Perguntas carry-forward**: nenhuma. `[NV]` para CI futuro/Forgejo/produção.
