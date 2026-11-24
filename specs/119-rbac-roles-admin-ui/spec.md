# Feature Specification: UI CRUD de administração de papéis RBAC em `/settings`

**Feature ID**: 119 · **Status**: specified · **Data**: 2026-11-23

## User Scenario

Como administrador autenticado de uma organização no Flora, quero visualizar os papéis nativos e personalizados da minha organização em `/settings`, criar novos papéis selecionando permissões canônicas, editar papéis personalizados e excluí-los com feedback claro em pt-BR e acessibilidade WCAG 2.2 AA — sem jamais poder alterar papéis nativos ou afetar outras organizações.

## Acceptance Scenarios

1. **SC-001 (Tipos, utilitários e `RoleStore` com isolamento por contexto)**: **Given** dados de API e alternância rápida de organização/sessão, **when** `toRoleItem`, `canModifyRole`, `formatRolePermissions`, `validateRoleForm` e `RoleStore` processam requisições, **then** filtram permissões inválidas, bloqueiam mutações em papéis `isBuiltin`, deduplicam cargas simultâneas no mesmo contexto e descartam respostas obsoletas após `syncContext`.
2. **SC-002 (Contrato HTTP completo `GET/POST/PATCH/DELETE /api/v1/roles`)**: **Given** cliente autenticado via Bearer JWT com organização selecionada, **when** `RoleManager.svelte` carrega ou envia formulários em `/settings`, **then** lista os papéis da organização (`GET`), cria papéis personalizados (`POST` → `201`), edita papéis personalizados (`PATCH` → `200`), exclui papéis personalizados (`DELETE` → `204`), mapeia `403` e `409` em `pt-BR` sem loop de `$effect` em respostas vazias ou erro e preserva papéis nativos somente leitura.
3. **SC-003 (Acessibilidade WCAG 2.2 AA nos temas escuro e claro)**: **Given** tema escuro (`.dark`) e tema claro em `/settings`, **when** Playwright + Axe auditam a tela `/settings` durante o ciclo completo CRUD, **then** não há violações WCAG 2.2 AA (incluindo WCAG 2.5.3 em botões `Editar`/`Excluir`), persistência sobrevive a `page.reload()` e capturas comprovam ambos os temas.
4. **SC-004 (Gates locais, Briefing `auth`/`rbac`, R4/R7 e ACR)**: **Given** trabalho concluído em `feature/119-rbac-roles-admin-ui`, **when** verificados `check`, `lint`, `test:unit`, `build`, `check_gate_integrity.py`, `test:e2e` e ACR independente, **then** todas as verificações passam sem regressão no gate DB (`82` pisos), R4 (`evidence.md` + `validate.md`), R7 (`0.11.0` MINOR) e Briefing `auth`/`rbac` estão completos.

## Requirements

- **FR-001**: Definir `RoleItem` (`models.ts`), utilitários puros (`utils/roles.ts`), funções API (`api/roles.ts`: `fetchRoles`, `createRole`, `updateRole`, `deleteRole`), `RoleStore.svelte.ts` (`syncContext`, `loadSequence`, `pendingLoad`, `upsert`, `remove`) e testes unitários (`tests/unit/roles.test.ts`).
- **FR-002**: Implementar `RoleManager.svelte` com listagem, recarga manual, criação, edição (`PATCH`), exclusão (`DELETE`), ocultação de mutações em papéis nativos, prevenção de loop infinito em `$effect`, chaves `pt-BR` em `pt-BR.ts` e integração em `routes/(app)/settings/+page.svelte`.
- **FR-003**: Expandir `apps/frontend/tests/e2e/seed-flow.e2e.ts` (Passo 9) para exercitar `/settings` em `.dark` e claro com auditoria Axe WCAG 2.2 AA, criação (`POST 201`), `page.reload()`, edição (`PATCH 200`), exclusão (`DELETE 204`) e captura de evidências visuais.
- **FR-004**: Produzir Briefing de Compreensão (`auth`/`rbac`), issues R6, roteiro e evidências R4, release R7 (`0.11.0` MINOR), atualização append-only da documentação e ACR independente.

## [NV]

Deploy externo em staging/produção (Coolify) e implementação de convites/atribuição por membro em interface web seguem fora deste ciclo.
