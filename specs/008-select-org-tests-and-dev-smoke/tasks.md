# Tasks: 008 — Select-Organization Tests & Reproducible Dev Smoke

**Status legend:** `[X]` done, `[~]` in progress, `[ ]` pending, `[NV]` blocked/not verifiable

## Phase 0 — Research & scoping
- [X] T001 Criar scaffolding de assessment/spec 008.
- [X] T002 Audit flora-tests/DB patterns (`ses_e1e9a5437ffeDtndGc5NhfHNUB`) — incorporado (connect_lazy/oneshot, sem harness DB, casos DB-backed exigem Postgres+migrations e gating).
- [X] T003 Audit dev-seed/compose/bootstrap (`ses_e1e9a5435ffexYwl8z3bRWNTW3`) — incorporado (IDs seeded, bootstrap viável via sessão DB + JWT assinado com segredo app, sem OIDC).
- [X] T004 Audit frontend minimal alignment (`ses_e1e9a542bffe1iScHCp3nDbNe7`) — incorporado (lacunas state/callback/select-org/workspace/task; frontend slice completo deferido).
- [X] T005 Multi-language research (`ses_e1e9a5428ffesJ4ikPt7V732Nm`, `ses_e1ea41e08ffeHj1Q40A2fgILJO`) — incorporado (MVP 80/20, single-org-per-token/jti rotation, DB-backed session authority, cookie/BFF futuro, sqlx/test isolation tradeoffs).
- [X] T006 Dark-factory workflow synthesis (`ses_e1e9a5426ffeFHRHkBZGVF8s6o`) — incorporado (gates, artefatos, NV explícito).
- [X] T007 Consolidar achados e finalizar decisão/escopo.

## Phase 1 — Tests
- [X] T010 Adicionar testes CI-safe para `select-organization` (auth ausente, org nil/422 pré-DB com bearer válido).
- [~] T011 Casos dependentes de DB (sucesso/membership negada/sessão inválida-inativa-inexistente) — **não automatizados em CI** (sem serviço Postgres em `.github/workflows/rust.yml`); caminho e pré-condições documentados, execução local **NV** neste ambiente.
- [X] T012 Executar fmt/check/tests relevantes e registrar evidência.

## Phase 2 — Dev smoke
- [X] T020 Implementar/bootstrap de credencial dev reproduzível (sem OIDC real): `scripts/dev-smoke-session.sql` + tokens pré-assinados determinísticos documentados.
- [~] T021 Documentar smoke `org → workspace → task` com docker-compose+seed (`docs/dev-smoke.md`, `test-flora.sh`); execução integrada real **NV** neste ambiente.

## Phase 3 — Delivery
- [~] T030 Atualizar artefatos 008 com resultados/lacunas/evidência.
- [ ] T031 PR GitHub (CI verde), merge, sync origin; higiene de branches.
