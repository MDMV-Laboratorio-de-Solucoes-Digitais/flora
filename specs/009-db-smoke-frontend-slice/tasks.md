# Tasks: 009 — DB-backed Select-Org Tests, Integrated Smoke & Minimal Frontend Slice

**Status legend:** `[X]` done, `[~]` in progress, `[ ]` pending, `[NV]` blocked/not verifiable

## Phase 0 — Research & scoping
- [X] T001 Criar scaffolding de assessment/spec 009.
- [X] T002 Dark-factory workflow/gates (`ses_e1e8f0341ffeS9O37ZKM946HT1`) — incorporado.
- [X] T003 Audit DB-backed tests + CI Postgres (`ses_e1e8f0341ffdZw46QTHDDb5BhM`) — incorporado.
- [X] T004 Audit frontend minimal slice (`ses_e1e8f0341ffcqTrfH71HTjgehq`) — incorporado.
- [X] T005 Pesquisa multi-idioma (`ses_e1e8f0340ffe4m9OxyO9VZASaF`) — incorporado.
- [X] T006 Probe viabilidade smoke integrado (`ses_e1e8f0340ffdtK4QZLcUFWjhpp`) — no-go as-is (5432 ocupado); alternativa executada.
- [X] T007 Consolidar achados e finalizar decisão/escopo.

## Phase 1 — DB-backed tests & CI
- [X] T010 Implementar testes DB-backed/gated para `select-organization` (5 casos).
- [X] T011 Ajustar CI (job `db-tests` com Postgres, sem alterar `build`).
- [X] T012 Executar/verificar testes e registrar evidência (5 passed local pg18; CI remoto [NV] até PR).

## Phase 2 — Integrated smoke
- [X] T020 Executar smoke integrado (`docs/dev-smoke.md`) com evidência (rc=0, stack alternativa pg18/ports 15432,16379,17700) e registrar NV de compatibilidade compose/pg16-`uuidv7`.

## Phase 3 — Frontend minimal slice
- [X] T030 Implementar callback `state` + select-org/token rotation + persistência/guards (`AuthState`, auth routes, `select-org`, layout).
- [X] T031 Implementar slice workspace → tarefa mínimo alinhado (seleção workspace, create/list tasks via API, assign-to-me).
- [~] T032 Verificações determinísticas frontend (`bun run check/build/test:unit`) — [NV] execução completa no ambiente atual não confirmada; toolchain presente (bun 1.4.4) mas script/ambiente a validar em CI/prepared env.

## Phase 4 — Delivery
- [X] T040 Atualizar artefatos 009 e alinhar `OBJETIVO.md`/`ROADMAP.md`.
- [ ] T041 PR GitHub (CI verde), merge, sync mirror, cleanup branches.
