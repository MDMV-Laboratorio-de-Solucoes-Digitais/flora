# Tasks: 011 — Frontend↔API Contract Alignment & Session Hardening (80/20)

**Status legend:** `[X]` done, `[~]` in progress, `[ ]` pending, `[NV]` blocked/not verifiable

## Phase 0 — Assessment & scope
- [X] T001 Scaffolding 011 e intake/research/problem/concept/decision (go) com base em auditorias/pesquisa.
- [X] T002 Consolidar achados de subagentes e definir escopo 80/20 executável.

## Phase 1 — Implementação
- [X] T010 Backend: `create_task` persiste `description` e `assignee_id`.
- [X] T011 Frontend: `ApiClient` com erro tipado (status/corpo) preservando lock em 401.
- [X] T012 Frontend: `AuthState.setOrganization` limpa `isInvalidated`.
- [X] T013 Frontend: guard auth/org em `/tasks` e alinhamento de tratamento de erro no slice.
- [X] T014 Testes frontend: atualizar `AuthState` e `api-interceptor` (determinísticos/CI-equivalentes).
- [X] T015 Docs: exemplo concreto de override de portas em `docs/dev-smoke.md`.

## Phase 2 — Verificação & SDD
- [X] T020 Executar verificações Rust e frontend (fmt/check/clippy/testes, check/lint/test:unit/build).
- [X] T021 Finalizar spec/plan/tasks/checklist, validação e release.
- [~] T022 PR para `dev`, CI verde, merge e sync `origin/dev` (manter `dev`/`main`).
