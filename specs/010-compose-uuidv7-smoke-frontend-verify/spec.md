# Feature Specification: 010 — Compose/Migration `uuidv7` Compatibility, Documented Smoke & Frontend Verification

**Feature Branch**: `feat/010-compose-uuidv7-smoke-frontend-verify`  
**Created**: 2026-11-15  
**Status**: Draft (in progress)  
**Assessment**: `.specify/assessments/010-compose-uuidv7-smoke-frontend-verify/{intake,research,decision}.md`  
**Depends on**: `specs/009-db-smoke-frontend-slice/`

## Goal
1. **Compatibilidade `uuidv7` no caminho documentado:** alinhar compose/docs para PostgreSQL 18 (nativo `uuidv7`) sem alterar migrações `sqlx` aplicadas, tornando o smoke dev reproduzível em fresh installs.
2. **Smoke integrado evidenciado:** executar `docs/dev-smoke.md` (`select org → workspace → task`) e registrar evidência; se porta padrão conflitar por ambiente, evidenciar caminho equivalente isolado com NV explícito.
3. **Verificação determinística frontend:** executar/ajustar checks mínimos do `apps/frontend` (check/lint/test:unit server/build) para o slice atual, com correções pequenas in-scope.

## User Scenarios & Testing
- **US1 (P1):** desenvolvedor sobe dependências pelo compose documentado (PG18) e aplica migrações sem erro de `uuidv7()`.
- **US2 (P1):** smoke `select org → workspace → task` completa com evidência (rc=0) em stack compatível.
- **US3 (P2):** checks determinísticos frontend relevantes passam (ou falhas/NV são registradas com causa).

## Requirements
- **FR-001:** `docker-compose.yml` usa imagem PostgreSQL compatível com `uuidv7()` (PG18). **Done (proposed)**
- **FR-002:** `docs/dev-smoke.md` documenta requisito PG18, notas de volume/porta e fluxo executável. **Done (proposed)**
- **FR-003:** Smoke integrado executado com evidência (documentado ou equivalente isolado com NV ambiental). **Done (equivalent isolated path, rc=0)**
- **FR-004:** Verificação determinística frontend executada com evidência; ajustes mínimos para viabilizar checks in-scope. **Done (local checks green)**

## Non-goals
- Edição de migração aplicada, polyfill PG16 para init, OIDC/JWKS e2e real, mudanças amplas de produto/UX.
