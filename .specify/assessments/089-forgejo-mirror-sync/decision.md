# Decision: espelho Forgejo com push fast-forward e job advisory (ciclo 089)

- **Slug**: 089-forgejo-mirror-sync
- **Decidido**: 2026-11-20
- **Verdict**: **go**
- **Branch**: `feature/089-forgejo-mirror-sync` @ `15203ee78`
- **Âncora**: `.specify/feature.json` → `specs/089-forgejo-mirror-sync`
- **Artefatos revisados**: `.github/workflows/mirror-to-forgejo.yml` (job de espelho) | spec | plan | tasks

## Contexto / Problema

O repositório tem dois remotos: `github` (fonte do CI autoritativo) e `origin`
= `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git` (espelho institucional
Forgejo). O histórico local e o espelho não divergiram: `origin/dev` =
`f12a328c` é ancestral de `dev`, portanto um push para o Forgejo é
*fast-forward*. O risco a evitar é um espelho que "sincroniza" apagando ou
reescrevendo histórico remoto, ou que transforme a sincronização em mais um
gate obrigatório de merge.

A regra ADR-0003 é explícita: jobs informativos/advisory **nunca** entram no
conjunto de required checks. O job de espelho é infraestrutura de publicação,
não verificação — ele não deve ser promovido a gate nem alterar a proteção de
`dev`/`main`. O gate autoritativo continua sendo
`scripts/ci/check_gate_integrity.py` com o oráculo dinâmico do ciclo 088
(10 legs / 58 contratos).

## Decisão

Adotar o job `.github/workflows/mirror-to-forgejo.yml` como espelho **one-way,
event-driven e advisory**:

1. **Fast-forward obrigatório**: antes de cada push de branch, o job atualiza
   `refs/remotes/forgejo/{dev,main}` e exige
   `git merge-base --is-ancestor forgejo/<branch> HEAD`. O push é feito com
   `git push forgejo HEAD:refs/heads/<branch>`, sem `--force`, sem `--mirror`,
   sem deleção de refs. Divergência, guarda falho ou push rejeitado = falha
   fechada, nunca reconciliação automática. Âncora observada: `origin/dev`
   `f12a328c` é ancestral de `dev` → FF válido.
2. **Credenciais isoladas**: `FORGEJO_USER` e `FORGEJO_TOKEN` vêm de
   `secrets.*` para o `env` do step; ausência de qualquer um deles encerra o
   job com erro. A URL do remoto (`FORGEJO_REMOTE`) não contém credenciais — a
   autenticação usa `http.<host>/.extraHeader` com Basic, e o valor é
   descartado (`unset`) após a configuração. Nenhum token aparece em logs,
   argumentos ou neste artefato.
3. **Escopo de refs**: gatilho apenas `push` em `dev`, `main` e tags `v*`;
   checkout com `fetch-depth: 0` e `persist-credentials: false`; serialização
   por ref (`cancel-in-progress: false`) para não cancelar um push em voo;
   `permissions: {}`.
4. **Fora do gate (ADR-0003)**: o job roda somente em `push`, seu nome não é
   adicionado ao conjunto de required checks, e nenhuma proteção de branch,
   `REQUIRED`, `ALLOWED_JOB_IF` ou job existente é alterado. CI green continua
   sendo a condição autoritativa de merge.
5. **Diff cirúrgico**: alterações limitadas ao workflow do espelho e aos
   quatro artefatos mínimos do ciclo 089. Nenhum arquivo em `validate/`,
   `release/` ou `.specify/memory/STATE.md` é tocado.

## Scorecard

| Critério | Nota | Justificativa |
|---|---|---|
| Validade do problema | strong | Forgejo é o remoto `origin` institucional; sem espelho explícito o histórico publicado fica defasado do `dev` local |
| Força da evidência | strong | `origin/dev` = `f12a328c` é ancestral de `dev` (FF verificável por `git merge-base`); remote e secrets nomeados no workflow |
| Valor vs. inação | strong | Publicação previsível e auditável no Forgejo sem reescrever histórico nem criar gate novo |
| Viabilidade | strong | Um workflow novo, sem dependências, sem migração, sem alteração de gate; job advisory por construção |
| Fit estratégico | strong | Reforça ADR-0003: espelho é infraestrutura informativa, não verificação; gate autoritativo permanece o check 088 |
| Risco | strong | Fail-closed em secret ausente, guarda de ancestralidade, push sem force, serialização por ref e escopo de diff confinado |

## Handoff

**Dentro**: `.github/workflows/mirror-to-forgejo.yml` e os quatro artefatos
089 (`decision.md`, `spec.md`, `plan.md`, `tasks.md`).

**Fora**: required checks e proteção de branch, `REQUIRED`/`ALLOWED_JOB_IF`,
workflows de CI existentes, `validate/`, `release/`, `.specify/memory/STATE.md`,
ADRs, constituição, código de aplicação, PRs e worktree espelho.

**Métricas de sucesso**: push em `dev` com `origin/dev` `f12a328c` ancestral
de `dev` → guarda FF passa e o push chega ao Forgejo sem force; secret
ausente ou divergência → falha fechada sem push; diff limitado ao workflow e
aos 4 artefatos 089, zero arquivos em `validate/`, `release/` ou `STATE.md`.

**[NV]** Estado ao vivo do remoto Forgejo, do job de espelho e da proteção de
branch: reconsultar antes de qualquer merge ou promoção. Nenhuma credencial de
Forgejo foi copiada, gravada ou exposta; gates, testes e formatters não foram
reexecutados nesta avaliação de documentação.
