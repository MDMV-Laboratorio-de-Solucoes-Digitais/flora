# Feature Specification: Dead invite UI 051

**Feature Branch**: `feature/051-dead-invite-ui`

**Created**: 2026-11-17

**Status**: Approved (decision `go (scoped)`)

**Input**: "Remove affordance de convite sem backend"

## User Scenarios & Testing *(mandatory)*

### User Story 1 — Settings sem ação que falha (Priority: P1)

Como usuário, quero ver apenas ações que funcionam na página de configurações,
para não submeter um formulário que sempre retorna 404.

**Why this priority**: é uma falsa affordance visível em toda sessão autenticada.

**Independent Test**: `rg 'users/invite|UserManager' apps/frontend/src` → 0.

**Acceptance Scenarios**:

1. **Given** a página de settings, **When** carregada, **Then** não exibe "Invite User".
2. **Given** o código frontend, **When** varrido, **Then** não há chamada a `/api/v1/users/invite`.

### Edge Cases

- OrganizationManager e WorkspaceCreate permanecem intactos e funcionais.
- O cenário histórico em spec 002 é marcado, não apagado.

## Requirements *(mandatory)*

- **FR-001**: remover `UserManager` da página de settings.
- **FR-002**: zero referências a `users/invite` no frontend.
- **FR-003**: marcar drift em `specs/002-.../spec.md:34`.
- **FR-004**: não alterar demais componentes.

## Success Criteria *(mandatory)*

- **SC-001**: `rg ...` → 0 ocorrências.
- **SC-002**: gates frontend/Rust verdes.
- **SC-003**: diff ≤ 3 arquivos existentes + artefatos SDD.

## Assumptions

- Nenhuma rota de convite será adicionada neste ciclo.
- Nenhum teste automatizado cobre o componente removido.

## Open Questions

Nenhumas; remoto **[NV]**.
