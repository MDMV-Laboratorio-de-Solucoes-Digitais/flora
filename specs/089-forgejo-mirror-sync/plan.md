# Implementation Plan: Espelho Forgejo com push fast-forward (ciclo 089)

**Branch**: `feature/089-forgejo-mirror-sync` @ `15203ee78` | **Date**: 2026-11-20 | **Spec**: [spec.md](./spec.md)

**Input**: `/specs/089-forgejo-mirror-sync/spec.md`

## Summary

Publicar `dev`, `main` e tags `v*` no Forgejo por um job event-driven que só
faz push quando o commit local descende do tip remoto (fast-forward). Um
workflow novo, nenhum gate novo, nenhuma credencial no repositório.

## Technical Context

**Language/Version**: YAML de Actions + `bash` (`set -euo pipefail`); Git
(`merge-base`, `fetch`, `push`); `base64` para o header Basic.

**Primary Dependencies**: `actions/checkout@v4` com `fetch-depth: 0` e
`persist-credentials: false`; secrets `FORGEJO_USER`/`FORGEJO_TOKEN`; remote
`FORGEJO_REMOTE=https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`.

**Storage**: nenhum. Nenhuma rede além do push ao Forgejo; nenhum segredo
persistido em arquivo, URL ou log.

**Testing**: guarda de ancestralidade com `origin/dev` = `f12a328c` (ancestral
de `dev`); falha fechada em secret ausente, ref divergente e push rejeitado.
Estado ao vivo do remoto/job: `[NV]`.

**Target Platform**: `.github/workflows/mirror-to-forgejo.yml`, job `mirror`,
somente `on: push` (`dev`, `main`, `v*`); `permissions: {}`; `concurrency` por
ref sem cancelamento.

**Constraints**: fast-forward obrigatório — sem `--force`, `--mirror` ou
deleção de refs; job advisory fora dos required checks (ADR-0003); não tocar
`validate/`, `release/` nem `.specify/memory/STATE.md`.

**Scale/Scope**: 1 workflow novo + 4 artefatos 089 (decision/spec/plan/tasks).

## Constitution Check

*Gate*: **PASS**. Nenhum check obrigatório novo: o job roda só em push e não
entra na proteção de branch. ADR-0003 preservado — gate autoritativo continua
sendo `check_gate_integrity.py` (10 legs / 58 contratos, ciclo 088). Falha
fechada em secret ausente e em divergência; tabela de complexidade vazia.

## Arquivos tocados

| Arquivo | Mudança | Evidência |
|---|---|---|
| `.github/workflows/mirror-to-forgejo.yml` | novo — job `mirror`: secrets no `env`, Basic via `extraHeader`, fetch de `dev`/`main`, guarda `merge-base --is-ancestor`, push FF, tags `v*`, `permissions: {}`, `concurrency` serial | branch 089 @ `15203ee78`; `origin/dev` = `f12a328c` ancestral de `dev` |
| `.specify/assessments/089-forgejo-mirror-sync/decision.md` | novo — verdict go, 2026-11-20 | premissas FF / secrets / job não required |
| `specs/089-forgejo-mirror-sync/spec.md` | preenchido (FR-001..005, SC-001..003) | antes template |
| `specs/089-forgejo-mirror-sync/plan.md` | novo | 2026-11-20 |
| `specs/089-forgejo-mirror-sync/tasks.md` | novo — T001..T006 `[X]` | evidência registrada por tarefa |

**Fora de escopo**: required checks e proteção de branch, workflows de CI
existentes, `validate/`, `release/`, `.specify/memory/STATE.md`, ADRs,
constituição, código de aplicação, PRs e worktree espelho.

## Structure Decision

Nenhuma estrutura nova além do workflow. O espelho é iterativo por ref:
para branches, busca o tip Forgejo, valida ancestralidade e faz push simples;
para tags `v*`, empurra a ref sem force. Um único `concurrency group` por
`github.ref` evita sobreposição de pushes na mesma branch.

## Next Phase

Fechamento documental do ciclo 089. Nenhuma fase adicional; `[NV]` reconsulta
do estado ao vivo do Forgejo e da proteção de branch antes de qualquer merge
ou promoção.
