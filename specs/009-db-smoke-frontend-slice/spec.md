# Feature Specification: 009 — DB-backed Select-Org Tests, Integrated Smoke & Minimal Frontend Slice

**Feature Branch**: `feat/009-db-smoke-frontend-slice`  
**Created**: 2026-11-15  
**Status**: Implemented (pending PR/CI/merge)  
**Assessment**: `.specify/assessments/009-db-smoke-frontend-slice/{intake,research,decision}.md`  
**Depends on**: `specs/008-select-org-tests-and-dev-smoke/`

## Goal
1. **Cobertura DB de `select-organization`:** testes automatizados env-gated/`#[ignore]` (e job CI com Postgres) para sucesso/rotação, membership negada e sessão ausente/inativa/inconsistente.
2. **Smoke integrado reproduzível:** executar `docs/dev-smoke.md` (`select org → workspace → task`) com evidência, ou registrar **NV** detalhado.
3. **Fatia mínima frontend↔backend:** alinhar callback `state`, seleção de organização/troca de token e slice `workspace → tarefa` no `apps/frontend`, mantendo segurança/tenancy.

## User Scenarios & Testing

- **US1 (P1):** membro com sessão ativa nil-org seleciona org e recebe sessão/JWT org-scoped rotacionado; sessão anterior revogada. **(DB-backed, automatizado, passou localmente)**
- **US2 (P1):** não-membro é rejeitado (403) sem rotação indevida. **(DB-backed, automatizado)**
- **US3 (P1):** sessão ausente/inativa ou usuário inconsistente é rejeitada (401). **(DB-backed, automatizado)**
- **US4 (P2):** smoke integrado `select org → workspace → task` reproduzível e evidenciado. **(executado, rc=0 em stack alternativa pg18/ports não-conflitantes; NV compose-as-documented por 5432/pg16-`uuidv7`)**
- **US5 (P2):** usuário completa callback (state), seleciona org (token rotacionado) e opera slice workspace→tarefa no frontend. **(implementado; verificação determinística completa NV/depende de toolchain)**

## Requirements

- **FR-001:** Cobertura DB automatizada/gated para casos-chave de `select-organization`. **Done** (`flora-tests/src/contract/auth.rs`, `db_select_organization::*`, 5 testes `#[ignore]`).
- **FR-002:** CI suporta testes DB em job separado com Postgres sem regressão do job existente. **Done** (`.github/workflows/rust.yml`, job `db-tests`).
- **FR-003:** Evidência de smoke integrado ou NV detalhado. **Done (evidenciado + NV nuances)**.
- **FR-004:** Frontend mínimo alinhado a auth/tenancy/contratos do slice com verificações determinísticas conforme viável. **Implemented (partial verification NV)**.

## Non-goals
- E2E OIDC real/JWKS completo, paridade de UI, realtime completo, deploy/produção.
- Reconciliação definitiva de compatibilidade de migração `uuidv7` vs `postgres:16-alpine` do compose (registrado como follow-up).

## Evidence (summary)
- `cargo fmt --all -- --check`: ok. `cargo check -p flora-api -p flora-core -p flora-tests`: ok. `cargo test -p flora-tests --lib`: 56 passed, 5 ignored. `cargo test -p flora-core --lib`: 8 passed.
- DB-backed: `FLORA_DATABASE__POSTGRES_URL=postgresql://postgres:postgres@127.0.0.1:15432/flora cargo test -p flora-tests contract::auth::db_select_organization -- --ignored --test-threads=1` → **5 passed** (pg18).
- Smoke integrado (alt-port): `select-organization` 200 + token org-scoped, workspaces listados (Main/Design), task criada (id/status retornados); `GET /api/v1/tasks` vazio esperado (assignee-scoped, task sem assignee). **rc=0**. Falha prévia documentada: compose pg16 em migração `uuidv7()`.
