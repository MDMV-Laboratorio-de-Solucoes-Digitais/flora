# Feature Specification: Real RBAC contract tests — 070

**Feature Branch**: `feature/070-rbac-contract`
**Created**: 2026-11-19
**Status**: Approved (`decision.md` go scoped)
**Input**: "Replace the vacuous `Ok(())` contract with DB-gated authorization/tenancy assertions"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Tenant-scoped role listing (Priority: P1)

Como maintainer, quero que o contrato prove que `GET /roles` retorna apenas papéis da organização da sessão, para detectar regressão de isolamento multi-tenant.

**Why this priority**: tenancy vazando é defeito de segurança; o stub atual não detecta nada.

**Independent Test**: executar `db_rbac_list_scoped_to_session_org` com Postgres descartável; deve ver o papel próprio e não o da outra org.

### User Story 2 — RoleWrite gate on creation (Priority: P1)

Como maintainer, quero prova de que criar papel exige `RoleWrite`/`OrgAdmin` e grava no `organization_id` da sessão.

**Acceptance Scenarios**: admin recebe 201 com o `organization_id` correto; membro com apenas `org:read` recebe 403.

### User Story 3 — Built-in immutability (Priority: P2)

Como maintainer, quero prova de que papéis built-in não podem ser alterados por PATCH, enquanto papéis customizados podem.

## Requirements

- **FR-001** módulo `db` fail-closed sem env; **FR-002** list scoped; **FR-003** create gate; **FR-004** built-in imutável; **FR-005** rastreabilidade.
- **SC-001** 3 contratos passam com DB real; sem env → falha fechada; **SC-002** gates do alvo verdes sem alterar pisos; **SC-003** diff restrito aos paths declarados.

## Scope / Out of scope

Dentro: 1 arquivo de teste + artefatos. Fora: handlers, migrations, config, workflows, ADRs, constituição, assign/revoke, produção.
