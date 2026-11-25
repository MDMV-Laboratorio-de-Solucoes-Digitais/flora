# Problem Definition: Publicação previsível no espelho institucional

- **Slug**: 089-forgejo-mirror-sync
- **Created**: 2026-11-25
- **Inputs used**: intake.md | research.md | decision.md

## Problem Statement

O repositório tem dois remotos com papéis distintos: `github`, fonte do CI autoritativo, e `origin`, o espelho institucional Forgejo (`decision.md:12-13`). Sem um mecanismo explícito de espelhamento, o histórico publicado pode ficar defasado do `dev` local, e uma "sincronização" mal limitada corre o risco de apagar ou reescrever histórico remoto, ou de se transformar em mais um gate obrigatório de merge — o que contrariaria a ADR-0003 (`decision.md:18-25`). A premissa observada no ciclo era `origin/dev` = `f12a328c`, ancestral de `dev`, de modo que um push seria fast-forward (`decision.md:14-16`); trata-se de snapshot de 2026-11-20, não de estado corrente (`research.md:50`). O estado ao vivo do remoto, do job e da proteção de branch é **[NV]** e o host do espelho não estava na allowlist, sem fetch nesta avaliação (`decision.md:82-85`; `research.md:49`).

## Affected Users & Stakeholders

- **Users**: mantenedor/operador de infraestrutura, que precisa publicar `dev`, `main` e tags `v*` no espelho sem reescrever histórico e sem adicionar required check (`research.md:25`); operador que precisa que credenciais ausentes nunca virem "sucesso" silencioso nem vazem para logs (`research.md:28`).
- **Stakeholders**: dono do processo ADR-0003; maintainers que consomem o espelho institucional; responsável pela proteção de `dev`/`main` e pela guarda dos secrets.

## Goals

- Manter o espelho institucional atualizado de forma previsível e auditável em relação ao histórico local.
- Garantir que a publicação não apague, reescreva ou force atualização de histórico remoto (`decision.md:32-38`).
- Manter a publicação como infraestrutura informativa, fora dos required checks, com o gate autoritativo inalterado (`decision.md:49-52`).
- Garantir falha fechada em divergência, guarda falho, push rejeitado ou credencial ausente, sem exposição de secrets em logs ou artefatos (`decision.md:39-44`).
- Restringir o escopo de publicação às refs acordadas, com serialização que não cancele um push em voo (`research.md:31`).

## Non-Goals

- Alterar required checks, proteção de branch, `REQUIRED`/`ALLOWED_JOB_IF` ou workflows de CI existentes.
- Promover o job de espelho a gate nem substituir o gate autoritativo (`decision.md:49-52`).
- Reconciliação reversa (espelho → local) ou reescrita de histórico; o caráter one-way é pressuposto registrado (`research.md:62`).
- Tocar `validate/`, `release/`, `.specify/memory/STATE.md`, ADRs, constituição ou código de aplicação.
- Afirmar o estado ao vivo do remoto, do job ou da proteção sem marcação **[NV]** (`decision.md:82-85`).

## Success Metrics

- Push de branch com o ref remoto ancestral do local chega ao espelho sem force (baseline: `origin/dev` = `f12a328c` ancestral de `dev` em snapshot de 2026-11-20 — `decision.md:14-16`).
- Divergência, secret ausente ou push rejeitado resulta em falha fechada sem publicação (baseline: risco registrado, não medido — `decision.md:36-44`).
- Job ausente dos required checks e nenhuma proteção de branch alterada (baseline: leitura do workflow e registro de governança; proteção ao vivo **[NV]** — `research.md:51`).
- Diff confinado ao workflow do espelho e aos artefatos do ciclo, com zero arquivos em `validate/`, `release/` ou `STATE.md` (baseline: registrado no handoff — `decision.md:68-80`).

## Cost of Inaction

Sem espelhamento explícito, o histórico institucional permanece defasado do `dev`, e uma publicação improvisada pode reescrever refs já divulgadas. A ausência de fronteira entre publicação e verificação abre espaço para que um job informativo seja promovido a required check, deslocando a autoridade de merge prevista na ADR-0003; sem falha fechada, uma credencial ausente pode passar por sucesso.

## Open Questions

- [NEEDS CLARIFICATION: **[NV]** qual é o estado ao vivo do remoto Forgejo, do job de espelho e da proteção de branch?] (`intake.md:36`; `decision.md:82-85`)
- [NEEDS CLARIFICATION: `FORGEJO_USER` e `FORGEJO_TOKEN` estão efetivamente provisionados como secrets e quem rotaciona ou valida esses valores?] (`intake.md:37`; `research.md:57`)
- [NEEDS CLARIFICATION: em caso de divergência futura entre local e espelho, qual é o processo humano de reconciliação, dado que o job falha fechado sem reescrever?] (`intake.md:39`)
- [NEEDS CLARIFICATION: o que acontece com branches remotas ausentes ou com refs fora do escopo `dev`/`main`/`v*` e quem revisa essas recusas?] (`intake.md:40`)
- [NEEDS CLARIFICATION: a serialização por ref (`cancel-in-progress: false`) basta para pushes simultâneos em refs diferentes ou é preciso coordenação adicional?] (`intake.md:40`; `research.md:60`)
- [NEEDS CLARIFICATION: o caminho de tags deve receber o mesmo guarda de ancestralidade aplicado às branches, ou a ausência dele é aceita?] (`research.md:61`)
