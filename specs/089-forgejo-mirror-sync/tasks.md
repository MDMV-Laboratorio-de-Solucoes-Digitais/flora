# Tasks: Espelho Forgejo com push fast-forward e job advisory

**Input**: Design documents from `/specs/089-forgejo-mirror-sync/`

**Prerequisites**: plan.md, spec.md, `.specify/assessments/089-forgejo-mirror-sync/decision.md`

**Tests**: incluídos porque a spec exige guarda de ancestralidade e falha
fechada (SC-001/SC-002).

**Status**: T001–T006 concluídas — evidência registrada abaixo.

## Format: `[ID] [P?] Description`

- **[P]**: pode rodar em paralelo (arquivos distintos, sem dependência)

---

## Phase 1: Setup & Diagnosis

- [X] **T001** Confirmar âncora do ciclo 089.
  **Evidência**: `.specify/feature.json` =
  `{"feature_directory":"specs/089-forgejo-mirror-sync"}`; branch
  `feature/089-forgejo-mirror-sync` @ `15203ee78`; `origin` =
  `https://forgejo.mdmv.qzz.io/MDMV/flora-workspace.git`.

- [X] **T002** [P] Diagnosticar o estado do espelho e o limite de gate.
  **Evidência**: `origin/dev` = `f12a328c` é ancestral de `dev` — push é
  fast-forward, sem divergência a reconciliar. ADR-0003 mantém o job de
  espelho fora dos required checks; o gate autoritativo continua sendo
  `check_gate_integrity.py` (10 legs / 58 contratos, ciclo 088). Secrets
  identificados: `FORGEJO_USER` e `FORGEJO_TOKEN`.

---

## Phase 2: Implementation

- [X] **T003** Isolar credenciais e fail-closed.
  **Evidência**: `.github/workflows/mirror-to-forgejo.yml` lê
  `FORGEJO_USER`/`FORGEJO_TOKEN` de `secrets.*` via `env` do step, encerra com
  erro se algum estiver vazio, configura autenticação Basic por
  `http.<host>/.extraHeader` com a URL livre de credenciais e descarta o valor
  com `unset`. Checkout com `fetch-depth: 0` e `persist-credentials: false`;
  `permissions: {}` (FR-001/FR-002).

- [X] **T004** Implementar guarda de fast-forward e push sem force.
  **Evidência**: o job faz `git fetch forgejo` de `dev`/`main`, exige
  `git merge-base --is-ancestor forgejo/<branch> HEAD` e envia com
  `git push forgejo HEAD:refs/heads/<branch>` — sem `--force`, `--mirror` ou
  deleção. Tags `v*` são empurradas pela própria ref; qualquer outro ref é
  recusado. Divergência/push rejeitado falham sem reescrita (FR-003).

- [X] **T005** [P] Manter o job advisory e o diff cirúrgico.
  **Evidência**: gatilho restrito a `push` em `dev`, `main` e `v*`; nome
  `mirror` ausente do conjunto de required checks (ADR-0003); nenhuma
  alteração em proteção de branch, `REQUIRED`, `ALLOWED_JOB_IF`, workflows de
  CI existentes, `validate/`, `release/` ou `STATE.md`. `concurrency` por ref
  com `cancel-in-progress: false` serializa pushes (FR-004/FR-005).

---

## Phase 3: Verification & Documentation

- [X] **T006** Validar cenários, escopo e fechar os 4 artefatos 089.
  **Evidência**: SC-001 — com `origin/dev` `f12a328c` ancestral de `dev`, o
  guarda FF autoriza o push e nenhum secret aparece na saída; SC-002 — secret
  ausente, divergência, ref indisponível ou push rejeitado falham fechado sem
  force; SC-003 — diff limitado ao workflow e aos 4 artefatos, zero arquivos
  em `validate/`, `release/` ou `STATE.md`. Artefatos: `decision.md`
  (verdict go, 2026-11-20), `spec.md` (FR-001..005, SC-001..003), `plan.md`,
  `tasks.md`. **[NV]** estado ao vivo do Forgejo, do job e da proteção de
  branch — reconsultar antes de qualquer merge/promoção; subagente de
  documentação não reexecutou gates, testes ou formatters.

## Dependencies

- T002 bloqueia T003/T004/T005 (diagnóstico de FF e de gate antes da edição).
- T003 → T004 → T005 são incrementos no mesmo workflow (ordem preservada).
- T006 depende de T003+T004+T005.
- Nenhuma tarefa depende de Postgres, rede ao vivo, provisionamento de secret,
  PR ou worktree espelho.

## Implementation Strategy

MVP = T001–T005 (um workflow novo). T006 fecha verificação e documentação. O
diff permanece cirúrgico: 1 arquivo de CI + 4 artefatos; o espelho não cria
novo required check e nunca reescreve o histórico do Forgejo.
