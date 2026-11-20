# Tasks: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

**Status legend:** `[X]` done, `[~]` in progress, `[ ]` pending, `[NV]` blocked/not verifiable

## Phase 0 — Research & scoping
- [X] T001 Scaffolding e intake 010.
- [X] T002 Probe de ambiente (`ses_e1e70c41cffe7ciYAWvFvne7bG`).
- [X] T003 Workflow/gates dark-factory (`ses_e1e70c42bffeEeB6ykdz0lHRlg`).
- [X] T004 Auditoria compose/migração `uuidv7` (`ses_e1e70c429ffelej5ra63x4mjkr`).
- [X] T005 Auditoria frontend determinístico (`ses_e1e70c421ffe8wdiTy72gocnXw`).
- [X] T006 Pesquisa multi-idioma (`ses_e1e70c41effe44U20bkhSQ83U5`).
- [X] T007 Auditoria migrações `sqlx` (`ses_e1e6f8d94ffeYNn6mu2GT06pKa`).
- [X] T008 Pesquisa polyfills `uuidv7` (`ses_e1e6f8d92ffeqp08mld5uEDEZy`).
- [X] T009 Consolidação e decisão `go`.

## Phase 1 — `uuidv7` compose/docs
- [X] T010 Atualizar `docker-compose.yml` para `postgres:18-alpine`.
- [X] T011 Atualizar `docs/dev-smoke.md` (PG18, volume/porta, consistência).

## Phase 2 — Verificação/smoke
- [X] T020 Executar smoke compatível/documentado com evidência (ajustar por conflito de porta se necessário).
- [X] T021 Ajustar/executar verificações frontend determinísticas (check/lint/test:unit server/build).

## Phase 3 — Delivery
- [X] T030 Finalizar artefatos/validação e atualizar docs de estado (OBJETIVO/ROADMAP) conforme necessário.
- [ ] T031 PR GitHub (CI verde), merge, sync `origin/dev`, manter `dev`/`main`.
