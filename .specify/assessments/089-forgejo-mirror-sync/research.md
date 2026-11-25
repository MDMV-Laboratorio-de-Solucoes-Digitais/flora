# Idea Research: Espelho Forgejo advisory

- **Slug**: 089-forgejo-mirror-sync
- **Created**: 2026-11-25
- **Evidence confidence (overall)**: medium
- **Cobertura idiomática real**: pt-BR_en (artefatos e specs em pt-BR; workflow, comandos Git e nomes de secrets em inglês técnico)

Observação de método: research coleta e cita; não decide, não desenha solução e não atribui veredito. Números de linha citados são do checkout da worktree isolada (base `github/dev@e88dc8e4d`); o estado ao vivo do Forgejo, do job e da proteção de branch é **[NV]** e não foi reconsultado. O host `forgejo.mdmv.qzz.io` não está na allowlist — nenhum fetch foi feito e isso é registrado em `Sources`.

## Users & Demand

- O demandante é o mantenedor que precisa publicar `dev`, `main` e tags `v*` no espelho institucional sem reescrever histórico e sem adicionar required check [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:23-32; specs/089-forgejo-mirror-sync/spec.md:9-13] (confidence: high).
- O risco a evitar é duplo: um espelho que "sincroniza" apagando/reescrevendo histórico remoto, ou que transforme a sincronização em mais um gate obrigatório de merge [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:13-19; .specify/assessments/089-forgejo-mirror-sync/decision.md:16-25] (confidence: high).
- A premissa observada no checkout naquele ciclo era `origin/dev` = `f12a328c`, ancestral de `dev`, o que tornava o push um fast-forward válido [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:12-16; specs/089-forgejo-mirror-sync/spec.md:9-13] (confidence: high).
- A spec descreve o usuário operador de infraestrutura, que precisa que secrets ausentes nunca virem "sucesso" silencioso nem vazem para logs [source: specs/089-forgejo-mirror-sync/spec.md:41-62] (confidence: high).
- A direção registrada é job one-way, event-driven e advisory, com guarda de ancestralidade e gatilho restrito a `dev`, `main` e `v*` [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:17-21; .specify/assessments/089-forgejo-mirror-sync/decision.md:27-55] (confidence: high).

## Prior Art

- O repositório já tinha dois remotos: `github` (CI autoritativo) e `origin` apontando para `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:12-13; .github/workflows/mirror-to-forgejo.yml:26] (confidence: high).
- Já houve sincronização manual imediata com `git push origin refs/heads/dev:refs/heads/dev`, registrada como `f12a328c4..15203ee78 dev -> dev`, exit 0, após verificação de ancestralidade — sem force-push [source: .specify/memory/STATE.md:758-762] (confidence: high).
- O padrão de job advisory já existia na governança local: o gate autoritativo é `check_gate_integrity.py` (oráculo do ciclo 088) e jobs informativos não entram em required checks [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:20-25; .specify/memory/STATE.md:749-754] (confidence: high).
- O workflow de auditoria de proteção de branch criado no ciclo 091 segue o mesmo padrão (`schedule`/`workflow_dispatch`, `permissions: {}`, job advisory) — referência de formato, não do escopo 089 [source: .specify/memory/STATE.md:778-779] (confidence: high).
- A prática de manter logs sem credenciais e `persist-credentials: false` no checkout já era usada nos workflows do repo [source: .github/workflows/mirror-to-forgejo.yml:31-32,53-57; .specify/assessments/089-forgejo-mirror-sync/decision.md:39-44] (confidence: high).

## Market & Context

- A separação de papéis é explícita: GitHub é a fonte do CI autoritativo; Forgejo é o espelho institucional; Coolify é o destino de deploy — são contextos distintos, sem promoção de um a gate do outro [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:12-13,49-52; .specify/memory/STATE.md:756-762] (confidence: high).
- A regra ADR-0003 é o contexto de governança: nenhum check informativo entra no conjunto de required checks, e nenhuma proteção de `dev`/`main` é alterada [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:20-25,49-52] (confidence: high).
- O gate autoritativo mencionado no artefato era o oráculo 088 com 10 legs/58 contratos; no checkout atual o guarda tem 11 entradas/87 [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:23-25; scripts/ci/check_gate_integrity.py:42-54] (confidence: high).
- O diff registrado ficou limitado ao workflow do espelho e aos quatro artefatos 089; nenhum arquivo em `validate/`, `release/` ou `STATE.md` foi tocado [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:53-55,68-80] (confidence: high).
- Estado ao vivo do remoto Forgejo, do job e da proteção de branch permanece **[NV]** no artefato e nesta leitura [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:82-85; .specify/memory/STATE.md:764] (confidence: high).

## Data & Constraints

- O workflow `.github/workflows/mirror-to-forgejo.yml` dispara somente em `push` nas branches `dev`/`main` e tags `v*`, com `permissions: {}` e `concurrency` por `github.ref` com `cancel-in-progress: false` [source: .github/workflows/mirror-to-forgejo.yml:1-19] (confidence: high).
- Checkout com `fetch-depth: 0` e `persist-credentials: false`; `FORGEJO_REMOTE` é URL não secreta; `FORGEJO_USER`/`FORGEJO_TOKEN` vêm de `secrets.*` para o `env` do step [source: .github/workflows/mirror-to-forgejo.yml:26-39] (confidence: high).
- Falha fechada em secret ausente; auth via `http.<host>/.extraHeader` com Basic; `AUTH_B64` é descartado com `unset` [source: .github/workflows/mirror-to-forgejo.yml:44-57] (confidence: high).
- Guarda de ancestralidade: `for branch in dev main; git fetch ... || true`, depois `git merge-base --is-ancestor forgejo/<REF_NAME> HEAD` e `git push forgejo HEAD:refs/heads/<REF_NAME>` para branches; nenhum `--force`, `--mirror` ou deleção de refs [source: .github/workflows/mirror-to-forgejo.yml:61-75] (confidence: high).
- Refs fora do escopo são recusadas em defesa em profundidade (`case` para branch `dev|main` e tag `v*`) [source: .github/workflows/mirror-to-forgejo.yml:67-87] (confidence: high).
- O push de tags usa `git push forgejo refs/tags/${REF_NAME}:refs/tags/${REF_NAME}` sem o guarda de ancestralidade aplicado a branches [source: .github/workflows/mirror-to-forgejo.yml:77-82] (confidence: high).
- Nenhuma credencial de Forgejo foi copiada ou gravada nos artefatos; o provisionamento dos secrets no GitHub é **[NV]** [source: .specify/memory/STATE.md:763-764; .specify/assessments/089-forgejo-mirror-sync/intake.md:37] (confidence: high).

## Evidence Against the Idea

- O guarda de fast-forward cobre apenas pushes de branch; o caminho de tags (`v*`) faz push sem `merge-base --is-ancestor`, de modo que a afirmação "nunca reescreve histórico" não é integralmente verificável pelo workflow para tags [source: .github/workflows/mirror-to-forgejo.yml:70-82; .specify/assessments/089-forgejo-mirror-sync/decision.md:32-38] (confidence: high).
- `cancel-in-progress: false` serializa execuções do **mesmo** `github.ref`; a pergunta sobre pushes simultâneos em refs diferentes permanece sem evidência de coordenação [source: .github/workflows/mirror-to-forgejo.yml:17-19; .specify/assessments/089-forgejo-mirror-sync/intake.md:40] (confidence: high).
- O `git fetch ... || true` tolera falha de fetch; o guarda de ancestralidade então falha fechado se o ref de destino não existir, o que pode impedir o espelho mesmo quando o remoto está apenas temporariamente indisponível [source: .github/workflows/mirror-to-forgejo.yml:61-73] (confidence: medium).
- Não foi possível verificar o estado ao vivo do remoto/job/proteção: o host `forgejo.mdmv.qzz.io` não é allowlisted (fetch auto-refused) e o estado do GitHub foi registrado como `[NV]` [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:82-85; .specify/memory/STATE.md:764] (confidence: high).
- A premissa `origin/dev` = `f12a328c` é um snapshot de 2026-11-20 e pode não refletir o tip atual do espelho; qualquer confirmação de FF depende de reconsulta ao vivo [source: .specify/assessments/089-forgejo-mirror-sync/decision.md:14-16,37-38] (confidence: high).
- A ausência do job nos required checks foi verificada por leitura do workflow e pelo registro de governança, mas a proteção de branch ao vivo não foi lida nesta avaliação — a coerência com ADR-0003 depende dessa leitura [source: .github/workflows/mirror-to-forgejo.yml:4-5; .specify/memory/STATE.md:749-754] (confidence: medium).
- Nenhum mecanismo de reconciliação humana para divergência futura está descrito nos artefatos; o job apenas falha fechado [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:39; .specify/assessments/089-forgejo-mirror-sync/decision.md:36-37] (confidence: high).

## Gaps & Open Questions

- [NEEDS CLARIFICATION: [NV] qual é o estado ao vivo do remoto Forgejo, do job de espelho e da proteção de branch?] [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:36; .specify/assessments/089-forgejo-mirror-sync/decision.md:82-85] (confidence: high).
- [NEEDS CLARIFICATION: `FORGEJO_USER` e `FORGEJO_TOKEN` estão efetivamente provisionados como secrets no repositório e quem rotaciona ou valida esses valores?] [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:37; .specify/memory/STATE.md:763-764] (confidence: high).
- [NEEDS CLARIFICATION: em caso de divergência futura entre local e espelho, qual é o processo humano de reconciliação, dado que o job falha fechado sem reescrever?] [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:39] (confidence: high).
- [NEEDS CLARIFICATION: o que acontece com branches remotas ausentes ou com branches/tags fora do escopo `dev`/`main`/`v*` e quem revisa essas recusas?] [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:40] (confidence: high).
- [NEEDS CLARIFICATION: a serialização por ref (`cancel-in-progress: false`) basta para pushes simultâneos em refs diferentes ou é preciso coordenação adicional?] [source: .specify/assessments/089-forgejo-mirror-sync/intake.md:40; .github/workflows/mirror-to-forgejo.yml:17-19] (confidence: high).
- [NEEDS CLARIFICATION: o caminho de tags deve receber o mesmo guarda de ancestralidade aplicado às branches, ou a ausência dele é aceita?] [source: .github/workflows/mirror-to-forgejo.yml:70-82] (confidence: high).
- [ASSUMPTION] O espelho é one-way e o histórico local é a fonte da verdade; nenhuma reconciliação reversa foi observada nos artefatos. (confidence: low).

## Sources

- `system` | `.specify/assessments/089-forgejo-mirror-sync/intake.md:1-42` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/assessments/089-forgejo-mirror-sync/decision.md:1-85` | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `specs/089-forgejo-mirror-sync/spec.md:9-177` e `plan.md:13-58` | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.github/workflows/mirror-to-forgejo.yml:1-87` (gatilhos, secrets, guarda FF, push de tags) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `.specify/memory/STATE.md:749-764,776-779` e `docs/PROGRESSO.md:222-236` (registro do ciclo 089 e auditorias posteriores) | lidos diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- `system` | `scripts/ci/check_gate_integrity.py:42-54` (estado atual do gate autoritativo) | lido diretamente 2026-11-24 | policy: system (repo local) | confidence: high.
- URL `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` | host `forgejo.mdmv.qzz.io` | `[UNVERIFIED — fetch skipped: host not allowlisted]` em 2026-11-24 | policy: auto-refused: host not allowlisted | nenhum redirect seguido, nenhuma credencial usada.
- `https://github.com/MDMV-Laboratorio-de-Solucoes-Digitais/flora` | host `github.com` | policy: allowlisted, **não consultado** — estado remoto/job/proteção marcado `[NV]`; nenhum token ou chamada à API em 2026-11-24.
