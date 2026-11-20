# Feature Specification: 012 — Session Authority Revalidation & Contract Confidence (80/20)

**Feature Branch**: `feat/012-session-authority-revalidation`  
**Created**: 2026-11-16  
**Status**: Implemented (pending PR/CI/merge)  
**Assessment**: `.specify/assessments/012-session-authority-revalidation/{intake,research,problem,concept,decision}.md`

## Goal
Tornar a sessão PostgreSQL autoritativa para todo request protegido e completar o contrato de erro 401 no cliente, com testes determinísticos e evidência DB-backed.

## User Scenarios & Testing
- **US1 (P0):** após logout/revogação ou expiração da sessão, um token JWT ainda dentro do `exp` é rejeitado no próximo request protegido (401).
- **US2 (P0):** token com `jti` inexistente ou vinculado a outro usuário/organização é rejeitado (401), sem vazar o motivo.
- **US3 (P1):** a UI mantém o lock de sessão em 401 e preserva `code`/`message`/`body` estruturados para feedback.
- **US4 (P2):** sessão nil-org (pré-seleção) continua válida para autenticar e é barrada apenas por `require_org` (422), preservando o fluxo de seleção de organização.
- **US5 (P2):** checks determinísticos e testes DB em Postgres 18 passam; CI `db-tests` cobre os novos casos.

## Requirements
- **FR-001:** helper compartilhado `authenticate_bearer` valida JWT + `jti` no PostgreSQL (`is_active`, `expires_at > now`, `user_id`/`organization_id` coerentes). **Done**
- **FR-002:** `AuthExtractor`, `UserContext` e `OrgContext` usam o helper; erros 401 genéricos e 500 para indisponibilidade do banco. **Done**
- **FR-003:** `ApiClient` preserva status/corpo/código/campo/mensagem em 401 e mantém `authState.lockSession()`. **Done**
- **FR-004:** testes DB-backed para sessão válida/revogada/expirada/ausente/user mismatch/org mismatch/nil-org. **Done**
- **FR-005:** job CI `db-tests` executa `contract::auth::db_` (`--ignored --test-threads=1`). **Done**
- **FR-006:** validação/release e artefatos SDD completos, com NV explícito. **Done (local)**

## Non-goals
- WebSocket upgrade, cache de sessão, invalidação distribuída, RBAC amplo, remoção de headers de tenant, OIDC/JWKS e2e real, `apps/web`, contratos chat/arquivos/telemetry, assignee membership em `POST /tasks`.
