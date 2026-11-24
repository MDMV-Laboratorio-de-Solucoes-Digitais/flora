# 119 — UI CRUD de administração de papéis RBAC em `/settings`

**Data**: 2026-11-23 · **Critério 80/20**: `1280/3125` · **Issues**: `#405`..`#408` · **Domínio crítico**: `auth` / `rbac`

## O que mudou

- **Camada pura, API e store RBAC (`T-001` / `#405`)**:
  - `apps/frontend/src/lib/types/models.ts`: tipo `RoleItem`.
  - `apps/frontend/src/lib/utils/roles.ts`: `ROLE_PERMISSION_OPTIONS` (27 permissões canônicas sem `*`), `toRoleItem`, `canModifyRole`, `canDeleteRole`, `pickEditablePermissions`, `formatRolePermissions`, `validateRoleForm`.
  - `apps/frontend/src/lib/api/roles.ts`: `fetchRoles`, `createRole`, `updateRole`, `deleteRole`.
  - `apps/frontend/src/lib/state/RoleStore.svelte.ts`: store Svelte 5 com `syncContext`, `loadSequence`, deduplicação `pendingLoad`, `upsert` e `remove`.
  - `apps/frontend/tests/unit/roles.test.ts`: `9` testes unitários cobrindo helpers, validação e corrida/deduplicação da store.
- **Interface RBAC em `pt-BR` e integração em `/settings` (`T-002` / `#406`)**:
  - `apps/frontend/src/lib/components/rbac/RoleManager.svelte`: listagem de papéis nativos (`Nativo`) e personalizados (`Personalizado`), recarga manual, criação (`POST`), edição (`PATCH`), exclusão (`DELETE`), ocultação de ações em nativos, carga única por contexto sem loop infinito e tratamento explícito de `403`/`409`.
  - `apps/frontend/src/routes/layout.css`: ajuste de `:root` (`--primary: 152 76% 25%` e `--ring: 152 76% 25%`, mantendo `.dark` inalterado) para elevar o contraste de botões e badges primários no tema claro de `2.72:1` para `5.85:1` (WCAG 2.2 AA).
  - `apps/frontend/src/lib/i18n/pt-BR.ts`: chaves `roles.*` e `error.{load,create,update,delete}Role`.
  - `apps/frontend/src/routes/(app)/settings/+page.svelte`: montagem do `<RoleManager />` abaixo de `WorkspaceCreate`.
- **E2E Chromium + WCAG 2.2 AA (`T-003` / `#407`)**:
  - `apps/frontend/tests/e2e/seed-flow.e2e.ts`: limpeza inicial de resíduos `Auditor E2E %` e Passo 9 exercitando `/settings` em tema `.dark` e claro (`14/14` auditorias WCAG 2.2 AA), listagem de papéis nativos, criação (`POST 201`), persistência pós-`page.reload()`, edição (`PATCH 200`), exclusão (`DELETE 204`) e capturas visuais.
- **Governança, R4 e R7 (`T-004` / `#408`)**:
  - Briefing `auth`/`rbac`, roteiro/evidências R4, release R7 (`0.10.1 → 0.11.0` MINOR) e docs append-only.

## [NV]

Deploy externo em staging/produção no Coolify (`workflow_dispatch`).

## Rollback

Novo commit revertendo os arquivos do ciclo 119 em `apps/frontend` e SDD; zero alterações de schema SQL, zero mudanças em Rust e pisos DB mantidos em `82`.
