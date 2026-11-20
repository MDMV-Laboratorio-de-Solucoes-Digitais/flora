# Tasks: 092 — npm lock sync

**Input**: `specs/092-npm-lock-sync/spec.md`, `plan.md`,
`.specify/assessments/092-npm-lock-sync/decision.md`

**Date**: 2026-11-20

**Format**: `[ID] Description`

## Phase 1 — Evidência

- [X] T001 Confirmar drift: `apps/frontend/package-lock.json` congelado e sem
  `shadcn-svelte`, enquanto o manifesto e `bun.lock` o contêm.
- [X] T002 Registrar evidência: `bun.lock` canónico para CI, 25 alertas do
  Dependabot no lockfile npm e comando contratual definido.

## Phase 2 — Execução

- [X] T003 Executar em `apps/frontend`:
  `npm install --package-lock-only --ignore-scripts`.

## Phase 3 — Verificação e fecho

- [X] T004 Verificar sincronia: `package-lock.json` contém `shadcn-svelte` e a
  reexecução do comando contratual é idempotente (SC-001).
- [X] T005 Verificar escopo: apenas `apps/frontend/package-lock.json` alterado;
  `package.json`, `bun.lock`, source e CI intactos; nenhum lifecycle script
  executado (SC-002, SC-003).
- [X] T006 Registrar a rastreabilidade em decision/spec/plan/tasks e fechar o
  ciclo, sem tocar em `validate`, `release` ou `STATE` (FR-005).

## Dependencies & Execution Order

T001 → T002 → T003 → T004 → T005 → T006 (sequencial; uma única alteração de
ficheiro, sem paralelismo real).
