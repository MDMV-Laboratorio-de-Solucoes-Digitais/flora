# Feature Specification: RBAC RoleWrite 066

**Feature Branch**: `feature/066-rbac-role-write`

**Created**: 2026-11-18

**Status**: Approved (SE authorization recorded)

**Input**: "Substituir placeholders de RBAC por CRUD com RoleWrite"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Autoridade explícita (Priority: P1)

Como administrador, quero que mudanças em papéis exijam a permissão
`RoleWrite` (que `OrgAdmin` já possui), para não depender de handlers no-op.

**Independent Test**: testes unitários de `can_manage_roles`.

**Acceptance Scenarios**:

1. **Given** papel com `role:write`, **When** checado, **Then** pode gerenciar.
2. **Given** papel sem `RoleWrite`, **When** checado, **Then** é rejeitado.
3. **Given** papel builtin, **When** update/delete, **Then** é rejeitado.

## Requirements *(mandatory)*

- **FR-001..006** conforme `define.md`; tenant sempre da sessão.

## Success Criteria *(mandatory)*

- **SC-001**: testes unitários cobrem RoleWrite e builtin.
- **SC-002/003**: gates e CI verdes.
- **SC-004**: sem RLS/ADR/constituição.

## Assumptions

- `revoke_role` remove a membership do usuário na org (registrado).
- `assign_role` usa delete+create (sem update no repositório).

## Open Questions

Nenhuma; produção **[NV]**.
