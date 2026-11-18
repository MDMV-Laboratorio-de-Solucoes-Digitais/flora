# Feature Specification: N3 org-scoped roles 064

**Feature Branch**: `feature/064-n3-org-scoped-roles`

**Created**: 2026-11-18

**Status**: Approved (SE authorization recorded in STATE 11:13Z)

**Input**: "Multi-tenancy: evitar que membership da org A use papel da org B"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Isolamento de autorização (Priority: P1)

Como administrador de uma organização, quero que apenas papéis da minha org
possam conferir permissões nela, para que um ponteiro cross-tenant não
conceda acesso indevido.

**Why this priority**: escalada de privilégio entre tenants é latente no modelo.

**Independent Test**: buscar papel com org errada retorna `None`; FK rejeita
insert cross-org/órfão em DB limpo.

**Acceptance Scenarios**:

1. **Given** papel da org B, **When** resolvido com org A, **Then** não é encontrado.
2. **Given** DB limpo, **When** a FK é aplicada, **Then** insert cross-org falha.
3. **Given** dados sujos em produção, **When** a migration roda, **Then** falha fechado (`RAISE EXCEPTION`).

### Edge Cases

- Papéis builtin são org-scoped (`is_builtin=true` com `organization_id`) —
  o filtro não os exclui.
- Cache de papéis em `RbacService` é por id; deve validar a org no hit.
- `list_permissions`/`update_role_permissions` ganham parâmetro de org (sem
  chamadores externos no repo).
- Produção não auditada: **[NV]**; nenhum deploy.

## Requirements *(mandatory)*

- **FR-001**: buscas escopadas por org em repositório/serviços/rotas.
- **FR-002**: `UNIQUE(organization_id, id)` em `roles`.
- **FR-003**: FK composta com `ON DELETE RESTRICT` e pré-voo fail-closed.
- **FR-004**: sem deploy/limpeza/ADR/C5/C6.

## Success Criteria *(mandatory)*

- **SC-001**: cross-org rejeitado na aplicação (teste).
- **SC-002**: migrations em DB limpo; FK rejeita insert inválido.
- **SC-003**: gates verdes.
- **SC-004**: auditoria local registrada; produção **[NV]**.

## Assumptions

- Autorização SE cobre código e migrations, não deploy.
- Nenhum papel global (sem org) é legítimo — verificado no seed/código.

## Open Questions

Nenhuma; produção **[NV]**.
