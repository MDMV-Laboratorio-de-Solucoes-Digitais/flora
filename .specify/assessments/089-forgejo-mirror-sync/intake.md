# Idea Intake: Espelho Forgejo advisory

- **Slug**: 089-forgejo-mirror-sync
- **Created**: 2026-11-20
- **Source**: repositório local — `.specify/assessments/089-forgejo-mirror-sync/decision.md` (+ `specs/089-forgejo-mirror-sync/spec.md`)
- **Type**: new-capability

## Idea (as captured)

O repositório tem dois remotos: `github`, fonte do CI autoritativo, e `origin`,
o espelho institucional Forgejo (`decision.md:12-13`). Histórico local e espelho
não divergiram: `origin/dev` = `f12a328c` é ancestral de `dev`, portanto um push
ao Forgejo seria fast-forward (`decision.md:14-16`). O risco registrado é um
espelho que "sincroniza" apagando ou reescrevendo histórico remoto, ou que
transforme a sincronização em mais um gate obrigatório de merge, contrariando a
ADR-0003 (`decision.md:18-25`). O artefato registra como direção adotar o job de
espelho como one-way, event-driven e advisory, com guarda de ancestralidade,
credenciais isoladas e gatilho restrito a `dev`, `main` e tags `v*`
(`decision.md:27-55`; `specs/089-forgejo-mirror-sync/spec.md:9-13,17-21`).
Estado ao vivo do remoto, do job e da proteção de branch: **[NV]** — reconsultar
antes de qualquer merge ou promoção (`decision.md:82-85`).

## Restated

Propõe-se publicar `dev`, `main` e tags `v*` no espelho Forgejo por push
fast-forward, sem reescrever histórico e sem adicionar required check, mantendo
o gate autoritativo existente como condição de merge.

## Origin & Context

- **Raised by**: Engenheiro de Software / maintainer da MDMV (`@lfgranja`), conforme registrado nos artefatos do ciclo.
- **Trigger**: existência do remoto institucional Forgejo sem mecanismo explícito de espelhamento, com `origin/dev` = `f12a328c` já ancestral de `dev` e a exigência da ADR-0003 de manter jobs informativos fora dos required checks (`decision.md:12-25`).

## First-Glance Unknowns

- [NEEDS CLARIFICATION: [NV] qual é o estado ao vivo do remoto Forgejo, do job de espelho e da proteção de branch?]
- [NEEDS CLARIFICATION: `FORGEJO_USER` e `FORGEJO_TOKEN` estão efetivamente provisionados como secrets no repositório e quem rotaciona ou valida esses valores?]
- [NEEDS CLARIFICATION: em caso de divergência futura entre local e espelho, qual é o processo humano de reconciliação, dado que o job falha fechado sem reescrever?]
- [NEEDS CLARIFICATION: o que acontece com branches remotas ausentes ou com branches/tags fora do escopo `dev`/`main`/`v*` e quem revisa essas recusas?]
- [NEEDS CLARIFICATION: a serialização por ref (`cancel-in-progress: false`) basta para pushes simultâneos em refs diferentes ou é preciso coordenação adicional?]
